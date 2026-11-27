//! Operator intent and its exact launch identities survive the wire.
use flow_ethos::{Address, Aspect, Model, Topic};
use meta_signal_flow::{
    Input, Output, ParkedLaunchDisposition, ParkedLaunchRelease, ParkedLaunchReleaseRejected_Data,
};
use signal_flow::LaunchRequestId;

fn address() -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: Topic("flow".into()),
        model: Model::Sol,
    }
}

fn orders() -> Vec<LaunchRequestId> {
    vec![
        LaunchRequestId("first".into()),
        LaunchRequestId("second".into()),
    ]
}

fn release(disposition: ParkedLaunchDisposition) -> ParkedLaunchRelease {
    ParkedLaunchRelease {
        address: address(),
        parked_launch_disposition: disposition,
        launch_request_id_vector: orders(),
    }
}

fn inputs() -> Vec<(&'static str, Input)> {
    vec![
        (
            "ReleasePermaflow.{ Mind flow Sol }",
            Input::ReleasePermaflow(address()),
        ),
        (
            "ReleaseParkedLaunches.{ { Mind flow Sol } Start [ first second ] }",
            Input::ReleaseParkedLaunches(release(ParkedLaunchDisposition::Start)),
        ),
        (
            "ReleaseParkedLaunches.{ { Mind flow Sol } Drop [ first second ] }",
            Input::ReleaseParkedLaunches(release(ParkedLaunchDisposition::Drop)),
        ),
    ]
}

fn outputs() -> Vec<(&'static str, Output)> {
    vec![
        (
            "PermaflowReleased.{ Mind flow Sol }",
            Output::PermaflowReleased(address()),
        ),
        (
            "ParkedLaunchesReleased.{ { Mind flow Sol } Start [ first second ] }",
            Output::ParkedLaunchesReleased(release(ParkedLaunchDisposition::Start)),
        ),
        (
            "ParkedLaunchesReleased.{ { Mind flow Sol } Drop [ first second ] }",
            Output::ParkedLaunchesReleased(release(ParkedLaunchDisposition::Drop)),
        ),
        (
            "ParkedLaunchReleaseRejected.{ { Mind flow Sol } [ first second ] }",
            Output::ParkedLaunchReleaseRejected(ParkedLaunchReleaseRejected_Data {
                address: address(),
                launch_request_id_vector: orders(),
            }),
        ),
    ]
}

#[test]
fn parked_launch_dispositions_and_order_identities_round_trip_on_the_wire() {
    for (_, value) in inputs() {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(),
            value
        );
    }
    for (_, value) in outputs() {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
            value
        );
    }
}

#[cfg(feature = "datom")]
#[test]
fn operator_can_read_actual_parked_ids_and_submit_them_with_an_explicit_choice() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };
    for (text, value) in inputs() {
        assert_eq!(
            Potential::<Input>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            value
        );
        assert_eq!(value.datomize(vec![]).protosize().compact(), text);
    }
    for (text, value) in outputs() {
        assert_eq!(
            Potential::<Output>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            value
        );
        assert_eq!(value.datomize(vec![]).protosize().compact(), text);
    }
    for text in [
        "ReleaseParkedLaunches.{ { Mind flow Sol } [ first second ] }",
        "ReleaseParkedLaunches.{ { Mind flow Sol } Start }",
    ] {
        assert!(
            Potential::<Input>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text}"
        );
    }
}
