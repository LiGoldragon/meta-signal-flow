# meta-signal-flow 48.0.0

Batch 8 pages event, send-outcome and flow reads and adds owner settlement of uncertain pane deliveries. DeliveryGrade gains NotPresented so both settlement outcomes are typed.

# Upgrades

## 52.0.0: every configured metaflow uses one launch behavior

Repins signal-flow 38.0.0. `ConfigureMetaflow` no longer carries
`FlowKind`; every configured metaflow uses the regular launch path.

## 51.0.0: share harness event reporting vocabulary

Repins signal-flow 37.0.0 so privileged clients share the ordinary
`Reporter` and `Notified` vocabulary with Flow. No meta wire type changes.

## 50.0.0: a binding's address owns its role

Repins signal-flow 36. A `FlowBinding` carries observed model and native
binding only; its role is the `Address` it is bound to. `Aspect`,
`PowerLevel`, and `RoleMismatch` are removed.

## 49.0.0: timings and limits are configuration families

ConfigurationChange and ConfigurationSnapshot gain complete Timings and Limits singletons. Fresh stores seed today's values.

## 47.0.0: store recovery operations


Adds queue, metaflow, launch, successor, lifecycle, hold, context and quarantine repairs; extends inspection and rebind.

## 46.0.0: metaflows declare their requirements

ConfigureMetaflow appends Requires.Vector<Address>. Stored declarations upgrade with an empty vector.

## 45.0.0: model configuration carries its effort

ConfigureModel carries ModelName and configured Effort.String.

## 44.0.0: pane commands and reads

HarnessCommand carries Line, Keys, and Clear; the owner can ReadPane.

## 43.0.0: delivery and command requests carry a gate

Deliver, Vet, Command, and Retire carry Gate.Kept or Gate.Forced.

## 42.0.0: delivered messages are complete datoms

Message is a priority head carrying Letter with Kind, Sender and Body.

## 41.0.0: abrupt requests may queue


Adds the store-only recovery surface: queue, metaflow, launch, successor,
lifecycle, hold, context and quarantine repairs; extends inspection with the
owed successor and rebind with native-session and Herdr-session repairs.
`FlowLifecycle` on an adopted binding is renamed `BindingLifecycle`; the flow
row lifecycle is the `signal-flow` type.

`ConfigureThreshold` names its numeric value `RefreshTokens`; `Refresh` is
therefore a unit succession cause rather than an integer-bearing variant.
