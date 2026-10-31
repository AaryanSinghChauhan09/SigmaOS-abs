# SigmaOS ABS

This crate is currently a `no_std` component identity scaffold. Its only
behavior is a `Component` type whose constructor returns the fixed name
`SigmaOS Component`.

No Arch Build System parsing, package building, signing, or repository
publishing behavior is implemented. The previous “basic implementation” status
was unsupported by the source and has been removed. Define expected package
inputs, trust model, outputs, and failure behavior before adding those features.

## Development

The crate has no third-party dependencies. `cargo check --lib` checks the
current scaffold; this does not validate any build-system capability.
