#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8KKg6xgMW6zgNHWXati6ok2TYuPSn82zZvtP3bnZZnwK");

#[program]
pub mod charity {
    use super::*;
    
    //create PDA
    pub fn create_charity(
      ctx: Context<CreateCharity>,
      charity_name: String,
      deposit_amount: u64,
    ) -> ProgramResult {
      let charity = &mut ctx.accounts.charity;
      charity.charity_name = charity_name;
      charity.deposit_amount = deposit_amount;
      charity.owner = *ctx.accounts.user.key; 
      Ok(())
    }
    
    
    // Transfer the deposit to charity
    pub fn make_deposit(
      ctx: Context<MakeDeposit>, amount: u64) -> ProgramResult {
        let transaction = anchor_lang::solana_program::system_instruction::transfer(
          &ctx.accounts.user.key(),  
          &ctx.accounts.charity.key(),   //destination PDA
          amount
      );

      anchor_lang::solana_program::program::invoke(
          &transaction,
          &[
              ctx.accounts.user.to_account_info(),  //payer account
              ctx.accounts.charity.to_account_info()   //charity
          ],
      )?;
      (&mut ctx.accounts.charity).balance += amount;  //updated balance with user payment to charity
      Ok(())
  }

  //   pub fn make_deposit(
  //     ctx: Context<MakeDeposit>,
  //     pub charity_name: String,
  //     pub deposit_amount: u64, 
  // ) -> Result<()> {
  //     msg!("Deposit made!");
  //     msg!("Charity: {}", charity_name);
  //     msg!("Amount Deposited: {}", deposit_amount);

  //     let charity_deposit = &mut ctx.accounts.charity_deposit;
  //     charity_deposit.owner = ctx.accounts.owner.key();
  //     charity_deposit.charity_name = charity_name;
  //     charity_deposit.deposit_amount = deposit_amount;
  //     Ok(())
  // }
}

  // pub fn close(_ctx: Context<CloseCounter>) -> Result<()> {
  //   Ok(())
  // }

  // pub fn decrement(ctx: Context<Update>) -> Result<()> {
  //   ctx.accounts.counter.count = ctx.accounts.counter.count.checked_sub(1).unwrap();
  //   Ok(())
  // }

  // pub fn increment(ctx: Context<Update>) -> Result<()> {
  //   ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
  //   Ok(())
  // }

  // pub fn initialize(_ctx: Context<InitializeCounter>) -> Result<()> {
  //   Ok(())
  // }

  // pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
  //   ctx.accounts.counter.count = value.clone();
  //   Ok(())
  // }
}

// #[derive(Accounts)]
// pub struct InitializeCounter<'info> {
//   #[account(mut)]
//   pub payer: Signer<'info>,

//   #[account(
//   init,
//   space = 8 + Counter::INIT_SPACE,
//   payer = payer
//   )]
//   pub counter: Account<'info, Counter>,
//   pub system_program: Program<'info, System>,
// }
// #[derive(Accounts)]
// pub struct CloseCounter<'info> {
//   #[account(mut)]
//   pub payer: Signer<'info>,

//   #[account(
//   mut,
//   close = payer, // close account and return lamports to payer
//   )]
//   pub counter: Account<'info, Counter>,
// }

// #[derive(Accounts)]
// pub struct Update<'info> {
//   #[account(mut)]
//   pub counter: Account<'info, Counter>,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct Counter {
//   count: u8,
// }

#[derive(Accounts)]
#[instruction(charity_name: String, deposit_amount: u64)]
pub struct CreateCharity<'info> {
    #[account(
        init_if_needed,
        seeds = [charity_name.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + CharityAccount::INIT_SPACE
    )]
    pub charity_name: Account<'info, CharityAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}



#[account]
#[derive(InitSpace)]
pub struct CharityAccount {
    pub owner: Pubkey,
    #[max_len(50)]
    pub charity_name: String,
    pub deposit_amount: u64, 
}