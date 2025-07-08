import * as anchor from "@coral-xyz/anchor";
import {
  AddressLookupTableAccount,
  BlockhashWithExpiryBlockHeight,
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  RpcResponseAndContext,
  SignatureResult,
  SimulatedTransactionResponse,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  TransactionSignature,
  VersionedTransaction,
} from "@solana/web3.js";
import { ClusterNetwork } from "./config";
import { BlockhashWithCache } from "./blockhashcache";
import { BlsError } from "./errors";

export type TxOptions = {
  signer?: PublicKey;
  signerKeypair?: Keypair[] | Keypair;
  computeUnitLimit?: number;
  getPriorityFeeMicroLamports?: (tx: VersionedTransaction) => Promise<number>;
  maxFeeLamports?: number;
  useMaxFee?: boolean;
  preInstructions?: TransactionInstruction[];
};

const DEFAULT_PRIORITY_FEE = 10_000;

export class Base {
  cluster: ClusterNetwork;
  provider: anchor.Provider;
  connection: Connection;
  wallet: anchor.Wallet;

  blockhashWithCache: BlockhashWithCache;

  constructor(cluster: ClusterNetwork, provider: anchor.Provider) {
    this.cluster = cluster;
    this.provider = provider;
    this.connection = provider.connection;
    this.wallet = (this.provider as anchor.AnchorProvider)
      .wallet as anchor.Wallet;
    this.blockhashWithCache = new BlockhashWithCache(provider, !!isBrowser);
  }

  public base64ToArray(base64: string): number[] {
    const binaryString = atob(base64);
    const bytes: number[] = new Array<number>(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
  }

  public async getBalance(pubkey: PublicKey) {
    return this.connection.getBalance(pubkey);
  }

  async calcSimulationComputeUnits(
    connection: Connection,
    instructions: Array<TransactionInstruction>,
    payer: PublicKey,
    lookupTables: Array<AddressLookupTableAccount> | [],
  ): Promise<number | undefined> {
    const testInstructions = [
      // Set an biggest high number which never reached in simulation
      ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 }),
      ...instructions,
    ];

    const testTransaction = new VersionedTransaction(
      new TransactionMessage({
        instructions: testInstructions,
        payerKey: payer,
        recentBlockhash: PublicKey.default.toString(),
      }).compileToV0Message(lookupTables),
    );

    const rpcResponse = await connection.simulateTransaction(testTransaction, {
      replaceRecentBlockhash: true,
      sigVerify: false,
    });

    this.getErrorFromRPCResponse(rpcResponse);
    return rpcResponse.value.unitsConsumed || undefined;
  }

  getErrorFromRPCResponse(
    rpcResponse: RpcResponseAndContext<
      SignatureResult | SimulatedTransactionResponse
    >,
  ) {
    const error = rpcResponse.value.err;
    if (error) {
      if (typeof error === "object") {
        const errorKeys = Object.keys(error);
        if (errorKeys.length === 1) {
          if (errorKeys[0] !== "InstructionError") {
            throw new Error(`Unknown RPC error: ${error}`);
          }
          // @ts-ignore
          const instructionError = error["InstructionError"];
          throw new Error(
            `Error in transaction: instruction index ${instructionError[0]}, custom program error ${instructionError[1]["Custom"]}`,
          );
        }
      }
      throw Error(error.toString());
    }
  }

  async getVersionedTransaction({
    tx,
    lookupTables,
    signer,
    computeUnitLimit,
    getPriorityFeeMicroLamports,
    latestBlockhash,
  }: {
    tx: Transaction;
    lookupTables?: Array<AddressLookupTableAccount> | [];
    signer?: PublicKey;
    computeUnitLimit?: number;
    getPriorityFeeMicroLamports?: (tx: VersionedTransaction) => Promise<number>;
    latestBlockhash?: BlockhashWithExpiryBlockHeight;
  }): Promise<VersionedTransaction> {
    lookupTables = lookupTables || [];
    signer = signer || this.getSigner();

    const instructions = tx.instructions;

    if (!this.isPhantom()) {
      // Set compute unit limit or autodetect by simulating the tx
      if (!computeUnitLimit) {
        try {
          computeUnitLimit = await this.calcSimulationComputeUnits(
            this.provider.connection,
            instructions,
            signer,
            lookupTables,
          );
        } catch (e) {
          //TODO
          console.log(e);
        }
      }
      if (computeUnitLimit) {
        // ComputeBudgetProgram.setComputeUnitLimit costs 150 CUs
        // Add 20%/50% more CUs to account for logs (mainnet logs are less verbose)
        computeUnitLimit += 150;
        if (this.isMainnet()) {
          computeUnitLimit *= 1.2;
        } else {
          computeUnitLimit *= 1.5;
        }
        instructions.unshift(
          ComputeBudgetProgram.setComputeUnitLimit({ units: computeUnitLimit }),
        );
      }
    }

    const recentBlockhash = (
      latestBlockhash ? latestBlockhash : await this.blockhashWithCache.get()
    ).blockhash;

    let priorityFee = DEFAULT_PRIORITY_FEE;
    if (getPriorityFeeMicroLamports) {
      try {
        const fee = await getPriorityFeeMicroLamports(
          new VersionedTransaction(
            new TransactionMessage({
              payerKey: signer,
              recentBlockhash,
              instructions,
            }).compileToV0Message(lookupTables),
          ),
        );
        priorityFee = Math.ceil(fee);
      } catch (e) {
        console.log(e);
      }
    }

    // Add the unit price instruction and return the final versioned transaction
    instructions.unshift(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: priorityFee,
      }),
    );
    return new VersionedTransaction(
      new TransactionMessage({
        payerKey: signer,
        recentBlockhash,
        instructions,
      }).compileToV0Message(lookupTables),
    );
  }

  isMainnet(): boolean {
    return this.cluster === ClusterNetwork.Mainnet;
  }

  getSigner(): PublicKey {
    const publicKey = this.wallet.publicKey;
    if (!publicKey) {
      throw new Error("Signer public key cannot be find.");
    }
    return publicKey;
  }

  isPhantom(): boolean {
    if (!isBrowser) return false;
    return (
      // @ts-ignore
      window?.phantom?.solana?.isPhantom && window?.phantom?.solana?.isConnected
    );
  }

  parseProgramLogs(logs?: null | string[]): [string, number | undefined] {
    let errorMsgLog;
    let code;
    const regex = new RegExp("[Cc]ustom program error: (0x[0-9A-Fa-f]+)");
    let idx = 0;
    let l;
    const plogs = logs || [];
    for (const line of plogs) {
      if (line.includes("Error Message:")) {
        errorMsgLog = line;
      }

      const matches = line.match(regex);
      if (matches?.length == 2 && l == undefined) {
        const c = matches.at(1);
        code = parseInt(c!);
        l = idx;
      }
      idx++;
    }

    if (errorMsgLog) {
      return [errorMsgLog.split("Error Message:")[1].trim(), code];
    } else if (l) {
      if (code != null && l - 1 < plogs.length) {
        return [plogs[l - 1], code];
      }
    }

    return ["Unknown error", undefined];
  }

  getWallet(): anchor.Wallet {
    return this.wallet;
  }

  setWallet(wallet: anchor.Wallet) {
    this.wallet = wallet;
  }

  async sendAndConfirmTransaction(tx: Transaction, txOpts?: TxOptions) {
    const versionTx = await this.getVersionedTransaction({ tx, ...txOpts });
    return await this.sendAndConfirm(versionTx, txOpts?.signerKeypair);
  }

  async sendAndConfirm(
    tx: VersionedTransaction | Transaction,
    signer?: Keypair | Keypair[],
  ): Promise<TransactionSignature> {
    let txConnection: Connection;
    if (this.cluster === ClusterNetwork.Mainnet) {
      const endpoint =
        process.env?.NEXT_PUBLIC_TX_RPC ||
        process.env.TX_RPC ||
        this.provider.connection.rpcEndpoint;
      txConnection = new Connection(endpoint, { commitment: "confirmed" });
    } else {
      txConnection = this.provider.connection;
    }

    if (tx instanceof Transaction) {
      if (tx.recentBlockhash == null) {
        const recentBlockhash = (await this.blockhashWithCache.get()).blockhash;
        tx.recentBlockhash = recentBlockhash;
      }
      if (tx.feePayer == null) {
        const feePayer = signer || this.getWallet().payer;
        let pubkey: PublicKey;
        if (feePayer instanceof Keypair) {
          pubkey = feePayer.publicKey;
        } else {
          pubkey = feePayer[0].publicKey;
        }
        tx.feePayer = pubkey;
      }
    }

    const connection = this.provider.connection;
    let serializedTx: Uint8Array;

    if (signer) {
      if (tx instanceof Transaction) {
        if (signer instanceof Keypair) {
          tx.sign(signer);
        } else {
          tx.sign(signer[0]);
        }
      }
      if (tx instanceof VersionedTransaction) {
        if (signer instanceof Keypair) {
          tx.sign([signer]);
        } else {
          tx.sign(signer);
        }
      }
      serializedTx = tx.serialize();
    } else {
      // https://github.com/coral-xyz/anchor/blob/v0.30.0/ts/packages/anchor/src/provider.ts#L159
      const wallet = this.getWallet();
      const signedTx = await wallet.signTransaction(tx);
      serializedTx = signedTx.serialize();
    }
    // const txSig = await txConnection.sendRawTransaction(serializedTx, { skipPreflight: true })
    const txSig = await txConnection.sendRawTransaction(serializedTx);

    // await confirmation
    const latestBlockhash = await this.blockhashWithCache.get();
    const res = await connection.confirmTransaction(
      {
        ...latestBlockhash,
        signature: txSig,
      },
      "confirmed",
    );

    // if the tx fails, throw an error including logs
    if (res.value.err) {
      const errTx = await connection.getTransaction(txSig, {
        maxSupportedTransactionVersion: 0,
      });
      const [err, code] = this.parseProgramLogs(errTx?.meta?.logMessages);
      const e = new BlsError(
        err,
        errTx?.meta?.err || undefined,
        errTx?.meta?.logMessages || [],
      );
      e.customCode = code;
      throw e;
    }
    return txSig;
  }
}

export const isBrowser =
  // eslint-disable-next-line no-prototype-builtins
  process.env.ANCHOR_BROWSER ||
  (typeof window !== "undefined" && !window.process?.hasOwnProperty("type"));
