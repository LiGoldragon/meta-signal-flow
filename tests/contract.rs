#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_flow::{Query, Response};
use protos::{Compactable, Protosizable, ReaderBudget};

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
        "Configure.Nexus.{ /home/li/primary { /etc/profiles/per-user/li/bin/codex-stable-flow-client /home/li/.codex /home/li/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra gpt-5.6-sol gpt-5.6-luna ] } { /etc/profiles/per-user/li/bin/codex-next-flow-client /home/li/.codex-next /home/li/.codex-next/app-server-control/app-server-control.sock [ gpt-6-sol gpt-6-luna gpt-6-astra ] } [ { Claude [ / «!» # ] [ esc esc ] [ enter ] } { Codex [ / «!» ] [ esc ] [] } ] [ Psyche ] /run/user/1001/message-nexus.sock /etc/profiles/per-user/li/bin/message-nexus 60 }",
        "Configuration",
        "ConsumeReset.{ attempt-1 Next }",
        "ConsumeReset.{ attempt-2 Specific.credit-7 }",
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable { da1e3f claude-session unavailable } Pending }",
        "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } [ { psyche-flow Psyche High claude-opus Claude psyche-native workspace wD:p9 wD:t9 psyche-terminal psyche-agent { 4200 1001 psyche-token } /home/li/primary } { mind-flow Mind High gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } { field-flow Field Medium gpt-6 Codex field-native workspace wQ:pF wQ:tF field-terminal field-agent { 4400 1001 field-token } /home/li/primary } ] }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn bind_existing_result_preserves_each_explicit_outcome() {
    let text = "BoundExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } [ Bound.{ psyche-flow RegisteredUnconfirmed } Refused.{ mind-flow AmbiguousPane } Refused.{ field-flow DeadProcess } ] }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn reset_outcome_round_trips_over_signal_and_datom() {
    let reply = Response::ResetConsumed(meta_signal_flow::ResetOutcome::Reset);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    let text = reply.datomize(vec![]).protosize().compact();
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(restored, reply);
}

#[test]
// These are Signal wire round-trips, not claims about configured store state.
fn configure_ack_is_unit_and_configuration_read_carries_the_snapshot() {
    let configured = Potential::<Response>::from("Configured")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(configured, Response::Configured);
    assert_eq!(
        configured.datomize(vec![]).protosize().compact(),
        "Configured"
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&configured).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        configured
    );

    let text = "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/app-server-control/app-server-control.sock [ gpt-6.1 ] } [ { Claude [ / «!» # ] [ esc esc ] [ enter ] } ] [ Psyche Mind ] /srv/message-nexus.sock /usr/bin/message-nexus 60 } [ { Secondary claude-opus } ] [ { Primary 300 900 } ] [ { Compensation compensationBookDistillation { meta-signal-flow 0000000000000000000000000000000000000000000000000000000000000000 /git/github.com/LiGoldragon/meta-signal-flow } } ] }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Configuration(configuration) = &response else {
        panic!("expected Configuration snapshot");
    };
    assert_eq!(configuration.nexus.source_root, "/srv/source");
    assert_eq!(
        configuration.nexus.stable_codex.client_path,
        "codex-stable-flow-client"
    );
    assert_eq!(configuration.nexus.stable_codex.home, "/home/someone/.codex");
    assert_eq!(
        configuration.nexus.stable_codex.control_socket_path,
        "/home/someone/.codex/app-server-control/app-server-control.sock"
    );
    assert_eq!(
        configuration.nexus.stable_codex.model_name_vector,
        vec!["gpt-5.6-terra".to_owned()]
    );
    assert_eq!(
        configuration.nexus.next_codex.model_name_vector,
        vec!["gpt-6.1".to_owned()]
    );
    assert_eq!(configuration.nexus.harness_profiles.len(), 1);
    assert_eq!(
        configuration.nexus.harness_profiles[0].harness_kind,
        meta_signal_flow::HarnessKind::Claude
    );
    assert_eq!(
        configuration.nexus.harness_profiles[0].command_sigil_vector,
        vec!["/".to_owned(), "!".to_owned(), "#".to_owned()]
    );
    assert_eq!(
        configuration.nexus.harness_profiles[0].interrupt_keys,
        vec!["esc".to_owned(), "esc".to_owned()]
    );
    assert_eq!(
        configuration.nexus.harness_profiles[0].submit_keys,
        vec!["enter".to_owned()]
    );
    assert_eq!(
        configuration.nexus.meta_aspects.as_slice(),
        &[signal_flow::FlowAspect::Psyche, signal_flow::FlowAspect::Mind]
    );
    assert_eq!(
        configuration.nexus.message_nexus_path,
        "/srv/message-nexus.sock"
    );
    assert_eq!(
        configuration.nexus.message_nexus_binary,
        "/usr/bin/message-nexus"
    );
    assert_eq!(configuration.nexus.lease, 60);
    assert_eq!(configuration.models.len(), 1);
    assert_eq!(
        configuration.models[0].layer,
        meta_signal_flow::Layer::Secondary
    );
    assert_eq!(configuration.models[0].native, "claude-opus");
    assert_eq!(configuration.thresholds.len(), 1);
    assert_eq!(
        configuration.thresholds[0].layer,
        meta_signal_flow::Layer::Primary
    );
    assert_eq!(configuration.thresholds[0].handover, 300);
    assert_eq!(configuration.thresholds[0].refresh, 900);
    assert_eq!(configuration.modules.len(), 1);
    assert_eq!(
        configuration.modules[0].subaspect,
        meta_signal_flow::Subaspect::Compensation
    );
    assert_eq!(
        configuration.modules[0].topic,
        "compensationBookDistillation"
    );
    assert_eq!(
        configuration.modules[0].source.repository,
        "meta-signal-flow"
    );
    assert_eq!(
        configuration.modules[0].source.hash,
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        configuration.modules[0].source.path,
        "/git/github.com/LiGoldragon/meta-signal-flow"
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);

    let unconfigured = Potential::<Response>::from("Unconfigured")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(unconfigured, Response::Unconfigured);
    assert_eq!(
        unconfigured.datomize(vec![]).protosize().compact(),
        "Unconfigured"
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&unconfigured).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        unconfigured
    );
}

#[test]
fn configure_missing_a_required_nexus_field_is_refused_by_the_reader() {
    let text = "Configure.Nexus.{ /srv/source { stable-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { next-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [] } [] [ Psyche ] /srv/message-nexus.sock /usr/bin/message-nexus }";
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
    assert_eq!(query.datomize(vec![]).protosize().compact(), request);

    for text in [
        "FlowRetired.{ d8df70 claude-session Claude Unavailable Unavailable { 88475f field-session turn-4 } Retired }",
        "RetireRejected.UnknownFlow",
        "RetireRejected.AlreadyGone",
        "RetireRejected.StoreRefused",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
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
        "Deliver.{ m-7f3a2c:7d41e0:0 7d41e0 Soft.{ m-7f3a2c Flow.e167d8 Text.«Stage 1 is deployed; run the tier tests.» } }",
        "Deliver.{ m-81b0e4:7d41e0:0 7d41e0 HardAbrupt.{ m-81b0e4 Owner Text.«Stop the ouranos build now.» } }",
        "Deliver.{ m-90c1aa:7d41e0:0 7d41e0 MiddleAbrupt.{ m-90c1aa Flow.88475f Psyche.{ «on build hosts» «Prometheus should be doing the builds.» } } }",
        "Vet.{ m-a2d913:7d41e0:0 7d41e0 Soft.{ m-a2d913 Flow.e167d8 Text./compact } }",
        "Command.{ 7d41e0 Compact }",
        "Command.{ 7d41e0 Interrupt }",
        "ResolvePeer.{ 48211 1001 8841220 }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn delivery_replies_have_concrete_datoms() {
    for text in [
        "Delivered.{ m-7f3a2c:7d41e0:0 7d41e0 NotRequested Presented }",
        "Delivered.{ m-81b0e4:7d41e0:0 7d41e0 Observed Transported }",
        "Delivered.{ m-81b0e5:7d41e0:1 7d41e0 Unobserved Uncertain }",
        "DeliveryRejected.RecipientBlocked",
        "DeliveryRejected.RecipientWorking",
        "DeliveryRejected.ComposerOccupied",
        "DeliveryRejected.FlowStopped",
        "DeliveryRejected.FlowRetired",
        "DeliveryRejected.FlowExited",
        "DeliveryRejected.BodyRefused.EmptyBody",
        "DeliveryRejected.BodyRefused.HarnessCommand./compact",
        "DeliveryRejected.BodyRefused.ControlCharacter.14",
        "Vetted.7d41e0",
        "Commanded.{ 7d41e0 Compact Transported }",
        "CommandRejected.UnsupportedForHarness",
        "CommandRejected.FlowRetired",
        "CommandRejected.FlowExited",
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
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn a_message_textualizes_with_its_priority_head_first() {
    let message = meta_signal_flow::Message::Soft(meta_signal_flow::Letter {
        message_id: "m-7f3a2c".into(),
        sender: meta_signal_flow::Sender::Flow("e167d8".into()),
        content: meta_signal_flow::Content::Text("Stage 1 is deployed; run the tier tests.".into()),
    });
    assert_eq!(
        message.datomize(vec![]).protosize().compact(),
        "Soft.{ m-7f3a2c Flow.e167d8 Text.«Stage 1 is deployed; run the tier tests.» }"
    );
}
