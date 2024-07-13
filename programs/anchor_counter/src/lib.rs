use anchor_lang::prelude::*;

// ------ Program address ------------------------------------------------------
declare_id!("8r48Y774vYRB4oMPVmScktP4QZYy28fN5BYKYJam49av");

// ------ Program instructions -------------------------------------------------
#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Reference to the counter account frm the Initialize struct
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Mutable reference to the counter account from the Increment struct
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // Increment the counter value stored in the counter account by 1
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

// ------ Structs for the instructions -----------------------------------------

// Account required by the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The account paying to create the counter account
    #[account(mut)]
    pub user: Signer<'info>, // specify account must be signer on the transaction

    // The counter account being created and initialized in the instruction
    #[account(
        init,         // specifies we are creating this account
        payer = user, // specifies account paying for the creation of the account
        space = 8 + 8 // space allocated to the new account (8 byte discriminator + 8 byte for u64)
    )]
    pub counter: Account<'info, Counter>, // specify account is `Counter` type
    pub system_program: Program<'info, System>, // specify account must be System Program
}

// Account required by the `increment` instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] // specify account is mutable because we are updating its data
    pub counter: Account<'info, Counter>, // specify account is `Counter` type
}

// ------ Structs for the accounts ---------------------------------------------

// Define structure of `Counter` account
#[account]
pub struct Counter {
    pub count: u64, // define count value type as `u64`
}