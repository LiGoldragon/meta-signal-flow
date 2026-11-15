#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Potential};
use meta_signal_flow::Input;
use protos::ReaderBudget;

#[test]
fn heartbeat_configuration_is_outside_the_current_input_vocabulary() {
    for text in [
        "Configure.Heartbeat.None",
        "Configure.Heartbeat.Some.{ 60 [ { Mind core Sol } ] }",
    ] {
        let mut budget = Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        };
        assert!(
            Potential::<Input>::from(text)
                .actualize(&mut budget)
                .is_err()
        );
    }
}
