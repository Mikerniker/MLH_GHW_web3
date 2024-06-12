#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

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

}


#[derive(Accounts)]
#[instruction(charity_name: String, deposit_amount: u64)]
pub struct CreateCharity<'info> {
    #[account(
        init_if_needed,
        seeds = [b"charity", user.key().as_ref()],
        bump,
        payer = user,
        space = 5000,
    )]
    pub charity: Account<'info, CharityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct MakeDeposit<'info> {   
   #[account(mut)]
    pub charity: Account<'info, CharityAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct CharityAccount {
    pub owner: Pubkey,
    #[max_len(50)]
    pub charity_name: String,
    pub balance: u64,
    pub deposit_amount: u64, 
}