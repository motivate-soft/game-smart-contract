const app = require("./config/express");
const txCtrl = require("./controller/transaction.controller");
const { Keypair } = require('@solana/web3.js');
const bs58 = require('bs58');

app.get('/transaction/:clientKey', txCtrl.getTransaction);

// const kp = Keypair.generate();
// console.log(kp.publicKey.toString());
// console.log(bs58.encode(kp.secretKey));

if (process.env.NODE_ENV === "development") {
  app.listen(app.get("port"), () => {
    console.log(
      `Server running at http://${app.get("host")}:${app.get("port")}`
    );
  });
} else {
  //
}
