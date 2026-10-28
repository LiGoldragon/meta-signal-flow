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
        "Configuration",
        "ConsumeReset.{ attempt-1 Next }",
        "ConsumeReset.{ attempt-2 Specific.credit-7 }",
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable Unbound Pending }",
        "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } { Mind message Primary } { mind-flow Mind High gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|_| panic!("not a Query Datom: {text}"));
        if text == "Configuration" {
            assert_eq!(query, Query::Configuration, "wrong unit query variant");
        }
        assert_eq!(
            query.datomize(vec![]).protosize().compact(),
            text,
            "Query Datom did not round-trip: {text}"
        );
    }
}

#[test]
fn bind_existing_result_preserves_each_explicit_outcome() {
    let text = "BoundExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } Bound.{ psyche-flow RegisteredUnconfirmed } }";
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
fn configure_ack_names_its_outcome_and_configuration_read_carries_the_snapshot() {
    for (text, expected) in [
        (
            "Configured.Replaced",
            Response::Configured(meta_signal_flow::ConfigureOutcome::Replaced),
        ),
        (
            "Configured.Unchanged",
            Response::Configured(meta_signal_flow::ConfigureOutcome::Unchanged),
        ),
    ] {
        let configured = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(configured, expected);
        assert_eq!(configured.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&configured).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            configured
        );
    }

    let text = "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/app-server-control/app-server-control.sock [ gpt-6.1 ] } [ { Claude [ / «!» # ] [ esc esc ] [ enter ] } ] [ Psyche Mind ] /srv/message-nexus.sock /usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 } [ { Secondary Psyche claude-opus } { Secondary Mind gpt-6.1 } ] [ { claude-opus-5-5 200000 } ] [ { { Field flow Quaternary } Claude [ SkillTool WorkflowTool ] } ] [ { /nix/store/codex/bin/codex /nix/store/send/bin/flow-send-mcp } ] [ { { Mind core Secondary } [ spirit:core ] [ vision:flow ] Regular Default Maintained «Keeps Flow running.» [ flows/a68420/vision/flow.md ] } ] Some.{ Psyche compaction Tertiary } }";
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
    assert_eq!(
        configuration.nexus.stable_codex.home,
        "/home/someone/.codex"
    );
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
        &[signal_flow::Aspect::Psyche, signal_flow::Aspect::Mind]
    );
    assert_eq!(
        configuration.nexus.message_nexus_path,
        "/srv/message-nexus.sock"
    );
    assert_eq!(
        configuration.nexus.message_nexus_binary,
        "/usr/bin/message-nexus"
    );
    assert_eq!(
        configuration.nexus.curriculum_nexus_path,
        "/srv/curriculum/curriculum.sock"
    );
    assert_eq!(configuration.nexus.lease, 60);
    // One layer holds a model per aspect.
    assert_eq!(
        configuration.models,
        vec![
            meta_signal_flow::ConfigureModel {
                layer: meta_signal_flow::Layer::Secondary,
                aspect: meta_signal_flow::Aspect::Psyche,
                native: "claude-opus".into(),
            },
            meta_signal_flow::ConfigureModel {
                layer: meta_signal_flow::Layer::Secondary,
                aspect: meta_signal_flow::Aspect::Mind,
                native: "gpt-6.1".into(),
            },
        ]
    );
    // A model that names no aspect is no longer a declaration.
    assert!(
        Potential::<Query>::from("Configure.Models.[ { Secondary claude-opus-5-5 } ]")
            .actualize(&mut budget())
            .is_err()
    );
    assert_eq!(configuration.thresholds.len(), 1);
    assert_eq!(configuration.thresholds[0].model_name, "claude-opus-5-5");
    assert_eq!(configuration.thresholds[0].refresh, 200000);
    assert_eq!(
        configuration
            .investigation
            .as_ref()
            .map(|address| address.topic.as_str()),
        Some("compaction")
    );
    assert_eq!(configuration.default_leave_outs.len(), 1);
    assert_eq!(
        configuration.default_leave_outs[0].harness_kind,
        meta_signal_flow::HarnessKind::Claude
    );
    assert_eq!(
        configuration.default_leave_outs[0].leave_outs,
        [
            signal_flow::LeaveOut::SkillTool,
            signal_flow::LeaveOut::WorkflowTool
        ]
    );
    assert_eq!(
        configuration.send_servers,
        vec![meta_signal_flow::ConfigureSendServer {
            codex_app_server_binary: "/nix/store/codex/bin/codex".into(),
            mcp_send_server_binary: "/nix/store/send/bin/flow-send-mcp".into(),
        }]
    );
    assert_eq!(
        configuration.metaflows,
        vec![meta_signal_flow::ConfigureMetaflow {
            address: signal_flow::Address {
                aspect: meta_signal_flow::Aspect::Mind,
                topic: "core".into(),
                layer: meta_signal_flow::Layer::Secondary,
            },
            system_modules: vec!["spirit:core".into()],
            user_modules: vec!["vision:flow".into()],
            flow_kind: signal_flow::FlowKind::Regular,
            tool_set: signal_flow::ToolSet::Default,
            continuity: meta_signal_flow::Continuity::Maintained,
            purpose: "Keeps Flow running.".into(),
            records: vec!["flows/a68420/vision/flow.md".into()],
        }]
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
    let text = "Configure.Nexus.{ /srv/source { stable-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { next-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [] } [] [ Psyche ] /srv/message-nexus.sock /usr/bin/message-nexus /srv/curriculum/curriculum.sock }";
    assert!(
        Potential::<Query>::from(text)
            .actualize(&mut budget())
            .is_err()
    );
}

#[test]
fn store_refusal_preserves_its_error_text() {
    let text = "Refused.Store.«database rejected configuration»";
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
        "FlowRetired.{ d8df70 claude-session Claude Unavailable Unavailable Flow.{ 88475f field-session } Retired }",
        "RetireRejected.UnknownFlow",
        "RetireRejected.AlreadyGone",
        "RetireRejected.StoreRefused",
        "RetireRejected.CloseRefused",
        "RetireRejected.Maintained",
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
        "DeliveryRejected.PastThreshold",
        "DeliveryRejected.BodyRefused.EmptyBody",
        "DeliveryRejected.BodyRefused.HarnessCommand./compact",
        "DeliveryRejected.BodyRefused.ControlCharacter.14",
        "Vetted.7d41e0",
        "Commanded.{ 7d41e0 Compact Transported }",
        "CommandRejected.UnsupportedForHarness",
        "CommandRejected.FlowRetired",
        "CommandRejected.FlowExited",
        "CommandRejected.CompactionRefused.BelowThreshold.{ 120000 200000 }",
        "CommandRejected.CompactionRefused.Unmeasured",
        "CommandRejected.CompactionRefused.NoThreshold",
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

#[test]
fn configure_carries_one_whole_configuration() {
    let text = "Configure.{ { /srv/source { codex-stable /home/stable /run/stable.sock [ gpt-stable ] } { codex-next /home/next /run/next.sock [ gpt-next ] } [ { Claude [ / ] [ esc esc ] [ enter ] } ] [ Psyche Mind ] /run/message.sock /bin/message /run/curriculum.sock 60 } [ { Secondary Psyche claude-opus } { Secondary Mind gpt-sol } ] [ { claude-opus 200000 } ] [ { { Field flow Quaternary } Claude [ SkillTool ] } ] [ { /bin/codex /bin/send } ] [ { { Mind core Secondary } [ spirit:core ] [ vision:flow ] Regular Default Maintained purpose [] } ] Some.{ Psyche compaction Tertiary } }";
    let query = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap_or_else(|_| panic!("not a Query Datom: {text}"));
    let Query::Configure(configuration) = &query else {
        panic!("not a complete Configuration")
    };
    assert_eq!(configuration.nexus.source_root, "/srv/source");
    assert_eq!(configuration.models.len(), 2);
    assert_eq!(configuration.thresholds.len(), 1);
    assert_eq!(configuration.default_leave_outs.len(), 1);
    assert_eq!(configuration.send_servers.len(), 1);
    assert_eq!(configuration.metaflows.len(), 1);
    assert!(configuration.investigation.is_some());
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );

    let without_investigation = text.replace("Some.{ Psyche compaction Tertiary } }", "None }");
    let Query::Configure(without_investigation) = Potential::<Query>::from(without_investigation)
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("not a complete Configuration")
    };
    assert!(without_investigation.investigation.is_none());

    // A family patch is no longer a Query: Configure replaces one complete
    // configuration value or nothing.
    for obsolete in [
        "Configure.Models.[ { Quaternary Psyche claude-haiku } ]",
        "Configure.Thresholds.[]",
        "Configure.LeaveOuts.[]",
        "Configure.Nexus.{ /srv }",
        "Configure.SendServers.[]",
        "Configure.Metaflows.[]",
        "Configure.Investigation.{ Psyche compaction Tertiary }",
    ] {
        assert!(
            Potential::<Query>::from(obsolete)
                .actualize(&mut budget())
                .is_err()
        );
    }

    for reply in [
        "ConfigureRejected.DuplicateEntry",
        "ConfigureRejected.CodexModelsOverlap",
        "ConfigureRejected.StoreRefused",
    ] {
        let response = Potential::<Response>::from(reply)
            .actualize(&mut budget())
            .unwrap_or_else(|_| panic!("not a Response Datom: {reply}"));
        assert_eq!(response.datomize(vec![]).protosize().compact(), reply);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
}

#[test]
fn a_leave_out_default_names_its_harness_and_a_foreign_leave_out_is_its_own_rejection() {
    // An entry without its harness is no longer a ConfigureLeaveOut.
    assert!(
        Potential::<Query>::from(
            "Configure.LeaveOuts.[ { { Psyche flow Secondary } [ SkillTool ClaudeMd ] } ]"
        )
        .actualize(&mut budget())
        .is_err()
    );
    let text = "ConfigureRejected.LeaveOutNotForHarness";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::ConfigureRejected(meta_signal_flow::ConfigureRejection::LeaveOutNotForHarness)
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
}

/// A configuration entry whose topic is not camelCase is refused by type,
/// carrying the topic as written.
#[test]
fn configure_topic_refusal_round_trips() {
    let text = "ConfigureRejected.TopicRefused.Flow_Repair";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::ConfigureRejected(meta_signal_flow::ConfigureRejection::TopicRefused(
            "Flow_Repair".into()
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
}
