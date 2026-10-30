# Upgrades

How to deploy each breaking change of meta-signal-flow.

## 46.0.0: lifecycle replies project private run identities away

- Registration, adoption, retirement, peer resolution, event reading,
  command and inspection replies use dedicated visible lifecycle values.
  They preserve the operational state without exposing a `FlowId` through
  an ordinary meta operation. `Debug.Address` remains the sole explicit
  interface that returns run and logical-delivery ids.
- The complete private flow node, binding, launch attempt and event types
  stay unchanged for Flow's routing and Memory. This is a wire projection
  only and changes no stored row or schema hash.
- Major: meta `Response` archives change. Consumers pin meta-signal-flow
  46.0.0 with signal-flow 37.0.0 and update every producer and consumer
  together.

## 45.0.0: Inspect projects queued delivery attempts

- `Inspected` carries `VisibleMetaflow` and `VisibleAttempt`: the metaflow's
  queue and the queued-delivery attempt omit private delivery keys while
  preserving sender, recipient, priority, request, native session, harness
  and standing. Its lifecycle Flow ids and launch branch remain for the
  separately versioned lifecycle projection.
- Major: the `Inspected` response archive changes. Consumers pin
  meta-signal-flow 45.0.0 with signal-flow 36.0.0.

## 44.0.0: operational replies hide private run and delivery ids

- The pane `Letter` carries a typed sender metaflow, recipient metaflow and
  content. Its `MessageId` and `Sender.Flow.FlowId` positions leave; waking
  content is a vector of public native inputs without logical delivery ids.
- Delivery, queue, send-outcome and archive replies use public projections
  without `FlowId` or `LogicalDeliveryId`. The private types used by the
  Nexus's existing stored families remain unchanged.
- `Debug.Address` is the one explicit meta query that answers the Address's
  run ids and logical delivery ids. No operational response exposes them.
- Major: Query, Response and Letter archives change shape. Consumers pin
  meta-signal-flow 44.0.0 with signal-flow 36.0.0 and update every producer
  and consumer together.

## 43.0.0: signal-flow 35.0.0; a refused Unidentified carries its seat evidence

- Pins signal-flow 09ad7a (35.0.0), where `Refusal.Unidentified` carries
  `SeatEvidence` instead of `Process`. meta-signal-flow declares no type of
  its own for it, but `OrdinarySendOutcome.Refused.Refusal` and
  `Failed.SendFailure` hold a `Refusal`, so `SendOutcomesRead` and every
  record `SendOutcomeRecord` shapes change in datom and rkyv archive:
  `Refused.Unidentified.{ Some.{ herdr-1 w1-2 } Some.«Claude research» None
  1761800000 { 4242 987654 } }`. Re-exports follow signal-flow.
- Major: a consumer built against 42.0.0 cannot read these records.
  Consumers pin meta-signal-flow 43.0.0 together with signal-flow 35.0.0; no
  compatibility path.

## 42.0.0: Configure.Counters; the snapshot ends with the counters

- `ConfigurationChange` appends `Counters.ConfigureCounters`, its own
  family: `Configure.Counters.{ 120 45 300 }` names the next logical
  delivery, the next queue attempt and the next send outcome a Nexus
  allocates. A store that has allocated nothing reads `{ 1 3 1 }`.
- `ConfigurationSnapshot` appends `Counters.ConfigureCounters`, so a
  snapshot ends with it: `Configuration.{ … None 45 { 120 45 300 } }`.
  An operator reads an old store's counters there before carrying them to
  a new one.
- `ConfigureRejection` appends `CountersRefused.CountersRefusal`:
  `NotEmpty` (the store has recorded a logical delivery, a queue attempt
  or a send outcome), `Lowered.ConfigureCounters` (a value is below its
  family's counter; it carries the counters as they stand), `NotRunning`
  (a Nexus opened for configuration only holds no counters).
- Major, not minor: the appended snapshot field changes the shape of
  every `Configuration` answer, positional in its datom and in its rkyv
  archive, so a consumer built against 41.0.0 cannot read a 42.0.0
  snapshot. The appended variants alone would be additive. Consumers pin
  42.0.0; no compatibility path.

## 41.0.0: variants removed; signal-flow 34.0.0

- Pins signal-flow fb13fa (34.0.0), the release that marks the removal of
  variants. Consumers pin meta-signal-flow 41.0.0 together with
  signal-flow 34.0.0; no compatibility path.

## 40.1.0: Configure.DefaultMinutes; a Letter carries a Waking; signal-flow 33.3.0

- `ConfigurationChange` appends `DefaultMinutes.Minutes`, its own family:
  `Configure.DefaultMinutes.45`. `ConfigurationSnapshot` appends
  `DefaultMinutes.Minutes`, so a snapshot ends with it:
  `Configuration.{ … None 45 }`. `ConfigureRejection` appends
  `MinutesRefused.MinutesRefusal` (signal-flow), answered when the declared
  Minutes is zero, negative, or gives a deadline the clock cannot hold.
- `Content.NativeInput.NativeInput` is replaced by
  `Content.Waking.Vector<NativeInput>`: what one hand-over types, every
  queued input under its own id, sender and priority, oldest first. A
  single input is a Waking of one. Consumers building a Letter build the
  vector; no compatibility path.
- Pins signal-flow c73ac8 (33.3.0): `Remind`, `Reminded` and
  `InstantUnreadable` are gone.

## 40.0.0: a metaflow declares its continuity, purpose and records; Inspect, Rebind and Withdraw repair one

- `ConfigureMetaflow` appends `Continuity.[ OnDemand Maintained ]`,
  `Purpose.String` and `Records.Vector<RecordPath>`:
  `{ { Psyche core Primary } [ spirit:core ] [] Regular Default Maintained
  «…» [ flows/a68420/vision/flow.md ] }`. A Maintained metaflow is never
  without a running flow; an OnDemand one has its harness and pane closed
  when its turn ends with nothing queued. A store keeping archived
  declarations upgrades them.
- `ConfigureRejection` appends `TopicRefused.Topic`.
- `ConfigureNexus` gains `CurriculumNexusPath.String` after
  `MessageNexusBinary`: the socket of the Curriculum Nexus that composes
  every launch's context, read when the Nexus opens. `Configure.Nexus.{ …
  /run/user/1001/message.sock /etc/profiles/per-user/li/bin/message-nexus
  /run/user/1001/curriculum/curriculum.sock 60 }`. A store's archived Nexus
  configuration upgrades with `$XDG_RUNTIME_DIR/curriculum/curriculum.sock`
  of the process that opens it.
- `Query` appends `Inspect.Address`, answered `Inspected.{ Metaflow Binding
  Vector<Lock> Vector<Attempt> }` or `InspectRejected.UnknownAddress`: the
  metaflow whole, its binding `Binding.{ Option<FlowNode> Option<Process> }`
  (the row of the flow it is Awake as, and the process bound to its
  Address), the sender locks held on it, and every attempt outstanding for
  it, `Attempt.[ Queued.QueueDelivery Launch.LaunchAttempt ]`. It changes
  nothing.
- `Query` appends `Rebind.{ Address Rebinding.[ Pane.HerdrPaneId
  Flow.FlowId ] }`, answered `Rebound.Address` or `RebindRejected.[
  UnknownAddress PaneAbsent FlowUnknown NotAwake HerdrUnavailable
  FlowTaken.Address ]`: `Rebind.{ { Mind flow Secondary } Pane.w7B }`
  records the pane the current flow lives in; `Rebind.{ { Mind flow
  Secondary } Flow.f1c841 }` makes that flow current, the one before it
  joining Past. No harness is touched.
- `Query` appends `Withdraw.{ Address LogicalDeliveryId }`, answered
  `Withdrawn.LogicalDeliveryId` or `WithdrawRejected.[ UnknownAddress
  NotQueued ]`: the request leaves the queue, and one that never reached
  the seat ends `Withdrawn` for its sender.
- `RetireRejection` appends `CloseRefused`: a Retire whose flow's pane is
  still there and whose close Herdr refuses records nothing and answers
  `RetireRejected.CloseRefused`. A Retire closes the flow's pane where it
  is still there.
- `RetireRejection` appends `Maintained`: a Retire of the flow a Maintained
  metaflow is Awake as changes nothing and answers
  `RetireRejected.Maintained`; its declaration's continuity is changed to
  `OnDemand` first.
- Pins signal-flow 32.0.0 (`Refresh.Address`, `DeliveryState.Withdrawn`,
  `Event.Started.Option<NativeSessionId>`, `Event.Cleared`,
  `StartRejection.CompositionRefused.CompositionRefusal`,
  `RetryCause.WakeRefused.StartRejection`): `EventsRead` carries
  `Started.Some.«session»`.

## 34.0.0: every ordinary Send outcome is durably countable

- `ReadSendOutcomes` returns each completed ordinary Send's server-minted
  record ID, caller when known, sender and recipient when known, request
  digest and typed response. The request body is not stored. Each response is
  committed before emission. The contract digest changes; every meta-wire
  peer repins.

## 32.0.0: a request Flow raised itself is sent by Flow

- Pins signal-flow 25.0.0 and flow-ethos (flow dc2cba6), whose `SentBy` is
  `[ Sender Unknown Flow ]`. A queue delivery read through
  `ReadQueueDeliveries` may carry `{ Flow Order.«…» }`. The variant is
  appended; every meta-socket peer repins with signal-flow 25.0.0. The
  contract digest is unchanged.

## 31.0.0: a refresh threshold per model; the Investigation seat; PastThreshold

- `ConfigureThreshold` is `{ ModelName Refresh.Integer }`, one per model:
  the context, in tokens, past which a flow running that model is
  refreshed. `Layer` and `Handover` are gone; a refreshed flow writes no
  handover. `Configure.Thresholds.[ { claude-opus-5-5 200000 } { gpt-6.1-sol
  150000 } ]`. A layer-keyed entry no longer reads. Compaction is judged
  against the flow's model's threshold.
- `ConfigurationChange` appends `Investigation.Address`: the seat Flow
  launches, from its `Metaflows` declaration, to investigate a flow that
  compacted while it kept working. `Configure.Investigation.{ Psyche
  compaction Tertiary }`. `ConfigurationSnapshot` appends
  `Investigation.Option<Address>`.
- `DeliveryRejection` appends `PastThreshold`: a Deliver into a flow whose
  last measured context is at or past its model's threshold is refused
  before anything reaches the pane.
- The contract digest changes: every meta-wire peer repins.

## 30.0.0: a metaflow is declared in Flow's configuration

- `ConfigurationChange` appends `Metaflows.Vector<ConfigureMetaflow>`,
  `ConfigureMetaflow.{ Address SystemModules UserModules }`: one declaration
  per Address, the modules an ordinary `Launch.{ Address InstructionPrompt }`
  (signal-flow 24.0.0) composes. `Configure.Metaflows.[ { { Psyche core
  Secondary } [ spirit-core ] [ compensation-truth ] } ]`.
- `ConfigurationSnapshot` appends its `Metaflows` vector: a peer reading
  `Configuration` rebuilds.
- Pins signal-flow 24.0.0 (8796628).

## 29.0.0: no Refresh; compaction refused below the Refresh threshold; no Ended metaflow

- `Query::Refresh`, `Response::Refreshed`, `Response::RefreshRejected` and
  `RefreshRejection` are gone: every wake of an Asleep metaflow starts a
  fresh flow composed from its modules (flow 0.35.22), so a mark asking for
  a fresh wake has nothing to change. A peer that sent `Refresh.{ ... }`
  drops the call.
- `CommandRejection` appends `CompactionRefused.CompactionRefusal`,
  `CompactionRefusal.[ BelowThreshold.ContextBelowThreshold Unmeasured
  NoThreshold ]`, `ContextBelowThreshold.{ ContextTokens RefreshTokens }`.
  Compaction is an emergency measure: `Command.{ 7d41e0 Compact }` is
  refused, before anything reaches the pane, unless the flow's context at
  its last Stop was at or above the Refresh threshold configured for its
  layer, the `Refresh` of that layer's `ConfigureThreshold` declared by
  `Configure.Thresholds`, which keeps its shape:
  `CommandRejected.CompactionRefused.BelowThreshold.{ 120000 200000 }`,
  `CommandRejected.CompactionRefused.Unmeasured` (no context size is
  recorded for the flow), `CommandRejected.CompactionRefused.NoThreshold`
  (no Refresh threshold applies to its layer). Appended last, it leaves
  every earlier discriminant unchanged.
- The imported types are pinned at signal-flow 23.0.0 (c7dcc07) and flow
  06171a5, whose `State` is `[ Awake.FlowId Asleep ]`: no metaflow is
  Ended.
- The contract digest changes from 809506754cfaf73d to 73ce9b26e5cd4583:
  every meta-wire peer repins (Flow, the Message Nexus).

## 28.0.0: models by layer and aspect, leave-outs by harness, send servers, queue deliveries

- `ConfigureModel` is `{ Layer Aspect Native.String }`: the Models family
  holds one entry per Layer and Aspect, so one layer runs a Claude model for
  Psyche and a Codex model for Mind and Field. A declaration naming one
  layer and aspect twice is `ConfigureRejected.DuplicateEntry`. A model
  declared without its aspect no longer reads:

  ```
  Configure.Models.[ { Secondary Psyche claude-opus-5-5 } { Secondary Mind gpt-6-sol } { Secondary Field gpt-6-sol } ]
  ```

  `Configuration` answers the same rows. The Nexus refuses a Start whose
  model is not the one configured for its Address's layer and aspect
  (signal-flow 22.0.0, `StartRejected.ModelRefused`).
- `ConfigureLeaveOut` is `{ Address HarnessKind LeaveOuts }`: the default
  leave-outs of one metaflow started on one harness. `Configure.LeaveOuts`
  holds one entry per Address and HarnessKind, so one metaflow may carry a
  Claude default and a Codex default, and a Start takes only the default of
  its own harness. `ConfigureRejection` appends `LeaveOutNotForHarness`: an
  entry naming a leave-out of another harness is refused and nothing is
  written:
  `Configure.LeaveOuts.[ { { Psyche flow Secondary } Claude [ SkillTool ClaudeMd ] } { { Mind core Primary } Codex [ CodexAgentsMd ] } ]`.
- `ConfigurationChange` appends `SendServers.Vector<ConfigureSendServer>`,
  `ConfigureSendServer.{ CodexAppServerBinary McpSendServerBinary }`, one
  entry per Codex app-server executable; each path is the executable the
  kernel reports for the running process:
  `Configure.SendServers.[ { /nix/store/…-codex/bin/codex /nix/store/…/bin/flow-send-mcp } ]`.
  signal-flow 22.0.0's `IdentifyThread` is accepted only from a process
  running a configured `McpSendServerBinary` whose parent runs the paired
  `CodexAppServerBinary`. `ConfigurationSnapshot` appends
  `SendServers.Vector<ConfigureSendServer>`; a store with none configured
  reads `[]`.
- `Query` appends `ReadQueueDeliveries.Address`; `Response` appends
  `QueueDeliveriesRead.{ Address Vector<QueueDelivery> }` and
  `ReadQueueDeliveriesRejected.[ UnknownAddress StoreRefused ]`. A
  `QueueDelivery` is `{ QueueAttemptId FlowId NativeSessionId HarnessKind
  QueuedRequest QueueDeliveryStanding }`, its standing `Prepared`,
  `Uncertain.Option<Undecided>`, `ReceiptRecorded`, `Absent.AbsenceProof`
  or `Resolved.QueueResolutionRecord`.
- `Query` appends `ResolveQueueDelivery.{ QueueAttemptId
  [ Delivered Absent ] QueueResolutionReason }`. `Delivered`: the request
  reached the seat and leaves the queue. `Absent`: it did not, and a new
  attempt is authorised. Flow records the outcome, the requesting meta
  peer's `ResolverIdentity.{ ProcessIdentity Option<FlowId> }`, the reason
  and the time (`UnixSeconds`) in the commit that settles or authorises.
  `Response` appends `QueueDeliveryResolved.{ Address
  QueueDeliveryResolution }` and `ResolveQueueDeliveryRejected.[
  UnknownAttempt NotUncertain NotOldest EmptyReason StoreRefused ]`.
- Appended variants leave every earlier discriminant unchanged.
- The imported signal-flow types are pinned at 22.0.0 (e6ccbddb), whose
  `Event.Stopped` carries `NativeStop` and which appends
  `Event.PromptSubmitted`: `EventsRead.{ f1c841 [ Started ToolUsed.Bash
  Stopped.{ Some.s1 None } ] }`.
- The contract digest changes from 7bbc37359da8843c to 809506754cfaf73d:
  every meta-wire peer repins (Flow, the Message Nexus), and every declarer
  of `Configure.Models` and `Configure.LeaveOuts` names each entry's aspect
  and harness.

## 24.0.1: pin signal-flow 20.0.0

The imported signal-flow types are pinned at 20.0.0 (6a496646) and flow-ethos
at flow 9e417bb. This contract's own source is unchanged, so its digest stays
7bbc37359da8843c; a peer rebuilds against signal-flow 20.0.0 so the shared
types have one identity.

## 24.0.0: MetaBindExisting verifies each binding against Herdr

`FlowBindingRefusalReason` appends `PaneAbsent`, `ProcessMismatch`,
`SessionMismatch`, `SessionUnreported`, `SessionUnproven` and
`HerdrUnreadable`; the existing variants keep their discriminants. The Nexus
refuses a binding whose pane its Herdr session does not show for the
binding's harness, whose pane does not run the binding's process, or whose
native session Herdr or (for a Codex pane that reports none) a rollout's
`session_meta` does not prove:
`Refused.{ codex-flow SessionUnproven }`.

The imported signal-flow types are pinned at 19.0.0 (37a41014). The contract
digest changes from 371d348ad5b3fbad to 7bbc37359da8843c: every meta-wire peer
repins.

## 23.0.0: Configure declares a whole family; Reconfigure is gone

`ConfigurationChange` is `Models.Vector<ConfigureModel>`,
`Thresholds.Vector<ConfigureThreshold>`, `LeaveOuts.Vector<ConfigureLeaveOut>`
or `Nexus.ConfigureNexus`. Each declaration replaces its family's committed
value whole in one atomic commit, so the effective family is always its last
declaration and an entry left out of it is removed. `Configured` carries
`ConfigureOutcome.[ Replaced Unchanged ]`. `ConfigureRejection` is
`[ StoreRefused DuplicateEntry CodexModelsOverlap ]`: one key named twice in a
declaration, or one model under both Codex endpoints of a Nexus.
`Query::Reconfigure`, `Response::Reconfigured`, `ReconfigureRejected` and
`ReconfigureRejection` are removed; `Configure.Nexus` is the one way to set or
replace the Nexus. The contract digest changes, so an older peer is refused at
the greeting. A declarer sends one `Configure.<Family>` with the whole
declared vector instead of one `Configure.LeaveOut` per metaflow, and reads
`Configured.Replaced` or `Configured.Unchanged`. Repin it in Flow (0.35.4)
and in every meta-wire peer.

## 22.0.1: signal-flow 18.0.0

The imported signal-flow types are pinned at 18.0.0 (59aab725). No type this
contract imports changed and its own source is unchanged, so its digest is the
same and no meta-wire peer repins.

## 22.0.0: Refresh(Address)

`Query::Refresh(Address)` marks one Asleep metaflow so its next wake starts a
fresh native session composed from its last launch's recorded module
identities and leave-outs, instead of resuming. It answers
`Response::Refreshed(Address)` (a repeat while marked answers the same), or
`Response::RefreshRejected(RefreshRejection)`: `Unknown`, `Awake`, `Ended`,
`NoLaunchProfile`, `StoreRefused`. A new variant changes the contract digest,
so an older peer is refused at the greeting. 21.0.0 is held by the unmerged
`SubagentRoles` branch; this is the next free major on main. Repin it in Flow
(Flow 0.35.2).

## 20.0.0: no configured modules; signal-flow 17.0.0

`ConfigureModule` and `ConfigurationChange.Module` are removed, and
`ConfigurationSnapshot` no longer carries `Modules`: a seat's context is
named by its launch's module identities (signal-flow 17.0.0), never
configured on the meta socket. `Configure.LeaveOut.{ Address LeaveOuts }` names the
default leave-outs of one metaflow, which a Start naming its own
replaces; `ConfigurationSnapshot` carries them as `DefaultLeaveOuts`. `Key` and `Source` are no longer imported.
The contract digest changes, so an older peer is refused at the greeting.
The build generates with ethos-zero 17.0.0 at 07714b0, whose output for
this source is byte-identical to 16.0.0's. Repin it in Flow together with signal-flow 17.0.0 (b619ea08); a
`Configure.Module` caller must stop sending it.

## 19.0.1: signal-flow 16.0.0

meta-signal-flow pins signal-flow 16.0.0 (229933d9), whose `LaunchProfile`
names an `Address`. The types this contract imports from signal-flow are
unchanged, and so is its own Ethos source and contract digest: a 19.0.0
peer still greets this one. Repin it in Flow together with signal-flow
16.0.0, so the workspace holds one signal-flow.

## 19.0.0: Reconfigure replaces the whole Nexus value

`Query::Reconfigure(ConfigureNexus)` replaces the committed Nexus value of a
configured store in one atomic commit and keeps every other row. It answers
`Response::Reconfigured`, or `Response::ReconfigureRejected` with
`ReconfigureRejection::Unconfigured` (no Nexus value yet; first setup stays
`Configure.Nexus`) or `ReconfigureRejection::CodexModelsOverlap` (one model
names both Codex endpoints), and then nothing changed. `Configure.Nexus`
still refuses a differing value as `Conflict`. A Nexus reads its Codex
endpoints when it opens, so a replaced endpoint is served from its next
start. The contract digest changes: repin this release in Flow and its meta
clients together; an 18.0.0 peer is refused at the greeting.

## 18.0.0: one shared Aspect type

`FlowBinding` and the imported `Caller` now use the Library's `Aspect`
directly; `FlowAspect` is removed. Regenerate and pin this release with the
matching `signal-flow` and `flow-ethos` sources, then rebuild Flow consumers
together. The contract digest changes, so an older peer is refused at the
greeting.

## 17.0.0: explicit store refusals

`Response::Refused.Store` carries the store's own error text when a meta
query cannot read or commit its requested state. `Configure.Module` now
contains the shared `flow_ethos::Key` and `flow_ethos::Source` types, so its
identity and source have the same nominal shapes as Flow's Library. Consumers
must regenerate and repin meta-signal-flow 17.0.0 with the matching Flow
Nexus; store failures must not be converted to an empty configuration or a
successful response.

## 14.0.0: signal 8.0.0, signal-flow 10.0.0

What breaks:

- meta-signal-flow depends on signal 8.0.0 (f35460de) and signal-flow
  10.0.0 (f95034de), where it depended on signal 7.0.0 and signal-flow
  9.0.0. The signal-flow types it imports and the `signal::Contracted` its
  `Query` implements are now those releases', so a crate holding the older
  ones alongside sees two distinct sets.
- The `datom` feature also enables `signal/datom`: signal 8.0.0 binds the
  same datom-codec and protos 0.32.2, so the graph holds one codec.
- The build reads the ethos with ethos-zero 16.0.0 at c2653dd8. The
  generated module is byte-identical (the build script asserts it), so
  `ETHOS`, the contract digest and every archive are unchanged from 13.0.0.

To deploy, in each consumer (signal-message, meta-signal-message, flow,
message): repin meta-signal-flow, signal-flow 10.0.0 and signal 8.0.0 in
one change. The wire is unchanged; a 13.0.0 peer still greets this one.

## 13.0.0: signal-flow 9.0.0; ReadEvents

What breaks:

- meta-signal-flow depends on signal-flow 9.0.0 (2cc48792), where it
  depended on 8.0.0. The signal-flow types it imports (`FlowNode`, `FlowId`,
  `Caller` and the rest) are now 9.0.0's, so a crate holding both versions
  sees two distinct sets. signal-flow 9.0.0 renamed its `Started` payload
  type `Launched`; meta-signal-flow names neither.
- New query `ReadEvents.FlowId`, answered `EventsRead.{ FlowId
  Vector<Event> }` (signal-flow's `Event`, imported) or
  `ReadEventsRejected.[ UnknownFlow StoreRefused ]`: the owner reads the
  harness events Flow recorded from `Report`s for one flow. `Query` and
  `Response` gain variants, so the contract digest changes and a 12.0.0
  peer is refused at the greeting.
- The ethos body is reprinted vertically by ethos-zero 16.0.0's `Printable`
  (every section was one line). No other declaration changed; the
  generated Rust differs only by the three new items, and the rkyv archive
  of every 12.0.0 value is unchanged.
- signal stays at 7.0.0 (66e7b153): signal main has no newer version.

To deploy, in flow: repin meta-signal-flow and signal-flow 9.0.0 together,
answer `ReadEvents` from the events Flow's Memory holds, then rebuild and
restart the Nexus and both CLIs together.

## 12.0.0: ethos-zero 16.0.0, signal 7.0.0, signal-flow 8.0.0, protos and datom-codec 0.32.2

What breaks:

- meta-signal-flow depends on signal 7.0.0 (the exchange layer) and
  signal-flow 8.0.0 (c297d987), where it depended on signal 5.0.0 and
  signal-flow 7.0.0. `Query` implements `signal::Contracted` over `ETHOS`;
  its digest differs from signal-flow's, so a peer greeting the meta socket
  with the ordinary contract is refused.
- The `datom` feature pins protos 0.32.2 (15b41da8) and datom-codec 0.32.2
  (4dff16b4) and no longer enables `signal/datom`. The unused signal-flow
  build-dependency is gone.
- protos 0.32's `textualize` prints vertically; one-line text, including a
  `Message` typed into a pane, is `Compactable::compact`.
- The generated Rust is byte-identical to 11.0.0's; the ethos source and the
  vocabulary are unchanged.

To deploy, in flow: repin meta-signal-flow and signal-flow together, repin
protos and datom-codec to 0.32.2, replace `.textualize()` with `.compact()`
where one line is meant, rebuild and restart the Nexus and both CLIs
together. The rkyv archive of every value is unchanged.

The ordinary Send ledger names only Send outcomes (Queued, Delivered, or its typed refusal). The owner socket's former Refusal type is named StoreRefusal so it is distinct from the imported ordinary Refusal.
