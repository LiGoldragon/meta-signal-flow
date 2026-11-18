//! Cancellation notice settings are one complete Configure value.

use meta_signal_flow::{CancelNotices, ConfigurationChange, Input, Output, Recipient, Sender, Telling};

#[test]
fn cancellation_notice_configuration_round_trips_on_the_binary_wire() {
    for sender in [Telling::Told, Telling::Untold] {
        for recipient in [Telling::Told, Telling::Untold] {
            let change = ConfigurationChange::CancelNotices(CancelNotices { sender: Sender(sender), recipient: Recipient(recipient) });
            let input = Input::Configure(change.clone());
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).unwrap();
            assert_eq!(rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(), input);
            let output = Output::Configured(change);
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&output).unwrap();
            assert_eq!(rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(), output);
        }
    }
}

#[cfg(feature = "datom")]
#[test]
fn cancel_notices_input_and_echo_round_trip_as_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    for text in ["Configure.CancelNotices.{ Untold Told }", "Configure.CancelNotices.{ Told Untold }"] {
        let mut budget = Budget { remaining: 4096, reader: ReaderBudget::try_from(4096).unwrap(), depth: 0, maximum_depth: 1024 };
        let input = Potential::<Input>::from(text).actualize(&mut budget).unwrap();
        assert_eq!(input.datomize(vec![]).protosize().compact(), text);
    }
    let text = "Configured.CancelNotices.{ Untold Told }";
    let mut budget = Budget { remaining: 4096, reader: ReaderBudget::try_from(4096).unwrap(), depth: 0, maximum_depth: 1024 };
    let output = Potential::<Output>::from(text).actualize(&mut budget).unwrap();
    assert_eq!(output.datomize(vec![]).protosize().compact(), text);
}
