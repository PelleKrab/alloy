//! Contains types related to the Amsterdam hardfork that will be used by RPC to communicate with
//! the beacon consensus engine.

/// Fields introduced in the Amsterdam hardfork that are not present in the `ExecutionPayload` RPC
/// object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
pub struct AmsterdamPayloadFields {}
