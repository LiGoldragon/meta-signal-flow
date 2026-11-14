#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_flow::{Query, Response};
use protos::{Compactable, Protosizable, ReaderBudget};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn privileged_requests_have_concrete_datoms() {
    for text in [
        "Configure.Nexus.{ /home/li/primary { /etc/profiles/per-user/li/bin/codex-stable-flow-client /home/li/.codex /home/li/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra gpt-5.6-sol gpt-5.6-luna ] } { /etc/profiles/per-user/li/bin/codex-next-flow-client /home/li/.codex-next /home/li/.codex-next/app-server-control/app-server-control.sock [ gpt-6-sol gpt-6-luna gpt-6-astra ] } [ { Claude [ / ! # ] [ esc esc ] [ enter ] } { Codex [ / ! ] [ esc ] [] } ] [ Psyche ] Some./run/user/1001/message-nexus.sock None /run/user/1001/curriculum/curriculum.sock 60 None main }",
        "Configuration",
        "ConsumeReset.{ attempt-1 Next }",
        "ConsumeReset.{ attempt-2 Specific.credit-7 }",
        "RegisterFlow.{ da1e3f claude-session Claude Unavailable Unavailable Unbound Pending }",
        "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } { Mind message Astra } { mind-flow Mind gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } }",
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

/// A binding names its flow's aspect and model and no power: a model
/// ranks above no other.
#[test]
fn a_flow_binding_carries_no_power() {
    let powered = "MetaBindExisting.{ { messaging-build /run/user/1001/herdr.sock { 4100 1001 server-token } owner-flow } { Mind message Astra } { mind-flow Mind High gpt-6 Codex mind-native workspace w12:p1 w12:t1 mind-terminal mind-agent { 4300 1001 mind-token } /home/li/primary } }";
    assert!(
        Potential::<Query>::from(powered)
            .actualize(&mut budget())
            .is_err(),
        "a binding carrying a power is refused"
    );
    assert!(
        Potential::<Response>::from("PeerResolved.{ e167d8 Psyche High claude-opus-5-5 }")
            .actualize(&mut budget())
            .is_err(),
        "a resolved peer carries no power"
    );
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

    let text = "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/app-server-control/app-server-control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/app-server-control/app-server-control.sock [ gpt-6.1 ] } [ { Claude [ / ! # ] [ esc esc ] [ enter ] } ] [ Psyche Mind ] Some./srv/message-nexus.sock Some./usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 None main } [ { Opus claude-opus } { Sol gpt-6.1 } ] [ { claude-opus-5-5 200000 } ] Unconfigured Unconfigured Unconfigured 60000 [ { /nix/store/codex/bin/codex /nix/store/send/bin/flow-send-mcp } ] [ { { Mind core Sol } Some./srv/core [ CodexAgentsMd CodexSkillListing ] [ spirit:core ] [ vision:flow ] Regular Default Maintained «Keeps Flow running.» [ flows/a68420/vision/flow.md ] } ] [ { { Mind core Sol } «--disallowedTools 'Artifact*'» } ] Some.{ Psyche compaction Sonnet } 30 None [] { 1 3 1 } 64 3 Configured./home/someone/.local/bin/flow-hook }";

    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Configuration(configuration) = &response else {
        panic!("expected Configuration snapshot");
    };
    let nexus = &configuration.configured_nexus.0;
    assert_eq!(nexus.source_root.0, "/srv/source");
    assert_eq!(
        nexus.first_codex_endpoint.client_path.0,
        "codex-stable-flow-client"
    );
    assert_eq!(nexus.first_codex_endpoint.home.0, "/home/someone/.codex");
    assert_eq!(
        nexus.first_codex_endpoint.control_socket_path.0,
        "/home/someone/.codex/app-server-control/app-server-control.sock"
    );
    assert_eq!(
        nexus.first_codex_endpoint.model_name_vector,
        vec![meta_signal_flow::ModelName("gpt-5.6-terra".to_owned())]
    );
    assert_eq!(
        nexus.second_codex_endpoint.model_name_vector,
        vec![meta_signal_flow::ModelName("gpt-6.1".to_owned())]
    );
    assert_eq!(nexus.harness_profiles.0.len(), 1);
    assert_eq!(
        nexus.harness_profiles.0[0].harness_kind,
        meta_signal_flow::HarnessKind::Claude
    );
    assert_eq!(
        nexus.harness_profiles.0[0].command_sigil_vector,
        vec![
            meta_signal_flow::CommandSigil("/".to_owned()),
            meta_signal_flow::CommandSigil("!".to_owned()),
            meta_signal_flow::CommandSigil("#".to_owned())
        ]
    );
    assert_eq!(
        nexus.harness_profiles.0[0].interrupt_keys,
        meta_signal_flow::InterruptKeys(vec![
            meta_signal_flow::KeyName("esc".to_owned()),
            meta_signal_flow::KeyName("esc".to_owned())
        ])
    );
    assert_eq!(
        nexus.harness_profiles.0[0].submit_keys,
        meta_signal_flow::SubmitKeys(vec![meta_signal_flow::KeyName("enter".to_owned())])
    );
    assert_eq!(
        nexus.meta_aspects.0.as_slice(),
        &[signal_flow::Aspect::Psyche, signal_flow::Aspect::Mind]
    );
    assert_eq!(
        nexus.message_nexus_path_option,
        Some(meta_signal_flow::MessageNexusPath(
            "/srv/message-nexus.sock".to_owned()
        ))
    );
    assert_eq!(
        nexus.message_nexus_binary_option,
        Some(meta_signal_flow::MessageNexusBinary(
            "/usr/bin/message-nexus".to_owned()
        ))
    );
    assert_eq!(
        nexus.curriculum_nexus_path.0,
        "/srv/curriculum/curriculum.sock"
    );
    assert_eq!(nexus.lease.0, 60);
    assert_eq!(nexus.presentation_prompt_option, None);
    assert_eq!(nexus.herdr_session_name.0, "main");
    // Each model holds its one native model.
    assert_eq!(
        configuration.configured_models.0,
        vec![
            meta_signal_flow::ConfigureModel {
                model: meta_signal_flow::Model::Opus,
                native: meta_signal_flow::Native("claude-opus".to_string()),
            },
            meta_signal_flow::ConfigureModel {
                model: meta_signal_flow::Model::Sol,
                native: meta_signal_flow::Native("gpt-6.1".to_string()),
            },
        ]
    );
    // A layer and an aspect no longer name a model.
    assert!(
        Potential::<Query>::from("Configure.Models.[ { Secondary Psyche claude-opus-5-5 } ]")
            .actualize(&mut budget())
            .is_err()
    );
    assert_eq!(configuration.configured_thresholds.0.len(), 1);
    assert_eq!(
        configuration.configured_thresholds.0[0].model_name.0,
        "claude-opus-5-5"
    );
    assert_eq!(
        configuration.configured_thresholds.0[0].refresh_threshold,
        meta_signal_flow::RefreshThreshold(200000)
    );
    assert_eq!(
        configuration
            .configured_investigation
            .0
            .as_ref()
            .map(|address| address.topic.0.as_str()),
        Some("compaction")
    );
    assert_eq!(configuration.configured_default_minutes.0.0, 30);
    assert_eq!(
        configuration.configured_harness_hook.0,
        meta_signal_flow::HarnessHookConfiguration::Configured(
            meta_signal_flow::generated::signal::Path(
                "/home/someone/.local/bin/flow-hook".to_string()
            )
        )
    );
    // A store that has allocated nothing names the first id of each family.
    assert_eq!(
        configuration.configured_counters.0,
        meta_signal_flow::ConfigureCounters {
            logical_delivery: meta_signal_flow::LogicalDelivery(1),
            queue_attempt: meta_signal_flow::QueueAttempt(3),
            send_outcome: meta_signal_flow::SendOutcome(1),
        }
    );
    assert_eq!(
        configuration.configured_send_servers.0,
        vec![meta_signal_flow::ConfigureSendServer {
            codex_app_server_binary: meta_signal_flow::CodexAppServerBinary(
                "/nix/store/codex/bin/codex".to_string()
            ),
            mcp_send_server_binary: meta_signal_flow::McpSendServerBinary(
                "/nix/store/send/bin/flow-send-mcp".to_string()
            ),
        }]
    );
    assert_eq!(
        configuration.configured_metaflows.0,
        vec![meta_signal_flow::ConfigureMetaflow {
            address: signal_flow::Address {
                aspect: meta_signal_flow::Aspect::Mind,
                topic: meta_signal_flow::Topic("core".to_string()),
                model: meta_signal_flow::Model::Sol,
            },
            root_option: Some(meta_signal_flow::Root("/srv/core".to_string())),
            // A permaflow carries its own leave-outs, of the harness its
            // Model runs on.
            leave_outs: signal_flow::LeaveOuts(vec![
                signal_flow::LeaveOut::CodexAgentsMd,
                signal_flow::LeaveOut::CodexSkillListing,
            ]),
            system_modules: signal_flow::SystemModules(vec![signal_flow::ModuleIdentity(
                "spirit:core".to_string()
            )]),
            user_modules: signal_flow::UserModules(vec![signal_flow::ModuleIdentity(
                "vision:flow".to_string()
            )]),
            flow_kind: signal_flow::FlowKind::Regular,
            tool_set: signal_flow::ToolSet::Default,
            continuity: meta_signal_flow::Continuity::Maintained,
            purpose: meta_signal_flow::Purpose("Keeps Flow running.".to_string()),
            records: meta_signal_flow::Records(vec![meta_signal_flow::RecordPath(
                "flows/a68420/vision/flow.md".to_string()
            )]),
        }]
    );
    assert_eq!(
        configuration.configured_harness_flags.0,
        vec![meta_signal_flow::ConfigureHarnessFlags {
            address: signal_flow::Address {
                aspect: meta_signal_flow::Aspect::Mind,
                topic: meta_signal_flow::Topic("core".to_string()),
                model: meta_signal_flow::Model::Sol,
            },
            harness_flag_text: meta_signal_flow::HarnessFlagText(
                "--disallowedTools 'Artifact*'".to_string()
            ),
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
fn presentation_prompt_round_trip_preserves_multiline_boundary_whitespace() {
    let prompt = "  Begin with the reviewed proposal.\nKeep each figure beside its ruling.  ";
    let text = format!(
        "Configure.Nexus.{{ /srv/source {{ stable-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] }} {{ next-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [] }} [] [ Psyche ] None None /srv/curriculum/curriculum.sock 60 Some.«{prompt}» main }}"
    );
    let query = Potential::<Query>::from(text.as_str())
        .actualize(&mut budget())
        .unwrap();
    let Query::Configure(meta_signal_flow::ConfigurationChange::Nexus(nexus)) = &query else {
        panic!("expected Nexus configuration");
    };
    assert_eq!(
        nexus
            .presentation_prompt_option
            .as_ref()
            .map(|p| p.0.as_str()),
        Some(prompt)
    );

    let encoded = query.datomize(vec![]).protosize().compact();
    let restored = Potential::<Query>::from(encoded.as_str())
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(restored, query);
    let Query::Configure(meta_signal_flow::ConfigurationChange::Nexus(restored_nexus)) = restored
    else {
        panic!("expected restored Nexus configuration");
    };
    assert_eq!(
        restored_nexus
            .presentation_prompt_option
            .as_ref()
            .map(|p| p.0.as_str()),
        Some(prompt)
    );

    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
}

#[test]
fn configure_missing_a_required_nexus_field_is_refused_by_the_reader() {
    let text = "Configure.Nexus.{ /srv/source { stable-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { next-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [] } [] [ Psyche ] Some./srv/message-nexus.sock Some./usr/bin/message-nexus /srv/curriculum/curriculum.sock }";
    assert!(
        Potential::<Query>::from(text)
            .actualize(&mut budget())
            .is_err()
    );
}

#[test]
fn configure_nexus_without_its_herdr_session_is_refused_by_the_reader() {
    let text = "Configure.Nexus.{ /srv/source { stable-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { next-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [] } [] [ Psyche ] None None /srv/curriculum/curriculum.sock 60 None }";
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
    assert!(matches!(&query, Query::Retire(flow_id) if flow_id.0 == "d8df70"));
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
        "Deliver.{ m-7f3a2c:7d41e0:0 7d41e0 Soft.{ Sender.{ Psyche flow Opus } Text.«Stage 1 is deployed; run the tier tests.» } }",
        "Deliver.{ m-81b0e4:7d41e0:0 7d41e0 HardAbrupt.{ Notification Text.«Stop the ouranos build now.» } }",
        "Deliver.{ m-90c1aa:7d41e0:0 7d41e0 MiddleAbrupt.{ Sender.{ Field core Astra } Living.{ «on build hosts» «Prometheus should be doing the builds.» } } }",
        "Vet.{ m-a2d913:7d41e0:0 7d41e0 Soft.{ Sender.{ Psyche flow Opus } Text./compact } }",
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
        "Delivered.{ NotRequested Presented }",
        "Delivered.{ Observed Transported }",
        "Delivered.{ Unobserved Uncertain }",
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
        "DeliveryRejected.Superseded.908786",
        "DeliveryRejected.Unconfirmed",
        "Vetted",
        "Commanded.{ 7d41e0 Compact Transported }",
        "CommandRejected.UnsupportedForHarness",
        "CommandRejected.FlowRetired",
        "CommandRejected.FlowExited",
        "CommandRejected.CompactionRefused.BelowThreshold.{ 120000 200000 }",
        "CommandRejected.CompactionRefused.Unmeasured",
        "CommandRejected.CompactionRefused.NoThreshold",
        "CommandRejected.Superseded.908786",
        "CommandRejected.Unconfirmed",
        "PeerResolved.{ e167d8 Psyche claude-opus-5-5 }",
        "PeerResolutionRejected.CallerUnknown",
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
        visible_sender: signal_flow::VisibleSender::Sender(signal_flow::Address {
            aspect: signal_flow::Aspect::Psyche,
            topic: meta_signal_flow::Topic("flow".to_string()),
            model: signal_flow::Model::Opus,
        }),
        content: meta_signal_flow::Content::Text("Stage 1 is deployed; run the tier tests.".into()),
    });
    assert_eq!(
        message.datomize(vec![]).protosize().compact(),
        "Soft.{ Sender.{ Psyche flow Opus } Text.«Stage 1 is deployed; run the tier tests.» }"
    );
}

#[test]
fn configure_declares_one_whole_family() {
    for text in [
        "Configure.Models.[ { Haiku claude-haiku-5-5 } { Opus claude-opus-5-5 } { Sol gpt-6-sol } { Luna gpt-6-luna } ]",
        "Configure.Thresholds.[ { claude-opus-5-5 200000 } { gpt-6.1-sol 150000 } ]",
        "Configure.Investigation.{ Psyche compaction Sonnet }",
        "Configure.HarnessHook./home/li/.local/bin/flow-hook",
        "ObserveConfiguration",
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
        Potential::<Query>::from("Configure.Models.[ { Haiku claude-haiku-5-5 } ]")
            .actualize(&mut budget())
            .unwrap()
    else {
        panic!("not a Models declaration")
    };
    assert_eq!(models.len(), 1);
    // One entry is not a family: a declaration is always the whole vector.
    assert!(
        Potential::<Query>::from("Configure.Models.{ Haiku claude-haiku-5-5 }")
            .actualize(&mut budget())
            .is_err()
    );
    // A threshold belongs to a native model, never to a Model, and has no handover.
    assert!(
        Potential::<Query>::from("Configure.Thresholds.[ { Fable 600000 700000 } ]")
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
                codex_app_server_binary: meta_signal_flow::CodexAppServerBinary(
                    "/nix/store/codex/bin/codex".to_string()
                ),
                mcp_send_server_binary: meta_signal_flow::McpSendServerBinary(
                    "/nix/store/send/bin/flow-send-mcp".to_string()
                ),
            }]
        ))
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

/// A metaflow declaration names the modules a Launch of its Address composes.
#[test]
fn configure_metaflows_has_a_concrete_datom() {
    let text = "Configure.Metaflows.[ { { Psyche core Opus } None [ ClaudeMd GitInstructions ] [ spirit:core:0 intent:psyche:0 ] [ compensation:truth:0 ] Regular Default OnDemand «» [] } ]";
    let query = Potential::<meta_signal_flow::Query>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert!(matches!(
        &query,
        meta_signal_flow::Query::Configure(meta_signal_flow::ConfigurationChange::Metaflows(declared))
            if declared.len() == 1 && declared[0].system_modules.0.len() == 2
    ));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

/// One declaration is added, replaced or removed alone, keyed by its name:
/// a permaflow by its Address, a harness profile by its harness, a model's
/// refresh by its model name.
#[test]
fn one_declaration_is_configured_and_removed_alone() {
    for text in [
        "Configure.Permaflow.{ { Psyche book Haiku } Some./home/li/book [ ClaudeMd ] [ spirit:core:0 ] [] Regular Default OnDemand «» [] }",
        "Configure.Harness.{ Claude [ / ] [ Escape Escape ] [ Enter ] }",
        "Configure.Refresh.{ claude-fable-5-2 280000 }",
        "Remove.Permaflow.{ Psyche book Haiku }",
        "Remove.Harness.Codex",
        "Remove.Refresh.claude-fable-5-2",
    ] {
        let query = Potential::<meta_signal_flow::Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<meta_signal_flow::Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
}

#[test]
fn configure_harness_flags_has_a_concrete_datom() {
    let text =
        "Configure.HarnessFlags.[ { { Psyche core Fable } «--disallowedTools 'Artifact*'» } ]";
    let query = Potential::<meta_signal_flow::Query>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert!(matches!(
        &query,
        meta_signal_flow::Query::Configure(meta_signal_flow::ConfigurationChange::HarnessFlags(flags))
            if flags.len() == 1 && flags[0].harness_flag_text.0 == "--disallowedTools 'Artifact*'"
    ));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

/// The leave-out family is gone: a permaflow carries its leave-outs, and
/// a leave-out of another harness than its Model runs on is refused
/// UnknownFeature, naming it.
#[test]
fn leave_outs_are_the_permaflow_s_and_a_foreign_one_is_an_unknown_feature() {
    assert!(
        Potential::<Query>::from(
            "Configure.LeaveOuts.[ { { Psyche flow Opus } Claude [ SkillTool ClaudeMd ] } ]"
        )
        .actualize(&mut budget())
        .is_err()
    );
    // A permaflow without its leave-outs is no longer a ConfigureMetaflow.
    assert!(
        Potential::<Query>::from(
            "Configure.Permaflow.{ { Psyche book Haiku } None [ spirit:core:0 ] [] Regular Default OnDemand «» [] }"
        )
        .actualize(&mut budget())
        .is_err()
    );
    let text = "ConfigureRejected.UnknownFeature.CodexAgentsMd";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::ConfigureRejected(meta_signal_flow::ConfigureRejection::UnknownFeature(
            signal_flow::LeaveOut::CodexAgentsMd
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
}

/// One value of one permaflow is changed alone: a single value replaced,
/// or one element added to or removed from its list.
#[test]
fn amend_changes_one_value_of_one_permaflow() {
    for text in [
        "Configure.Amend.{ { Mind atomicTrialAlpha Luna } Purpose.«Alpha after» }",
        "Configure.Amend.{ { Psyche book Haiku } Workspace.Some./home/li/book }",
        "Configure.Amend.{ { Psyche book Haiku } Workspace.None }",
        "Configure.Amend.{ { Psyche book Haiku } Kind.Simplified }",
        "Configure.Amend.{ { Psyche book Haiku } Continuity.Maintained }",
        "Configure.Amend.{ { Psyche core Fable } Add.PromptModule.vision:energy-nexus:1 }",
        "Configure.Amend.{ { Psyche core Fable } Add.SystemModule.spirit:core:0 }",
        "Configure.Amend.{ { Psyche core Fable } Remove.PromptModule.vision:energy-nexus:1 }",
        "Configure.Amend.{ { Psyche core Fable } Add.LeaveOut.ClaudeMd }",
        "Configure.Amend.{ { Psyche core Fable } Remove.LeaveOut.ClaudeMd }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert!(matches!(
            &query,
            Query::Configure(meta_signal_flow::ConfigurationChange::Amend(_))
        ));
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    let Query::Configure(meta_signal_flow::ConfigurationChange::Amend(amend)) =
        Potential::<Query>::from(
            "Configure.Amend.{ { Psyche core Fable } Add.PromptModule.vision:energy-nexus:1 }",
        )
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("an Amend");
    };
    assert_eq!(amend.address.topic.0, "core");
    assert_eq!(
        amend.change,
        meta_signal_flow::Change::Add(meta_signal_flow::Element::PromptModule(
            signal_flow::ModuleIdentity("vision:energy-nexus:1".to_string())
        ))
    );
    for (text, rejection) in [
        (
            "ConfigureRejected.UnknownPermaflow.{ Mind atomicTrialGamma Luna }",
            meta_signal_flow::ConfigureRejection::UnknownPermaflow(signal_flow::Address {
                aspect: meta_signal_flow::Aspect::Mind,
                topic: meta_signal_flow::Topic("atomicTrialGamma".to_string()),
                model: meta_signal_flow::Model::Luna,
            }),
        ),
        (
            "ConfigureRejected.UnknownModule.vision:absent:0",
            meta_signal_flow::ConfigureRejection::UnknownModule(signal_flow::ModuleIdentity(
                "vision:absent:0".to_string(),
            )),
        ),
        (
            "ConfigureRejected.CurriculumUnavailable",
            meta_signal_flow::ConfigureRejection::CurriculumUnavailable,
        ),
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(response, Response::ConfigureRejected(rejection));
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
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
            meta_signal_flow::Topic("Flow_Repair".to_string())
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
}

/// The default Minutes is a configuration family of its own: one whole
/// number of minutes, declared whole over Configure and read back as the
/// snapshot's last field. A declaration the Minutes rule refuses answers
/// ConfigureRejected.MinutesRefused and changes nothing.
#[test]
fn configure_declares_the_default_minutes_and_the_snapshot_carries_it() {
    let text = "Configure.DefaultMinutes.45";
    let query = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap_or_else(|error| panic!("{text}: {error:?}"));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    for reply in [
        "ConfigureRejected.MinutesRefused.Zero",
        "ConfigureRejected.MinutesRefused.Negative",
        "ConfigureRejected.MinutesRefused.Unreachable",
        "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [ gpt-6.1 ] } [] [ Psyche ] Some./srv/message-nexus.sock Some./usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 None main } [] [] Unconfigured Unconfigured Unconfigured 60000 [] [] [] [] None 45 None [] { 1 3 1 } 64 3 Unconfigured }",
    ] {
        let response = Potential::<Response>::from(reply)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{reply}: {error:?}"));
        assert_eq!(response.datomize(vec![]).protosize().compact(), reply);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
}

/// What one hand-over types is a Waking: every queued input names its sender
/// and priority, one top-level vector, oldest first. Private ledger
/// keys never enter the prompt.
#[test]
fn a_letter_carries_a_waking_of_native_inputs_each_naming_its_sender() {
    let text = "Deliver.{ attempt-7 7d41e0 MiddleAbrupt.{ Notification Waking.[ { Sender.{ Psyche flow Fable } MiddleAbrupt.Task.{ t001 30 «Draft the proposal.» } } { Notification Soft.Notice.{ n001 «Reminder 1 of 4: task has no Done.» } } { Sender.{ Mind flow Sol } Soft.Done.{ t001 «Proposal drafted.» } } ] } }";
    let query = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap_or_else(|error| panic!("{text}: {error:?}"));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    // One input alone is a Waking of one; the single-input form is gone.
    assert!(
        Potential::<Query>::from(
            "Deliver.{ attempt-8 7d41e0 Soft.{ Notification NativeInput.{ Notification Soft.Notice.«One alone.» } } }"
        )
        .actualize(&mut budget())
        .is_err()
    );
}

/// The id counters are a configuration family of their own: the next id
/// each family allocates, a logical delivery, a queue attempt and a send
/// outcome, in that order. Declared whole over Configure and read back as
/// the snapshot's last field. A declaration on a store that has recorded
/// any of them, one lower than a counter now stands, or one sent to a
/// Nexus opened for configuration only answers
/// ConfigureRejected.CountersRefused and changes nothing; Lowered carries
/// the counters as they stand.
#[test]
fn configure_declares_the_counters_and_the_snapshot_carries_them() {
    let text = "Configure.Counters.{ 120 45 300 }";
    let query = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap_or_else(|error| panic!("{text}: {error:?}"));
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    for reply in [
        "ConfigureRejected.CountersRefused.NotEmpty",
        "ConfigureRejected.CountersRefused.Lowered.{ 121 46 301 }",
        "ConfigureRejected.CountersRefused.NotRunning",
        "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [ gpt-6.1 ] } [] [ Psyche ] Some./srv/message-nexus.sock Some./usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 None main } [] [] Unconfigured Unconfigured Unconfigured 60000 [] [] [] [] None 45 None [] { 120 45 300 } 64 3 Unconfigured }",
    ] {
        let response = Potential::<Response>::from(reply)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{reply}: {error:?}"));
        assert_eq!(response.datomize(vec![]).protosize().compact(), reply);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
}

/// A stored send outcome refused for an unidentified sender holds the raw
/// evidence signal-flow 35 carries; the record round-trips datom → rkyv →
/// datom.
#[test]
fn a_send_outcome_refused_unidentified_carries_its_seat_evidence() {
    let text = "SendOutcomesRead.[ { Some.{ 4242 987654 } Unbound None digest-1 Refused.Unidentified.{ Some.{ herdr-1 w1-2 } Some.«Claude research» Some.01a0fdcc-c1a1 1761800000 { 4242 987654 } } } ]";
    let value = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    let read = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
    assert_eq!(read, value);
    assert_eq!(read.datomize(vec![]).protosize().compact(), text);
}

/// ProtectedAddresses is a configuration family of its own: the Addresses
/// whose flows Flow keeps, declared whole over Configure and read back in
/// the snapshot before the counters. Restart and Refresh are taken here as
/// on the ordinary socket, with their answers.
#[test]
fn protected_addresses_and_the_meta_restart_and_refresh_round_trip() {
    for text in [
        "Configure.ProtectedAddresses.[ { Psyche core Fable } ]",
        "Configure.ProtectedAddresses.[]",
        "Restart.c1a0de",
        "Refresh.{ Psyche core Fable }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    for reply in [
        "Restarted.c1a0de",
        "RestartRefused.Unsupported.c1a0de",
        "Refreshed.{ { Psyche core Fable } c1a0de }",
        "RefreshQueued.{ { Psyche core Fable } c1a0de }",
        "ReapPending.{ { Psyche core Fable } c1a0de { 908786 session-2 Unavailable Unbound } Owed }",
        "ReapPending.{ { Psyche core Fable } c1a0de { 908786 session-2 Unavailable Unbound } CloseRefused }",
        "OrdinaryRefused.Unknown.Address.{ Psyche core Fable }",
        "Configuration.{ { /srv/source { codex-stable-flow-client /home/someone/.codex /home/someone/.codex/control.sock [ gpt-5.6-terra ] } { codex-next-flow-client /home/someone/.codex-next /home/someone/.codex-next/control.sock [ gpt-6.1 ] } [] [ Psyche ] Some./srv/message-nexus.sock Some./usr/bin/message-nexus /srv/curriculum/curriculum.sock 60 None main } [] [] Unconfigured Unconfigured Unconfigured 60000 [] [] [] [] None 30 None [ { Psyche core Fable } ] { 1 3 1 } 64 3 Unconfigured }",
    ] {
        let response = Potential::<Response>::from(reply)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{reply}: {error:?}"));
        assert_eq!(response.datomize(vec![]).protosize().compact(), reply);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
}

#[test]
fn handover_configuration_round_trips_zero_count_policy_and_archive_path() {
    for text in [
        "Configure.HandoverCount.0",
        "Configure.PublicationDeadline.60000",
        "Configure.HandoverArchive./srv/handover-archive",
        "Configure.HandoverPolicies.[ { claude-opus-5-5 15 «Continue every five minutes.» 5 } ]",
        "Configured.Replaced",
    ] {
        if text.starts_with("Configure.") {
            let query = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
            let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
                query
            );
        } else {
            let response = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap_or_else(|error| panic!("{text}: {error:?}"));
            assert_eq!(response.datomize(vec![]).protosize().compact(), text);
            let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
                response
            );
        }
    }
}
