//! Effort is independently configurable at model and permaflow scope.
use meta_signal_flow::{ConfigurationChange, Effort, Input, Model, ModelEffort};
#[test]
fn every_model_effort_round_trips_on_the_wire() {
    for effort in [Effort::Default, Effort::High, Effort::Medium, Effort::Low] {
        let input = Input::Configure(ConfigurationChange::ModelEffort(ModelEffort { model: Model::Astra, effort }));
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).unwrap();
        assert_eq!(rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(), input);
    }
}
#[cfg(feature = "datom")]
#[test]
fn model_and_permaflow_efforts_have_concrete_datoms() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;
    for effort in ["Default", "High", "Medium", "Low"] {
        for text in [format!("Configure.ModelEffort.{{ Astra {effort} }}"), format!("Configure.Amend.{{ {{ Psyche core Fable }} {effort} }}")] {
            let mut budget = Budget { remaining: 16384, reader: ReaderBudget::try_from(16384).unwrap(), depth: 0, maximum_depth: 1024 };
            assert!(Potential::<Input>::from(text.clone()).actualize(&mut budget).is_ok(), "{text}");
        }
    }
}
