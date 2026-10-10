#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NexusRuntimeSettings {
    pub ordinary_socket_path: flow_ethos::OrdinarySocketPath,
    pub meta_socket_path: flow_ethos::MetaSocketPath,
    pub source_root: flow_ethos::SourceRoot,
    pub stable_codex: flow_ethos::StableCodex,
    pub next_codex: flow_ethos::NextCodex,
    pub harness_profile_vector: std::vec::Vec<flow_ethos::HarnessProfile>,
    pub meta_aspects: flow_ethos::MetaAspects,
    pub message_nexus_path: flow_ethos::MessageNexusPath,
    pub message_nexus_binary: flow_ethos::MessageNexusBinary,
    pub lease: flow_ethos::Lease,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureModule {
    pub subaspect: flow_ethos::Subaspect,
    pub topic: flow_ethos::Topic,
    pub source: flow_ethos::Source,
}
#[rustfmt::skip]
pub type Native = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureModel {
    pub layer: flow_ethos::Layer,
    pub native: Native,
}
#[rustfmt::skip]
pub type Handover = i64;
#[rustfmt::skip]
pub type Refresh = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureThreshold {
    pub layer: flow_ethos::Layer,
    pub handover: Handover,
    pub refresh: Refresh,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureNexus {
    pub source_root: flow_ethos::SourceRoot,
    pub stable_codex: flow_ethos::StableCodex,
    pub next_codex: flow_ethos::NextCodex,
    pub harness_profiles: flow_ethos::HarnessProfiles,
    pub meta_aspects: flow_ethos::MetaAspects,
    pub message_nexus_path: flow_ethos::MessageNexusPath,
    pub message_nexus_binary: flow_ethos::MessageNexusBinary,
    pub lease: flow_ethos::Lease,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Configuration {
    Module(ConfigureModule),
    Model(ConfigureModel),
    Threshold(ConfigureThreshold),
    Nexus(ConfigureNexus),
}
#[rustfmt::skip]
pub type Nexus = ConfigureNexus;
#[rustfmt::skip]
pub type Models = std::vec::Vec<ConfigureModel>;
#[rustfmt::skip]
pub type Thresholds = std::vec::Vec<ConfigureThreshold>;
#[rustfmt::skip]
pub type Modules = std::vec::Vec<ConfigureModule>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationSnapshot {
    pub nexus: Nexus,
    pub models: Models,
    pub thresholds: Thresholds,
    pub modules: Modules,
}
#[rustfmt::skip]
pub type IdempotencyKey = String;
#[rustfmt::skip]
pub type CreditId = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CreditSelection {
    Next,
    Specific(CreditId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ResetRequest {
    pub idempotency_key: IdempotencyKey,
    pub credit_selection: CreditSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResetOutcome {
    Reset,
    NothingToReset,
    NoCredit,
    AlreadyRedeemed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigureRejection {
    StoreRefused,
    Conflict,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResetRejection {
    AdapterUnavailable,
    ProtocolRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowRegistrationRejection {
    StoreRefused,
    UnknownOrUnclaimedIdentity,
    ConflictingBinding,
}
#[rustfmt::skip]
pub type ProcessId = i64;
#[rustfmt::skip]
pub type ProcessUserId = i64;
#[rustfmt::skip]
pub type ProcessStartToken = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProcessIdentity {
    pub process_id: ProcessId,
    pub process_user_id: ProcessUserId,
    pub process_start_token: ProcessStartToken,
}
#[rustfmt::skip]
pub type HerdrServerSocketPath = String;
#[rustfmt::skip]
pub type HerdrServerProcessIdentity = ProcessIdentity;
#[rustfmt::skip]
pub type MetaFlowOwnerId = signal_flow::FlowId;
#[rustfmt::skip]
pub type HerdrTabId = String;
#[rustfmt::skip]
pub type WorkingDirectory = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowLifecycle {
    RegisteredUnconfirmed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowContainer {
    pub herdr_session_name: signal_flow::HerdrSessionName,
    pub herdr_server_socket_path: HerdrServerSocketPath,
    pub herdr_server_process_identity: HerdrServerProcessIdentity,
    pub meta_flow_owner_id: MetaFlowOwnerId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowBinding {
    pub flow_id: signal_flow::FlowId,
    pub flow_aspect: signal_flow::FlowAspect,
    pub power_level: signal_flow::PowerLevel,
    pub model_name: signal_flow::ModelName,
    pub harness_kind: signal_flow::HarnessKind,
    pub native_session_id: signal_flow::NativeSessionId,
    pub herdr_workspace_id: signal_flow::HerdrWorkspaceId,
    pub herdr_pane_id: signal_flow::HerdrPaneId,
    pub herdr_tab_id: HerdrTabId,
    pub herdr_terminal_id: signal_flow::HerdrTerminalId,
    pub herdr_agent_name: signal_flow::HerdrAgentName,
    pub process_identity: ProcessIdentity,
    pub working_directory: WorkingDirectory,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaBindExisting {
    pub flow_container: FlowContainer,
    pub flow_binding_vector: std::vec::Vec<FlowBinding>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BoundFlowBinding {
    pub flow_id: signal_flow::FlowId,
    pub flow_lifecycle: FlowLifecycle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowBindingRefusalReason {
    AmbiguousPane,
    DeadProcess,
    DuplicateFlowId,
    AnatomyMismatch,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefusedFlowBinding {
    pub flow_id: signal_flow::FlowId,
    pub flow_binding_refusal_reason: FlowBindingRefusalReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowBindingResult {
    Bound(BoundFlowBinding),
    Refused(RefusedFlowBinding),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BoundExisting {
    pub flow_container: FlowContainer,
    pub flow_binding_result_vector: std::vec::Vec<FlowBindingResult>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BindExistingRejection {
    ContainerUnavailable,
    ContainerIdentityMismatch,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetireRejection {
    UnknownFlow,
    AlreadyGone,
    StoreRefused,
}
#[rustfmt::skip]
pub type DeliveryId = String;
#[rustfmt::skip]
pub type MessageId = String;
#[rustfmt::skip]
pub type CommandLine = String;
#[rustfmt::skip]
pub type ByteOffset = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Sender {
    Flow(signal_flow::FlowId),
    Owner,
}
#[rustfmt::skip]
pub type PsycheContext = String;
#[rustfmt::skip]
pub type PsycheVerbatim = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Psyche_Data {
    pub psyche_context: PsycheContext,
    pub psyche_verbatim: PsycheVerbatim,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Content {
    Text(String),
    Psyche(Psyche_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Letter {
    pub message_id: MessageId,
    pub sender: Sender,
    pub content: Content,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Message {
    HardAbrupt(Letter),
    MiddleAbrupt(Letter),
    Soft(Letter),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryRequest {
    pub delivery_id: DeliveryId,
    pub flow_id: signal_flow::FlowId,
    pub message: Message,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InterruptWitness {
    NotRequested,
    Observed,
    Unobserved,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryGrade {
    Transported,
    Presented,
    Uncertain,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Delivery {
    pub delivery_id: DeliveryId,
    pub flow_id: signal_flow::FlowId,
    pub interrupt_witness: InterruptWitness,
    pub delivery_grade: DeliveryGrade,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BodyRefusal {
    EmptyBody,
    HarnessCommand(CommandLine),
    ControlCharacter(ByteOffset),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryRejection {
    UnknownFlow,
    FlowStopped,
    RouteUnavailable,
    RecipientWorking,
    RecipientBlocked,
    ComposerOccupied,
    BodyRefused(BodyRefusal),
    NotDelivered,
    PersistenceRefused,
    FlowRetired,
    FlowExited,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessCommand {
    Compact,
    Interrupt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommandRequest {
    pub flow_id: signal_flow::FlowId,
    pub harness_command: HarnessCommand,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CommandGrade {
    Transported,
    Observed,
    Uncertain,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommandOutcome {
    pub flow_id: signal_flow::FlowId,
    pub harness_command: HarnessCommand,
    pub command_grade: CommandGrade,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CommandRejection {
    UnknownFlow,
    FlowStopped,
    RouteUnavailable,
    RecipientBlocked,
    UnsupportedForHarness,
    NotDelivered,
    FlowRetired,
    FlowExited,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaRefusal {
    PeerUnknown,
    PeerNotAuthorized(signal_flow::Caller),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configuration(Configuration),
    Configure(Configuration),
    ConsumeReset(ResetRequest),
    RegisterFlow(signal_flow::FlowNode),
    MetaBindExisting(MetaBindExisting),
    Retire(signal_flow::FlowId),
    Deliver(DeliveryRequest),
    Vet(DeliveryRequest),
    Command(CommandRequest),
    ResolvePeer(ProcessIdentity),
    ReadEvents(signal_flow::FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventsRead_Data {
    pub flow_id: signal_flow::FlowId,
    pub event_vector: std::vec::Vec<signal_flow::Event>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReadEventsRejected_Data {
    UnknownFlow,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured,
    Configuration(ConfigurationSnapshot),
    Unconfigured,
    ResetConsumed(ResetOutcome),
    FlowRegistered(signal_flow::FlowNode),
    ConfigureRejected(ConfigureRejection),
    ResetRejected(ResetRejection),
    FlowRegistrationRejected(FlowRegistrationRejection),
    BoundExisting(BoundExisting),
    BindExistingRejected(BindExistingRejection),
    FlowRetired(signal_flow::FlowNode),
    RetireRejected(RetireRejection),
    Delivered(Delivery),
    DeliveryRejected(DeliveryRejection),
    Vetted(signal_flow::FlowId),
    Commanded(CommandOutcome),
    CommandRejected(CommandRejection),
    PeerResolved(signal_flow::Caller),
    PeerResolutionRejected(signal_flow::CallerResolutionRejection),
    MetaRefused(MetaRefusal),
    EventsRead(EventsRead_Data),
    ReadEventsRejected(ReadEventsRejected_Data),
}
