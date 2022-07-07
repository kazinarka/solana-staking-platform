export * from './client'

import http from "http"

import { getStakingPageInfo, getAmountAndSupply } from "./controller"

const myServer = http.createServer((req, res) => {
  if (req.method == "GET" && req.url == "/api/staking-page-info") {
    return getStakingPageInfo(req, res)
  }
  if (req.method == "GET" && req.url == "/api/amount-supply") {
    return getAmountAndSupply(req, res)
  }
})

myServer.listen(3000, () => {
  console.log("Server is running on port 3000")
})