# Upgrades

How to deploy each breaking change of meta-signal-flow.

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
