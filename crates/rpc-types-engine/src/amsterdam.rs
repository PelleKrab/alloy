//! Contains types related to the Amsterdam hardfork that will be used by RPC to communicate with the
//! beacon consensus engine.

use alloc::vec::Vec;

/// Fields introduced in `engine_newPayloadV5` that are not present in the `ExecutionPayload` RPC
/// object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct AmsterdamPayloadFields {
    /// The Inclusion List (IL) for Amsterdam.
    pub il: Vec<Vec<u8>>,
}

impl AmsterdamPayloadFields {
    /// Returns a new [`AmsterdamPayloadFields`] instance.
    pub fn new(il: Vec<Vec<u8>>) -> Self {
        Self { il }
    }
}

/// A container type for [AmsterdamPayloadFields] that may or may not be present.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct MaybeAmsterdamPayloadFields {
    fields: Option<AmsterdamPayloadFields>,
}

impl MaybeAmsterdamPayloadFields {
    /// Returns a new [`MaybeAmsterdamPayloadFields`] with no Amsterdam fields.
    pub const fn none() -> Self {
        Self { fields: None }
    }

    /// Returns a new [`MaybeAmsterdamPayloadFields`] with the given Amsterdam fields.
    pub fn from_fields(fields: AmsterdamPayloadFields) -> Self {
        Self { fields: Some(fields) }
    }

    /// Consumes the type and returns the [`AmsterdamPayloadFields`], if any.
    pub fn into_inner(self) -> Option<AmsterdamPayloadFields> {
        self.fields
    }

    /// Returns the Inclusion List (IL), if any.
    pub fn il(&self) -> Option<&Vec<Vec<u8>>> {
        self.fields.as_ref().map(|fields| &fields.il)
    }

    /// Returns a reference to the inner fields.
    pub const fn as_ref(&self) -> Option<&AmsterdamPayloadFields> {
        self.fields.as_ref()
    }
}

impl From<AmsterdamPayloadFields> for MaybeAmsterdamPayloadFields {
    #[inline]
    fn from(fields: AmsterdamPayloadFields) -> Self {
        Self { fields: Some(fields) }
    }
}

impl From<Option<AmsterdamPayloadFields>> for MaybeAmsterdamPayloadFields {
    #[inline]
    fn from(fields: Option<AmsterdamPayloadFields>) -> Self {
        Self { fields }
    }
}
