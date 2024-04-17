use anchor_lang::prelude::*;

declare_id!("EDimXSJLG8u83mqyZJYtxrRFLQBaePexbYFtuw5ijBau");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //Reference to the coutner account from the Initialize struct
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; //store bump seed in "Counter" account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
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
        seeds = [b"counter2"], // optional seeds for pda
        bump,                  //bump seed for pda
        payer = user,   //specifies account paying for the creation of the account
        space = 8 + Counter::INIT_SPACE    //space allocated to the new account (8 byte discrimator + 8byte for u64)
    )]
    //data type definition
    pub counter: Account<'info, Counter>, //specify account is 'Counter' type
    pub system_program: Program<'info, System>, //specify account must be system program
}

#[derive(Accounts)]
pub struct Increment<'info> {
    // the address of the 'Counter' account must be a PDA derived with the specific 'seeds'
    #[account(
        mut,
        seeds = [b"counter2"], //optinal seeds for pda
        bump = counter.bump, // bump seeds for pda
    )] //specify account is mutable becuase we are updating its data
    pub counter: Account<'info, Counter>, //specify account is 'Counter' type
}

//struct for the account
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, //define count value type as u64 - 8bytes
    pub bump: u8,   // 1 byte
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
