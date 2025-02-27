use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint, msg,
        pubkey::Pubkey,
    },
    uint::construct_uint,
};

// Define U256 type for large number calculations
construct_uint! {
    pub struct U256(4);
}

/// The type of state managed by this program. The type defined here
/// must match the `GreetingAccount` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// The number of greetings that have been sent to this account.
    pub counter: u32,
}

// Pool state structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolState {
    pub base_balance: u64,
    pub quote_balance: u64,
    pub base_need_take_pnl: u64,
    pub quote_need_take_pnl: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    // Initialize pool state
    let pool_state = PoolState {
        base_balance: 1_000_000_000,      // 1000 tokens
        quote_balance: 2_000_000_000,     // 2000 tokens
        base_need_take_pnl: 1_000,
        quote_need_take_pnl: 2_000,
        swap_fee_numerator: 25,           // 0.25% fee
        swap_fee_denominator: 10_000,
    };

    let amount_in: u64 = 100_000;
    let (amount_out, mid_price) = swap_base_in(&pool_state, amount_in);

    // Add success log
    msg!("Swap executed successfully: {} in -> {} out at price {}", 
        amount_in, amount_out, mid_price);

    Ok(())
}

fn swap_base_in(pool_state: &PoolState, amount_in: u64) -> (u64, f64) {
    let swap_fee = (amount_in as f64 * pool_state.swap_fee_numerator as f64
        / pool_state.swap_fee_denominator as f64) as u64;

    let input_after_fee = amount_in - swap_fee;
    let input = U256::from(input_after_fee);

    let x = U256::from(pool_state.base_balance - pool_state.base_need_take_pnl);
    let y = U256::from(pool_state.quote_balance - pool_state.quote_need_take_pnl);

    let after_x = x + input;
    let amount_out = (input * y / after_x).as_u64();
    
    let mid_price = y.as_u64() as f64 / x.as_u64() as f64;

    (amount_out, mid_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_base_in() {
        let pool_state = PoolState {
            base_balance: 1_000_000_000,
            quote_balance: 2_000_000_000,
            base_need_take_pnl: 1_000,
            quote_need_take_pnl: 2_000,
            swap_fee_numerator: 25,
            swap_fee_denominator: 10_000,
        };

        let amount_in = 100_000;
        let (amount_out, mid_price) = swap_base_in(&pool_state, amount_in);

        println!("Amount in: {}", amount_in);
        println!("Amount out: {}", amount_out);
        println!("Mid price: {}", mid_price);

        assert!(amount_out > 0, "Amount out should be greater than 0");
        assert!(mid_price > 0.0, "Mid price should be greater than 0");
    }
}
