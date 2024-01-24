import express from "./config/express";
import { getBalances, play } from "./controller/game.controller";
import { getSettings, setSettings, getAdminSettings, setNonceAccount, getNonceAccount } from "./controller/setting.controller";
import { getClaimTransaction, sendCalimTransaction, sendDepositTransaction } from "./controller/transaction.controller";
import { authorizedAdmin, authorizedPlayer } from "./middleware/auth.middleware";
// const { Keypair } = require('@solana/web3.js');
// const bs58 = require('bs58');

express.post('/transaction/deposit/:tokenMint', authorizedPlayer, sendDepositTransaction);
express.get('/transaction/claim/:tokenMint/:amount', authorizedPlayer, getClaimTransaction);
express.post('/transaction/claim/:tokenMint', authorizedPlayer, sendCalimTransaction);
express.get('/settings/admin', authorizedAdmin, getAdminSettings);
express.post('/settings/admin', authorizedAdmin, setSettings);
express.get('/settings', authorizedPlayer, getSettings);
express.post('/game/play', authorizedPlayer, play);
express.get('/game/balances', authorizedPlayer, getBalances);
express.post('/config/', authorizedAdmin, setNonceAccount);
express.get('/config/nonceAccount', authorizedPlayer, getNonceAccount);
// const kp = Keypair.generate();
// console.log(kp.publicKey.toString());
// console.log(bs58.encode(kp.secretKey));

if (process.env.NODE_ENV === "development") {
  express.listen(express.get("port"), () => {
    console.log(
      `Server running at http://${express.get("host")}:${express.get("port")}`
    );
  });
} else {
  //
}
