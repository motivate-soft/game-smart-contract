import { PublicKey, Keypair, Connection, RpcResponseAndContext, SignatureResult } from '@solana/web3.js';
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from '@solana/spl-token';
import { AnchorProvider } from '@project-serum/anchor';
import nacl from 'tweetnacl';
import jwt from 'jsonwebtoken';
import axios from 'axios';
const config = require('../config.json');

export const plinko_pda_seed = "plinko_game_pda";
export const game_name = "test1";
export const game_owner = new PublicKey("3qWq2ehELrVJrTg2JKKERm67cN6vYjm1EyhCEzfQ6jMd");
export const backendKp = Keypair.fromSecretKey(
  bs58.decode(process.env.BACKEND_SECRET_KEY || '')
);
export const nonceAccountAuth = Keypair.fromSecretKey(
  bs58.decode("4QexB3reKGvf8tpENZLktDVhdujC3XoSvT88mxfrXre94XKcndhoVSjMgD8zSQ6z1FPZbPhK6uvRdqUhzbWdM8dV")
)

export const getGameAddress = async (programId: PublicKey, game_name: string, game_owner: PublicKey) => (
  await PublicKey.findProgramAddress(
    [
      Buffer.from(game_name),
      Buffer.from(plinko_pda_seed),
      game_owner.toBuffer(),
    ],
    programId
  )
);

export async function getAta(mint: PublicKey, owner: PublicKey, allowOffCurve: boolean = false) {
  return await getAssociatedTokenAddress(
    mint,
    owner,
    allowOffCurve
  );
}

export async function getCreateAtaInstruction(provider: AnchorProvider, ata: PublicKey, mint: PublicKey, owner: PublicKey) {
  let account = await provider.connection.getAccountInfo(ata);
  if (!account) {
    return createAssociatedTokenAccountInstruction(
      owner,
      ata,
      owner,
      mint,
    );
  }
}

export function signMessage(message: string) {
  const signature = nacl.sign.detached(new Uint8Array(Buffer.from(message)), backendKp.secretKey);
  return { message, signature, wallet: backendKp.publicKey.toString() };
}


export function getJWT() {
  const token = jwt.sign(signMessage("I am backend"), process.env.TOKEN_SECRET_KEY || '', { expiresIn: 60 });
  return token;
}

// console.log(getJWT());

export async function sendRequest(uri: string, body?: any) {
  axios.defaults.headers.common.Authorization = `${getJWT()}`;
  try {
    const { data } = await axios.post(uri, body);
    console.log(data);
    return data;
  } catch (error) {
    console.log(error);
  }
}

// sendRequest('https://api.servica.io/extorio/apis/blinko?endpoint=getPlayers');
export const sleep = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};


/* Will retry to confirm tx for 10 times, 1 sec sleep between retires */
export async function confirmTransactionSafe(connection: Connection, txSignature: string, retries: number = 10, sleepMS: number = 1000): Promise<RpcResponseAndContext<SignatureResult>> {
  const txConfirmation: any = null;
  while (retries > 0) {
    try {
      console.log(`Confirming ${txSignature}... retries: ${retries}`);
      const txConfirmation = await connection.confirmTransaction(txSignature, "confirmed");

      console.log(`Confirmed https://solscan.io/tx/${txSignature}`);
      return txConfirmation;
    }
    catch (e) {
      console.info("Failed confirmation:", e);

      retries--;
      await sleep(sleepMS);
    }
  }

  return txConfirmation;
}