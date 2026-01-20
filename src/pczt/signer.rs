use rand::{CryptoRng, RngCore};

use crate::{keys::SpendAuthorizingKey, primitives::redpallas::{self, SpendAuth}};

impl super::Action {
    /// Signs the Orchard spend with the given spend authorizing key.
    ///
    /// It is the caller's responsibility to perform any semantic validity checks on the
    /// PCZT (for example, comfirming that the change amounts are correct) before calling
    /// this method.
    pub fn sign<R: RngCore + CryptoRng>(
        &mut self,
        sighash: [u8; 32],
        ask: &SpendAuthorizingKey,
        rng: R,
    ) -> Result<(), SignerError> {
        let alpha = self
            .spend
            .alpha
            .ok_or(SignerError::MissingSpendAuthRandomizer)?;

        let rsk = ask.randomize(&alpha);
        let rk = redpallas::VerificationKey::from(&rsk);

        if self.spend.rk == rk {
            self.spend.spend_auth_sig = Some(rsk.sign(rng, &sighash));
            Ok(())
        } else {
            Err(SignerError::WrongSpendAuthorizingKey)
        }
    }

    /// Applies an externally-computed spend authorization signature.
    ///
    /// This is used for external signing where the signature is computed outside
    /// of this crate (e.g., on a hardware wallet, secure enclave, or constrained device
    /// like Apple Watch).
    ///
    /// The signature should be produced by signing the `sighash` with the randomized
    /// private spending key `rsk = ask * alpha`, where `alpha` is the spend authorization
    /// randomizer from this action's spend.
    ///
    /// It is the caller's responsibility to ensure the signature is valid for the
    /// transaction's sighash and was produced using the correct randomizer.
    pub fn apply_external_signature(
        &mut self,
        signature: redpallas::Signature<SpendAuth>,
    ) {
        self.spend.spend_auth_sig = Some(signature);
    }

    /// Applies an externally-computed spend authorization signature from raw bytes.
    ///
    /// This is a convenience method that accepts a 64-byte signature directly,
    /// useful when receiving signatures from external devices that produce raw bytes.
    ///
    /// Returns an error if the signature bytes are not exactly 64 bytes.
    pub fn apply_external_signature_bytes(
        &mut self,
        signature_bytes: [u8; 64],
    ) {
        let signature = redpallas::Signature::<SpendAuth>::from(signature_bytes);
        self.spend.spend_auth_sig = Some(signature);
    }
}

/// Errors that can occur while signing an Orchard action in a PCZT.
#[derive(Debug)]
pub enum SignerError {
    /// The Signer role requires `alpha` to be set.
    MissingSpendAuthRandomizer,
    /// The provided `ask` does not own the action's spent note.
    WrongSpendAuthorizingKey,
}
