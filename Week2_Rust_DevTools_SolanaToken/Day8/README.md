### Solana Tokens
1. Follow the instructions from the lesson and use the spl-token-cli to create
a) A fungible token with a supply of 10,000 
- [Transaction Link](https://solscan.io/tx/4uQAA5HwJsrXpgmXDA1XfNHCvkLrd9Z3xnXQ7QUeffhQhjpxVUsyBWgrD7fp3Wxz9TqgBuD2Bd1QQm7Uj3WRVwqB?cluster=devnet)

b) An NFT
- [Transaction Link](https://solscan.io/tx/5ExdSrvkYYcpH89Tvihkh3Uqdkj3Ahk64kQALbwj6xy5fRno5SWGcnBJKUPK58XojGHSJKH5bRF4KnpcHzQv8TiG?cluster=devnet)

2. Try sending these tokens to others in your team , and use the command line to find
details about the tokens.
Sent tokens using transfer --fund-recipient approach
- [Transaction Link](https://solscan.io/tx/57HE9Y8LuMNJ457kPj44NaEgKjGgPA5JLZcooXsnu1GpsLgBi8WULi559TYgSqamBbUAVHuJiAahHZ72aUkQDig5?cluster=devnet)

### Solana Programs
**with Notes from Park and Seth**
Using the examples in the repo

1. Modify the existing msg! in example1-helloworld to log the program ID.

```msg!("Hello World, this is the Program ID: {:?}", _program_id);```

2. Retrieve the total program size of example1-helloworld. 

solana account ```5ScEQEEciCLPp5CSVEixisB4X19ngkVbydq34chHRAwp```
```Size: Length: 36 (0x24) bytes```

3. Retrieve the lamport balance of example2- counter. 
LAMPORT BALANCE: 0.00114144 SOL 
```
npm run deploy:2

solana balance 88TSaKgVJpzD5oMHtGxDDhgfzzCCu6ruCNr92ZnjRBF1 --url https://api.devnet.solana.com
```

4. Modify the client for example2-counter to feed an incorrect address for the greeting
hint use solana account <ADDRESS> to find out more about a given address content

*Notes from Seth:*

```solana account 88TSaKgVJpzD5oMHtGxDDhgfzzCCu6ruCNr92ZnjRBF1```

```
const instruction = new TransactionInstruction({
      keys: [{ pubkey: myIncorrectPubKey, isSigner: false, isWritable: true }],
      programId,
      data: Buffer.alloc(0), // All instructions are hellos
    });
   ```