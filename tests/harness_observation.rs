//! ObserveHarness: what each Harness Nexus told of each flow, beside Flow's
//! own state of it, and the disagreements that stood their grace. The
//! expected texts are written out here, not computed through the codec.

use meta_signal_flow::{
    HarnessDisagreement, HarnessObserved_Data, HarnessSocket, HarnessTokens, HarnessWitness,
    Input, Output, Seen, State,
};
use signal_flow::FlowId;

fn socket() -> HarnessSocket {
    HarnessSocket("/run/user/1001/nexus/main/claude-harness-subscription.sock".into())
}

fn harness_observed() -> Output {
    Output::HarnessObserved(HarnessObserved_Data {
        harness_witness_vector: vec![
            HarnessWitness {
                flow_id: FlowId("c1a0de".into()),
                harness_socket: socket(),
                harness_event: harness::Event::ContextSize(harness::Tokens(41_234)),
                harness_state: harness::State::Flowing(harness::Flow::Rested),
                harness_tokens: HarnessTokens(41_234),
                state_option: Some(State::Resting),
            },
            HarnessWitness {
                flow_id: FlowId("f1c841".into()),
                harness_socket: socket(),
                harness_event: harness::Event::InFlight(vec![]),
                harness_state: harness::State::Dry,
                harness_tokens: HarnessTokens(0),
                state_option: None,
            },
        ],
        harness_disagreement_vector: vec![HarnessDisagreement {
            flow_id: FlowId("c1a0de".into()),
            harness_socket: socket(),
            seen: Seen(1_795_000_000),
            harness_state: harness::State::Ended,
            state_option: Some(State::Acting),
        }],
    })
}

#[test]
fn observe_harness_round_trips_as_an_archive() {
    for value in [
        harness_observed(),
        Output::HarnessObserved(HarnessObserved_Data {
            harness_witness_vector: vec![],
            harness_disagreement_vector: vec![],
        }),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            value
        );
    }
    let query = Input::ObserveHarness;
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
}

#[cfg(feature = "datom")]
mod datom {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};

    fn budget() -> Budget {
        Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        }
    }

    const OBSERVED: &str = "HarnessObserved.{ [ { c1a0de /run/user/1001/nexus/main/claude-harness-subscription.sock ContextSize.41234 Flowing.Rested 41234 Some.Resting } { f1c841 /run/user/1001/nexus/main/claude-harness-subscription.sock InFlight.[] Dry 0 None } ] [ { c1a0de /run/user/1001/nexus/main/claude-harness-subscription.sock 1795000000 Ended Some.Acting } ] }";

    #[test]
    fn observe_harness_has_concrete_datoms() {
        assert_eq!(
            Potential::<Input>::from("ObserveHarness")
                .actualize(&mut budget())
                .unwrap(),
            Input::ObserveHarness
        );
        assert_eq!(
            harness_observed().datomize(vec![]).protosize().compact(),
            OBSERVED
        );
        assert_eq!(
            Potential::<Output>::from(OBSERVED)
                .actualize(&mut budget())
                .unwrap(),
            harness_observed()
        );
    }
}
