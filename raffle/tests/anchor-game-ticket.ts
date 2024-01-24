import * as anchor from "@project-serum/anchor";
import { AnchorProvider, IdlTypes, Program, Provider, Wallet } from "@project-serum/anchor";
// @ts-ignore
import { AnchorRaffleTicket } from "../target/types/anchor_raffle_ticket";
import {
    Keypair, LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram, SYSVAR_RENT_PUBKEY, Transaction,
    TransactionInstruction
} from '@solana/web3.js';

import {
    ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction,
    createMint,
    getAssociatedTokenAddress,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import {Buffer} from "buffer";
const MEMO_PROGRAM_ID: PublicKey = new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr');
const VAULT_SKT_SEED_PREFIX = "skt_pool";
const GLOBAL_ACCOUNT_SEED = "global_account";

async function getAndPrintAccount(program: any, raffleAddress: PublicKey) {
    let account = await program.account.raffle.fetchNullable(raffleAddress);

    if (account) {
        account.pricePerTicketNum = account.pricePerTicket.toNumber();
        account.pricePerTicketFloat = account.pricePerTicket.toNumber() / anchor.web3.LAMPORTS_PER_SOL;
        account.tokenSplAddress = account.tokenSplAddress.toString();
    }

    console.log('Account details:', account);

    return account;
}

async function getSPLTokensBalance(account: PublicKey) {
    const program = anchor.workspace.AnchorRaffleTicket as anchor.Program<AnchorRaffleTicket>;

    const balance = await program.provider.connection.getParsedTokenAccountsByOwner(account, { programId: TOKEN_PROGRAM_ID });

    if (balance.value) {
        console.log(`=========================================================================`);
        console.log(`SPL Tokens Balance for ${account.toString()}:`);
        balance.value.forEach((accountInfo) => {
            let pubKey = accountInfo.pubkey.toBase58();
            let mint = accountInfo.account.data["parsed"]["info"]["mint"];
            let owner = accountInfo.account.data["parsed"]["info"]["owner"];
            let decimal = accountInfo.account.data["parsed"]["info"]["tokenAmount"]["decimals"];
            let amount = accountInfo.account.data["parsed"]["info"]["tokenAmount"]["amount"];
            console.log(`mint: ${mint} | ${Math.ceil(Number(amount))} | ${Math.ceil(amount / LAMPORTS_PER_SOL)}`);
            //console.log(`owner: ${owner} | pubKey: ${pubKey}`);
        });
        console.log(`=========================================================================`);
    }
}

async function spawnMoney(
    program: anchor.Program<AnchorRaffleTicket>,
    to: PublicKey,
    sol: number
): Promise<anchor.web3.TransactionSignature> {
    const lamports = sol * anchor.web3.LAMPORTS_PER_SOL;
    const transaction = new anchor.web3.Transaction();
    transaction.add(
        anchor.web3.SystemProgram.transfer({
            // @ts-ignore
            fromPubkey: program.provider.wallet.publicKey,
            lamports,
            toPubkey: to,
        })
    );

    // @ts-ignore
    console.log(`Sending SOL: ${program.provider.wallet.publicKey.toString()} sent ${sol} to ${to.toString()} `);

    const tx = await program.provider.sendAndConfirm(transaction, [], {
        commitment: "confirmed",
    });
    ;

    console.log("DONE:", tx);
    return tx;
}

async function createNewMint(program: anchor.Program<AnchorRaffleTicket>, payer: Keypair): Promise<anchor.web3.PublicKey> {
    return await createMint(
        program.provider.connection,
        payer,
        program.provider.publicKey,
        program.provider.publicKey,
        0,
    );
}

describe("anchor-game-ticket", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.AnchorRaffleTicket as anchor.Program<AnchorRaffleTicket>;

    const initializedGlobalActive = true;
    const initializedVaultTestActive = false;
    const initializedAndWithdrawVaultTestActive = false;
    const initializedTestActive = false;
    const buyTicketSOLTestActive = false;
    const buyTicketSPLTokenTestActive = false;

    it("Global Init!", async () =>
    {
        if (!initializedGlobalActive) return;

        // @ts-ignore
        const wallet = program.provider.wallet;

        //console.log(program.provider.connection);
        console.log(program.programId.toString());
        console.log(wallet.publicKey.toString());

        const [global] = await PublicKey.findProgramAddress(
            [
                Buffer.from(GLOBAL_ACCOUNT_SEED)
            ],
            program.programId
        );

        console.log("Global:", global.toString());

        const txSignature = await program.rpc.initializeGlobal({
            accounts: {
                payer: wallet.publicKey,
                global,
                admin: wallet.publicKey,
                systemProgram: SystemProgram.programId
            },
        });

        console.log(txSignature);
        await program.provider.connection.confirmTransaction(txSignature, "confirmed");
        console.log("Global Address: ", global.toString());

        const globalAccount = await program.account.global.fetchNullable(global);
        console.log("Global Account:", globalAccount);
    });

    it("Program Init Vault and Withdraw!", async () =>
    {
        if (!initializedVaultTestActive) return;

        const client = Keypair.generate();
        await spawnMoney(program, client.publicKey, 0.01);

        const payer = Keypair.generate();
        await spawnMoney(program, payer.publicKey, 0.01);

        const tokenSPLAddressKP = Keypair.generate();
        await spawnMoney(program, payer.publicKey, 0.01);

        const _vaultKeypair = Keypair.generate();
        const [_vaultPool, bump] = await PublicKey.findProgramAddress([Buffer.from(VAULT_SKT_SEED_PREFIX), _vaultKeypair.publicKey.toBuffer()], program.programId);
        console.log("Vault:", _vaultKeypair.publicKey.toString());
        console.log("Vault Pool:", _vaultPool.toString(), bump);

        // await program.rpc.test(bump, {
        //     accounts: {
        //         authority: payer.publicKey,
        //         vault: _vaultKeypair.publicKey,
        //         vaultPool: _vaultPool,
        //         systemProgram: SystemProgram.programId
        //     },
        //     signers: [payer, _vaultKeypair]
        // });

        console.log('Successfully tested!');

        //const tokenSPLAddressKP = Keypair.generate();
        const tokenSPLAddress = await createMint(program.provider.connection, payer, payer.publicKey, payer.publicKey, 9, tokenSPLAddressKP);
        //const tokenSPLAddress = new PublicKey("SKTsW8KvzopQPdamXsPhvkPfwzTenegv3c3PEX4DT1o");
        console.log("Vault SPL Token Mint:", tokenSPLAddress.toString());

        // const _vaultPoolSktAccount = await getOrCreateAssociatedTokenAccount(program.provider.connection, payer, tokenSPLAddress, _vaultPool, true);
        const _vaultPoolSktAccount = await getAssociatedTokenAddress(tokenSPLAddress, _vaultPool, true);
        console.log("Vault ATA", _vaultPoolSktAccount.toString());

        // Init Vault
        {
            await program.rpc.initializeVault(bump, SystemProgram.programId,
                {
                    accounts:
                    {
                        payer: payer.publicKey,
                        vault: _vaultKeypair.publicKey,
                        vaultPool: _vaultPool,
                        vaultPoolSktAccount: _vaultPoolSktAccount,
                        sktMint: tokenSPLAddress,
                        memo: MEMO_PROGRAM_ID,
                        rent: SYSVAR_RENT_PUBKEY,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                    },
                    signers: [payer, _vaultKeypair]
                });
        }

        console.log("Init Vault DONE!");
        return;

        let vault = await program.account.vault.fetchNullable(_vaultKeypair.publicKey);
        console.log(vault.tokenType, vault.tokenType?.toString());
        console.log(vault.vaultBump);

        console.log("--> Vault Balance Current:");
        await getSPLTokensBalance(_vaultPool);

        await mintTo(program.provider.connection, payer, tokenSPLAddress, _vaultPoolSktAccount, payer.publicKey, 100);

        console.log("--> Vault Balance After Mint:");
        await getSPLTokensBalance(_vaultPool);

        if (!initializedAndWithdrawVaultTestActive) return;

        const _clientATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, client, tokenSPLAddress, client.publicKey);
        console.log("Client ATA", _clientATA.address.toString());

        const [global] = await PublicKey.findProgramAddress(
            [
                Buffer.from("global_account")
            ],
            program.programId
        );

        await program.rpc.withdrawVault(
            new anchor.BN(10 * LAMPORTS_PER_SOL),
            {
                accounts:
                {
                    global,
                    claimer: client.publicKey,
                    claimerSktAccount: _clientATA.address,
                    vault: _vaultKeypair.publicKey,
                    vaultPool: _vaultPool,
                    vaultPoolSktAccount: _vaultPoolSktAccount,
                    sktMint: tokenSPLAddress,
                    rent: SYSVAR_RENT_PUBKEY,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                },
                signers: [client],
            });

        console.log("--> Client Balance:");
        await getSPLTokensBalance(client.publicKey);

        console.log("--> Vault Balance:");
        await getSPLTokensBalance(_vaultPool);
    });

    it("Program Init!", async () => {
        if (!initializedTestActive) return;

        const isLocalNet = program.provider.connection.rpcEndpoint.includes("localhost");

        console.log("START:");
        console.log("========");

        const recipient = new PublicKey("3xeW8eLMunbmMW83n2wLqNkiEr4GsUFJjzM6h19fhwot"); // raffle bank

        const senderWallet = Keypair.generate();
        // const senderWallet = Keypair.fromSecretKey(new Uint8Array([133, 230, 105, 82, 126, 147, 188, 49, 144, 121, 98, 112, 160, 239, 106, 142, 105, 92, 58, 193, 34, 169, 161, 57, 57, 231, 154, 146, 19, 17, 244, 172, 16, 123, 70, 229, 190, 105, 161, 60, 53, 123, 148, 82, 214, 237, 122, 193, 24, 62, 101, 168, 243, 70, 149, 117, 33, 159, 75, 104, 193, 83, 97, 231]));
        console.log("Sender:", senderWallet.publicKey.toString());
        console.log("Sender Key:", senderWallet.secretKey.toString());

        await spawnMoney(program, senderWallet.publicKey, 0.1);

        let tokenSPLAddress = PublicKey.default;
        // const tokenSPLAddress = new PublicKey("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ");
        console.log('Token SPL Address:', tokenSPLAddress.toString());

        const raffle = Keypair.generate();
        // const raffle = Keypair.fromSecretKey(new Uint8Array([116, 70, 177, 15, 159, 21, 163, 29, 18, 111, 62, 73, 143, 52, 203, 88, 129, 60, 61, 116, 176, 164, 238, 178, 105, 163, 25, 225, 65, 211, 117, 131, 188, 197, 246, 113, 242, 134, 90, 196, 40, 170, 246, 139, 143, 141, 232, 15, 196, 251, 28, 76, 66, 22, 115, 20, 32, 220, 89, 54, 14, 235, 241, 65]));
        console.log("Raffle :", raffle.publicKey.toString());
        console.log("Raffle Key:", raffle.secretKey.toString());

        const nftMint = await createNewMint(program, senderWallet);
        const tokenSPLKP = Keypair.fromSecretKey(new Uint8Array([49, 126, 59, 3, 106, 46, 22, 87, 188, 63, 0, 238, 192, 16, 55, 75, 177, 173, 142, 218, 56, 96, 93, 143, 170, 249, 239, 112, 251, 48, 162, 219, 2, 49, 81, 147, 24, 20, 128, 249, 157, 159, 165, 51, 122, 99, 64, 51, 129, 48, 26, 141, 193, 94, 225, 33, 234, 172, 105, 92, 112, 94, 134, 168]));
        console.log("nft mint address :", nftMint.toString());


        if (isLocalNet) {
            console.log("localnet...\n");

            tokenSPLAddress = await createMint(program.provider.connection, senderWallet, senderWallet.publicKey, null, 9, tokenSPLKP);

            const senderATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet, tokenSPLAddress, senderWallet.publicKey);

            console.log("Token SPL Address:", tokenSPLAddress.toString());
            console.log("senderATA:", senderATA.address.toString());

            // Since we're on localnet, mint some tokens to sender
            const tokensAmount = 1; // simulate 1 nft
            await mintTo(program.provider.connection, senderWallet, tokenSPLAddress, senderATA.address, senderWallet.publicKey, tokensAmount);
        }

        await getSPLTokensBalance(senderWallet.publicKey);

        const sourcePublicKey = senderWallet.publicKey;
        const destPublicKey = recipient;

        const sourceATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, tokenSPLKP, tokenSPLKP.publicKey, sourcePublicKey);
        let recipientATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet, tokenSPLKP.publicKey, destPublicKey);

        // Make sure receiver has a token account active
        const receiverAccount = await program.provider.connection.getAccountInfo(recipientATA.address);
        let transaction = new Transaction();
        if (receiverAccount === null) {
            console.log("Creating recipientATA:", recipientATA.toString());
            transaction.add(createAssociatedTokenAccountInstruction(tokenSPLAddress, recipientATA.address, destPublicKey, sourcePublicKey));
        }

        let account = await getAndPrintAccount(program, raffle.publicKey);

        if (!account) {
            const price = 1;
            const priceBN = new anchor.BN(price * anchor.web3.LAMPORTS_PER_SOL);
            const amount = 8;
            // @ts-ignore
            await program.rpc.initialize(tokenSPLAddress, priceBN, amount,
                {
                    accounts: {
                        // Init Raffle
                        payer: senderWallet.publicKey,
                        raffle: raffle.publicKey,
                        systemProgram: SystemProgram.programId,

                        // Token Transfer
                        //senderTokens: sourceATA.address,
                        //recipientTokens: recipientATA.address,
                        tokenProgram: TOKEN_PROGRAM_ID
                    },
                    signers: [senderWallet, raffle],
                    options: {
                        commitment: "confirmed"
                    }
                });
        }

        console.log("Init DONE");
        await getAndPrintAccount(program, raffle.publicKey);
    });

    it("Buy Ticket with SOL!", async () => {
        if (!buyTicketSOLTestActive) return;

        console.log("\nStep-1:");

        // @ts-ignore
        console.log("Program Wallet:", program.provider.wallet.publicKey.toString());

        //const receiver = Keypair.generate();
        const receiver = Keypair.fromSecretKey(new Uint8Array([133, 230, 105, 82, 126, 147, 188, 49, 144, 121, 98, 112, 160, 239, 106, 142, 105, 92, 58, 193, 34, 169, 161, 57, 57, 231, 154, 146, 19, 17, 244, 172, 16, 123, 70, 229, 190, 105, 161, 60, 53, 123, 148, 82, 214, 237, 122, 193, 24, 62, 101, 168, 243, 70, 149, 117, 33, 159, 75, 104, 193, 83, 97, 231]));
        console.log("receiver:", receiver.publicKey.toString());
        //console.log(receiver.secretKey.toString());
        await spawnMoney(program, receiver.publicKey, 0.1);

        const tokenSPLAddress = PublicKey.default;
        // const tokenSPLAddress = new PublicKey("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ");
        console.log('Token type:', tokenSPLAddress.toString());

        // const raffle = Keypair.generate();
        const raffle = Keypair.fromSecretKey(new Uint8Array([116, 70, 177, 15, 159, 21, 163, 29, 18, 111, 62, 73, 143, 52, 203, 88, 129, 60, 61, 116, 176, 164, 238, 178, 105, 163, 25, 225, 65, 211, 117, 131, 188, 197, 246, 113, 242, 134, 90, 196, 40, 170, 246, 139, 143, 141, 232, 15, 196, 251, 28, 76, 66, 22, 115, 20, 32, 220, 89, 54, 14, 235, 241, 65]));
        console.log("Raffle :", raffle.publicKey.toString());
        console.log("Raffle Secret:", raffle.secretKey.toString());

        await getAndPrintAccount(program, raffle.publicKey);

        console.log("\nStep-2:");
        const buyer = Keypair.generate();
        console.log("Buyer:", buyer.publicKey.toString());
        await spawnMoney(program, buyer.publicKey, 1);

        const ticketsAmountToBuy = 1;
        const ticketsPrice = 0.1;
        const ticketPriceLAMPORTS = ticketsPrice * anchor.web3.LAMPORTS_PER_SOL;

        console.log("\nStep-3:");
        console.log(`Wants to buy tickets: ${ticketsAmountToBuy} price: ${ticketsPrice} tokenSPLAddress: ${tokenSPLAddress}`);

        const listener = program.addEventListener("BuyEvent", (event, slot) => {
            console.log("BuyEvent:", event.buyer.toString(), event.amount, event.soldTickets, event.totalTickets, event.remainingTickets, slot);
        })

        console.log("\nStep-4 Buying...");

        // @ts-ignore
        await program.rpc.buyTicketSol(ticketsAmountToBuy, new anchor.BN(ticketPriceLAMPORTS), tokenSPLAddress,
            {
                accounts: {
                    buyer: buyer.publicKey,
                    recipient: receiver.publicKey,
                    raffle: raffle.publicKey,
                    systemProgram: SystemProgram.programId,
                },
                signers: [buyer],
                options: {
                    commitment: "confirmed"
                }
            });

        let account = await getAndPrintAccount(program, raffle.publicKey);

        console.log("\nDONE:", account);

        await program.removeEventListener(listener);
    });

    it("Buy ticket with Spl Token!", async () => {
        if (!buyTicketSPLTokenTestActive) return;

        const isLocalNet = program.provider.connection.rpcEndpoint.includes("localhost");

        const senderWallet = Keypair.fromSecretKey(new Uint8Array([133, 230, 105, 82, 126, 147, 188, 49, 144, 121, 98, 112, 160, 239, 106, 142, 105, 92, 58, 193, 34, 169, 161, 57, 57, 231, 154, 146, 19, 17, 244, 172, 16, 123, 70, 229, 190, 105, 161, 60, 53, 123, 148, 82, 214, 237, 122, 193, 24, 62, 101, 168, 243, 70, 149, 117, 33, 159, 75, 104, 193, 83, 97, 231]));
        console.log("Sender:", senderWallet.publicKey, "| LocalNet:", isLocalNet);

        const raffle = new PublicKey("Dhtj8XBj94hyawArEkE1TRE1vqaBdmmfaBgMYBHAR75r");
        console.log("Raffle :", raffle.toString());

        let tokenSPLAddress = new PublicKey("ASxC3n3smkcUkA7Z58EUKZ2NfHoQ8eZrkTRK7ergYr2a"); // $CRECK devnet

        if (isLocalNet) {
            const tokenSPLKP = Keypair.fromSecretKey(new Uint8Array([49, 126, 59, 3, 106, 46, 22, 87, 188, 63, 0, 238, 192, 16, 55, 75, 177, 173, 142, 218, 56, 96, 93, 143, 170, 249, 239, 112, 251, 48, 162, 219, 2, 49, 81, 147, 24, 20, 128, 249, 157, 159, 165, 51, 122, 99, 64, 51, 129, 48, 26, 141, 193, 94, 225, 33, 234, 172, 105, 92, 112, 94, 134, 168]));
            tokenSPLAddress = tokenSPLKP.publicKey; //await createMint(program.provider.connection, senderWallet.payer, senderWallet.publicKey, null, 9);

            const senderATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet, tokenSPLAddress, senderWallet.publicKey);

            console.log("tokenSPLAddress:", tokenSPLAddress.toString());
            console.log("senderATA:", senderATA.address.toString());
        }

        await getSPLTokensBalance(senderWallet.publicKey);

        const senderATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet, tokenSPLAddress, senderWallet.publicKey);

        const recipient = new PublicKey("3xeW8eLMunbmMW83n2wLqNkiEr4GsUFJjzM6h19fhwot"); // raffle bank
        const recipientATA = await getOrCreateAssociatedTokenAccount(program.provider.connection, senderWallet, tokenSPLAddress, recipient);

        await getAndPrintAccount(program, raffle);

        console.log("tokenSPLAddress:", tokenSPLAddress.toString());
        console.log("recipientATA:", recipientATA.address.toString());

        const ticketsAmountToBuy = 1;
        const ticketsPrice = 1;
        const ticketPriceLAMPORTS = Math.ceil(ticketsPrice * anchor.web3.LAMPORTS_PER_SOL);

        // @ts-ignore
        await program.rpc.buyTicketSpl(ticketsAmountToBuy, new anchor.BN(ticketPriceLAMPORTS), tokenSPLAddress,
            {
                accounts:
                {
                    raffle: raffle,
                    sender: senderWallet.publicKey,
                    senderTokens: senderATA.address,
                    recipientTokens: recipientATA.address,
                    tokenProgram: TOKEN_PROGRAM_ID
                },
                signers: [senderWallet]
            });

        console.log("Ticket Purchase DONE!");

        await getAndPrintAccount(program, raffle);
        await getSPLTokensBalance(senderWallet.publicKey);
    });

    it("Memo", async () => {
        await program.rpc.memo({
            accounts: {
                memo: MEMO_PROGRAM_ID
            }
        });
    });
});
