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
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Active }",
        "SubmitBindingRegistration.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Active registration-1 Bootstrap readiness-1 digest-1 }",
        "SubmitBindingRegistration.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Active registration-2 Refresh.transition-1 readiness-2 digest-2 }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn binding_registration_rejection_round_trips_without_a_trusted_binding_payload() {
    let reply = Response::BindingRegistrationRejected(
        meta_signal_flow::BindingRegistrationRejection::VerifierUnavailable,
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    assert_eq!(
        reply.datomize(vec![]).protosize().textualize(),
        "BindingRegistrationRejected.VerifierUnavailable"
    );
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
