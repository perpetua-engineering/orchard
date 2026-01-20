# Perpetua: External Signer Support for orchard

This fork adds external signer support methods to the orchard crate, enabling constrained devices (Apple Watch, hardware wallets, TEEs) to apply pre-computed signatures to Orchard actions.

## Overview

The standard orchard PCZT signer expects to perform signing internally using the spend authorizing key. This fork adds methods that allow setting signatures computed externally:

## New Methods

### `Action::apply_external_signature`

```rust
/// Applies an externally-computed spend authorization signature.
pub fn apply_external_signature(
    &mut self,
    signature: redpallas::Signature<SpendAuth>,
)
```

### `Action::apply_external_signature_bytes`

```rust
/// Applies an externally-computed spend authorization signature from raw bytes.
pub fn apply_external_signature_bytes(
    &mut self,
    signature_bytes: [u8; 64],
)
```

## Usage Example

```rust
use orchard::pczt::Action;

// On the phone: extract sighash and alpha from PCZT
let sighash = signer.shielded_sighash();
let alpha = action.spend().alpha();

// Send sighash and alpha to constrained device...
// Device computes: signature = sign(rsk, sighash) where rsk = ask * alpha

// On the phone: apply the signature received from the device
action.apply_external_signature_bytes(signature_bytes);
```

## Building

This fork is designed to be used via cargo's patch mechanism:

```toml
[patch.crates-io]
orchard = { path = "../orchard-fork" }
```

## Upstream Status

This method should eventually be upstreamed to the official orchard crate. The API is minimal and focused on the external signing use case.

## Related Repositories

- [pczt-fork](../pczt-fork) - Exposes sighash/alpha extraction
- [sapling-crypto-fork](../sapling-crypto-fork) - Same changes for Sapling
- [zcash-light-client-ffi](../zcash-light-client-ffi) - FFI layer using these forks
