// The generated meta Signal names the override panel it carries by its
// crate path, `meta_signal_flow::panel::Input`.
extern crate self as meta_signal_flow;

pub mod generated;
/// The override panel, the Signal the meta input Override carries.
pub use generated::panel;
pub use generated::signal::ConfigurationChange as Configuration;
pub use generated::signal::*;

pub use flow_ethos::{
    Aspect, Blake3, ClientPath, CodexEndpoint, CommandSigil, ControlSocketPath, HarnessKind,
    HarnessProfile, HarnessProfiles, Hash, Home, InterruptKeys, Key, KeyName, MessageNexusBinary,
    MessageNexusPath, MetaAspects, MetaSocketPath, Model, ModelName, OrdinarySocketPath, Path,
    Repository, Source, SourceRoot, Subaspect, SubmitKeys, Topic,
};

pub use flow_ethos::Hash as SourceHash;
pub use flow_ethos::Path as SourcePath;
pub use flow_ethos::Repository as SourceRepository;

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The greeting every meta connection opens with, re-exported so a peer
/// greets through this contract alone.
pub use signal::{ContractDigest, Contracted, Handshake, HandshakeReceipt, HandshakeRejection};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The override panel's Ethos, which the meta input Override carries.
pub const PANEL_ETHOS: &str = include_str!("../ethos/panel.ethos");
/// The whole contract this socket speaks: the meta Signal, then the panel
/// it carries, so a change to either changes the greeting's digest.
pub const CONTRACT: &str = concat!(
    include_str!("../ethos/signal.ethos"),
    include_str!("../ethos/panel.ethos")
);
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source, the meta Signal and its panel; the querying side greets
/// with it.
impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = CONTRACT;
}
