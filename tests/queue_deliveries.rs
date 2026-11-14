//! ReadQueueDeliveries: the owner reads every unsettled hand-over of one
//! metaflow's queue. The expected texts are written out here, not computed
//! through the codec.

use flow_ethos::{Address, Aspect, Model, Topic};
use meta_signal_flow::{
    AbsenceProof, Input, Output, ProcessId, ProcessIdentity, ProcessStartToken, ProcessUserId,
    QueueAttemptId, QueueDeliveriesRead_Data, QueueDeliveryResolution, QueueDeliveryResolved_Data,
    QueueResolutionOutcome, QueueResolutionReason, ReadQueueDeliveriesRejected_Data, Undecided,
    UnixSeconds, VisibleQueueDelivery, VisibleQueueDeliveryResolution,
    VisibleQueueDeliveryStanding, VisibleQueueEvidence, VisibleQueueEvidence_Legacy_Data,
    VisibleQueueResolutionRecord, VisibleResolveQueueDeliveryRejected,
};
use signal_flow::{HarnessKind, NativeSessionId, VisibleRequest, VisibleSender};

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: Topic("flow".to_string()),
        model: Model::Sol,
    }
}

fn delivery(standing: VisibleQueueDeliveryStanding) -> VisibleQueueDelivery {
    VisibleQueueDelivery {
        native_session_id: NativeSessionId("s1".to_string()),
        harness_kind: HarnessKind::Claude,
        visible_queue_evidence: VisibleQueueEvidence::Legacy(VisibleQueueEvidence_Legacy_Data {
            visible_sender: VisibleSender::Sender(Address {
                aspect: Aspect::Psyche,
                topic: Topic("core".to_string()),
                model: Model::Opus,
            }),
            visible_request: VisibleRequest::Order("go".into()),
        }),
        visible_queue_delivery_standing: standing,
    }
}

fn read() -> Output {
    Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
        address: address(),
        visible_queue_delivery_vector: vec![
            delivery(VisibleQueueDeliveryStanding::Prepared),
            delivery(VisibleQueueDeliveryStanding::Uncertain(None)),
            delivery(VisibleQueueDeliveryStanding::Uncertain(Some(
                Undecided::AbsenceUnprovable,
            ))),
            delivery(VisibleQueueDeliveryStanding::ReceiptRecorded),
            delivery(VisibleQueueDeliveryStanding::Absent(
                AbsenceProof::HistoryAbsent,
            )),
        ],
    })
}

#[test]
fn read_queue_deliveries_round_trips_as_an_archive() {
    for reply in [
        read(),
        Output::ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data::UnknownAddress),
        Output::ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data::StoreRefused),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
    }
    let query = Input::ReadQueueDeliveries(address());
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
}

fn resolution(outcome: QueueResolutionOutcome) -> QueueDeliveryResolution {
    QueueDeliveryResolution {
        queue_attempt_id: QueueAttemptId("queue-attempt-3".to_string()),
        queue_resolution_outcome: outcome,
        queue_resolution_reason: QueueResolutionReason("seen in the pane".to_string()),
    }
}

fn record() -> VisibleQueueResolutionRecord {
    VisibleQueueResolutionRecord {
        queue_resolution_outcome: QueueResolutionOutcome::Absent,
        process_identity: ProcessIdentity {
            process_id: ProcessId(7),
            process_user_id: ProcessUserId(1001),
            process_start_token: ProcessStartToken("99".to_string()),
        },
        queue_resolution_reason: QueueResolutionReason("seen in the pane".to_string()),
        unix_seconds: UnixSeconds(1_790_000_000),
    }
}

#[test]
fn resolve_queue_delivery_round_trips_as_an_archive() {
    let queries = [
        Input::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Delivered)),
        Input::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Absent)),
    ];
    for query in queries {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    let replies = [
        Output::QueueDeliveryResolved(QueueDeliveryResolved_Data {
            address: address(),
            visible_queue_delivery_resolution: VisibleQueueDeliveryResolution {
                queue_resolution_outcome: QueueResolutionOutcome::Absent,
                queue_resolution_reason: QueueResolutionReason("seen in the pane".to_string()),
            },
        }),
        Output::ResolveQueueDeliveryRejected(VisibleResolveQueueDeliveryRejected::UnknownAttempt),
        Output::ResolveQueueDeliveryRejected(VisibleResolveQueueDeliveryRejected::NotUncertain),
        Output::ResolveQueueDeliveryRejected(VisibleResolveQueueDeliveryRejected::NotOldest),
        Output::ResolveQueueDeliveryRejected(VisibleResolveQueueDeliveryRejected::EmptyReason),
        Output::ResolveQueueDeliveryRejected(VisibleResolveQueueDeliveryRejected::StoreRefused),
        Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            visible_queue_delivery_vector: vec![delivery(VisibleQueueDeliveryStanding::Resolved(
                record(),
            ))],
        }),
    ];
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

    #[test]
    fn read_queue_deliveries_has_concrete_datoms() {
        assert_eq!(
            Potential::<Input>::from("ReadQueueDeliveries.{ Mind flow Sol }")
                .actualize(&mut budget())
                .unwrap(),
            Input::ReadQueueDeliveries(address())
        );
        let one = Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            visible_queue_delivery_vector: vec![delivery(VisibleQueueDeliveryStanding::Uncertain(
                Some(Undecided::PendingInHarness),
            ))],
        });
        let text = "QueueDeliveriesRead.{ { Mind flow Sol } [ { s1 Claude Legacy.{ Sender.{ Psyche core Opus } Order.go } Uncertain.Some.PendingInHarness } ] }";
        assert_eq!(one.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Output>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            one
        );
        assert_eq!(
            Output::ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data::UnknownAddress)
                .datomize(vec![])
                .protosize()
                .compact(),
            "ReadQueueDeliveriesRejected.UnknownAddress"
        );
    }

    #[test]
    fn resolve_queue_delivery_has_concrete_datoms() {
        let query = Input::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Absent));
        let text = "ResolveQueueDelivery.{ queue-attempt-3 Absent «seen in the pane» }";
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Input>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            query
        );
        let one = Output::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            visible_queue_delivery_vector: vec![delivery(VisibleQueueDeliveryStanding::Resolved(
                record(),
            ))],
        });
        let text = "QueueDeliveriesRead.{ { Mind flow Sol } [ { s1 Claude Legacy.{ Sender.{ Psyche core Opus } Order.go } Resolved.{ Absent { 7 1001 99 } «seen in the pane» 1790000000 } } ] }";
        assert_eq!(one.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Output>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            one
        );
    }
}
