use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

// Configuration
pub const LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const CUSTODIANS_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const ICP_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const SALE_ROYALTY_MEMORY_ID: MemoryId = MemoryId::new(13);

// Swap
pub const LISTINGS_MEMORY_ID: MemoryId = MemoryId::new(20);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
