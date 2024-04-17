use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, StableVec};

use crate::app::memory::{
    CUSTODIANS_MEMORY_ID, ICP_LEDGER_CANISTER_MEMORY_ID, LEDGER_CANISTER_MEMORY_ID, MEMORY_MANAGER,
    SALE_ROYALTY_MEMORY_ID,
};
use crate::constants::{DEFAULT_SALE_ROYALTY, ICP_LEDGER_CANISTER_PRINCIPAL};
use crate::storable::StorablePrincipal;

thread_local! {

    static ICP_LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(ICP_LEDGER_CANISTER_MEMORY_ID)), StorablePrincipal::from(Principal::from_text(ICP_LEDGER_CANISTER_PRINCIPAL).unwrap())).unwrap()
    );

    static LEDGER_CANISTER: RefCell<StableCell<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(LEDGER_CANISTER_MEMORY_ID)), StorablePrincipal::from(Principal::anonymous())).unwrap()
    );

    static SALE_ROYALTY: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(SALE_ROYALTY_MEMORY_ID)), DEFAULT_SALE_ROYALTY).unwrap()
    );

    /// Canister custodians
    static CUSTODIANS: RefCell<StableVec<StorablePrincipal, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableVec::new(MEMORY_MANAGER.with(|mm| mm.get(CUSTODIANS_MEMORY_ID))).unwrap()
    );

}

pub struct Configuration;

impl Configuration {
    pub fn set_icp_ledger_canister(id: Principal) {
        ICP_LEDGER_CANISTER
            .with_borrow_mut(|cell| cell.set(id.into()))
            .expect("failed to ledger canister");
    }

    pub fn get_icp_ledger_canister() -> Principal {
        ICP_LEDGER_CANISTER.with_borrow(|id| id.get().0)
    }

    pub fn set_ledger_canister(id: Principal) {
        LEDGER_CANISTER
            .with_borrow_mut(|cell| cell.set(id.into()))
            .expect("failed to ledger canister");
    }

    pub fn get_ledger_canister() -> Principal {
        LEDGER_CANISTER.with_borrow(|id| id.get().0)
    }

    pub fn set_sale_royalty(royalty: u64) {
        SALE_ROYALTY
            .with_borrow_mut(|cell| cell.set(royalty))
            .expect("failed to set sale royalty");
    }

    pub fn get_sale_royalty() -> u64 {
        SALE_ROYALTY.with_borrow(|cell| *cell.get())
    }

    pub fn set_custodians(custodians: &[Principal]) {
        CUSTODIANS.with_borrow_mut(|cell| {
            for _ in 0..cell.len() {
                cell.pop();
            }
            for custodian in custodians
                .iter()
                .map(|principal| StorablePrincipal::from(*principal))
            {
                cell.push(&custodian).expect("failed to push");
            }
        });
    }

    #[allow(dead_code)]
    pub fn get_custodians() -> Vec<Principal> {
        CUSTODIANS.with_borrow(|cell| {
            cell.iter()
                .map(|custodian| *custodian.as_principal())
                .collect()
        })
    }

    pub fn is_custodian(caller: Principal) -> bool {
        CUSTODIANS.with_borrow(|cell| {
            cell.iter()
                .any(|custodian| custodian.as_principal() == &caller)
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::alice;

    #[test]
    fn test_should_get_and_set_icp_ledger_id() {
        Configuration::set_icp_ledger_canister(alice());
        assert_eq!(Configuration::get_icp_ledger_canister(), alice());
    }

    #[test]
    fn test_should_get_and_set_ledger_id() {
        Configuration::set_ledger_canister(alice());
        assert_eq!(Configuration::get_ledger_canister(), alice());
    }

    #[test]
    fn test_should_get_and_set_sale_royalty() {
        Configuration::set_sale_royalty(10);
        assert_eq!(Configuration::get_sale_royalty(), 10);
    }

    #[test]
    fn test_should_get_and_set_custodians() {
        let custodians = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[3; 24]),
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal"),
        ];
        Configuration::set_custodians(&custodians);
        assert_eq!(Configuration::get_custodians(), custodians);
        assert!(Configuration::is_custodian(Principal::from_slice(&[1; 29])));
        assert!(Configuration::is_custodian(Principal::from_slice(&[3; 24])));
        assert!(Configuration::is_custodian(
            Principal::from_text("mfufu-x6j4c-gomzb-geilq").expect("valid principal")
        ));
        assert!(!Configuration::is_custodian(
            Principal::from_text("aaaaa-aa").expect("valid principal")
        ));
    }
}
