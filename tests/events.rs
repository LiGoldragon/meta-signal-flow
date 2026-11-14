//! ReadEvents: the owner reads the harness events Flow holds for one flow.
//! The expected texts are written out here, not computed through the codec.

use meta_signal_flow::{
    EventsRead_Data, HarnessEventMetadata, Input, Output, ReadEventsRejected_Data, StopMetadata,
};
use signal_flow::{
    BackgroundTask, BackgroundTaskId, BackgroundTaskKind, BackgroundTaskStatus, BackgroundWork,
    Description, FlowId, NativePrompt, NativeSessionId, PromptId, PromptSha256, TranscriptPath,
};

fn events_read() -> Output {
    Output::EventsRead(EventsRead_Data {
        flow_id: FlowId("f1c841".to_string()),
        harness_event_metadata_vector: vec![
            HarnessEventMetadata::Started(Some(NativeSessionId("s1".to_string()))),
            HarnessEventMetadata::ToolUsed("Bash".into()),
            HarnessEventMetadata::Stopped(StopMetadata {
                native_session_id_option: Some(NativeSessionId("s1".to_string())),
                transcript_path_option: Some(TranscriptPath("/tmp/transcript".into())),
                background_work: BackgroundWork(vec![BackgroundTask {
                    background_task_id: BackgroundTaskId("job1".into()),
                    background_task_kind: BackgroundTaskKind::Shell,
                    background_task_status: BackgroundTaskStatus("running".into()),
                    description: Description("build".into()),
                }]),
            }),
            HarnessEventMetadata::PromptSubmitted(NativePrompt {
                native_session_id_option: Some(NativeSessionId("s1".into())),
                prompt_id_option: Some(PromptId("p1".into())),
                prompt_sha256: PromptSha256("digest".into()),
            }),
            HarnessEventMetadata::Ended(None),
            HarnessEventMetadata::Cleared(Some(NativeSessionId("s1".to_string()))),
            HarnessEventMetadata::Started(Some(NativeSessionId("s2".to_string()))),
        ],
    })
}

#[test]
fn read_events_round_trips_as_an_archive() {
    for reply in [
        events_read(),
        Output::ReadEventsRejected(ReadEventsRejected_Data::UnknownFlow),
        Output::ReadEventsRejected(ReadEventsRejected_Data::StoreRefused),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
    }
    let query = Input::ReadEvents(FlowId("f1c841".to_string()));
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

    #[test]
    fn stop_metadata_has_exactly_three_positions() {
        let text = "Stopped.{ None None [] }";
        let stop = Potential::<HarnessEventMetadata>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            stop,
            HarnessEventMetadata::Stopped(StopMetadata {
                native_session_id_option: None,
                transcript_path_option: None,
                background_work: BackgroundWork(vec![]),
            })
        );
        assert_eq!(stop.datomize(vec![]).protosize().compact(), text);
        for old in [
            "Stopped.{ None None None [] }",
            "Stopped.{ None None Some.«assistant marker» [] }",
        ] {
            assert!(
                Potential::<HarnessEventMetadata>::from(old)
                    .actualize(&mut budget())
                    .is_err(),
                "previous four-position Stop must be refused: {old}"
            );
        }
    }

    #[test]
    fn read_events_has_concrete_datoms() {
        let query = Potential::<Input>::from("ReadEvents.f1c841")
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query, Input::ReadEvents(FlowId("f1c841".to_string())));
        assert_eq!(
            events_read().datomize(vec![]).protosize().compact(),
            "EventsRead.{ f1c841 [ Started.Some.s1 ToolUsed.Bash Stopped.{ Some.s1 Some./tmp/transcript [ { job1 Shell running build } ] } PromptSubmitted.{ Some.s1 Some.p1 digest } Ended.None Cleared.Some.s1 Started.Some.s2 ] }"
        );
        assert_eq!(
            Output::ReadEventsRejected(ReadEventsRejected_Data::UnknownFlow)
                .datomize(vec![])
                .protosize()
                .compact(),
            "ReadEventsRejected.UnknownFlow"
        );
        let read =
            Potential::<Output>::from("EventsRead.{ f1c841 [ Started.Some.s1 ToolUsed.Bash Stopped.{ Some.s1 Some./tmp/transcript [ { job1 Shell running build } ] } PromptSubmitted.{ Some.s1 Some.p1 digest } Ended.None Cleared.Some.s1 Started.Some.s2 ] }")
                .actualize(&mut budget())
                .unwrap();
        assert_eq!(read, events_read());
    }
}
