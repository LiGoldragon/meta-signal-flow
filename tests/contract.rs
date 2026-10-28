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
        "Configure.Nexus.{ /home/li/primary /home/li/primary/flows [ Psyche ] /run/user/1001/message-nexus.sock /etc/profiles/per-user/li/bin/message-nexus /run/user/1001/curriculum/curriculum.sock 60 { herdr flow-id flow-hook } default }",
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

    let text = "Configuration.{ { /srv/source /srv/source/flows [ Psyche Mind ] /srv/message-nexus.sock /usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 { /usr/bin/herdr /usr/bin/flow-id /usr/bin/flow-hook } default } [ { codex Codex fixture /usr/bin/codex /home/someone/.codex Some./home/someone/.codex/control.sock [ { gpt-6.1 gpt-6.1 } ] [] [] [] [] [] [] { [ / «!» ] [ esc ] [] /compact { › LineByLine.[ ctrl+u backspace ] 12 32 } } } ] [ { Secondary Psyche claude-opus } { Secondary Mind gpt-6.1 } ] [ { claude-opus-5-5 200000 } ] [ { { Field flow Quaternary } Claude [ SkillTool WorkflowTool ] } ] [ { /nix/store/codex/bin/codex /nix/store/send/bin/flow-send-mcp } ] [ { { Mind core Secondary } [ spirit:core ] [ vision:flow ] Default Maintained «Keeps Flow running.» [ flows/a68420/vision/flow.md ] [ { Field helper Tertiary } ] } ] Some.{ Psyche compaction Tertiary } { 5000 30000 10000 10000 30000 30000 30000 15000 30000 5000 10000 3000 350 250 500 1000 60000 50 2000 600000 } { 65536 1048576 67108864 16384 67108864 4194304 1048576 1024 1048576 16 16 64 3 12 40 50 } }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Configuration(configuration) = &response else {
        panic!("expected Configuration snapshot");
    };
    assert_eq!(configuration.nexus.source_root, "/srv/source");
    assert_eq!(configuration.nexus.flows_root, "/srv/source/flows");
    assert_eq!(configuration.harnesses.len(), 1);
    assert_eq!(configuration.harnesses[0].harness_name, "codex");
    assert_eq!(configuration.harnesses[0].executable, "/usr/bin/codex");
    assert_eq!(configuration.harnesses[0].served_models[0].model_name, "gpt-6.1");
    assert_eq!(
        configuration.harnesses[0].pane_profile.interrupt_keys,
        vec!["esc".to_owned()]
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
    assert_eq!(configuration.timings.socket_timeout, 5000);
    assert_eq!(configuration.timings.compaction_watch_margin, 600000);
    assert_eq!(configuration.limits.send_body, 65536);
    assert_eq!(configuration.limits.output_lines, 50);
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
            tool_set: signal_flow::ToolSet::Default,
            continuity: meta_signal_flow::Continuity::Maintained,
            purpose: "Keeps Flow running.".into(),
            records: vec!["flows/a68420/vision/flow.md".into()],
            requires: vec![signal_flow::Address {
                aspect: signal_flow::Aspect::Field,
                topic: "helper".into(),
                layer: signal_flow::Layer::Tertiary
            }],
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
    let request = "Retire.{ d8df70 Kept }";
    let query = Potential::<Query>::from(request)
        .actualize(&mut budget())
        .unwrap();
    assert!(
        matches!(&query, Query::Retire(request) if request.flow_id == "d8df70" && request.gate == meta_signal_flow::Gate::Kept)
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), request);

    for text in [
        "FlowRetired.{ d8df70 claude-session Claude Unavailable Unavailable Flow.{ 88475f field-session } Retired }",
        "RetireRejected.UnknownFlow",
        "RetireRejected.AlreadyGone",
        "RetireRejected.StoreRefused",
        "RetireRejected.CloseRefused.IdentityUnavailable",
        "RetireRejected.CloseRefused.IdentityMismatch",
        "RetireRejected.CloseRefused.StopTimedOut",
        "RetireRejected.CloseRefused.HerdrCloseFailed",
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
        "Deliver.{ m-7f3a2c:7d41e0:0 7d41e0 Soft.{ Notice { { Psyche core Secondary } e167d8 } Words.«Stage 1 is deployed; run the tier tests.» } Kept }",
        "Deliver.{ m-81b0e4:7d41e0:0 7d41e0 HardAbrupt.{ Order { { Mind flow Secondary } 88475f } Words.«Stop the ouranos build now.» } Kept }",
        "Deliver.{ m-90c1aa:7d41e0:0 7d41e0 MiddleAbrupt.{ Psyche { { Psyche core Secondary } 88475f } Psyche.{ «on build hosts» «Prometheus should be doing the builds.» } } Kept }",
        "Vet.{ m-a2d913:7d41e0:0 7d41e0 Soft.{ Notice { { Psyche core Secondary } e167d8 } Words./compact } Kept }",
        "Command.{ 7d41e0 Compact Kept }",
        "Command.{ 7d41e0 Interrupt Kept }",
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
        kind: meta_signal_flow::Kind::Notice,
        sender: meta_signal_flow::Sender {
            address: flow_ethos::Address {
                aspect: flow_ethos::Aspect::Psyche,
                topic: "core".into(),
                layer: flow_ethos::Layer::Secondary,
            },
            flow_id: "e167d8".into(),
        },
        body: meta_signal_flow::Body::Words("Stage 1 is deployed; run the tier tests.".into()),
    });
    assert_eq!(
        message.datomize(vec![]).protosize().compact(),
        "Soft.{ Notice { { Psyche core Secondary } e167d8 } Words.«Stage 1 is deployed; run the tier tests.» }"
    );
}

#[test]
fn configure_declares_one_whole_family() {
    for text in [
        "Configure.Models.[ { Quaternary Psyche claude-haiku-5-5 } { Secondary Psyche claude-opus-5-5 } { Secondary Mind gpt-6-sol } { Secondary Field gpt-6-sol } ]",
        "Configure.Thresholds.[ { claude-opus-5-5 200000 } { gpt-6.1-sol 150000 } ]",
        "Configure.Investigation.{ Psyche compaction Tertiary }",
        "Configure.LeaveOuts.[ { { Field core Primary } Codex [ CodexAgentsMd CodexSkillListing ] } { { Psyche flow Secondary } Claude [ SkillTool ClaudeMd ] } { { Psyche flow Secondary } Codex [ CodexAgentsMd ] } ]",
        "Configure.LeaveOuts.[]",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|_| panic!("not a Query Datom: {text}"));
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    let Query::Configure(meta_signal_flow::Configuration::Models(models)) =
        Potential::<Query>::from("Configure.Models.[ { Quaternary Psyche claude-haiku-5-5 } ]")
            .actualize(&mut budget())
            .unwrap()
    else {
        panic!("not a Models declaration")
    };
    assert_eq!(models.len(), 1);
    // One entry is not a family: a declaration is always the whole vector.
    assert!(
        Potential::<Query>::from("Configure.Models.{ Quaternary Psyche claude-haiku-5-5 }")
            .actualize(&mut budget())
            .is_err()
    );
    // A threshold belongs to a model, never to a layer, and has no handover.
    assert!(
        Potential::<Query>::from("Configure.Thresholds.[ { Primary 600000 700000 } ]")
            .actualize(&mut budget())
            .is_err()
    );
    // Configure.Nexus is the one way to set or replace the Nexus.
    assert!(
        Potential::<Query>::from("Reconfigure.{ /srv }")
            .actualize(&mut budget())
            .is_err()
    );
    for reply in [
        "ConfigureRejected.DuplicateEntry",
        "ConfigureRejected.ModelServedTwice.gpt-6-sol",
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

/// A send-servers declaration pairs each Codex app-server executable with
/// the MCP send-server executable its child must run.
#[test]
fn configure_send_servers_has_a_concrete_datom() {
    let text = "Configure.SendServers.[ { /nix/store/codex/bin/codex /nix/store/send/bin/flow-send-mcp } ]";
    let query = Potential::<meta_signal_flow::Query>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        query,
        meta_signal_flow::Query::Configure(meta_signal_flow::ConfigurationChange::SendServers(
            vec![meta_signal_flow::ConfigureSendServer {
                codex_app_server_binary: "/nix/store/codex/bin/codex".into(),
                mcp_send_server_binary: "/nix/store/send/bin/flow-send-mcp".into(),
            }]
        ))
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

/// A metaflow declaration names the modules a Launch of its Address composes.
#[test]
fn configure_metaflows_has_a_concrete_datom() {
    let text = "Configure.Metaflows.[ { { Psyche core Secondary } [ spirit:core intent:psyche ] [ compensation:truth ] Default OnDemand «» [] [] } ]";
    let query = Potential::<meta_signal_flow::Query>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert!(matches!(
        &query,
        meta_signal_flow::Query::Configure(meta_signal_flow::ConfigurationChange::Metaflows(declared))
            if declared.len() == 1 && declared[0].system_modules.len() == 2
    ));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn timings_and_limits_have_concrete_datoms() {
    for text in [
        "Configure.Timings.{ 5000 30000 10000 10000 30000 30000 30000 15000 30000 5000 10000 3000 350 250 500 1000 60000 50 2000 600000 }",
        "Configure.Limits.{ 65536 1048576 67108864 16384 67108864 4194304 1048576 1024 1048576 16 16 64 3 12 40 50 }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
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
