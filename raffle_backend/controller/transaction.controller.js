const anchor = require("@project-serum/anchor");
const AnchorRaffleTicket = require("../idl/anchor_raffle_ticket.json");
const {
  Connection,
  clusterApiUrl,
  Keypair,
  PublicKey,
  Transaction,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");
const {
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
} = require("@solana/spl-token");
const { bs58 } = require("@project-serum/anchor/dist/cjs/utils/bytes");
const programId = AnchorRaffleTicket.metadata.address;
const connection = new Connection(clusterApiUrl("devnet"));
const backendKp = Keypair.fromSecretKey(
  bs58.decode(process.env.BACKEND_SECRET_KEY)
);
const sktToken = new PublicKey(process.env.SKT_MINT_ADDRESS);
const provider = new anchor.AnchorProvider(
  connection,
  new anchor.Wallet(backendKp)
);
anchor.setProvider(provider);
const program = new anchor.Program(AnchorRaffleTicket, programId, provider);
const vault = new PublicKey(process.env.VAULT_KEY);

const getTransaction = async (req, res) => {
  const { clientKey } = req.params;
  const claimer = new PublicKey(clientKey);
  const [vaultPool] = await PublicKey.findProgramAddress(
    [Buffer.from("skt_pool"), vault.toBuffer()],
    program.programId
  );
  const amount = new anchor.BN(1 * LAMPORTS_PER_SOL);
  const claimerAta = await getAssociatedTokenAddress(sktToken, claimer);
  const vaultPoolAta = await getAssociatedTokenAddress(
    sktToken,
    vaultPool,
    true
  );
  const tx = new Transaction();
  const claimerSktAccount = await program.provider.connection.getAccountInfo(
    claimerAta
  );
  if (claimerSktAccount === null) {
    tx.add(
      createAssociatedTokenAccountInstruction(
        claimer,
        claimerAta,
        claimer,
        sktToken,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    );
  }
  tx.add(
    program.transaction.claimSkt(amount, {
      accounts: {
        claimer,
        backend: backendKp.publicKey,
        claimerSktAccount: claimerAta,
        sktMint: sktToken,
        vault,
        vaultPool,
        vaultPoolSktAccount: vaultPoolAta,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
    })
  );
  const { blockhash } = await program.provider.connection.getLatestBlockhash("finalized");
  tx.recentBlockhash = blockhash;
  tx.feePayer = claimer;
  tx.partialSign(backendKp);
  const serializedTx = tx.serialize({
    requireAllSignatures: false
  });
  const txBase64 = serializedTx.toString("base64");

  res.json(txBase64);
  console.log("result:", txBase64);
};

module.exports = {
  getTransaction,
};
