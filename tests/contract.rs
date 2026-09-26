#![cfg(feature = "datom")]

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
        "Configure.{ /run/user/1001/flow/flow.sock /run/user/1001/flow/flow-meta.sock /home/li/primary { /etc/profiles/per-user/li/bin/codex-stable-flow-client /home/li/.codex /home/li/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra gpt-5.6-sol gpt-5.6-luna ] } { /etc/profiles/per-user/li/bin/codex-next-flow-client /home/li/.codex-next /home/li/.codex-next/app-server-control/app-server-control.sock [ gpt-6-sol gpt-6-luna gpt-6-astra ] } [ { Claude [ / «!» # ] [ esc esc ] [ enter ] } { Codex [ / «!» ] [ esc ] [] } ] [ Psyche ] /etc/profiles/per-user/li/bin/message-nexus }",
        "ConsumeReset.{ attempt-1 Next }",
        "ConsumeReset.{ attempt-2 Specific.credit-7 }",
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Pending }",
        "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } [ { psyche-flow Psyche High claude-opus Claude psyche-native workspace wD:p9 wD:t9 psyche-terminal psyche-agent { 4200 1001 psyche-token } /home/li/primary } { mind-flow Mind High gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } { field-flow Field Medium gpt-6 Codex field-native workspace wQ:pF wQ:tF field-terminal field-agent { 4400 1001 field-token } /home/li/primary } ] }",
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

#[test]
fn configured_carries_source_root_and_both_codex_endpoints() {
    let text = "Configured.{ { /run/user/1001/flow/flow.sock /run/user/1001/flow/flow-meta.sock /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/app-server-control/app-server-control.sock [] } [] [ Psyche ] /srv/message-nexus } NexusRestartRequired }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Configured(configured) = &response else {
        panic!("expected Configured");
    };
    let configuration = &configured.configuration;
    assert_eq!(configuration.source_root, "/srv/source");
    assert_eq!(
        configuration.stable_codex.client_path,
        "codex-stable-flow-client"
    );
    assert_eq!(configuration.stable_codex.home, "/home/someone/.codex");
    assert_eq!(
        configuration.stable_codex.control_socket_path,
        "/home/someone/.codex/app-server-control/app-server-control.sock"
    );
    assert_eq!(
        configuration.stable_codex.model_name_vector,
        vec!["gpt-5.6-terra".to_owned()]
    );
    assert!(configuration.next_codex.model_name_vector.is_empty());
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
    assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
}

#[test]
fn configure_missing_an_endpoint_is_refused_by_the_reader() {
    let text = "Configure.{ /run/user/1001/flow/flow.sock /run/user/1001/flow/flow-meta.sock /srv/source }";
    assert!(
        Potential::<Query>::from(text)
            .actualize(&mut budget())
            .is_err()
    );
}

/// Retire is the privileged way to take a seat out of Flow without pretending
/// Flow closed it. The reply carries the whole row, so the caller sees the
/// history that was kept rather than an acknowledgement that it is gone.
#[test]
fn retire_names_one_flow_and_answers_with_the_row_it_kept() {
    let request = "Retire.d8df70";
    let query = Potential::<Query>::from(request)
        .actualize(&mut budget())
        .unwrap();
    assert!(matches!(&query, Query::Retire(flow_id) if flow_id == "d8df70"));
    assert_eq!(query.datomize(vec![]).protosize().textualize(), request);

    for text in [
        "FlowRetired.{ d8df70 claude-session Claude Unavailable Unavailable { 88475f field-session turn-4 } Retired }",
        "RetireRejected.UnknownFlow",
        "RetireRejected.AlreadyGone",
        "RetireRejected.StoreRefused",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
}

#[test]
fn delivery_requests_have_concrete_datoms() {
    for text in [
        "Deliver.{ m-7f3a2c 7d41e0 Soft.{ Flow.e167d8 Text.«Stage 1 is deployed; run the tier tests.» } }",
        "Deliver.{ m-81b0e4 7d41e0 HardAbrupt.{ Owner Text.«Stop the ouranos build now.» } }",
        "Deliver.{ m-90c1aa 7d41e0 MiddleAbrupt.{ Flow.88475f Psyche.{ «on build hosts» «Prometheus should be doing the builds.» } } }",
        "Vet.{ m-a2d913 7d41e0 Soft.{ Flow.e167d8 Text./compact } }",
        "Command.{ 7d41e0 Compact }",
        "Command.{ 7d41e0 Interrupt }",
        "ResolvePeer.{ 48211 1001 8841220 }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn delivery_replies_have_concrete_datoms() {
    for text in [
        "Delivered.{ m-7f3a2c 7d41e0 NotRequested Presented }",
        "Delivered.{ m-81b0e4 7d41e0 Observed Transported }",
        "Delivered.{ m-81b0e5 7d41e0 Unobserved Uncertain }",
        "DeliveryRejected.RecipientBlocked",
        "DeliveryRejected.RecipientWorking",
        "DeliveryRejected.ComposerOccupied",
        "DeliveryRejected.BodyRefused.EmptyBody",
        "DeliveryRejected.BodyRefused.HarnessCommand./compact",
        "DeliveryRejected.BodyRefused.ControlCharacter.14",
        "Vetted.7d41e0",
        "Commanded.{ 7d41e0 Compact Transported }",
        "CommandRejected.UnsupportedForHarness",
        "PeerResolved.{ e167d8 Psyche High claude-opus-5-5 }",
        "PeerResolutionRejected.CallerUnknown",
        "MetaRefused.PeerNotAuthorized.{ da88cf Field Medium gpt-5.5 }",
        "MetaRefused.PeerUnknown",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn a_message_textualizes_with_its_priority_head_first() {
    let message = meta_signal_flow::Message::Soft(meta_signal_flow::Letter {
        sender: meta_signal_flow::Sender::Flow("e167d8".into()),
        content: meta_signal_flow::Content::Text(
            "Stage 1 is deployed; run the tier tests.".into(),
        ),
    });
    assert_eq!(
        message.datomize(vec![]).protosize().textualize(),
        "Soft.{ Flow.e167d8 Text.«Stage 1 is deployed; run the tier tests.» }"
    );
}
