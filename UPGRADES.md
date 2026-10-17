# Upgrades

How to deploy each breaking change of meta-signal-flow.

## 25.0.0: a leave-out default names its harness

`ConfigureLeaveOut` is `{ Address HarnessKind LeaveOuts }`: the default
leave-outs of one metaflow started on one harness. `Configure.LeaveOuts`
holds one entry per Address and HarnessKind, so one metaflow may carry a
Claude default and a Codex default, and a Start takes only the default of
its own harness. `ConfigureRejection` appends `LeaveOutNotForHarness`: an
entry naming a leave-out of another harness is refused and nothing is
written. The existing variants keep their discriminants.

`Configure.LeaveOuts.[ { { Psyche flow Secondary } Claude [ SkillTool ClaudeMd ] } { { Mind core Primary } Codex [ CodexAgentsMd ] } ]`

The contract digest changes: every meta-wire peer repins (Flow 0.35.13, the
Message Nexus), and every declarer of `Configure.LeaveOuts` names each
entry's harness.

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
