### Homework 10

1. Try a simple client transaction in Solana playground (https://beta.solpg.io/)
. Make sure you are connected to the devnet and you have a wallet set up
. Run the default client code, this will tell you your balance.


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
Final Version:
```commandline


```