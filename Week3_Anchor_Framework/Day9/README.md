## Anchor Homework Notes:
Anchor Homework : Create a hello world project.

In client.ts:
```commandline
// Import anchor
use anchor_lang::prelude::*;

declare_id!("22633HTZVAuULraZ7wYt1KMt5uYPrgy74btboC1ow2A1");


#[program]
mod hello_world {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
```

In lib.rs:
```commandline
// Import anchor
use anchor_lang::prelude::*;

declare_id!("22633HTZVAuULraZ7wYt1KMt5uYPrgy74btboC1ow2A1");


#[program]
mod hello_world {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}

```

![anchor_hello_world](https://github.com/Mikerniker/Encode_Solana_Bootcamp_HW/assets/63586831/c5232bbe-1821-4dd4-8027-ad6c4823cf31)
