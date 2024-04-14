

*Note this is not complete yet, still confused* 
```commandline
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
```