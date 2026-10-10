pub mod generated;
pub use generated::signal::*;
pub use generated::signal::ConfigurationChange as Configuration;

pub use flow_ethos::{
    Blake3, ClientPath, CodexEndpoint, CommandSigil, ControlSocketPath, HarnessKind,
    HarnessProfile, HarnessProfiles, Hash, Home, InterruptKeys, KeyName, Layer,
    MessageNexusBinary, MessageNexusPath, MetaAspects, MetaSocketPath, ModelName,
    NextCodex, OrdinarySocketPath, Path, Repository, Source, SourceRoot, StableCodex,
    Subaspect, SubmitKeys, Topic,
};

pub use flow_ethos::Hash as SourceHash;
pub use flow_ethos::Path as SourcePath;
pub use flow_ethos::Repository as SourceRepository;

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
