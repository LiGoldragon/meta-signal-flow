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
