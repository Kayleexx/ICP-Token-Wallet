use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct TokenWallet {
    owner: Principal,
    balances: HashMap<Principal, u64>,
    token_symbol: String,
    token_name: String,
    decimals: u8,
    total_supply: u64,
}

thread_local! {
    static WALLET: RefCell<TokenWallet> = RefCell::new(TokenWallet {
        owner: Principal::anonymous(),
        balances: HashMap::new(),
        token_symbol: "ICPT".to_string(),
        token_name: "ICP Test Token".to_string(),
        decimals: 8,
        total_supply: 0,
    });
}

#[ic_cdk_macros::init]
fn init() {
    let caller = ic_cdk::caller();
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        wallet.owner = caller;
        wallet.balances.insert(caller, 1_000_000_000);
        wallet.total_supply = 1_000_000_000;
    });
}


#[ic_cdk_macros::query]
fn balance_of(owner: Principal) -> u64 {
    WALLET.with(|wallet| {
        wallet.borrow().balances.get(&owner).copied().unwrap_or(0)
    })
}

#[ic_cdk_macros::update]
fn transfer(to: Principal, amount: u64) -> bool {
    let caller = ic_cdk::caller();
    
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        
        // Check if sender has sufficient balance
        let sender_balance = wallet.balances.get(&caller).copied().unwrap_or(0);
        if sender_balance < amount {
            return false;
        }
        
        // Update balances
        wallet.balances.insert(caller, sender_balance - amount);
        let recipient_balance = wallet.balances.get(&to).copied().unwrap_or(0);
        wallet.balances.insert(to, recipient_balance + amount);
        
        true
    })
}

#[ic_cdk_macros::query]
fn token_info() -> (String, String, u8, u64) {
    WALLET.with(|wallet| {
        let wallet = wallet.borrow();
        (
            wallet.token_name.clone(),
            wallet.token_symbol.clone(),
            wallet.decimals,
            wallet.total_supply,
        )
    })
}

#[ic_cdk_macros::update]
fn mint(to: Principal, amount: u64) -> bool {
    let caller = ic_cdk::caller();
    
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        if wallet.owner != caller {
            return false;
        }
        
        let recipient_balance = wallet.balances.get(&to).copied().unwrap_or(0);
        wallet.balances.insert(to, recipient_balance + amount);
        wallet.total_supply += amount;
        
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_wallet() -> TokenWallet {
        TokenWallet {
            owner: Principal::anonymous(),
            balances: HashMap::new(),
            token_symbol: "ICPT".to_string(),
            token_name: "ICP Test Token".to_string(),
            decimals: 8,
            total_supply: 0,
        }
    }

    #[test]
    fn test_init() {
        let wallet = setup_test_wallet();
        assert_eq!(wallet.token_symbol, "ICPT");
        assert_eq!(wallet.decimals, 8);
    }

    #[test]
    fn test_transfer() {
        WALLET.with(|wallet| {
            let mut wallet = wallet.borrow_mut();
            let sender = Principal::anonymous();
            let receiver = Principal::from_text("2vxsx-fae").unwrap();
            
            // Setup initial balance
            wallet.balances.insert(sender, 1000);
            
            // Test transfer
            assert!(transfer(receiver, 500));
            assert_eq!(wallet.balances.get(&sender).copied().unwrap_or(0), 500);
            assert_eq!(wallet.balances.get(&receiver).copied().unwrap_or(0), 500);
        });
    }

    #[test]
    fn test_insufficient_balance() {
        WALLET.with(|wallet| {
            let mut wallet = wallet.borrow_mut();
            let sender = Principal::anonymous();
            let receiver = Principal::from_text("2vxsx-fae").unwrap();
            
            // Setup initial balance
            wallet.balances.insert(sender, 100);
            
            // Test transfer with insufficient balance
            assert!(!transfer(receiver, 500));
            assert_eq!(wallet.balances.get(&sender).copied().unwrap_or(0), 100);
            assert_eq!(wallet.balances.get(&receiver).copied().unwrap_or(0), 0);
        });
    }
}