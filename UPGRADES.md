# Upgrades

How to deploy each breaking change of meta-signal-flow.

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
