//! The message typed into a pane is one complete priority-headed datom.

use flow_ethos::{Address, Aspect, Layer};
use meta_signal_flow::{Body, Kind, Letter, Message, Sender};

fn sender() -> Sender {
    Sender {
        address: Address {
            aspect: Aspect::Psyche,
            topic: "core".into(),
            layer: Layer::Secondary,
        },
        flow_id: "1a2b3c".into(),
    }
}

#[test]
fn messages_round_trip_as_archives() {
    for message in [
        Message::Soft(Letter {
            kind: Kind::Notice,
            sender: sender(),
            body: Body::Words("batch 1 is in a commit".into()),
        }),
        Message::MiddleAbrupt(Letter {
            kind: Kind::Psyche,
            sender: sender(),
            body: Body::Psyche(meta_signal_flow::PsycheWords {
                first_string: "the living, direct, 12:40".into(),
                second_string: "his words here".into(),
            }),
        }),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&message).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Message, rkyv::rancor::Error>(&archive).unwrap(),
            message
        );
    }
}

#[cfg(feature = "datom")]
#[test]
fn a_guillemet_string_inside_the_letter_struct_reads_and_writes() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let mut budget = Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    };
    let expected = Message::Soft(Letter {
        kind: Kind::Notice,
        sender: sender(),
        body: Body::Words("batch 1 is in a commit".into()),
    });
    let text =
        "Soft.{ Notice { { Psyche core Secondary } 1a2b3c } Words.«batch 1 is in a commit» }";
    assert_eq!(expected.datomize(vec![]).protosize().compact(), text);
    assert_eq!(
        Potential::<Message>::from(text)
            .actualize(&mut budget)
            .unwrap(),
        expected
    );
}

#[cfg(feature = "datom")]
#[test]
fn a_cohort_is_one_vector_of_complete_messages() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let cohort = vec![
        Message::HardAbrupt(Letter {
            kind: Kind::Order,
            sender: sender(),
            body: Body::Words("first".into()),
        }),
        Message::Soft(Letter {
            kind: Kind::Question,
            sender: Sender {
                address: Address {
                    aspect: Aspect::Mind,
                    topic: "flow".into(),
                    layer: Layer::Tertiary,
                },
                flow_id: "4d5e6f".into(),
            },
            body: Body::Words("second?".into()),
        }),
    ];
    let text = cohort.datomize(vec![]).protosize().compact();
    let mut budget = Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    };
    let read = Potential::<Vec<Message>>::from(text.as_str())
        .actualize(&mut budget)
        .unwrap();
    assert_eq!(read, cohort);
    assert_eq!(read.datomize(vec![]).protosize().compact(), text);
}
