//! Inspect, Rebind and Withdraw: the owner reads a metaflow whole, repairs
//! where its flow lives, and takes one queued request out of its queue. The
//! expected texts are written out here, not computed through the codec.

use flow_ethos::{Address, Aspect, Layer, Lock};
use meta_signal_flow::{
    InspectRejected_Data, Inspected_Data, Query, Rebind_Data, RebindRejected_Data, Rebinding,
    Response, VisibleAttempt, VisibleBinding, VisibleQueueDelivery, VisibleQueueDeliveryStanding,
    VisibleQueueEvidence, Withdraw_Data, WithdrawRejected_Data,
};
use signal_flow::{
    EndpointSelection, FlowLifecycle, HarnessKind, HerdrRoute, HerdrRouteSelection, Process,
    VisibleFlowNode, VisibleMessage, VisibleMetaflow, VisibleNativeInput, VisibleOrigin,
    VisibleRequest, VisibleSender, VisibleState,
};

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: "flow".into(),
        layer: Layer::Secondary,
    }
}

fn sender() -> Address {
    Address {
        aspect: Aspect::Psyche,
        topic: "core".into(),
        layer: Layer::Secondary,
    }
}

fn node() -> VisibleFlowNode {
    VisibleFlowNode {
        session_id: "s1".into(),
        harness_kind: HarnessKind::Claude,
        endpoint_selection: EndpointSelection::Unavailable,
        herdr_route_selection: HerdrRouteSelection::Available(HerdrRoute {
            herdr_session_name: "default".into(),
            herdr_agent_name: "mind-flow".into(),
            herdr_pane_id: "w7B".into(),
            herdr_terminal_id: "t1".into(),
        }),
        visible_origin: VisibleOrigin::Unbound,
        flow_lifecycle: FlowLifecycle::Active,
    }
}

fn visible_entry() -> VisibleNativeInput {
    VisibleNativeInput {
        visible_sender: VisibleSender::Sender(sender()),
        address: address(),
        visible_message: VisibleMessage::Soft(VisibleRequest::Order("go".into())),
    }
}

fn attempt() -> VisibleAttempt {
    VisibleAttempt::Queued(VisibleQueueDelivery {
        native_session_id: "s1".into(),
        harness_kind: HarnessKind::Claude,
        visible_queue_evidence: VisibleQueueEvidence::Logical(visible_entry()),
        visible_queue_delivery_standing: VisibleQueueDeliveryStanding::Prepared,
    })
}

fn inspected() -> Response {
    Response::Inspected(Inspected_Data {
        visible_metaflow: VisibleMetaflow {
            address: address(),
            visible_state: VisibleState::Awake,
            integer: 1,
            visible_native_input_vector: vec![visible_entry()],
        },
        visible_binding: VisibleBinding {
            visible_flow_node_option: Some(node()),
            process_option: Some(Process {
                pid: 4242,
                started: 17,
            }),
        },
        lock_vector: vec![Lock {
            sender: sender(),
            address: address(),
            until: 1_790_000_060,
        }],
        visible_attempt_vector: vec![attempt()],
    })
}

fn queries() -> Vec<Query> {
    vec![
        Query::Inspect(address()),
        Query::Rebind(Rebind_Data {
            address: address(),
            rebinding: Rebinding::Pane("w7B".into()),
        }),
        Query::Rebind(Rebind_Data {
            address: address(),
            rebinding: Rebinding::Flow("f1c841".into()),
        }),
        Query::Withdraw(Withdraw_Data {
            address: address(),
            logical_delivery_id: "logical-7".into(),
        }),
    ]
}

fn replies() -> Vec<Response> {
    vec![
        inspected(),
        Response::Inspected(Inspected_Data {
            visible_metaflow: VisibleMetaflow {
                address: address(),
                visible_state: VisibleState::Asleep,
                integer: 0,
                visible_native_input_vector: vec![],
            },
            visible_binding: VisibleBinding {
                visible_flow_node_option: None,
                process_option: None,
            },
            lock_vector: vec![],
            visible_attempt_vector: vec![],
        }),
        Response::InspectRejected(InspectRejected_Data::UnknownAddress),
        Response::Rebound(address()),
        Response::RebindRejected(RebindRejected_Data::UnknownAddress),
        Response::RebindRejected(RebindRejected_Data::PaneAbsent),
        Response::RebindRejected(RebindRejected_Data::FlowUnknown),
        Response::RebindRejected(RebindRejected_Data::NotAwake),
        Response::RebindRejected(RebindRejected_Data::HerdrUnavailable),
        Response::RebindRejected(RebindRejected_Data::FlowTaken(address())),
        Response::Withdrawn,
        Response::WithdrawRejected(WithdrawRejected_Data::UnknownAddress),
        Response::WithdrawRejected(WithdrawRejected_Data::NotQueued),
    ]
}

#[test]
fn repairs_round_trip_as_an_archive() {
    for query in queries() {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    for reply in replies() {
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
    fn repair_queries_have_concrete_datoms() {
        let texts = [
            "Inspect.{ Mind flow Secondary }",
            "Rebind.{ { Mind flow Secondary } Pane.w7B }",
            "Rebind.{ { Mind flow Secondary } Flow.f1c841 }",
            "Withdraw.{ { Mind flow Secondary } logical-7 }",
        ];
        for (query, text) in queries().into_iter().zip(texts) {
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
            assert_eq!(
                Potential::<Query>::from(text)
                    .actualize(&mut budget())
                    .unwrap(),
                query
            );
        }
    }

    #[test]
    fn repair_replies_have_concrete_datoms() {
        let texts = [
            "Inspected.{ { { Mind flow Secondary } Awake 1 [ { Sender.{ Psyche core Secondary } { Mind flow Secondary } Soft.Order.go } ] } { Some.{ s1 Claude Unavailable Available.{ default mind-flow w7B t1 } Unbound Active } Some.{ 4242 17 } } [ { { Psyche core Secondary } { Mind flow Secondary } 1790000060 } ] [ Queued.{ s1 Claude Logical.{ Sender.{ Psyche core Secondary } { Mind flow Secondary } Soft.Order.go } Prepared } ] }",
            "Inspected.{ { { Mind flow Secondary } Asleep 0 [] } { None None } [] [] }",
            "InspectRejected.UnknownAddress",
            "Rebound.{ Mind flow Secondary }",
            "RebindRejected.UnknownAddress",
            "RebindRejected.PaneAbsent",
            "RebindRejected.FlowUnknown",
            "RebindRejected.NotAwake",
            "RebindRejected.HerdrUnavailable",
            "RebindRejected.FlowTaken.{ Mind flow Secondary }",
            "Withdrawn",
            "WithdrawRejected.UnknownAddress",
            "WithdrawRejected.NotQueued",
        ];
        for (reply, text) in replies().into_iter().zip(texts) {
            assert_eq!(reply.datomize(vec![]).protosize().compact(), text);
            assert_eq!(
                Potential::<Response>::from(text)
                    .actualize(&mut budget())
                    .unwrap(),
                reply
            );
        }
    }
}
