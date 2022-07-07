import { Connection } from "@solana/web3.js"

export class Chain {
  connection: Connection

  constructor(connection: Connection) {
    this.connection = connection
  }

  async timestamp(): Promise<number> {
    let slot = await this.connection.getSlot()
    let block = await this.connection.getBlock(slot)
    if (block == null) {
      throw "Failed to get timestamp"
    }
    return block.blockTime!
  }
}