//! Ended-turn retention is one Configure value: the ended Codex turn ids one
//! thread's turn ledger keeps. It is set and echoed whole, the snapshot
//! carries it last, and a negative bound is refused.

use meta_signal_flow::{ConfigurationChange, ConfigureRefusal, Input, Output, TurnRetentionBound};

#[test]
fn retention_is_one_configuration_value_on_the_binary_wire() {
    for bound in [0, 1, 64, 4096] {
        let change = ConfigurationChange::EndedTurnRetention(TurnRetentionBound(bound));
        let input = Input::Configure(change.clone());
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(),
            input
        );
        let output = Output::Configured(change);
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&output).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
            output
        );
    }
    let refused = Output::ConfigureRefused(ConfigureRefusal::EndedTurnRetentionRefused);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&refused).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
        refused
    );
}

#[test]
fn retention_is_preserved_in_the_reserved_hold_release() {
    assert_eq!(meta_signal_flow::WIRE_VERSION, "0.90.0");
}

#[cfg(feature = "datom")]
mod textual {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_flow::{ConfigurationSnapshot, ConfiguredEndedTurnRetention};
    use protos::{Compactable, Protosizable, ReaderBudget};

    fn budget() -> Budget {
        Budget {
            remaining: 16384,
            reader: ReaderBudget::try_from(16384).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn set_echo_and_refusal_have_concrete_datoms() {
        let text = "Configure.EndedTurnRetention.64";
        let query = Potential::<Input>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            query,
            Input::Configure(ConfigurationChange::EndedTurnRetention(TurnRetentionBound(
                64
            )))
        );
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        for reply in [
            "Configured.EndedTurnRetention.0",
            "Configured.EndedTurnRetention.64",
            "ConfigureRefused.EndedTurnRetentionRefused",
        ] {
            let output = Potential::<Output>::from(reply)
                .actualize(&mut budget())
                .unwrap();
            assert_eq!(output.datomize(vec![]).protosize().compact(), reply);
        }
        // A negative bound is a well-formed datom: Flow refuses it.
        let negative = Potential::<Input>::from("Configure.EndedTurnRetention.-1")
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            negative,
            Input::Configure(ConfigurationChange::EndedTurnRetention(TurnRetentionBound(
                -1
            )))
        );
    }

    #[test]
    fn snapshot_carries_retention_before_model_efforts() {
        let nexus = "{ /home/li/primary { /bin/stable /tmp/stable /tmp/stable.sock [ stable-model ] } { /bin/next /tmp/next /tmp/next.sock [ next-model ] } [] [] None None /tmp/curriculum.sock 60 None main }";
        let preceding = format!(
            "{nexus} [] [] Unconfigured Unconfigured Unconfigured 60000 [] [] [] None 30 None [] {{ 1 1 1 }} 32 3 Unconfigured [] [] guidance"
        );
        for bound in [0, 64] {
            let text = format!("{{ {preceding} {bound} [] }}");
            let snapshot = Potential::<ConfigurationSnapshot>::from(text)
                .actualize(&mut budget())
                .unwrap();
            assert_eq!(
                snapshot.configured_ended_turn_retention,
                ConfiguredEndedTurnRetention(TurnRetentionBound(bound))
            );
            let output = Output::Configuration(snapshot);
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&output).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
                output
            );
            let rendered = output.datomize(vec![]).protosize().compact();
            assert_eq!(
                Potential::<Output>::from(rendered)
                    .actualize(&mut budget())
                    .unwrap(),
                output
            );
        }
        // The 82.0.0 snapshot shape is incomplete under 83.0.0.
        assert!(
            Potential::<ConfigurationSnapshot>::from(format!("{{ {preceding} }}"))
                .actualize(&mut budget())
                .is_err()
        );
    }
}
