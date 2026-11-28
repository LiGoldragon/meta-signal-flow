//! Effort is the native spelling, held per model and per permaflow; Flow
//! knows no level, so any spelling is configured and carried unchanged.
use meta_signal_flow::{Change, ConfigurationChange, Effort, Input, Model, ModelEffort};

/// Spellings a harness takes, a spelling no harness takes, and the
/// absence that omits the argument.
fn efforts() -> Vec<Option<Effort>> {
    [
        "low", "medium", "high", "xhigh", "max", "minimal", "none", "ultra",
    ]
    .into_iter()
    .map(|spelling| Some(Effort(spelling.to_string())))
    .chain([None])
    .collect()
}

#[test]
fn every_effort_round_trips_unchanged_on_the_wire() {
    for effort_option in efforts() {
        for input in [
            Input::Configure(ConfigurationChange::ModelEffort(ModelEffort {
                model: Model::Haiku,
                effort_option: effort_option.clone(),
            })),
            Input::Configure(ConfigurationChange::Amend(meta_signal_flow::Amend_Data {
                address: signal_flow::Address {
                    aspect: meta_signal_flow::Aspect::Psyche,
                    topic: meta_signal_flow::Topic("core".to_string()),
                    model: Model::Fable,
                },
                change: Change::Effort(effort_option.clone()),
            })),
        ] {
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(),
                input
            );
        }
    }
}

#[cfg(feature = "datom")]
#[test]
fn any_spelling_is_read_from_its_datom_as_written() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;
    let budget = || Budget {
        remaining: 16384,
        reader: ReaderBudget::try_from(16384).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };
    let fable = signal_flow::Address {
        aspect: meta_signal_flow::Aspect::Psyche,
        topic: meta_signal_flow::Topic("core".to_string()),
        model: Model::Fable,
    };
    for (written, effort_option) in [
        ("Some.xhigh", Some(Effort("xhigh".to_string()))),
        ("Some.max", Some(Effort("max".to_string()))),
        ("Some.ultra", Some(Effort("ultra".to_string()))),
        ("Some.«extra high»", Some(Effort("extra high".to_string()))),
        ("None", None),
    ] {
        assert_eq!(
            Potential::<Input>::from(format!("Configure.ModelEffort.{{ Haiku {written} }}"))
                .actualize(&mut budget())
                .unwrap(),
            Input::Configure(ConfigurationChange::ModelEffort(ModelEffort {
                model: Model::Haiku,
                effort_option: effort_option.clone(),
            })),
            "{written}"
        );
        assert_eq!(
            Potential::<Input>::from(format!(
                "Configure.Amend.{{ {{ Psyche core Fable }} Effort.{written} }}"
            ))
            .actualize(&mut budget())
            .unwrap(),
            Input::Configure(ConfigurationChange::Amend(meta_signal_flow::Amend_Data {
                address: fable.clone(),
                change: Change::Effort(effort_option),
            })),
            "{written}"
        );
    }
}
