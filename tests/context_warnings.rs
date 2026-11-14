#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_flow::{
    ConfigurationChange, ContextUsageProjection, ContextWarningLimit, Input, Output, Removal,
};
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
fn one_model_policy_carries_native_evidence_and_each_typed_stage() {
    let text = "Configure.ContextWarningPolicy.{ measured-model NativeEffectiveLimit.{ 200000 { Codex native-version /tmp/native.jsonl boundary-1 «unreported items are not included» } } CodexLastTotal [ { Emergency 10 HardAbrupt «your handover right now, or you lose everything when you are refreshed.» } { Middle 25 MiddleAbrupt «write your handover now» } { Early 50 Soft «you are at {percent} percent of your context» } ] }";
    let query = Potential::<Input>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Input::Configure(ConfigurationChange::ContextWarningPolicy(policy)) = &query else {
        panic!("not a one-model policy");
    };
    assert_eq!(policy.model_name.0, "measured-model");
    let ContextWarningLimit::NativeEffectiveLimit(limit) = &policy.context_warning_limit else {
        panic!("not a native effective limit");
    };
    assert_eq!(limit.warning_tokens.0, 200000);
    assert_eq!(
        policy.context_usage_projection,
        ContextUsageProjection::CodexLastTotal
    );
    assert_eq!(policy.warning_boundary_vector.len(), 3);
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let reply = Output::Configured(match query {
        Input::Configure(change) => change,
        _ => unreachable!(),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
        reply
    );
}

#[test]
fn provisional_policy_is_distinct_from_a_native_limit_on_text_and_wire() {
    let text = "Configure.ContextWarningPolicy.{ provisional-model ProvisionalPolicy.{ 916600 { Claude native-version /tmp/native.jsonl auto-boundary «last completed call is a policy choice, not a trigger bound» } } ClaudeCompletedInputWithCache [] }";
    let query = Potential::<Input>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Input::Configure(ConfigurationChange::ContextWarningPolicy(policy)) = &query else {
        panic!("not a model policy");
    };
    let ContextWarningLimit::ProvisionalPolicy(limit) = &policy.context_warning_limit else {
        panic!("provisional policy lost its basis");
    };
    assert_eq!(limit.warning_tokens.0, 916600);
    assert_eq!(
        policy.context_usage_projection,
        ContextUsageProjection::ClaudeCompletedInputWithCache
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(),
        query
    );
}

#[test]
fn installed_hook_events_are_explicit_configuration() {
    let text =
        "Configure.HarnessHookEvents.[ SessionStart PostToolUse UserPromptSubmit Stop SessionEnd ]";
    let query = Potential::<Input>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert!(
        matches!(&query, Input::Configure(ConfigurationChange::HarnessHookEvents(events)) if events.len() == 5)
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn one_model_policy_can_be_removed_by_its_model_name() {
    let text = "Remove.ContextWarning.measured-model";
    let query = Potential::<Input>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert!(
        matches!(&query, Input::Remove(Removal::ContextWarning(model)) if model.0 == "measured-model")
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn invalid_policy_is_a_typed_configuration_refusal() {
    let text = "ConfigureRefused.ContextWarningConfigurationRefused.MissingStage";
    let reply = Potential::<Output>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(reply.datomize(vec![]).protosize().compact(), text);
}
