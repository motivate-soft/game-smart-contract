export type AnchorRaffleTicket = {
  "version": "0.1.0",
  "name": "anchor_raffle_ticket",
  "instructions": [
    {
      "name": "memo",
      "accounts": [
        {
          "name": "memo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "transferSplToken",
      "accounts": [
        {
          "name": "sender",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "senderTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipientTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeVault",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vaultPool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "vaultBump",
          "type": "u8"
        },
        {
          "name": "tokenType",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawVault",
      "accounts": [
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "claimSkt",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "backend",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "convertSktSol",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "exchangeOption",
          "type": "u8"
        },
        {
          "name": "isHolder",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeGlobal",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "authorizeAdmin",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "unauthorizeAdmin",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "senderAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "storeBuyers",
          "type": "bool"
        },
        {
          "name": "transferToken",
          "type": "bool"
        },
        {
          "name": "nftMintAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeWithPda",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "senderAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolBump",
          "type": "u8"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "storeBuyers",
          "type": "bool"
        },
        {
          "name": "nftMintAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "buyTicketSol",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "recipient",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "buyTicketSpl",
      "accounts": [
        {
          "name": "sender",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "senderTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipientTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawFromPda",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dstAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "raffleFinalize",
      "accounts": [
        {
          "name": "raffleBank",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleNftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "winnerNftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleSplAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ownerSplAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "raffleRoyalties",
          "type": "u8"
        }
      ],
      "returns": null
    }
  ],
  "accounts": [
    {
      "name": "vault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenType",
            "type": "publicKey"
          },
          {
            "name": "vaultBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "global",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "authorizedAdmins",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "raffle",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolBump",
            "type": "u8"
          },
          {
            "name": "totalTickets",
            "type": "u32"
          },
          {
            "name": "soldTickets",
            "type": "u32"
          },
          {
            "name": "pricePerTicket",
            "type": "u64"
          },
          {
            "name": "tokenSplAddress",
            "type": "publicKey"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "nftMintAddress",
            "type": "publicKey"
          },
          {
            "name": "storeBuyers",
            "type": "bool"
          },
          {
            "name": "buyers",
            "type": {
              "vec": {
                "defined": "Buyer"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Buyer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "tickets",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "BuyEvent",
      "fields": [
        {
          "name": "buyer",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u32",
          "index": false
        },
        {
          "name": "soldTickets",
          "type": "u32",
          "index": false
        },
        {
          "name": "totalTickets",
          "type": "u32",
          "index": false
        },
        {
          "name": "remainingTickets",
          "type": "u32",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NoTicketsLeft",
      "msg": "No more tickets left for purchase."
    },
    {
      "code": 6001,
      "name": "RafflePriceMismatched",
      "msg": "Raffle price mismatched."
    },
    {
      "code": 6002,
      "name": "RaffleTokenSPLAddressMismatched",
      "msg": "Token Address mismatched."
    },
    {
      "code": 6003,
      "name": "NotEnoughTokens",
      "msg": "Not Enough Tokens."
    },
    {
      "code": 6004,
      "name": "ErrorCustom",
      "msg": "Custom Error."
    },
    {
      "code": 6005,
      "name": "AlreadyAuthorizedAdmin",
      "msg": "Already authorized admin"
    },
    {
      "code": 6006,
      "name": "NotAuthorizedAdmin",
      "msg": "Not authorized admin"
    },
    {
      "code": 6007,
      "name": "ExceedMaxWithdrawAmount",
      "msg": "Cannot withdraw more than 10,000"
    }
  ]
};

export const IDL: AnchorRaffleTicket = {
  "version": "0.1.0",
  "name": "anchor_raffle_ticket",
  "instructions": [
    {
      "name": "memo",
      "accounts": [
        {
          "name": "memo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "transferSplToken",
      "accounts": [
        {
          "name": "sender",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "senderTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipientTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeVault",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vaultPool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "vaultBump",
          "type": "u8"
        },
        {
          "name": "tokenType",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawVault",
      "accounts": [
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "claimSkt",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "backend",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "convertSktSol",
      "accounts": [
        {
          "name": "claimer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "claimerSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sktMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultPoolSktAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "exchangeOption",
          "type": "u8"
        },
        {
          "name": "isHolder",
          "type": "bool"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeGlobal",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "authorizeAdmin",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "unauthorizeAdmin",
      "accounts": [
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [],
      "returns": null
    },
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "senderAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "storeBuyers",
          "type": "bool"
        },
        {
          "name": "transferToken",
          "type": "bool"
        },
        {
          "name": "nftMintAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "initializeWithPda",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "senderAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "poolBump",
          "type": "u8"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "storeBuyers",
          "type": "bool"
        },
        {
          "name": "nftMintAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "buyTicketSol",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "recipient",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "buyTicketSpl",
      "accounts": [
        {
          "name": "sender",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "senderTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "recipientTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u32"
        },
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "tokenSplAddress",
          "type": "publicKey"
        }
      ],
      "returns": null
    },
    {
      "name": "withdrawFromPda",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "global",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "rafflePoolAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "dstAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ],
      "returns": null
    },
    {
      "name": "raffleFinalize",
      "accounts": [
        {
          "name": "raffleBank",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raffle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleNftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "winnerNftAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raffleSplAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ownerSplAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "raffleRoyalties",
          "type": "u8"
        }
      ],
      "returns": null
    }
  ],
  "accounts": [
    {
      "name": "vault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenType",
            "type": "publicKey"
          },
          {
            "name": "vaultBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "global",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "authorizedAdmins",
            "type": {
              "vec": "publicKey"
            }
          }
        ]
      }
    },
    {
      "name": "raffle",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolBump",
            "type": "u8"
          },
          {
            "name": "totalTickets",
            "type": "u32"
          },
          {
            "name": "soldTickets",
            "type": "u32"
          },
          {
            "name": "pricePerTicket",
            "type": "u64"
          },
          {
            "name": "tokenSplAddress",
            "type": "publicKey"
          },
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "nftMintAddress",
            "type": "publicKey"
          },
          {
            "name": "storeBuyers",
            "type": "bool"
          },
          {
            "name": "buyers",
            "type": {
              "vec": {
                "defined": "Buyer"
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Buyer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "publicKey"
          },
          {
            "name": "tickets",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "BuyEvent",
      "fields": [
        {
          "name": "buyer",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u32",
          "index": false
        },
        {
          "name": "soldTickets",
          "type": "u32",
          "index": false
        },
        {
          "name": "totalTickets",
          "type": "u32",
          "index": false
        },
        {
          "name": "remainingTickets",
          "type": "u32",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NoTicketsLeft",
      "msg": "No more tickets left for purchase."
    },
    {
      "code": 6001,
      "name": "RafflePriceMismatched",
      "msg": "Raffle price mismatched."
    },
    {
      "code": 6002,
      "name": "RaffleTokenSPLAddressMismatched",
      "msg": "Token Address mismatched."
    },
    {
      "code": 6003,
      "name": "NotEnoughTokens",
      "msg": "Not Enough Tokens."
    },
    {
      "code": 6004,
      "name": "ErrorCustom",
      "msg": "Custom Error."
    },
    {
      "code": 6005,
      "name": "AlreadyAuthorizedAdmin",
      "msg": "Already authorized admin"
    },
    {
      "code": 6006,
      "name": "NotAuthorizedAdmin",
      "msg": "Not authorized admin"
    },
    {
      "code": 6007,
      "name": "ExceedMaxWithdrawAmount",
      "msg": "Cannot withdraw more than 10,000"
    }
  ]
};
