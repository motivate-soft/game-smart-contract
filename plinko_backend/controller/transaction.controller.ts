import { createCloseAccountInstruction, NATIVE_MINT, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Request, Response } from 'express';
import { getAta, getCreateAtaInstruction, getGameAddress, game_name, game_owner, backendKp, sendRequest, confirmTransactionSafe, nonceAccountAuth } from './utils';
import { getPayload } from '../middleware/auth.middleware';
import {
  Connection,
  clusterApiUrl,
  // Keypair,
  PublicKey,
  Transaction,
  LAMPORTS_PER_SOL,
  SystemProgram,
  // NONCE_ACCOUNT_LENGTH,
  // TransactionInstruction,
  NonceAccount,
} from"@solana/web3.js";

import { AnchorProvider, Wallet, Program, setProvider, BN } from "@project-serum/anchor";
// import { bs58 } from '@project-serum/anchor/dist/cjs/utils/bytes';

const Plinko = require("../idl/plinko.json");
const programId = Plinko.metadata.address;
const connection = new Connection(clusterApiUrl("devnet"));
const provider = new AnchorProvider(
  connection,
  new Wallet(backendKp),
  AnchorProvider.defaultOptions()
);
setProvider(provider);
const program = new Program(Plinko, programId, provider);

export const getClaimTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint, amount } = req.params;
  const claimer = new PublicKey(wallet);
  
  // get claim amount from DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  // console.log(player.balance, typeof player.balance);
  const am = parseFloat(amount);
  const claimAmount = !am ? player.balance : (am > player.balance ? player.balance : am);
  const [game] = await getGameAddress(program.programId, game_name, game_owner);
  const gameData = await program.account.game.fetchNullable(game);
  if (!gameData) return res.status(500).json({ error: "Cannot get game data" });
  const mint = new PublicKey(tokenMint);

  const transaction = new Transaction();
  
  // const nonceAccountAuth = Keypair.generate();
  // console.log(bs58.encode(nonceAccountAuth.secretKey));
  // console.log(nonceAccountAuth.secretKey.);
  // console.log(nonceAccountAuth.secretKey);
  // let nonceAccount = Keypair.generate();
  // console.log(nonceAccount.publicKey.toString());
  const config = require("../config.json");
  let nonceAccount = new PublicKey(config.nonceAccount);
  transaction.add(
    // SystemProgram.createAccount({
    //   fromPubkey: claimer,
    //   newAccountPubkey: nonceAccount.publicKey,
    //   lamports: await connection.getMinimumBalanceForRentExemption(
    //     NONCE_ACCOUNT_LENGTH
    //   ),
    //   space: NONCE_ACCOUNT_LENGTH,
    //   programId: SystemProgram.programId,
    // }),
    // // init nonce account
    // SystemProgram.nonceInitialize({
    //   noncePubkey: nonceAccount.publicKey, // nonce account pubkey
    //   authorizedPubkey: nonceAccountAuth.publicKey, // nonce account authority (for advance and close)
    // }),
    SystemProgram.nonceAdvance({
      // noncePubkey: nonceAccount.publicKey,
      noncePubkey: nonceAccount,
      authorizedPubkey: nonceAccountAuth.publicKey,
    }),
  );  

  const claimerAta = await getAta(mint, claimer);
  const instruction = await getCreateAtaInstruction(provider, claimerAta, mint, claimer);
  if (instruction) transaction.add(instruction);
  const gameTreasuryAta = await getAta(mint, game, true);
  transaction.add(
    program.transaction.claim(new BN(claimAmount * LAMPORTS_PER_SOL), {
      accounts: {
        claimer,
        backend: backendKp.publicKey,
        claimerAta,
        game,
        gameTreasuryAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    })
  );
  if (mint.toString() === NATIVE_MINT.toString()) {
    transaction.add(createCloseAccountInstruction(claimerAta, claimer, claimer));
  }

  // transaction.add(
  //   new TransactionInstruction({
  //     keys: [
  //       {
  //         pubkey: nonceAccount.publicKey,
  //         isSigner: false,
  //         isWritable: true,
  //       },
  //       {
  //         pubkey: claimer,
  //         isSigner: false,
  //         isWritable: true,
  //       },
  //     ],
  //     programId: SystemProgram.programId,
  //   })
  // )
  
  transaction.feePayer = claimer;
  // transaction.recentBlockhash = (await program.provider.connection.getLatestBlockhash("confirmed")).blockhash;
  let accountInfo = await connection.getAccountInfo(nonceAccount);
  if (!accountInfo) return res.status(500).json('Cannot find nonce account');
  let nonceAccountData = NonceAccount.fromAccountData(accountInfo.data);

  transaction.recentBlockhash = nonceAccountData.nonce;

  // console.log("Transaction made by BE:", transaction);
  // transaction.partialSign(backendKp, nonceAccount);
  transaction.partialSign(backendKp, nonceAccountAuth);
  // console.log("Partial Signed Transaction by BE Keypair:", transaction);
  let serializedBuffer = transaction.serialize({ requireAllSignatures: false }).toString("base64");

  return res.json(serializedBuffer);
};

export const sendCalimTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint } = req.params;

  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize(), { skipPreflight: false });
  const txConfirmation = await confirmTransactionSafe(program.provider.connection, txSignature);
  console.log(txSignature);
  if (txConfirmation.value.err) return res.status(500).json('Transaction Failed');
  // Get claim amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Claim amount: ", amount.toString());
  // Decrease wallet's tokeMint amount in DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  const balance = player.balance - amount.toNumber() / LAMPORTS_PER_SOL;
  await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'updatePlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint,
    balance
  }); 

  return res.json({ txSignature, claimAmount: amount.toString() })
}

export const sendDepositTransaction = async (req: Request, res: Response) => {
  const payload = getPayload(req);
  if (!payload) return;

  const { wallet } = payload;
  const { tokenMint } = req.params;

  const { serializedBuffer } = req.body;
  const recoveredTx = Transaction.from(Buffer.from(serializedBuffer, "base64"));
  const txSignature = await program.provider.connection.sendRawTransaction(recoveredTx.serialize(), { skipPreflight: false });
  const txConfirmation = await confirmTransactionSafe(program.provider.connection, txSignature);
  console.log(txSignature);
  if (txConfirmation.value.err) return res.status(500).json('Transaction Failed');
  
  // Get deposit amount from Transacction
  const programInstructions = recoveredTx.instructions.filter(instruction => instruction.programId.toString() === programId);
  const claimInstruction = programInstructions[0];
  const amountBytes = claimInstruction.data.slice(8).reverse();
  const amount = new BN(amountBytes);
  console.log("Deposit amount: ", amount.toString());
  // Increase wallet's tokeMint amount in DB
  const player = await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'getPlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint
  }); 
  if (!player) {
    return res.status(500).json("There's no balance");
  }
  const balance = player.balance + amount.toNumber() / LAMPORTS_PER_SOL;
  await sendRequest(`https://api.servica.io/extorio/apis/blinko`, {
    endpoint: 'updatePlayer',
    gameName: 'blinko',
    walletAddress: wallet,
    tokenSPLAddress: tokenMint,
    balance
  }); 

  return res.json({ txSignature, depositAmount: amount.toString() })
}