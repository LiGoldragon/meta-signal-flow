use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_flow::{Query, Response};
use protos::{Protosizable, ReaderBudget, Textualizable};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn privileged_requests_have_concrete_datoms() {
    for text in [
        "Configure.{ /run/user/1001/flow/flow.sock /run/user/1001/flow/flow-meta.sock }",
        "ConsumeReset.{ attempt-1 Next }",
        "ConsumeReset.{ attempt-2 Specific.credit-7 }",
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Pending }",
        "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } [ { psyche-flow Psyche High claude-opus Claude psyche-native workspace wD:p9 wD:t9 psyche-terminal psyche-agent { 4200 1001 psyche-token } /home/li/primary } { mind-flow Mind High gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } { field-flow Field Medium gpt-6 Codex field-native workspace wQ:pF wQ:tF field-terminal field-agent { 4400 1001 field-token } /home/li/primary } ] }",
        "MetaConfirmExisting.{ mind-flow mind-native mind-terminal }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn bind_existing_result_preserves_each_explicit_outcome() {
    let text = "BoundExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } [ Bound.{ psyche-flow RegisteredUnconfirmed } Refused.{ mind-flow AmbiguousPane } Refused.{ field-flow DeadProcess } ] }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
}

#[test]
fn confirm_existing_result_and_refusal_round_trip() {
    for text in [
        "ConfirmedExisting.{ mind-flow mind-native }",
        "ConfirmExistingRejected.PromptDigestMismatch",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn reset_outcome_round_trips_over_signal_and_datom() {
    let reply = Response::ResetConsumed(meta_signal_flow::ResetOutcome::Reset);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    let text = reply.datomize(vec![]).protosize().textualize();
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(restored, reply);
}
