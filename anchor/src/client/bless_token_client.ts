import {
  Keypair,
  PublicKey,
  TransactionInstruction,
  TransactionSignature,
} from "@solana/web3.js";
import { TxOptions } from "./base";
import { BlsBaseClient } from "./base_client";

export class BlessTokenAccounts {
  public walletInvestor: PublicKey;
  public walletTeamAdvisor: PublicKey;
  public walletFoundation: PublicKey;
  public walletEcosystemLiquidityprovisionTgtmarketing: PublicKey;
  public walletCommunityRewards: PublicKey;
}

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

export class BlsTokenClient {
  baseClient: BlsBaseClient;

  constructor(base: BlsBaseClient) {
    this.baseClient = base;
  }

  public getMetadataSync(mint: PublicKey) {
    const [metadataPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    );
    return metadataPDA;
  }

  public async fetchBlessState(mint: PublicKey) {
    let state = PublicKey.findProgramAddressSync(
      [Buffer.from("bless_contract_state"), mint.toBuffer()],
      this.baseClient.programId,
    );
    return await this.baseClient.program.account.blessTokenState.fetch(
      state[0],
    );
  }

  public async initialBlessTokenState(
    accounts: BlessTokenAccounts,
    blessMint: PublicKey,
    mintAuthority: Keypair,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .initializeBlessToken()
      .accountsPartial({
        payer,
        blessMint,
        walletInvestor: accounts.walletInvestor,
        walletTeamAdvisor: accounts.walletTeamAdvisor,
        walletFoundation: accounts.walletFoundation,
        walletEcosystemLiquidityprovisionTgtmarketing:
          accounts.walletEcosystemLiquidityprovisionTgtmarketing,
        walletCommunityRewards: accounts.walletCommunityRewards,
        currentAuthority: mintAuthority.publicKey,
      })
      .preInstructions(preIxs)
      .transaction();
    const versioned = await this.baseClient.getVersionedTransaction({
      tx,
      ...txOptions,
    });
    return this.baseClient.sendAndConfirm(versioned, txOptions.signerKeypair);
  }

  public async initialBlessTokenMetaState(
    blessMint: PublicKey,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .initializeBlessTokenMetaState()
      .accountsPartial({
        payer,
        blessMint,
      })
      .preInstructions(preIxs)
      .transaction();
    const versioned = await this.baseClient.getVersionedTransaction({
      tx,
      ...txOptions,
    });
    return this.baseClient.sendAndConfirm(versioned, txOptions.signerKeypair);
  }

  public async createMetadata(
    blessMint: PublicKey,
    metaPda: PublicKey,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .createMetadata()
      .accountsPartial({
        payer,
        blessMint,
        metadataProgram: TOKEN_METADATA_PROGRAM_ID,
        admin: payer,
        metaPda,
      })
      .preInstructions(preIxs)
      .transaction();
    const versioned = await this.baseClient.getVersionedTransaction({
      tx,
      ...txOptions,
    });
    return this.baseClient.sendAndConfirm(versioned, txOptions.signerKeypair);
  }
}
