//! ReadQueueDeliveries: the owner reads every unsettled hand-over of one
//! metaflow's queue. The expected texts are written out here, not computed
//! through the codec.

use flow_ethos::{Address, Aspect, Layer, QueuedRequest, Request, SentBy};
use meta_signal_flow::{
    AbsenceProof, ProcessIdentity, Query, QueueDeliveriesRead_Data, QueueDelivery,
    QueueDeliveryResolution, QueueDeliveryResolved_Data, QueueDeliveryStanding,
    QueueResolutionOutcome, QueueResolutionRecord, ReadQueueDeliveriesRejected_Data,
    ResolveQueueDeliveryRejected_Data, ResolverIdentity, Response, Undecided,
};
use signal_flow::HarnessKind;

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: "flow".into(),
        layer: Layer::Secondary,
    }
}

fn delivery(standing: QueueDeliveryStanding) -> QueueDelivery {
    QueueDelivery {
        queue_attempt_id: "queue-attempt-3".into(),
        flow_id: "f1c841".into(),
        native_session_id: "s1".into(),
        harness_kind: HarnessKind::Claude,
        queued_request: QueuedRequest {
            sent_by: SentBy::Sender(Address {
                aspect: Aspect::Psyche,
                topic: "core".into(),
                layer: Layer::Secondary,
            }),
            request: Request::Order("go".into()),
        },
        queue_delivery_standing: standing,
    }
}

fn read() -> Response {
    Response::QueueDeliveriesRead(QueueDeliveriesRead_Data {
        address: address(),
        queue_delivery_vector: vec![
            delivery(QueueDeliveryStanding::Prepared),
            delivery(QueueDeliveryStanding::Uncertain(None)),
            delivery(QueueDeliveryStanding::Uncertain(Some(
                Undecided::AbsenceUnprovable,
            ))),
            delivery(QueueDeliveryStanding::ReceiptRecorded),
            delivery(QueueDeliveryStanding::Absent(AbsenceProof::HistoryAbsent)),
        ],
    })
}

#[test]
fn read_queue_deliveries_round_trips_as_an_archive() {
    for reply in [
        read(),
        Response::ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data::UnknownAddress),
        Response::ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data::StoreRefused),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
    }
    let query = Query::ReadQueueDeliveries(address());
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
}

fn resolution(outcome: QueueResolutionOutcome) -> QueueDeliveryResolution {
    QueueDeliveryResolution {
        queue_attempt_id: "queue-attempt-3".into(),
        queue_resolution_outcome: outcome,
        queue_resolution_reason: "seen in the pane".into(),
    }
}

fn record() -> QueueResolutionRecord {
    QueueResolutionRecord {
        queue_resolution_outcome: QueueResolutionOutcome::Absent,
        resolver_identity: ResolverIdentity {
            process_identity: ProcessIdentity {
                process_id: 7,
                process_user_id: 1001,
                process_start_token: "99".into(),
            },
            flow_id_option: Some("aa034f".into()),
        },
        queue_resolution_reason: "seen in the pane".into(),
        unix_seconds: 1_790_000_000,
    }
}

#[test]
fn resolve_queue_delivery_round_trips_as_an_archive() {
    let queries = [
        Query::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Delivered)),
        Query::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Absent)),
    ];
    for query in queries {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    let replies = [
        Response::QueueDeliveryResolved(QueueDeliveryResolved_Data {
            address: address(),
            queue_delivery_resolution: resolution(QueueResolutionOutcome::Absent),
        }),
        Response::ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data::UnknownAttempt),
        Response::ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data::NotUncertain),
        Response::ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data::NotOldest),
        Response::ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data::EmptyReason),
        Response::ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data::StoreRefused),
        Response::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            queue_delivery_vector: vec![delivery(QueueDeliveryStanding::Resolved(record()))],
        }),
    ];
    for reply in replies {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
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
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn read_queue_deliveries_has_concrete_datoms() {
        assert_eq!(
            Potential::<Query>::from("ReadQueueDeliveries.{ Mind flow Secondary }")
                .actualize(&mut budget())
                .unwrap(),
            Query::ReadQueueDeliveries(address())
        );
        let one = Response::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            queue_delivery_vector: vec![delivery(QueueDeliveryStanding::Uncertain(Some(
                Undecided::PendingInHarness,
            )))],
        });
        let text = "QueueDeliveriesRead.{ { Mind flow Secondary } [ { queue-attempt-3 f1c841 s1 Claude { Sender.{ Psyche core Secondary } Order.go } Uncertain.Some.PendingInHarness } ] }";
        assert_eq!(one.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            one
        );
        assert_eq!(
            Response::ReadQueueDeliveriesRejected(
                ReadQueueDeliveriesRejected_Data::UnknownAddress
            )
            .datomize(vec![])
            .protosize()
            .compact(),
            "ReadQueueDeliveriesRejected.UnknownAddress"
        );
    }

    #[test]
    fn resolve_queue_delivery_has_concrete_datoms() {
        let query = Query::ResolveQueueDelivery(resolution(QueueResolutionOutcome::Absent));
        let text = "ResolveQueueDelivery.{ queue-attempt-3 Absent «seen in the pane» }";
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            query
        );
        let one = Response::QueueDeliveriesRead(QueueDeliveriesRead_Data {
            address: address(),
            queue_delivery_vector: vec![delivery(QueueDeliveryStanding::Resolved(record()))],
        });
        let text = "QueueDeliveriesRead.{ { Mind flow Secondary } [ { queue-attempt-3 f1c841 s1 Claude { Sender.{ Psyche core Secondary } Order.go } Resolved.{ Absent { { 7 1001 99 } Some.aa034f } «seen in the pane» 1790000000 } } ] }";
        assert_eq!(one.datomize(vec![]).protosize().compact(), text);
        assert_eq!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            one
        );
    }
}
