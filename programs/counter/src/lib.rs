use anchor_lang::prelude::*;

declare_id!("5fZosDLD4gDsNpzqxmwnUTGK3EKXk2TciajMwEttx3Jj");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //Reference to the coutner account from the Initialize struct
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        //Mutable reference to the counter account from the Increment struct
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // Increment the count value stored on the counter account by 1
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incrementad! Current count: {}", counter.count);
        Ok(())
    }
}

// Accounts required by the initialize instruction
// anchor macro and a struct
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] //account paying to create counter account
    pub user: Signer<'info>, // specify account must be a signer of the transaction

    //The counter account being created and initialized in the instruction
    // an anchor attribute that generates more code
    #[account(
        init,           //specifies we are creating this account
        payer = user,   //specifies account paying for the creation of the account
        space = 8 + 8    //space allocated to the new account (8 byte discrimator + 8byte for u64)
    )]
    //data type definition
    pub counter: Account<'info, Counter>, //specify account is 'Counter' type
    pub system_program: Program<'info, System>, //specify account must be system program
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] //specify account is mutable becuase we are updating its data
    pub counter: Account<'info, Counter>, //specify account is 'Counter' type
}

//struct for the account
#[account]
pub struct Counter {
    pub count: u64, //define count value type as u64
}

// #[program]
// pub mod counter {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}
