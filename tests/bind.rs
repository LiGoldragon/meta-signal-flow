//! Bind: a session's transcript becomes the flow of a permaflow. The
//! expected texts are written out here, not computed through the codec.

use flow_ethos::{Address, Aspect, Model, Topic};
use meta_signal_flow::{BindRefused_Data, Bound_Data, Input, Output};
use signal_flow::{FlowId, TranscriptPath};

fn address() -> Address {
    Address {
        aspect: Aspect::Psyche,
        topic: Topic("test".into()),
        model: Model::Opus,
    }
}

fn path() -> TranscriptPath {
    TranscriptPath(
        "/home/li/.claude/projects/-home-li-primary/15c0ce3a-7d41-4b2e-9f10-0c8a5e2d6b77.jsonl"
            .into(),
    )
}

fn outputs() -> Vec<Output> {
    vec![
        Output::Bound(Bound_Data {
            address: address(),
            flow_id: FlowId("15c0ce".into()),
            flow_id_option: Some(FlowId("a68420".into())),
        }),
        Output::Bound(Bound_Data {
            address: address(),
            flow_id: FlowId("15c0ce".into()),
            flow_id_option: None,
        }),
        Output::BindRefused(BindRefused_Data::UnknownPermaflow(address())),
        Output::BindRefused(BindRefused_Data::Unreadable(path())),
    ]
}

#[test]
fn bind_round_trips_as_an_archive() {
    let query = Input::Bind(meta_signal_flow::Bind_Data {
        address: address(),
        transcript_path: path(),
    });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    for value in outputs() {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            value
        );
    }
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

    const BIND: &str = "Bind.{ { Psyche test Opus } /home/li/.claude/projects/-home-li-primary/15c0ce3a-7d41-4b2e-9f10-0c8a5e2d6b77.jsonl }";

    const OUTPUT_TEXTS: [&str; 4] = [
        "Bound.{ { Psyche test Opus } 15c0ce Some.a68420 }",
        "Bound.{ { Psyche test Opus } 15c0ce None }",
        "BindRefused.UnknownPermaflow.{ Psyche test Opus }",
        "BindRefused.Unreadable./home/li/.claude/projects/-home-li-primary/15c0ce3a-7d41-4b2e-9f10-0c8a5e2d6b77.jsonl",
    ];

    #[test]
    fn bind_has_concrete_datoms() {
        let input = Input::Bind(meta_signal_flow::Bind_Data {
            address: address(),
            transcript_path: path(),
        });
        assert_eq!(
            Potential::<Input>::from(BIND)
                .actualize(&mut budget())
                .unwrap(),
            input
        );
        assert_eq!(input.datomize(vec![]).protosize().compact(), BIND);
    }

    #[test]
    fn bound_and_both_refusals_have_concrete_datoms() {
        for (value, text) in outputs().into_iter().zip(OUTPUT_TEXTS) {
            assert_eq!(value.datomize(vec![]).protosize().compact(), text);
            assert_eq!(
                Potential::<Output>::from(text)
                    .actualize(&mut budget())
                    .unwrap(),
                value
            );
        }
    }
}
