export type Solzen = {
  "version": "0.1.0",
  "name": "solzen",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "zendao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "validation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "founder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "token",
          "type": "publicKey"
        },
        {
          "name": "minBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeDao",
      "accounts": [
        {
          "name": "zendao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "validateTelegramUser",
      "accounts": [
        {
          "name": "telegramUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zendao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u64"
        }
      ]
    },
    {
      "name": "validateHuman",
      "accounts": [
        {
          "name": "validation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "parentValidation",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zendao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "parent",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "child",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "zendao",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "token",
            "type": "publicKey"
          },
          {
            "name": "minBalance",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "validation",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "parent",
            "type": "publicKey"
          },
          {
            "name": "child",
            "type": "publicKey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "telegramUser",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "dao",
            "type": "publicKey"
          },
          {
            "name": "id",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InsuficientAmount",
      "msg": "Insuficient amount"
    },
    {
      "code": 6001,
      "name": "WrongToken",
      "msg": "Wrong token"
    },
    {
      "code": 6002,
      "name": "WrongOwner",
      "msg": "Wrong owner"
    },
    {
      "code": 6003,
      "name": "WrongParentValidation",
      "msg": "Wrong parent validation"
    }
  ]
};

export const IDL: Solzen = {
  "version": "0.1.0",
  "name": "solzen",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "zendao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "validation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "founder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "token",
          "type": "publicKey"
        },
        {
          "name": "minBalance",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeDao",
      "accounts": [
        {
          "name": "zendao",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "validateTelegramUser",
      "accounts": [
        {
          "name": "telegramUser",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "zendao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u64"
        }
      ]
    },
    {
      "name": "validateHuman",
      "accounts": [
        {
          "name": "validation",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "parentValidation",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "zendao",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "parent",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "child",
          "type": "publicKey"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "zendao",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "token",
            "type": "publicKey"
          },
          {
            "name": "minBalance",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "validation",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "parent",
            "type": "publicKey"
          },
          {
            "name": "child",
            "type": "publicKey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "telegramUser",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "dao",
            "type": "publicKey"
          },
          {
            "name": "id",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InsuficientAmount",
      "msg": "Insuficient amount"
    },
    {
      "code": 6001,
      "name": "WrongToken",
      "msg": "Wrong token"
    },
    {
      "code": 6002,
      "name": "WrongOwner",
      "msg": "Wrong owner"
    },
    {
      "code": 6003,
      "name": "WrongParentValidation",
      "msg": "Wrong parent validation"
    }
  ]
};
