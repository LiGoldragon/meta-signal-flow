use meta_signal_flow::{Contracted, ETHOS, Handshake, HandshakeReceipt, HandshakeRejection, Input};

#[test]
fn a_peer_built_from_this_source_is_greeted() {
    let digest = Input::contract_digest();
    assert_eq!(digest, Handshake::of_source(ETHOS).contract_digest);
    assert_eq!(
        Input::receipt(&Handshake::of_source(ETHOS)),
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
