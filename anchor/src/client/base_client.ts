import { Base } from './base'
import * as anchor from '@coral-xyz/anchor'

import { Connection } from '@solana/web3.js'
import { BlsClientConfig, ClusterNetwork } from './config'
import { BlessTime, BlessTimeProgram, BlessTimeProgramIDL, getBlessTimeProgramId } from '../exports'

export class BlsBaseClient extends Base {
  public program: BlessTimeProgram

  public get programId() {
    return this.program.programId
  }

  public constructor(config?: BlsClientConfig) {
    let provider: anchor.Provider
    if (config?.provider) {
      provider = config.provider
    } else {
      const defaultProvider = anchor.AnchorProvider.env()
      const url = defaultProvider.connection.rpcEndpoint
      const connection = new Connection(url, {
        commitment: 'confirmed',
        confirmTransactionInitialTimeout: 45000, // default timeout is 30s, we extend it to 45s
      })
      provider = new anchor.AnchorProvider(connection, config?.wallet || defaultProvider.wallet, {
        ...defaultProvider.opts,
        commitment: 'confirmed',
        preflightCommitment: 'confirmed',
      })
      anchor.setProvider(provider)
    }

    const defaultCluster = provider.connection.rpcEndpoint.includes('devnet')
      ? ClusterNetwork.Devnet
      : ClusterNetwork.Mainnet
    let cluster = config?.cluster || defaultCluster
    let program: BlessTimeProgram
    if (cluster === ClusterNetwork.Mainnet) {
      program = new anchor.Program(BlessTimeProgramIDL as BlessTime, provider) as BlessTimeProgram
    } else {
      const BlsIDL = JSON.parse(JSON.stringify(BlessTimeProgramIDL))
      BlsIDL.address = getBlessTimeProgramId(ClusterNetwork.Devnet).toBase58()
      program = new anchor.Program(BlsIDL as BlessTime, provider) as BlessTimeProgram
    }
    super(cluster, provider)
    this.program = program
  }
}
