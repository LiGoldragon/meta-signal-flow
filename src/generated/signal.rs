#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type OrdinarySocketPath = String;
#[rustfmt::skip]
pub type MetaSocketPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Configuration {
    pub ordinary_socket_path: OrdinarySocketPath,
    pub meta_socket_path: MetaSocketPath,
}
#[rustfmt::skip]
pub type ConfigureRequest = Configuration;
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
pub enum Activation {
    NexusRestartRequired,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Configured {
    pub configuration: Configuration,
    pub activation: Activation,
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
pub struct FirstStartReceiptReference {
    pub launch_request_id: signal_flow::LaunchRequestId,
    pub receipt_sha256: signal_flow::ReceiptSha256,
}
#[rustfmt::skip]
pub type FinalTitleEvidenceSha256 = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FinalTitleEvidenceReference {
    pub herdr_terminal_id: signal_flow::HerdrTerminalId,
    pub final_title_evidence_sha256: FinalTitleEvidenceSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaConfirmExisting {
    pub flow_id: signal_flow::FlowId,
    pub native_session_id: signal_flow::NativeSessionId,
    pub first_start_receipt_reference: FirstStartReceiptReference,
    pub model_name: signal_flow::ModelName,
    pub effort: signal_flow::Effort,
    pub prompt_sha256: signal_flow::PromptSha256,
    pub native_turn_id: signal_flow::NativeTurnId,
    pub native_skill_selection_vector: std::vec::Vec<signal_flow::NativeSkillSelection>,
    pub final_title_evidence_reference: FinalTitleEvidenceReference,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfirmedExisting {
    pub flow_id: signal_flow::FlowId,
    pub native_session_id: signal_flow::NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfirmExistingRejection {
    UnknownFlow,
    BindingNotRegisteredUnconfirmed,
    FirstStartReceiptUnavailable,
    FirstStartReceiptMismatch,
    NativeSessionMismatch,
    ModelMismatch,
    EffortMismatch,
    PromptDigestMismatch,
    NativeTurnMismatch,
    SkillManifestMismatch,
    FinalTitleEvidenceMismatch,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configure(ConfigureRequest),
    ConsumeReset(ResetRequest),
    RegisterFlow(signal_flow::FlowNode),
    MetaBindExisting(MetaBindExisting),
    MetaConfirmExisting(MetaConfirmExisting),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured(Configured),
    ResetConsumed(ResetOutcome),
    FlowRegistered(signal_flow::FlowNode),
    ConfigureRejected(ConfigureRejection),
    ResetRejected(ResetRejection),
    FlowRegistrationRejected(FlowRegistrationRejection),
    BoundExisting(BoundExisting),
    BindExistingRejected(BindExistingRejection),
    ConfirmedExisting(ConfirmedExisting),
    ConfirmExistingRejected(ConfirmExistingRejection),
}
