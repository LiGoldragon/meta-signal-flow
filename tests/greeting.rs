use meta_signal_flow::{Contracted, ETHOS, Handshake, HandshakeReceipt, HandshakeRejection, Query};

#[test]
fn a_peer_built_from_this_source_is_greeted() {
    let digest = Query::contract_digest();
    assert_eq!(digest, Handshake::of_source(ETHOS).contract_digest);
    assert_eq!(
        Query::receipt(&Handshake::of_source(ETHOS)),
        HandshakeReceipt::Greeted(digest)
    );
}

/// A peer built on meta-signal-flow 76.0.0 greets with 76's digest and is
/// refused with 77's, before any of its queries is read.
#[test]
fn a_meta_76_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/meta-signal-flow-76.ethos");
    assert_ne!(Handshake::of_source(older), Query::greeting());
    assert_eq!(
        Query::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}

#[test]
fn the_ordinary_contract_is_refused_on_the_meta_socket() {
    assert_eq!(
        Query::receipt(&<signal_flow::Query as Contracted>::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}
