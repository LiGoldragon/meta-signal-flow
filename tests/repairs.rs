//! Inspect, Rebind and Withdraw: the owner reads a metaflow whole, repairs
//! where its flow lives, and takes one queued request out of its queue. The
//! expected texts are written out here, not computed through the codec.

use flow_ethos::{
    Address, Aspect, Lock, LogicalDeliveryId, Model, PermaflowState, Pid, Started, Topic, Until,
};
use meta_signal_flow::{
    Attempt, Attempts, Binding, InspectRejected_Data, Inspected_Data, OldSeat, Query, Rebind_Data,
    RebindRejected_Data, Rebinding, Response, RestartFailure, RestartPhase, RestartRecord,
    VisibleAttempt, VisiblePermaflow, VisibleQueueDelivery, VisibleQueueDeliveryStanding,
    VisibleQueueEvidence, Withdraw_Data, WithdrawRejected_Data,
};
use signal_flow::{
    DeliveredMessage, DeliveredRequest, EndpointSelection, FlowId, FlowLifecycle, FlowNode,
    HarnessKind, HerdrAgentName, HerdrPaneId, HerdrRoute, HerdrRouteSelection, HerdrSessionName,
    HerdrTerminalId, NativeSessionId, Origin, Process, RequestId, SessionId, VisibleNativeInput,
    VisibleSender,
};
use signal_flow::{
    HandoverCloseRefusal, HandoverOwner, HandoverOwnerRefusal, HandoverPending, Launched,
    Predecessor, ReapPending, ReapStanding,
};

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: Topic("flow".to_string()),
        model: Model::Sol,
    }
}

fn sender() -> Address {
    Address {
        aspect: Aspect::Psyche,
        topic: Topic("core".to_string()),
        model: Model::Opus,
    }
}

fn node() -> FlowNode {
    FlowNode {
        flow_id: FlowId("f1c841".to_string()),
        session_id: SessionId("s1".to_string()),
        harness_kind: HarnessKind::Claude,
        endpoint_selection: EndpointSelection::Unavailable,
        herdr_route_selection: HerdrRouteSelection::Available(HerdrRoute {
            herdr_session_name: HerdrSessionName("default".to_string()),
            herdr_agent_name: HerdrAgentName("mind-flow".to_string()),
            herdr_pane_id: HerdrPaneId("w7B".to_string()),
            herdr_terminal_id: HerdrTerminalId("t1".to_string()),
        }),
        origin: Origin::Unbound,
        flow_lifecycle: FlowLifecycle::Active,
    }
}

fn visible_entry() -> VisibleNativeInput {
    VisibleNativeInput {
        visible_sender: VisibleSender::Sender(sender()),
        delivered_message: DeliveredMessage::Soft(DeliveredRequest::Order(
            signal_flow::Order_Data {
                request_id: RequestId("req1".to_string()),
                string: "go".into(),
            },
        )),
    }
}

fn attempt() -> VisibleAttempt {
    VisibleAttempt::Queued(VisibleQueueDelivery {
        native_session_id: NativeSessionId("s1".to_string()),
        harness_kind: HarnessKind::Claude,
        visible_queue_evidence: VisibleQueueEvidence::Logical(visible_entry()),
        visible_queue_delivery_standing: VisibleQueueDeliveryStanding::Prepared,
    })
}

fn inspected() -> Response {
    Response::Inspected(Inspected_Data {
        visible_permaflow: VisiblePermaflow {
            address: address(),
            permaflow_state: PermaflowState::Awake(FlowId("f1c841".to_string())),
            flow_id_vector: vec![FlowId("9a83ec".to_string())],
            visible_native_input_vector: vec![visible_entry()],
        },
        binding: Binding {
            flow_node_option: Some(node()),
            process_option: Some(Process {
                pid: Pid(4242),
                started: Started(17),
            }),
        },
        lock_vector: vec![Lock {
            first_address: sender(),
            second_address: address(),
            until: Until(1_790_000_060),
        }],
        visible_attempt_vector: vec![attempt()],
        restart_record_option: Some(restart_record()),
    })
}

/// A restart that failed at its bind, on its second run: the record keeps
/// the session, the old route and the birth of the old harness.
fn restart_record() -> RestartRecord {
    RestartRecord {
        flow_id: FlowId("f1c841".to_string()),
        native_session_id: NativeSessionId("s1".to_string()),
        address: address(),
        old_seat: OldSeat {
            herdr_route: HerdrRoute {
                herdr_session_name: HerdrSessionName("default".to_string()),
                herdr_agent_name: HerdrAgentName("mind-flow".to_string()),
                herdr_pane_id: HerdrPaneId("w7B".to_string()),
                herdr_terminal_id: HerdrTerminalId("t1".to_string()),
            },
            process_option: Some(Process {
                pid: Pid(4242),
                started: Started(17),
            }),
        },
        restart_phase: RestartPhase::Failed(RestartFailure::BindMismatch),
        attempts: Attempts(2),
    }
}

fn queries() -> Vec<Query> {
    vec![
        Query::Inspect(address()),
        Query::Rebind(Rebind_Data {
            address: address(),
            rebinding: Rebinding::Pane(HerdrPaneId("w7B".to_string())),
        }),
        Query::Rebind(Rebind_Data {
            address: address(),
            rebinding: Rebinding::Flow(FlowId("f1c841".to_string())),
        }),
        Query::Withdraw(Withdraw_Data {
            address: address(),
            logical_delivery_id: LogicalDeliveryId(7),
        }),
    ]
}

fn replies() -> Vec<Response> {
    vec![
        inspected(),
        Response::Inspected(Inspected_Data {
            visible_permaflow: VisiblePermaflow {
                address: address(),
                permaflow_state: PermaflowState::Asleep,
                flow_id_vector: vec![],
                visible_native_input_vector: vec![],
            },
            binding: Binding {
                flow_node_option: None,
                process_option: None,
            },
            lock_vector: vec![],
            visible_attempt_vector: vec![],
            restart_record_option: None,
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

fn launched() -> Launched {
    Launched {
        flow_id: FlowId("f1c841".to_string()),
        session_id: SessionId("s1".to_string()),
        herdr_route_selection: HerdrRouteSelection::Unavailable,
        origin: Origin::Unbound,
    }
}

fn owner() -> FlowId {
    FlowId("751881".to_string())
}

/// Every reap owed at the Address and the handover owed at it, at each
/// standing: Missing and Unknown each as itself, never folded into Live.
fn obligations() -> Vec<VisibleAttempt> {
    let reaps = [
        ReapStanding::Owed,
        ReapStanding::Unrecorded,
        ReapStanding::PredecessorMissing,
        ReapStanding::PredecessorUnknown,
        ReapStanding::HerdrUnavailable,
        ReapStanding::CloseRefused,
        ReapStanding::HandoverRefused(HandoverCloseRefusal::Owner(HandoverOwnerRefusal::Live(
            owner(),
        ))),
        ReapStanding::SettleUnrecorded,
    ]
    .into_iter()
    .map(|reap_standing| {
        VisibleAttempt::Reap(ReapPending {
            address: address(),
            predecessor: Predecessor(FlowId("9a83ec".to_string())),
            launched: launched(),
            reap_standing,
        })
    });
    let handovers = [
        HandoverCloseRefusal::PersistenceRefused,
        HandoverCloseRefusal::Owner(HandoverOwnerRefusal::Live(owner())),
        HandoverCloseRefusal::Owner(HandoverOwnerRefusal::Missing(owner())),
        HandoverCloseRefusal::Owner(HandoverOwnerRefusal::Unknown(owner())),
    ]
    .into_iter()
    .map(|handover_close_refusal| {
        VisibleAttempt::Handover(HandoverPending {
            address: address(),
            handover_owner: HandoverOwner(owner()),
            launched: launched(),
            handover_close_refusal,
        })
    });
    reaps.chain(handovers).collect()
}

fn inspected_with_obligations() -> Response {
    let Response::Inspected(mut data) = inspected() else {
        unreachable!()
    };
    data.visible_attempt_vector.extend(obligations());
    Response::Inspected(data)
}

#[test]
fn inspect_lists_pending_reaps_and_handovers_as_an_archive() {
    let reply = inspected_with_obligations();
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    let private = Attempt::Reap(ReapPending {
        address: address(),
        predecessor: Predecessor(FlowId("9a83ec".to_string())),
        launched: launched(),
        reap_standing: ReapStanding::Owed,
    });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&private).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Attempt, rkyv::rancor::Error>(&archive).unwrap(),
        private
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
    fn repair_queries_have_concrete_datoms() {
        let texts = [
            "Inspect.{ Mind flow Sol }",
            "Rebind.{ { Mind flow Sol } Pane.w7B }",
            "Rebind.{ { Mind flow Sol } Flow.f1c841 }",
            "Withdraw.{ { Mind flow Sol } 7 }",
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
            "Inspected.{ { { Mind flow Sol } Awake.f1c841 [ 9a83ec ] [ { Sender.{ Psyche core Opus } Soft.Order.{ req1 go } } ] } { Some.{ f1c841 s1 Claude Unavailable Available.{ default mind-flow w7B t1 } Unbound Active } Some.{ 4242 17 } } [ { { Psyche core Opus } { Mind flow Sol } 1790000060 } ] [ Queued.{ s1 Claude Logical.{ Sender.{ Psyche core Opus } Soft.Order.{ req1 go } } Prepared } ] Some.{ f1c841 s1 { Mind flow Sol } { { default mind-flow w7B t1 } Some.{ 4242 17 } } Failed.BindMismatch 2 } }",
            "Inspected.{ { { Mind flow Sol } Asleep [] [] } { None None } [] [] None }",
            "InspectRejected.UnknownAddress",
            "Rebound.{ Mind flow Sol }",
            "RebindRejected.UnknownAddress",
            "RebindRejected.PaneAbsent",
            "RebindRejected.FlowUnknown",
            "RebindRejected.NotAwake",
            "RebindRejected.HerdrUnavailable",
            "RebindRejected.FlowTaken.{ Mind flow Sol }",
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

    #[test]
    fn inspect_lists_pending_reaps_and_handovers_with_missing_and_unknown_typed() {
        let launched = "{ f1c841 s1 Unavailable Unbound }";
        let mut texts = Vec::new();
        for standing in [
            "Owed",
            "Unrecorded",
            "PredecessorMissing",
            "PredecessorUnknown",
            "HerdrUnavailable",
            "CloseRefused",
            "HandoverRefused.Owner.Live.751881",
            "SettleUnrecorded",
        ] {
            texts.push(format!(
                "Reap.{{ {{ Mind flow Sol }} 9a83ec {launched} {standing} }}"
            ));
        }
        for refusal in [
            "PersistenceRefused",
            "Owner.Live.751881",
            "Owner.Missing.751881",
            "Owner.Unknown.751881",
        ] {
            texts.push(format!(
                "Handover.{{ {{ Mind flow Sol }} 751881 {launched} {refusal} }}"
            ));
        }
        let attempts = obligations();
        assert_eq!(attempts.len(), texts.len());
        for (attempt, text) in attempts.into_iter().zip(&texts) {
            assert_eq!(&attempt.datomize(vec![]).protosize().compact(), text);
            assert_eq!(
                Potential::<VisibleAttempt>::from(text.as_str())
                    .actualize(&mut budget())
                    .unwrap(),
                attempt
            );
        }
        let reply = inspected_with_obligations();
        let text = reply.datomize(vec![]).protosize().compact();
        assert_eq!(
            Potential::<Response>::from(text.as_str())
                .actualize(&mut budget())
                .unwrap(),
            reply
        );
    }
}
