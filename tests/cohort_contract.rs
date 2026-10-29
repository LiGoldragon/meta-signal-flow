#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use meta_signal_flow::{Query, Response};
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
fn delivery_operations_carry_only_the_durable_reference() {
    for text in [
        "Deliver.{ delivery-1 }",
        "Vet.{ delivery-1 }",
        "ReplayDelivery.{ delivery-1 }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
    assert!(
        Potential::<Query>::from("Deliver.{ delivery-1 forged-sender forged-message }")
            .actualize(&mut budget())
            .is_err()
    );
    for response in [
        Response::DeliveryReplayed("delivery-1".into()),
        Response::ReplayDeliveryRejected(
            meta_signal_flow::ReplayDeliveryRejected_Data::CallerNotAuthorized,
        ),
    ] {
        let text = response.datomize(vec![]).protosize().compact();
        assert_eq!(
            Potential::<Response>::from(text.as_str())
                .actualize(&mut budget())
                .unwrap(),
            response
        );
    }
}

#[test]
fn limits_keep_message_admission_and_cohort_transport_separate() {
    let limits = meta_signal_flow::Limits {
        send_body: 65_536,
        cohort: 122_880,
        frame: 1_048_576,
        curriculum_answer: 1_048_576,
        app_server_header: 65_536,
        app_server_message: 1_048_576,
        context_tail: 65_536,
        herdr_line: 131_083,
        herdr_envelope: 12,
        codex_text: 1_048_576,
        history_page: 100,
        history_pages: 10,
        remembered_turns: 20,
        interrupt_presses: 2,
        submission_reads: 3,
        title_readbacks: 4,
        output_lines: 200,
    };
    let text = limits.datomize(vec![]).protosize().compact();
    assert_eq!(
        Potential::<meta_signal_flow::Limits>::from(text.as_str())
            .actualize(&mut budget())
            .unwrap(),
        limits
    );
    assert_ne!(limits.send_body, limits.cohort);
}
