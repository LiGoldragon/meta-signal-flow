//! An effort is Default or the native spelling, held per model and per
//! permaflow; Flow knows no level, so any spelling is configured and
//! carried as written.
use meta_signal_flow::{
    Amend_Data, Change, ConfigurationChange, Effort, Input, Model, ModelEffort,
};

/// Spellings a harness takes, spellings no harness takes, and Default.
fn efforts() -> Vec<Effort> {
    [
        "low", "medium", "high", "xhigh", "max", "minimal", "none", "ultra",
    ]
    .into_iter()
    .map(|spelling| Effort::Native(spelling.to_string()))
    .chain([Effort::Default])
    .collect()
}

fn fable() -> signal_flow::Address {
    signal_flow::Address {
        aspect: meta_signal_flow::Aspect::Psyche,
        topic: meta_signal_flow::Topic("core".to_string()),
        model: Model::Fable,
    }
}

#[test]
fn every_effort_round_trips_unchanged_on_the_wire() {
    for effort in efforts() {
        for input in [
            Input::Configure(ConfigurationChange::ModelEffort(ModelEffort {
                model: Model::Haiku,
                effort: effort.clone(),
            })),
            Input::Configure(ConfigurationChange::Amend(Amend_Data {
                address: fable(),
                change: Change::Effort(effort.clone()),
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
    for (written, effort) in [
        ("Native.xhigh", Effort::Native("xhigh".to_string())),
        ("Native.max", Effort::Native("max".to_string())),
        ("Native.ultra", Effort::Native("ultra".to_string())),
        (
            "Native.«extra high»",
            Effort::Native("extra high".to_string()),
        ),
        ("Default", Effort::Default),
    ] {
        assert_eq!(
            Potential::<Input>::from(format!("Configure.ModelEffort.{{ Sol {written} }}"))
                .actualize(&mut budget())
                .unwrap(),
            Input::Configure(ConfigurationChange::ModelEffort(ModelEffort {
                model: Model::Sol,
                effort: effort.clone(),
            })),
            "{written}"
        );
        assert_eq!(
            Potential::<Input>::from(format!(
                "Configure.Amend.{{ {{ Psyche core Fable }} Effort.{written} }}"
            ))
            .actualize(&mut budget())
            .unwrap(),
            Input::Configure(ConfigurationChange::Amend(Amend_Data {
                address: fable(),
                change: Change::Effort(effort),
            })),
            "{written}"
        );
    }
}
