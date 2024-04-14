### Homework 12

Further develop the anchor program you started in the last homework
1. Add a function to allow the balance to be updated in steps of 100 up to a maximum of 1000.
2. If you try to update the balance when it is at
its maximum value, throw a custom error with
an appropriate error message.
3. What constraints should your program have ?

<!--- 
use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("24gY4gEFHmtEyDJRE5uTZamUdDNrkMmnZ4o5Gn8D3Ehc");

// Define the program namespace
#[program]
pub mod makebalance {
    use super::*;

    // Instruction to initialize the balance
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut data = ctx.accounts.data.load_init()?;
        data.balance = 100;
        data.authority = *ctx.accounts.initializer.key;
        data.save()?;
        Ok(())
    }

    // Instruction to update the balance
    pub fn update_balance(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
        let mut data = ctx.accounts.data.load_mut()?;
        if data.balance == 1000 {
            // Throw a custom error if balance is at its maximum value
            return Err(ErrorCode::BalanceOverflow.into());
        }
        // Ensure balance is updated in steps of 100
        if amount % 100 != 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        // Update balance
        data.balance += amount;
        data.save()?;
        Ok(())
    }

    // Custom error code for balance overflow
    #[error]
    pub enum ErrorCode {
        #[msg("Balance has reached its maximum value")]
        BalanceOverflow,
        #[msg("Amount must be in steps of 100")]
        InvalidAmount,
    }
}

// Account definition for balance data
#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = initializer, space = 16)]
    pub data: Account<'info, BalanceData>,
    pub initializer: AccountInfo<'info>,
}

// Account definition for balance data
#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub data: Account<'info, BalanceData>,
    pub authority: Signer<'info>,
}

// Account data structure for balance
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
        let mut program = ProgramTest::new("makebalance", id(), processor!(makebalance::initialize));
        let mut context = program.start_with_context().await;
        let account = &context.accounts.initializer;
        assert_eq!(account.balance, 100);
    }

    #[test]
    fn test_update_balance() {
        let mut program = ProgramTest::new("makebalance", id(), processor!(makebalance::initialize));
        let mut context = program.start_with_context().await;

        // Test updating balance with a valid amount
        makebalance::update_balance(&mut context, 100).unwrap();
        let account = &context.accounts.initializer;
        assert_eq!(account.balance, 200);

        // Test updating balance with an invalid amount
        let err = makebalance::update_balance(&mut context, 50).unwrap_err();
        assert_eq!(err.to_string(), "Amount must be in steps of 100");

        // Test updating balance beyond the maximum value
        let err = makebalance::update_balance(&mut context, 800).unwrap_err();
        assert_eq!(err.to_string(), "Balance has reached its maximum value");
    }
}



--->
