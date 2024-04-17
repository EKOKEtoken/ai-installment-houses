use candid::Principal;

use super::configuration::Configuration;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian(caller: Principal) -> bool {
        Configuration::is_custodian(caller)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_custodian(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        Configuration::set_custodians(&[caller]);
        assert_eq!(Inspect::inspect_is_custodian(caller), true);
    }
}
