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

const SEED_BLESS_CONTRACT_STATE = "bless_contract_state";

const SEED_BLESS_TOKEN_META_STATE = "bless_token_meta_state";

export type Metadata = {
  name: string;
  symbol: string;
  uri: string;
};

export class BlsTokenClient {
  baseClient: BlsBaseClient;

  constructor(base: BlsBaseClient) {
    this.baseClient = base;
  }

  public getMetadataSync(mint: PublicKey): PublicKey {
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
    meta: Metadata,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .createMetadata(meta.name, meta.symbol, meta.uri)
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

  public async updateMetadata(
    blessMint: PublicKey,
    metaPda: PublicKey,
    meta: Metadata,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .updateMetadata(meta.name, meta.symbol, meta.uri)
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

  /**
   * This function should take the new administrator's public key as an argument.
   * It must be callable only by the current admin account.
   * Its logic should update the pending admin account field with the new administrator's address.
   * @param blessMint
   * @param pendingAdmin
   * @param txOptions
   * @returns
   */
  public async setPendingAdminAccount(
    blessMint: PublicKey,
    pendingAdmin: PublicKey,
    admin: PublicKey,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .setPendingAdminAccount()
      .accountsPartial({
        payer,
        admin,
        pendingAdmin,
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

  /**
   *
   * This instruction must be callable only by the key stored in pending_admin_account.
   * Its logic should update the admin_account to the pending_admin_account's address
   * and then clear the pending_admin_account to finalize the transfer.
   * @param blessMint
   * @param txOptions
   * @returns
   */
  public async acceptAdmin(
    blessMint: PublicKey,
    pendingAdmin: PublicKey,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .acceptAdmin()
      .accountsPartial({
        pendingAdmin,
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

  public findBlessTokenMetaStateAddress(mint: PublicKey) {
    const bless_pda = PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_BLESS_CONTRACT_STATE), mint.toBuffer()],
      this.baseClient.programId,
    );

    const pda = PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_BLESS_TOKEN_META_STATE), bless_pda[0].toBuffer()],
      this.baseClient.programId,
    );
    return pda;
  }

  public async getBlessTokenMetaState(mint: PublicKey) {
    const pda = this.findBlessTokenMetaStateAddress(mint);
    return await this.baseClient.program.account.blessTokenMetaState.fetch(
      pda[0],
    );
  }
}
