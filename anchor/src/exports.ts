// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { Cluster, PublicKey } from "@solana/web3.js";
import BlessTokenProgramIDL from "../target/idl/bless_token.json";
import type { BlessToken } from "../target/types/bless_token";

// Re-export the generated IDL and type
export { BlessTokenProgramIDL, BlessToken };

// The programId is imported from the program IDL.
export const BLESSTOKEN_PROGRAM_ID = new PublicKey(
  BlessTokenProgramIDL.address,
);

export type BlessTokenProgram = Program<BlessToken>;

export let DEV_BLESSTOKEN_PROGRAM_ID: PublicKey | null = null;

// This is a helper function to get the Bless Token Anchor program.
export function getBlessTokenProgram(
  provider: AnchorProvider,
  address?: PublicKey,
): Program<BlessToken> {
  return new Program<BlessToken>(
    {
      ...BlessTokenProgramIDL,
      address: address ? address.toBase58() : BlessTokenProgramIDL.address,
    } as BlessToken,
    provider,
  );
}

// This is a helper function to get the program ID for the Bless Token program depending on the cluster.
export function getBlessTokenProgramId(cluster: Cluster) {
  switch (cluster) {
    case "devnet":
    case "testnet":
      if (DEV_BLESSTOKEN_PROGRAM_ID != null) {
        return DEV_BLESSTOKEN_PROGRAM_ID;
      }
      // This is the program ID for the Bless Token program on devnet and testnet.
      return new PublicKey("H5vW4kkhaFhR5CBAyL46rREFkWfCD4cVwTw4xXZ6wtAy");
    case "mainnet-beta":
    default:
      return BLESSTOKEN_PROGRAM_ID;
  }
}
