### Homework 11
1. Use the Anchor command line tools to create a new project.
. Adapt the default program as follows:
  - In an account we want to store a balance of type u64.
  - On initialisation, this balance should be set to 100.
  - Write a test to check that the balance was initialised correctly.
  
**Note this is not complete yet, still confused...** 
In lib.rs:

```commandline
use anchor_lang::prelude::*;

declare_id!("24gY4gEFHmtEyDJRE5uTZamUdDNrkMmnZ4o5Gn8D3Ehc");

#[program]
pub mod makebalance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut data = ctx.accounts.data;
        data.balance = 100;
        data.authority = ctx.accounts.admin.key();
              
        Ok(())
    }
```

<!--

use anchor_lang::prelude::*;

declare_id!("24gY4gEFHmtEyDJRE5uTZamUdDNrkMmnZ4o5Gn8D3Ehc");

#[program]
pub mod makebalance {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut data = ctx.accounts.data.load_init()?;
        data.balance = 100;
        data.authority = *ctx.accounts.initializer.key;
        data.save()?;
       
        Ok(())
    }
    pub fn test_balance(ctx: Context<TestBalance>) -> Result<()> {
        let data = ctx.accounts.initializer.load()?;
        assert_eq!(data.balance, 100, "Balance was not initialized correctly");
        Ok(())
}
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = initializer, space = 16)]
    pub data: Account<'info, BalanceData>,
    pub initializer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TestBalance<'info> {
    #[account(mut)]
    pub initializer: Account<'info, BalanceData>,
}

#[account]
pub struct BalanceData {
    pub balance: u64,
    pub authority: Pubkey,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::test::*;

    #[test]
    fn test_initialize() {
        // Prepare the program test environment
        let mut program = ProgramTest::new("makebalance", id(), processor!(initialize));

        // Start the test environment
        let mut context = program.start_with_context().await;

        // Fetch the initialized account
        let account = &context.accounts.initializer;

        // Check that the balance was initialized correctly
        assert_eq!(account.balance, 100);
    }
}
--->

**Lottery Program**

  - **example1-lottery/src/lib.rs**
1. Modify the lottery program so that the payout is only 90% of the total deposits.

```commandline
  // Calculate payout (90% of total deposits)
        let payout_amount = (balance * 90) / 100;
```
2. Add a function that allows lottery admin to withdraw funds after the winner is picked.
```commandline
    // Withdraw funds by admin after the winner is picked  
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;
        let admin: &mut Signer = &mut ctx.accounts.admin;

        // Only admin can withdraw funds
        assert!(admin.key() == &lottery.authority, "Only admin can withdraw funds.");

        // Transfer funds to admin
        **admin.to_account_info().try_borrow_mut_lamports()? += lottery.to_account_info().lamports();

        // Reset lottery
        lottery.count = 0;
        lottery.winner_index = 0;

        Ok(())
    }
```
and
```commandline
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub admin: Signer<'info>,
}
```



<!--- 
use anchor_lang::prelude::*;

declare_id!("EnNAUhQEdDNtNszfguvK5RSkSLDStPLtUqeLpbjayoNq");

#[program]
mod example1 {
    use super::*;       

    // Creates an account for the lottery
    pub fn initialise_lottery(ctx: Context<Create>, ticket_price: u64, oracle_pubkey: Pubkey) -> Result<()> {        
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;        
        lottery.authority = ctx.accounts.admin.key();                
        lottery.count = 0;           
        lottery.ticket_price = ticket_price;
        lottery.oracle = oracle_pubkey;

        Ok(())
    }

    // Buy a lottery ticket
    pub fn buy_ticket(ctx: Context<Submit>) -> Result<()> {
        
        // Deserialise lottery account
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;          
        let player: &mut Signer = &mut ctx.accounts.player;                 

        // Transfer lamports to the lottery account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &player.key(),
            &lottery.key(),
            lottery.ticket_price,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                player.to_account_info(),
                lottery.to_account_info(),
            ],
        )?;

        // Deserialise ticket account
        let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;                

        // Set submitter field as the address pays for account creation
        ticket.submitter = ctx.accounts.player.key();

        // Set ticket index equal to the counter
        ticket.idx = lottery.count;        

        // Increment total submissions counter
        lottery.count += 1;                      

        Ok(())  
    }
    
    // Oracle picks winner index
    pub fn pick_winner(ctx: Context<Winner>, winner: u32) -> Result<()> {

        // Deserialise lottery account
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;
        
        // Set winning index
        lottery.winner_index = winner;                

        Ok(())
    }    

    // Payout prize to the winner
    pub fn pay_out_winner(ctx: Context<Payout>) -> Result<()> {

        // Check if it matches the winner address
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;
        let recipient: &mut AccountInfo =  &mut ctx.accounts.winner;        

        // Get total money stored under original lottery account
        let balance: u64 = lottery.to_account_info().lamports();   
        
        // Calculate payout (90% of total deposits)
        let payout_amount = (balance * 90) / 100;     //ADDED
            
        **lottery.to_account_info().try_borrow_mut_lamports()? -= balance;
        **recipient.to_account_info().try_borrow_mut_lamports()? += balance; 
        
        Ok(())
    }
    // Withdraw funds by admin after the winner is picked  #ADDED
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;
        let admin: &mut Signer = &mut ctx.accounts.admin;

        // Only admin can withdraw funds
        assert!(admin.key() == &lottery.authority, "Only admin can withdraw funds.");

        // Transfer funds to admin
        **admin.to_account_info().try_borrow_mut_lamports()? += lottery.to_account_info().lamports();

        // Reset lottery
        lottery.count = 0;
        lottery.winner_index = 0;

        Ok(())
    }
}

// Contexts
////////////////////////////////////////////////////////////////

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = admin, space = 8 + 180)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub admin: Signer<'info>,    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Submit<'info> {            
    #[account(init, 
        seeds = [
            &lottery.count.to_be_bytes(), 
            lottery.key().as_ref()
        ], 
        constraint = player.to_account_info().lamports() >= lottery.ticket_price,
        bump, 
        payer = player, 
        space=80
    )]
    pub ticket: Account<'info, Ticket>,        
    #[account(mut)]                                 
    pub player: Signer<'info>,                     // Payer for account creation    
    #[account(mut)]       
    pub lottery: Account<'info, Lottery>,          // To retrieve and increment counter        
    pub system_program: Program<'info, System>,    
}

#[derive(Accounts)]
pub struct Winner<'info> {    
    #[account(mut, constraint = lottery.oracle == *oracle.key)]
    pub lottery: Account<'info, Lottery>,        
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct Payout<'info> {             
    #[account(mut, 
        constraint = 
        ticket.submitter == *winner.key && 
        ticket.idx == lottery.winner_index        
    )]       
    pub lottery: Account<'info, Lottery>,          // To assert winner and withdraw lamports
    #[account(mut)]       
    /// CHECK: Not dangerous as it only receives lamports
    pub winner: AccountInfo<'info>,                // Winner account
    #[account(mut)]                  
    pub ticket: Account<'info, Ticket>,            // Winning PDA
}

//ADDED
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

// Accounts
////////////////////////////////////////////////////////////////

// Lottery account 
#[account]
pub struct Lottery {    
    pub authority: Pubkey, 
    pub oracle: Pubkey, 
    pub winner: Pubkey,
    pub winner_index: u32, 
    pub count: u32,
    pub ticket_price: u64,
}

// Ticket PDA
#[account]
#[derive(Default)] 
pub struct Ticket {    
    pub submitter: Pubkey,    
    pub idx: u32,
}

---> 