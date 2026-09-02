# How signals and streams preserve runtime identity

Use a typed `Stream<T>` to catch composition errors in Rust. Use `SignalSpec`
and `SignalEnvelope` when you need runtime or cross-language identity.

`Stream<T>` is a Rust declaration wrapper. An external package implements
`StreamSignal` for its own marker type and returns a validated `SignalSpec`.
`TypedOperator<Input, Output>` verifies those marker specifications against the
operator manifest's selected named ports. Invalid composition fails before the
Session starts.

## Runtime types

At runtime, the engine uses:

- `SignalSpec`: class, stable custom identifier, semantic role, and schema;
- `SignalEnvelope`: payload plus source-independent lineage and timing;
- `EdgeContract`: bounded delivery, loss/backpressure, copy, clock, and
  observation policy;
- `RuntimePlan`: compiled executable topology.

The core does not define customer/domain marker types. A provider package may
define its own transcript type; an instrumentation package may define its own
measurement type. Both compile to stable signal identifiers and schemas. Other
languages project those same stable identities using their native type systems;
they do not receive Rust's `T`.

`AudioFrame` remains the optimized realtime representation. It is not replaced
by a universal generic envelope.

## Continue developing

- [Send Session audio to an external system](../guides/connectors.md)
- [Add a Source, Operator, or Endpoint](../guides/extensions.md)
- [Read the Session architecture](../architecture/overview.md)
