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
pub type Native = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureModel {
    pub layer: flow_ethos::Layer,
    pub aspect: signal_flow::Aspect,
    pub native: Native,
}
#[rustfmt::skip]
pub type Refresh = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureThreshold {
    pub model_name: signal_flow::ModelName,
    pub refresh: Refresh,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureLeaveOut {
    pub address: flow_ethos::Address,
    pub harness_kind: signal_flow::HarnessKind,
    pub leave_outs: signal_flow::LeaveOuts,
}
#[rustfmt::skip]
pub type CurriculumNexusPath = String;
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
    pub curriculum_nexus_path: CurriculumNexusPath,
    pub lease: flow_ethos::Lease,
}
#[rustfmt::skip]
pub type CodexAppServerBinary = String;
#[rustfmt::skip]
pub type McpSendServerBinary = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureSendServer {
    pub codex_app_server_binary: CodexAppServerBinary,
    pub mcp_send_server_binary: McpSendServerBinary,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Continuity {
    OnDemand,
    Maintained,
}
#[rustfmt::skip]
pub type Purpose = String;
#[rustfmt::skip]
pub type RecordPath = String;
#[rustfmt::skip]
pub type Records = std::vec::Vec<RecordPath>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureMetaflow {
    pub address: flow_ethos::Address,
    pub system_modules: signal_flow::SystemModules,
    pub user_modules: signal_flow::UserModules,
    pub flow_kind: signal_flow::FlowKind,
    pub tool_set: signal_flow::ToolSet,
    pub continuity: Continuity,
    pub purpose: Purpose,
    pub records: Records,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationChange {
    Models(std::vec::Vec<ConfigureModel>),
    Thresholds(std::vec::Vec<ConfigureThreshold>),
    LeaveOuts(std::vec::Vec<ConfigureLeaveOut>),
    Nexus(ConfigureNexus),
    SendServers(std::vec::Vec<ConfigureSendServer>),
    Metaflows(std::vec::Vec<ConfigureMetaflow>),
    Investigation(flow_ethos::Address),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigureOutcome {
    Replaced,
    Unchanged,
}
#[rustfmt::skip]
pub type Nexus = ConfigureNexus;
#[rustfmt::skip]
pub type Models = std::vec::Vec<ConfigureModel>;
#[rustfmt::skip]
pub type Thresholds = std::vec::Vec<ConfigureThreshold>;
#[rustfmt::skip]
pub type DefaultLeaveOuts = std::vec::Vec<ConfigureLeaveOut>;
#[rustfmt::skip]
pub type SendServers = std::vec::Vec<ConfigureSendServer>;
#[rustfmt::skip]
pub type Metaflows = std::vec::Vec<ConfigureMetaflow>;
#[rustfmt::skip]
pub type Investigation = std::option::Option<flow_ethos::Address>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationSnapshot {
    pub nexus: Nexus,
    pub models: Models,
    pub thresholds: Thresholds,
    pub default_leave_outs: DefaultLeaveOuts,
    pub send_servers: SendServers,
    pub metaflows: Metaflows,
    pub investigation: Investigation,
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
    DuplicateEntry,
    CodexModelsOverlap,
    LeaveOutNotForHarness,
    TopicRefused(flow_ethos::Topic),
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
    AdoptedActive,
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
    pub aspect: signal_flow::Aspect,
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
    pub address: flow_ethos::Address,
    pub flow_binding: FlowBinding,
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
    PaneAbsent,
    ProcessMismatch,
    SessionMismatch,
    SessionUnreported,
    SessionUnproven,
    HerdrUnreadable,
    RoleMismatch,
    AddressOwnerLive,
    AddressOwnerUnknown,
    NativeIdentityMismatch,
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
    pub flow_binding_result: FlowBindingResult,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BindExistingRejection {
    ContainerUnavailable,
    ContainerIdentityMismatch,
    OwnerMismatch,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetireRejection {
    UnknownFlow,
    AlreadyGone,
    StoreRefused,
    CloseRefused,
    Maintained,
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
    NativeInput(flow_ethos::NativeInput),
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
pub enum Gate {
    Kept,
    Forced,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryRequest {
    pub delivery_id: DeliveryId,
    pub flow_id: signal_flow::FlowId,
    pub message: Message,
    pub gate: Gate,
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
    PastThreshold,
}
#[rustfmt::skip]
pub type Keys = std::vec::Vec<flow_ethos::KeyName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessCommand {
    Compact,
    Interrupt,
    Line(CommandLine),
    Keys(Keys),
    Clear,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CommandRequest {
    pub flow_id: signal_flow::FlowId,
    pub harness_command: HarnessCommand,
    pub gate: Gate,
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
pub type ContextTokens = i64;
#[rustfmt::skip]
pub type RefreshTokens = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ContextBelowThreshold {
    pub context_tokens: ContextTokens,
    pub refresh_tokens: RefreshTokens,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CompactionRefusal {
    BelowThreshold(ContextBelowThreshold),
    Unmeasured,
    NoThreshold,
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
    CompactionRefused(CompactionRefusal),
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
pub enum StoreRefusal {
    Store(String),
}
#[rustfmt::skip]
pub type QueueAttemptId = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Undecided {
    HistoryUnreadable,
    HistoryReplaced,
    PendingInHarness,
    Ambiguous,
    SeatNotResting,
    AbsenceUnprovable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AbsenceProof {
    HandOverRefused,
    HistoryAbsent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueueDeliveryStanding {
    Prepared,
    Uncertain(std::option::Option<Undecided>),
    ReceiptRecorded,
    Absent(AbsenceProof),
    Resolved(QueueResolutionRecord),
}
#[rustfmt::skip]
pub type QueueResolutionReason = String;
#[rustfmt::skip]
pub type UnixSeconds = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueueResolutionOutcome {
    Delivered,
    Absent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueueDeliveryResolution {
    pub queue_attempt_id: QueueAttemptId,
    pub queue_resolution_outcome: QueueResolutionOutcome,
    pub queue_resolution_reason: QueueResolutionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ResolverIdentity {
    pub process_identity: ProcessIdentity,
    pub flow_id_option: std::option::Option<signal_flow::FlowId>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueueResolutionRecord {
    pub queue_resolution_outcome: QueueResolutionOutcome,
    pub resolver_identity: ResolverIdentity,
    pub queue_resolution_reason: QueueResolutionReason,
    pub unix_seconds: UnixSeconds,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueueDelivery {
    pub queue_attempt_id: QueueAttemptId,
    pub flow_id: signal_flow::FlowId,
    pub native_session_id: signal_flow::NativeSessionId,
    pub harness_kind: signal_flow::HarnessKind,
    pub queue_evidence: QueueEvidence,
    pub queue_delivery_standing: QueueDeliveryStanding,
}
#[rustfmt::skip]
pub type QueueRequestId = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LegacyArchiveEntry {
    pub queue_request_id: QueueRequestId,
    pub address: flow_ethos::Address,
    pub queue_evidence: QueueEvidence,
    pub queue_attempt_id_vector: std::vec::Vec<QueueAttemptId>,
}
#[rustfmt::skip]
pub type SendOutcomeId = String;
#[rustfmt::skip]
pub type RequestDigest = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Legacy_Data {
    pub sent_by: signal_flow::SentBy,
    pub request: signal_flow::Request,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueueEvidence {
    Logical(flow_ethos::QueueEntry),
    Legacy(Legacy_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PriorSendOutcome {
    Queued,
    Delivered,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OrdinarySendOutcome {
    Accepted(signal_flow::SendAcceptance),
    Refused(signal_flow::Refusal),
    PriorAdmission(PriorSendOutcome),
    Sent(signal_flow::SendReceipt),
    Failed(signal_flow::SendFailure),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendOutcomeRecord {
    pub send_outcome_id: SendOutcomeId,
    pub process_option: std::option::Option<signal_flow::Process>,
    pub first_address_option: std::option::Option<flow_ethos::Address>,
    pub second_address_option: std::option::Option<flow_ethos::Address>,
    pub request_digest: RequestDigest,
    pub ordinary_send_outcome: OrdinarySendOutcome,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Binding {
    pub flow_node_option: std::option::Option<signal_flow::FlowNode>,
    pub process_option: std::option::Option<signal_flow::Process>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Attempt {
    Queued(QueueDelivery),
    Launch(signal_flow::LaunchAttempt),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Retire_Data {
    pub flow_id: signal_flow::FlowId,
    pub gate: Gate,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Rebinding {
    Pane(signal_flow::HerdrPaneId),
    Flow(signal_flow::FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Rebind_Data {
    pub address: flow_ethos::Address,
    pub rebinding: Rebinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Withdraw_Data {
    pub address: flow_ethos::Address,
    pub logical_delivery_id: flow_ethos::LogicalDeliveryId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configuration,
    Configure(ConfigurationChange),
    ConsumeReset(ResetRequest),
    RegisterFlow(signal_flow::FlowNode),
    MetaBindExisting(MetaBindExisting),
    Retire(Retire_Data),
    Deliver(DeliveryRequest),
    Vet(DeliveryRequest),
    Command(CommandRequest),
    ReadPane(signal_flow::FlowId),
    ResolvePeer(ProcessIdentity),
    ReadEvents(signal_flow::FlowId),
    ReadQueueDeliveries(flow_ethos::Address),
    ReadSendOutcomes,
    ObserveLegacyArchive,
    ResolveQueueDelivery(QueueDeliveryResolution),
    Inspect(flow_ethos::Address),
    Rebind(Rebind_Data),
    Withdraw(Withdraw_Data),
}
#[rustfmt::skip]
pub type Screen = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PaneRead_Data {
    pub flow_id: signal_flow::FlowId,
    pub agent_state: signal_flow::AgentState,
    pub screen: Screen,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReadPaneRejected_Data {
    UnknownFlow,
    RouteUnavailable,
    HerdrUnavailable,
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
pub struct QueueDeliveriesRead_Data {
    pub address: flow_ethos::Address,
    pub queue_delivery_vector: std::vec::Vec<QueueDelivery>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReadQueueDeliveriesRejected_Data {
    UnknownAddress,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReadSendOutcomesRejected_Data {
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueueDeliveryResolved_Data {
    pub address: flow_ethos::Address,
    pub queue_delivery_resolution: QueueDeliveryResolution,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResolveQueueDeliveryRejected_Data {
    UnknownAttempt,
    NotUncertain,
    NotOldest,
    EmptyReason,
    StoreRefused,
    NativeReceiptRequired,
    LegacyUncorrelated,
    AlreadyResolved(QueueResolutionRecord),
    AlreadyDelivered(signal_flow::NativeReceipt),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Inspected_Data {
    pub metaflow: flow_ethos::Metaflow,
    pub binding: Binding,
    pub lock_vector: std::vec::Vec<flow_ethos::Lock>,
    pub attempt_vector: std::vec::Vec<Attempt>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InspectRejected_Data {
    UnknownAddress,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RebindRejected_Data {
    UnknownAddress,
    PaneAbsent,
    FlowUnknown,
    NotAwake,
    HerdrUnavailable,
    FlowTaken(flow_ethos::Address),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WithdrawRejected_Data {
    UnknownAddress,
    NotQueued,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured(ConfigureOutcome),
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
    PaneRead(PaneRead_Data),
    ReadPaneRejected(ReadPaneRejected_Data),
    PeerResolved(signal_flow::Caller),
    PeerResolutionRejected(signal_flow::CallerResolutionRejection),
    MetaRefused(MetaRefusal),
    Refused(StoreRefusal),
    EventsRead(EventsRead_Data),
    ReadEventsRejected(ReadEventsRejected_Data),
    QueueDeliveriesRead(QueueDeliveriesRead_Data),
    ReadQueueDeliveriesRejected(ReadQueueDeliveriesRejected_Data),
    SendOutcomesRead(std::vec::Vec<SendOutcomeRecord>),
    ReadSendOutcomesRejected(ReadSendOutcomesRejected_Data),
    LegacyArchiveObserved(std::vec::Vec<LegacyArchiveEntry>),
    ObserveLegacyArchiveRejected(StoreRefusal),
    QueueDeliveryResolved(QueueDeliveryResolved_Data),
    ResolveQueueDeliveryRejected(ResolveQueueDeliveryRejected_Data),
    Inspected(Inspected_Data),
    InspectRejected(InspectRejected_Data),
    Rebound(flow_ethos::Address),
    RebindRejected(RebindRejected_Data),
    Withdrawn(flow_ethos::LogicalDeliveryId),
    WithdrawRejected(WithdrawRejected_Data),
}
