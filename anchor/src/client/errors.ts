import { TransactionError } from '@solana/web3.js'

export class BlsError extends Error {
  message: string
  customCode?: number
  rawError?: TransactionError
  programLogs?: string[]
  constructor(message: string, rawError?: TransactionError, programLogs?: string[]) {
    super(message)
    this.message = message
    this.rawError = rawError
    this.programLogs = programLogs
  }
}
