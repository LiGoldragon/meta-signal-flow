# Upgrades

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
