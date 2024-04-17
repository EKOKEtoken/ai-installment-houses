use std::borrow::Cow;

use candid::{Nat, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use num_bigint::BigUint;

/// Storable principal. May be used as a stable storage key.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct StorablePrincipal(pub Principal);

impl StorablePrincipal {
    pub const MAX_PRINCIPAL_LENGTH_IN_BYTES: usize = 29;

    /// get principal
    pub fn as_principal(&self) -> &Principal {
        &self.0
    }
}

impl From<Principal> for StorablePrincipal {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        self.0.as_slice().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: Self::MAX_PRINCIPAL_LENGTH_IN_BYTES as u32,
        is_fixed_size: false,
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct StorableNat(pub Nat);

impl From<Nat> for StorableNat {
    fn from(value: Nat) -> Self {
        Self(value)
    }
}

impl Storable for StorableNat {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let big_uint = &self.0 .0;
        big_uint.to_bytes_be().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let big_uint = BigUint::from_bytes_be(bytes.as_ref());
        Self(Nat::from(big_uint))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 24,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_nat_roundtrip() {
        let value = Nat::from(8_888_888_u64);
        let storable = StorableNat::from(value.clone());
        let bytes = storable.to_bytes();
        let storable_actual = StorableNat::from_bytes(bytes);
        assert_eq!(storable_actual, storable);
    }

    #[test]
    fn test_storable_principal_roundtrip() {
        let principal_01 = Principal::from_slice(&[1; 29]);
        let principal_02 = Principal::from_slice(&[3; 24]);
        let principal_03 =
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal");

        let principals = vec![principal_01, principal_02, principal_03];

        for principal in principals {
            let source = StorablePrincipal(principal);
            let bytes = source.to_bytes();
            let decoded = StorablePrincipal::from_bytes(bytes);
            assert_eq!(source, decoded);
        }
    }
}
