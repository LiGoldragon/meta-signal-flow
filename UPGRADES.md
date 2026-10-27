# Upgrades

## 42.0.0

Adds the store-only recovery surface: queue, metaflow, launch, successor,
lifecycle, hold, context and quarantine repairs; extends inspection with the
owed successor and rebind with native-session and Herdr-session repairs.
`FlowLifecycle` on an adopted binding is renamed `BindingLifecycle`; the flow
row lifecycle is the `signal-flow` type.

## 43.0.0

Repins signal-flow 35. A `FlowBinding` carries the observed model and native
binding only; its role is the `Address` it is bound to. `Aspect`, `PowerLevel`,
and `RoleMismatch` are removed.

`ConfigureThreshold` names its numeric value `RefreshTokens`; `Refresh` is
therefore a unit succession cause rather than an integer-bearing variant.
