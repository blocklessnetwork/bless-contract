# How the bless token contract works.

## BLESS Token — Mint Authority & Wallet Setup

This guide describes how to prepare the mint authority keypair, create the BLESS token, initialize the five major wallets, and invoke the BLESS contract logic.

Prerequisites

 - Solana CLI installed

 - SPL Token CLI installed

 - Node.js installed

- Access to the BLESS contract CLI: https://github.com/blocklessnetwork/bless-contract-cli


### 1. Prepare the mint authority keypair

The mint authority keypair controls the ability to mint new BLESS tokens. This keypair must be created and securely stored before the token is deployed.

```bash
# Prepare the mint Authority keypair
solana-keygen new -o authority.json
```

Output files:

authority.json — Private key for the mint authority (keep offline & secure)

Public key — will be used when creating the token

Best Practices:

Generate on an offline or air-gapped machine if possible

Store authority.json in an encrypted backup

### 2. Create the Mint Token

```bash
spl-token create-token --mint-authority authority.json
```

Example output:

```bash
Ep9fFc5oKgbtkd1kVSKTFy7mariTdavtU6jJrT2SBcff
```

This is your token mint address — keep it safe.

### 3. Create the 5 Major Wallets.

```bash
solana-keygen new -o payer.json
solana-keygen new -o w1.json
solana-keygen new -o w2.json
solana-keygen new -o w3.json
solana-keygen new -o w4.json
solana-keygen new -o w5.json
```

For the mainnet use the Wallet(MetaMask & Phantom & Others) to create the  5 Major Wallets

### 4.Fund the Payer(Devnet or Localnet)

```bash
solana airdrop 1 $(solana-keygen pubkey payer.json)
```

Guarantee the payer have the SOL in Mainnet

### 5. Create Associated Token Accounts

```bash
spl-token create-account --owner `solana-keygen pubkey w1.json` APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx --fee-payer=payer.json
spl-token create-account --owner `solana-keygen pubkey w2.json` APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx --fee-payer=payer.json
spl-token create-account --owner `solana-keygen pubkey w3.json` APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx --fee-payer=payer.json
spl-token create-account --owner `solana-keygen pubkey w4.json` APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx --fee-payer=payer.json
spl-token create-account --owner `solana-keygen pubkey w5.json` APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx --fee-payer=payer.json
```

For the mainnet, the `solana-keygen pubkey w1.json` repalce with the public key from the Wallet(MetaMask & Phantom & Others)

### 6. Invoke the BLESS Contract Logic

```bash
npx @blessnetwork/blesscontract blesstoken  AT1jeAztBA9ncZTmkLMM5gk6KggLmDWrqak1Pz7by1C6,F5pmJkx8eN7QXqjkkhuANQtPjw2FBV9oMRehwCXCt4Nj,573feBHUThYjuHQGJD3BfaFvhom7N1D9nbYs6D8BsvJL,E1t37tVeAaoFfonMyfJ4Jo3mBPJxwje42vFXQQfFaouo,7tzmaKhbQW6i6FrdQMXYJvByishwQKVfTuHrn7kDP2PQ APmAe4nWwKwUuQ25tggcvW5kS6DRf4zE6ivSnHYVawZx authority.json
```
Arguments:

First parameter → Comma-separated list of the 5 ATA accounts created by step 5

Second parameter → Bless token mint address created by step 2

Third parameter → Mint authority keypair file created by step 1

### Security Notes

- Always backup your authority.json and wallet keypairs securely (preferably offline).

- Do not commit any .json keypair files to source control.

- Consider using a hardware wallet for long-term storage.
