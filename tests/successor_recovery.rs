use meta_signal_flow::{ConfigurationChange, Input, Output, SuccessorRecoveryText};

#[test]
fn recovery_text_is_one_scalar_configuration_value_on_the_binary_wire() {
    for text in [
        "",
        "Recover the predecessor transcript.",
        "Changed guidance.\nKeep its provenance.",
    ] {
        let value = SuccessorRecoveryText(text.to_owned());
        let change = ConfigurationChange::SuccessorRecoveryText(value.clone());
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
        assert_eq!(value.0, text);
    }
}

#[cfg(feature = "datom")]
mod textual {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_flow::ConfigurationSnapshot;
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
    fn meta_set_and_echo_have_one_bare_named_type_payload() {
        let text = "Configure.SuccessorRecoveryText.«Recover the predecessor transcript.»";
        let query = Potential::<Input>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let Input::Configure(ConfigurationChange::SuccessorRecoveryText(value)) = &query else {
            panic!("not the typed recovery configuration");
        };
        assert_eq!(value.0, "Recover the predecessor transcript.");
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        let Input::Configure(change) = query else {
            unreachable!()
        };
        let response = Output::Configured(change);
        let rendered = response.datomize(vec![]).protosize().compact();
        assert_eq!(
            rendered,
            "Configured.SuccessorRecoveryText.«Recover the predecessor transcript.»"
        );
        assert_eq!(
            Potential::<Output>::from(rendered)
                .actualize(&mut budget())
                .unwrap(),
            response
        );
    }

    #[test]
    fn snapshot_carries_the_current_text_as_its_final_typed_position() {
        let nexus = "{ /home/li/primary { /bin/stable /tmp/stable /tmp/stable.sock [ stable-model ] } { /bin/next /tmp/next /tmp/next.sock [ next-model ] } [] [] None None /tmp/curriculum.sock 60 None main }";
        let preceding = format!(
            "{nexus} [] [] Unconfigured Unconfigured Unconfigured 60000 [] [] [] None 30 None [] {{ 1 1 1 }} 32 3 Unconfigured [] []"
        );
        for current in ["initial-guidance", "changed-guidance"] {
            let text = format!("{{ {preceding} {current} 64 }}");
            let snapshot = Potential::<ConfigurationSnapshot>::from(text)
                .actualize(&mut budget())
                .unwrap();
            assert_eq!(
                snapshot.successor_recovery_text,
                SuccessorRecoveryText(current.to_owned())
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
        // The previous snapshot shape is incomplete under this new contract.
        assert!(
            Potential::<ConfigurationSnapshot>::from(format!("{{ {preceding} }}"))
                .actualize(&mut budget())
                .is_err()
        );
    }
}
