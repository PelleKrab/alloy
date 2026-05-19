//! Contains types related to the Hegota hardfork (EIP-7805 / FOCIL / Bogota) that will be used
//! by RPC to communicate with the beacon consensus engine.

use alloc::vec::Vec;
use alloy_primitives::Bytes;

/// Fields introduced in `engine_newPayloadV6` (Hegota / Bogota / EIP-7805) that are not present
/// in the `ExecutionPayload` RPC object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct HegotaPayloadFields {
    /// The Inclusion List (IL) transactions for Hegota (EIP-7805).
    pub inclusion_list_transactions: Vec<Bytes>,
}

impl HegotaPayloadFields {
    /// Returns a new [`HegotaPayloadFields`] instance.
    pub fn new(inclusion_list_transactions: Vec<Bytes>) -> Self {
        Self { inclusion_list_transactions }
    }
}

/// A container type for [`HegotaPayloadFields`] that may or may not be present.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct MaybeHegotaPayloadFields {
    fields: Option<HegotaPayloadFields>,
}

impl MaybeHegotaPayloadFields {
    /// Returns a new [`MaybeHegotaPayloadFields`] with no Hegota fields.
    pub const fn none() -> Self {
        Self { fields: None }
    }

    /// Returns a new [`MaybeHegotaPayloadFields`] with the given Hegota fields.
    pub fn from_fields(fields: HegotaPayloadFields) -> Self {
        Self { fields: Some(fields) }
    }

    /// Consumes the type and returns the [`HegotaPayloadFields`], if any.
    pub fn into_inner(self) -> Option<HegotaPayloadFields> {
        self.fields
    }

    /// Returns the Inclusion List (IL) transactions, if any.
    pub fn inclusion_list_transactions(&self) -> Option<&Vec<Bytes>> {
        self.fields.as_ref().map(|fields| &fields.inclusion_list_transactions)
    }

    /// Returns a reference to the inner fields.
    pub const fn as_ref(&self) -> Option<&HegotaPayloadFields> {
        self.fields.as_ref()
    }
}

impl From<HegotaPayloadFields> for MaybeHegotaPayloadFields {
    #[inline]
    fn from(fields: HegotaPayloadFields) -> Self {
        Self { fields: Some(fields) }
    }
}

impl From<Option<HegotaPayloadFields>> for MaybeHegotaPayloadFields {
    #[inline]
    fn from(fields: Option<HegotaPayloadFields>) -> Self {
        Self { fields }
    }
}
