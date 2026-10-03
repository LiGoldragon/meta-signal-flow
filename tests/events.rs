//! ReadEvents: the owner reads the harness events Flow holds for one flow.
//! The expected texts are written out here, not computed through the codec.

use meta_signal_flow::{EventsRead_Data, Query, ReadEventsRejected_Data, Response};
use signal_flow::Event;

fn events_read() -> Response {
    Response::EventsRead(EventsRead_Data {
        flow_id: "f1c841".into(),
        event_vector: vec![
            Event::Started,
            Event::ToolUsed("Bash".into()),
            Event::Stopped,
        ],
    })
}

#[test]
fn read_events_round_trips_as_an_archive() {
    for reply in [
        events_read(),
        Response::ReadEventsRejected(ReadEventsRejected_Data::UnknownFlow),
        Response::ReadEventsRejected(ReadEventsRejected_Data::StoreRefused),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
    }
    let query = Query::ReadEvents("f1c841".into());
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
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
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn read_events_has_concrete_datoms() {
        let query = Potential::<Query>::from("ReadEvents.f1c841")
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query, Query::ReadEvents("f1c841".into()));
        assert_eq!(
            events_read().datomize(vec![]).protosize().compact(),
            "EventsRead.{ f1c841 [ Started ToolUsed.Bash Stopped ] }"
        );
        assert_eq!(
            Response::ReadEventsRejected(ReadEventsRejected_Data::UnknownFlow)
                .datomize(vec![])
                .protosize()
                .compact(),
            "ReadEventsRejected.UnknownFlow"
        );
        let read =
            Potential::<Response>::from("EventsRead.{ f1c841 [ Started ToolUsed.Bash Stopped ] }")
                .actualize(&mut budget())
                .unwrap();
        assert_eq!(read, events_read());
    }
}
