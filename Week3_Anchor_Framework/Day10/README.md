### Homework 10

1. Try a simple client transaction in Solana playground (https://beta.solpg.io/)
. Make sure you are connected to the devnet and you have a wallet set up
. Run the default client code, this will tell you your balance.

![part10a](https://github.com/Mikerniker/Encode_Solana_Bootcamp_HW/assets/63586831/e4bc04c8-7646-42c1-b53e-eadd8933f99c)

  - Create an airdrop signature and request the airdrop from the connection object ```pg.connection.requestAirdrop``` you will need to add your public key and the
number of lamports you want.

```commandline
// Added to client.ts
// Request Airdrop
const airdropAmountLamports = 1000000;
const airdropSignature = await pg.connection.requestAirdrop(
    pg.wallet.publicKey,
    airdropAmountLamports
);
console.log("Airdrop requested. Waiting for confirmation...");

```

  - Use ```await pg.connection.confirmTransaction```; to confirm the transaction.
```commandline
// Wait for confirmation
await pg.connection.confirmTransaction(airdropSignature);
console.log("Airdrop confirmed!");
```
Final Version client.ts:
```commandline
// Client
console.log("My address:", pg.wallet.publicKey.toString());

// Request Airdrop
const airdropAmountLamports = 1000000;
const airdropSignature = await pg.connection.requestAirdrop(
    pg.wallet.publicKey,
    airdropAmountLamports
);
console.log("Airdrop requested. Waiting for confirmation...");


// Wait for confirmation
await pg.connection.confirmTransaction(airdropSignature);
console.log("Airdrop confirmed!");


const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

```
**Result:**

![part10b](https://github.com/Mikerniker/Encode_Solana_Bootcamp_HW/assets/63586831/72a86b2a-39b5-449b-a050-2b6629618b02)

2. Investigating Dapp Scaffold
