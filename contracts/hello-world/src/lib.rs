#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

// Define the states of the escrow transaction
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowState {
    Funded,   // Funds are locked in the contract
    Disputed, // A dispute has been raised
    Resolved, // Transaction is complete (funds have been released)
}

// Data structure to store the information of an escrow transaction
#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    pub buyer: Address,
    pub seller: Address,
    pub guardian: Address,
    pub token: Address,
    pub amount: i128,
    pub state: EscrowState,
}

// Keys used for storage on Soroban
#[contracttype]
pub enum DataKey {
    Escrow(u64), // Store each Escrow by its ID
    Counter,     // Counter to auto-generate transaction IDs
}

#[contract]
pub struct TrustGuardContract;

#[contractimpl]
impl TrustGuardContract {
    /// 1. CREATE ESCROW (Called by Buyer)
    /// Lock Buyer's USDC/XLM into the Smart Contract
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        guardian: Address,
        token: Address,
        amount: i128,
    ) -> u64 {
        // Require the buyer to sign (authorize) this transaction
        buyer.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than 0");
        }

        // Transfer tokens from the buyer's wallet to the Smart Contract's address
        let token_client = token::Client::new(&env, &token);
        let contract_address = env.current_contract_address();
        token_client.transfer(&buyer, &contract_address, &amount);

        // Auto-increment ID for the transaction
        let mut count: u64 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::Counter, &count);

        // Save Escrow information to the blockchain
        let escrow = Escrow {
            buyer,
            seller,
            guardian,
            token,
            amount,
            state: EscrowState::Funded,
        };
        env.storage().instance().set(&DataKey::Escrow(count), &escrow);

        // Return the transaction ID for Frontend usage
        count
    }

    /// 2. APPROVE PAYMENT (Happy path - Called by Buyer)
    /// Goods are OK -> Transfer funds to the Seller
    pub fn approve_payment(env: Env, escrow_id: u64) {
        let mut escrow = Self::get_escrow(&env, escrow_id);

        // Only the buyer has the authority to approve payment
        escrow.buyer.require_auth();

        if escrow.state != EscrowState::Funded {
            panic!("Invalid transaction state for approval");
        }

        // Update state to Resolved
        escrow.state = EscrowState::Resolved;
        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);

        // Transfer funds from the Contract to the Seller
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&env.current_contract_address(), &escrow.seller, &escrow.amount);
    }

    /// 3. RAISE DISPUTE (Called by Buyer)
    /// Goods are faulty -> Lock funds, wait for Guardian to resolve
    pub fn raise_dispute(env: Env, escrow_id: u64) {
        let mut escrow = Self::get_escrow(&env, escrow_id);

        // Only the buyer can raise a dispute in this MVP
        escrow.buyer.require_auth();

        if escrow.state != EscrowState::Funded {
            panic!("Can only raise dispute when funds are locked (Funded)");
        }

        // Change state to Disputed
        escrow.state = EscrowState::Disputed;
        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);
    }

    /// 4. RESOLVE DISPUTE (Called by Guardian)
    /// Guardian verifies evidence and decides who receives the funds
    pub fn resolve_dispute(env: Env, escrow_id: u64, winner: Address) {
        let mut escrow = Self::get_escrow(&env, escrow_id);

        // Require the Guardian to sign this transaction
        escrow.guardian.require_auth();

        if escrow.state != EscrowState::Disputed {
            panic!("No active dispute to resolve");
        }

        if winner != escrow.buyer && winner != escrow.seller {
            panic!("Winner must be either the Buyer or the Seller");
        }

        // Update state
        escrow.state = EscrowState::Resolved;
        env.storage().instance().set(&DataKey::Escrow(escrow_id), &escrow);

        // Transfer funds from the Contract to the winner
        let token_client = token::Client::new(&env, &escrow.token);
        token_client.transfer(&env.current_contract_address(), &winner, &escrow.amount);
    }

    /// Helper function: Get transaction information by ID
    pub fn get_escrow_info(env: Env, escrow_id: u64) -> Escrow {
        Self::get_escrow(&env, escrow_id)
    }

    // Internal function to reuse data fetching logic
    fn get_escrow(env: &Env, escrow_id: u64) -> Escrow {
        env.storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("Transaction with this ID not found"))
    }
}
