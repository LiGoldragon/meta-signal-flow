use meta_signal_flow::{
    CONTRACT, Contracted, Handshake, HandshakeReceipt, HandshakeRejection, Input,
};

#[test]
fn a_peer_built_from_this_source_is_greeted() {
    let digest = Input::contract_digest();
    assert_eq!(digest, Handshake::of_source(CONTRACT).contract_digest);
    assert_eq!(
        Input::receipt(&Handshake::of_source(CONTRACT)),
        HandshakeReceipt::Greeted(digest)
    );
}

/// A peer built on meta-signal-flow 79.0.0 greets with 79's digest and is
/// refused with 81's, before any of its inputs is read.
#[test]
fn a_meta_79_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-79.ethos");
    assert_ne!(Handshake::of_source(older), Input::greeting());
    assert_eq!(
        Input::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

/// A peer built on meta-signal-flow 82.0.0 greets with 82's digest and is
/// refused with 83's, before any of its inputs is read.
#[test]
fn a_meta_82_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-82.ethos");
    assert_ne!(Handshake::of_source(older), Input::greeting());
    assert_eq!(
        Input::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

/// A peer built on meta-signal-flow 83.0.0, whose Retire and Refresh the
/// override panel replaces, greets with 83's digest and is refused, before
/// any of its inputs is read.
#[test]
fn a_meta_83_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-83.ethos");
    assert_ne!(Handshake::of_source(older), Input::greeting());
    assert_eq!(
        Input::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

/// A peer built on meta-signal-flow 86.0.0, which carried the override
/// panel wrapped as Override, greets with 86's digest and is refused with
/// 87's, before any of its inputs is read.
#[test]
fn a_meta_86_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-86.ethos");
    assert_ne!(Handshake::of_source(older), Input::greeting());
    assert_eq!(
        Input::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

/// A peer built on meta-signal-flow 0.90.0, which had no
/// AbandonQueueDelivery, greets with 0.90's digest and is refused with
/// 0.91's, before any of its inputs is read.
#[test]
fn a_meta_0_90_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-0.90.ethos");
    assert_ne!(Handshake::of_source(older), Input::greeting());
    assert_eq!(
        Input::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

#[test]
fn the_meta_contract_without_refresh_mode_is_refused() {
    let previous = Handshake::of_source(include_str!("fixtures/meta-signal-flow-0.91.ethos"));
    assert_ne!(previous, Input::greeting());
    assert_eq!(
        Input::receipt(&previous),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

#[test]
fn the_ordinary_contract_is_refused_on_the_meta_socket() {
    assert_eq!(
        Input::receipt(&<signal_flow::Input as Contracted>::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

#[test]
fn the_peer_whose_release_could_start_parked_launches_is_refused_before_dispatch() {
    let previous = Handshake::of_source(include_str!("fixtures/meta-signal-flow-0.92.ethos"));
    assert_ne!(previous, Input::greeting());
    assert_eq!(
        Input::receipt(&previous),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}

#[test]
fn the_peer_whose_effort_was_a_closed_level_set_is_refused_before_dispatch() {
    let previous = Handshake::of_source(include_str!("fixtures/meta-signal-flow-0.93.ethos"));
    assert_ne!(previous, Input::greeting());
    assert_eq!(
        Input::receipt(&previous),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Input::contract_digest()
        ))
    );
}
