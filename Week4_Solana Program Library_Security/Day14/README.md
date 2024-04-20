#### Homework 14
- Work through the Tiny Adventure tutorial from the Solana Cookbook.

Notes:
- Did it on Solana Playground and downloaded my code into the `TinyAdventure` directory.


**How to run locally**

Install Anchor

**Install dependencies**

**Run:**

`yarn`

**Build**

`anchor build`

**Test**

`anchor test`

**Run client**

`anchor run client`

*Solana Playground Note:* 
You might need to adjust the client and test code to fully work in local 
Node environment since there are playground exclusive features, 
e.g. if you are using pg.wallets.myWallet, you'll need to manually load
each keypair.