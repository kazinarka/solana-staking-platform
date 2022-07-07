import { ServerResponse, IncomingMessage } from "http"
import { StakingPageInfo } from "./IRequestData"
import { Client } from "./client"

export { getStakingPageInfo, getAmountAndSupply }

const getStakingPageInfo = (req: IncomingMessage, res: ServerResponse) => {
  let data = ""

  req.on("data", (chunk) => {
    data += chunk.toString()
  })

  req.on("end", async () => {
    let args: StakingPageInfo = JSON.parse(data)

    let client = new Client()

    let result = await client.getStakingPageInfo(args.owner)

    res.writeHead(200, { "Content-Type": "application/json" })
    res.end(
      JSON.stringify({
        success: true,
        message: result,
      })
    )
  })
}

const getAmountAndSupply = (req: IncomingMessage, res: ServerResponse) => {
  req.on("end", async () => {
    let client = new Client()

    let result = await client.getStakedNftsAmountAndSupply()

    res.writeHead(200, { "Content-Type": "application/json" })
    res.end(
      JSON.stringify({
        success: true,
        message: result,
      })
    )
  })
}
