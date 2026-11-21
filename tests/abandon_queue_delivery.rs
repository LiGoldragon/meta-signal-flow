//! AbandonQueueDelivery: the owner gives up one unfinished attempt, and
//! an attempt given up lists as Abandoned with who gave it up and why. The
//! expected texts are written out here, not computed through the codec.

use flow_ethos::{Address, Aspect, Model, RequestId, Topic};
use meta_signal_flow::{
    AbandonQueueDelivery_Data, AbandonQueueDeliveryRejected_Data, AbandonmentCause,
    AbandonmentRecord, Input, Output, ProcessId, ProcessIdentity, ProcessStartToken, ProcessUserId,
    QueueAttemptId, QueueDeliveriesRead_Data, QueueDeliveryAbandoned_Data, QueueResolutionReason,
    ResolverIdentity, UnixSeconds, VisibleAbandonmentCause, VisibleAbandonmentRecord,
    VisibleQueueDelivery, VisibleQueueDeliveryStanding, VisibleQueueEvidence,
    VisibleQueueEvidence_Legacy_Data,
};
use signal_flow::{FlowId, HarnessKind, NativeSessionId, VisibleRequest, VisibleSender};

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: Topic("flow".to_string()),
        model: Model::Sol,
    }
}

fn owner() -> ResolverIdentity {
    ResolverIdentity {
        process_identity: ProcessIdentity {
            process_id: ProcessId(7),
            process_user_id: ProcessUserId(1001),
            process_start_token: ProcessStartToken("99".to_string()),
        },
        flow_id_option: None,
    }
}

fn record(abandonment_cause: AbandonmentCause) -> AbandonmentRecord {
    AbandonmentRecord {
        abandonment_cause,
        queue_resolution_reason: QueueResolutionReason("its seat is gone".to_string()),
        unix_seconds: UnixSeconds(1_790_000_000),
    }
}

fn causes() -> Vec<AbandonmentCause> {
    vec![
        AbandonmentCause::Owner(owner()),
        AbandonmentCause::FlowRetired(FlowId("74e4d1".to_string())),
        AbandonmentCause::RequestEnded(RequestId("k7ph".to_string())),
    ]
}

fn abandoned(visible_abandonment_cause: VisibleAbandonmentCause) -> VisibleQueueDelivery {
    VisibleQueueDelivery {
        native_session_id: NativeSessionId("s1".to_string()),
        harness_kind: HarnessKind::Codex,
        visible_queue_evidence: VisibleQueueEvidence::Legacy(VisibleQueueEvidence_Legacy_Data {
            visible_sender: VisibleSender::Unbound,
            visible_request: VisibleRequest::Order("go".into()),
        }),
        visible_queue_delivery_standing: VisibleQueueDeliveryStanding::Abandoned(
            VisibleAbandonmentRecord {
                visible_abandonment_cause,
                queue_resolution_reason: QueueResolutionReason("its seat is gone".to_string()),
                unix_seconds: UnixSeconds(1_790_000_000),
            },
        ),
    }
}

#[test]
fn abandon_queue_delivery_and_its_answers_round_trip_as_archives() {
    let query = Input::AbandonQueueDelivery(AbandonQueueDelivery_Data {
        queue_attempt_id: QueueAttemptId("queue-attempt-3".to_string()),
        queue_resolution_reason: QueueResolutionReason("its seat is gone".to_string()),
    });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    let mut replies = vec![
        Output::AbandonQueueDeliveryRejected(AbandonQueueDeliveryRejected_Data::UnknownAttempt),
        Output::AbandonQueueDeliveryRejected(AbandonQueueDeliveryRejected_Data::AlreadyFinished),
        Output::AbandonQueueDeliveryRejected(AbandonQueueDeliveryRejected_Data::EmptyReason),
    ];
    for cause in causes() {
        replies.push(Output::QueueDeliveryAbandoned(
            QueueDeliveryAbandoned_Data {
                address: address(),
                abandonment_record: record(cause),
            },
        ));
    }
    for cause in [
        VisibleAbandonmentCause::Owner(owner().process_identity),
        VisibleAbandonmentCause::FlowRetired,
        VisibleAbandonmentCause::RequestEnded(RequestId("k7ph".to_string())),
    ] {
        replies.push(Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            visible_queue_delivery_vector: vec![abandoned(cause)],
        }));
    }
    for reply in replies {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            reply
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

    fn input_reads_and_prints_back_as(text: &str, input: Input) {
        assert_eq!(input.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Input>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            input
        );
    }

    fn output_reads_and_prints_back_as(text: &str, output: Output) {
        assert_eq!(output.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Output>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            output
        );
    }

    #[test]
    fn abandon_queue_delivery_has_concrete_datoms() {
        input_reads_and_prints_back_as(
            "AbandonQueueDelivery.{ queue-attempt-3 «its seat is gone» }",
            Input::AbandonQueueDelivery(AbandonQueueDelivery_Data {
                queue_attempt_id: QueueAttemptId("queue-attempt-3".to_string()),
                queue_resolution_reason: QueueResolutionReason("its seat is gone".to_string()),
            }),
        );
        output_reads_and_prints_back_as(
            "QueueDeliveryAbandoned.{ { Mind flow Sol } { Owner.{ { 7 1001 99 } None } «its seat is gone» 1790000000 } }",
            Output::QueueDeliveryAbandoned(QueueDeliveryAbandoned_Data {
                address: address(),
                abandonment_record: record(AbandonmentCause::Owner(owner())),
            }),
        );
        output_reads_and_prints_back_as(
            "QueueDeliveriesRead.{ { Mind flow Sol } [ { s1 Codex Legacy.{ Unbound Order.go } Abandoned.{ FlowRetired «its seat is gone» 1790000000 } } ] }",
            Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
                address: address(),
                visible_queue_delivery_vector: vec![abandoned(
                    VisibleAbandonmentCause::FlowRetired,
                )],
            }),
        );
        output_reads_and_prints_back_as(
            "QueueDeliveriesRead.{ { Mind flow Sol } [ { s1 Codex Legacy.{ Unbound Order.go } Abandoned.{ RequestEnded.k7ph «its seat is gone» 1790000000 } } ] }",
            Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
                address: address(),
                visible_queue_delivery_vector: vec![abandoned(
                    VisibleAbandonmentCause::RequestEnded(RequestId("k7ph".to_string())),
                )],
            }),
        );
        output_reads_and_prints_back_as(
            "QueueDeliveriesRead.{ { Mind flow Sol } [ { s1 Codex Legacy.{ Unbound Order.go } Abandoned.{ Owner.{ 7 1001 99 } «its seat is gone» 1790000000 } } ] }",
            Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
                address: address(),
                visible_queue_delivery_vector: vec![abandoned(VisibleAbandonmentCause::Owner(
                    owner().process_identity,
                ))],
            }),
        );
        for (text, rejection) in [
            (
                "AbandonQueueDeliveryRejected.UnknownAttempt",
                AbandonQueueDeliveryRejected_Data::UnknownAttempt,
            ),
            (
                "AbandonQueueDeliveryRejected.AlreadyFinished",
                AbandonQueueDeliveryRejected_Data::AlreadyFinished,
            ),
            (
                "AbandonQueueDeliveryRejected.EmptyReason",
                AbandonQueueDeliveryRejected_Data::EmptyReason,
            ),
        ] {
            output_reads_and_prints_back_as(text, Output::AbandonQueueDeliveryRejected(rejection));
        }
    }
}
