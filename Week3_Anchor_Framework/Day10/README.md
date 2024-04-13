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
Follow the installation instructions in the notes.
Try following the functionality it provides:
- Try a simple client transaction in Solana playground (https://beta.solpg.io/) Make sure you are connected to the devnet and you have a wallet set up:
  - Try the airdrop to give yourself some SOL
   ![hw10a](https://github.com/Mikerniker/Encode_Solana_Bootcamp_HW/assets/63586831/ce22f209-ef22-4479-85ac-18bd981aeacd)

  - Try to sign a message
   ![hw10b](https://github.com/Mikerniker/Encode_Solana_Bootcamp_HW/assets/63586831/3a5d685e-bc8e-4d25-99b2-ae0e7feb0732)


<!---
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { SystemProgram, Transaction, TransactionSignature, PublicKey } from '@solana/web3.js';
import { FC, useCallback } from 'react';
import { notify } from "../utils/notifications";

// Hardcoded recipient address
const RECIPIENT_ADDRESS = "5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj";

export const SendTransaction: FC = () => {
    const { connection } = useConnection(); // Use useConnection hook to get connection
    const { publicKey, sendTransaction } = useWallet();
    const onClick = useCallback(async () => {
        if (!publicKey) {
            notify({ type: 'error', message: `Wallet not connected!` });
            console.log('error', `Send Transaction: Wallet not connected!`);
            return;
        }

        let signature: TransactionSignature = '';
        try {
            // Create instruction to send SOL to the hardcoded address
            const instruction = SystemProgram.transfer({
                fromPubkey: publicKey,
                toPubkey: new PublicKey(RECIPIENT_ADDRESS),
                lamports: 1_000_000, // Amount in lamports (1 SOL)
            });

            // Create a new Transaction
            const transaction = new Transaction().add(instruction);

            // Send transaction and await for signature
            signature = await sendTransaction(transaction, connection);

            // Confirm transaction
            await connection.confirmTransaction(signature, 'confirmed');

            console.log(signature);
            notify({ type: 'success', message: 'Transaction successful!', txid: signature });
        } catch (error: any) {
            notify({ type: 'error', message: `Transaction failed!`, description: error?.message, txid: signature });
            console.log('error', `Transaction failed! ${error?.message}`, signature);
        }
    }, [publicKey, notify, connection, sendTransaction]);

    return (
        <div className="flex flex-row justify-center">
            <div className="relative group items-center">
                <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500 
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={onClick} disabled={!publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Send Transaction
                        </span>
                    </button>
             </div>
        </div>
    );
};

-->
