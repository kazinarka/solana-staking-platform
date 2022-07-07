// export * from './client'

import http from "http";

import {getTasks} from "./controller";

const myServer = http.createServer((req, res) => {
    if (req.method == "GET" && req.url == "/api/tasks") {
        return getTasks(req, res)
    }
});

myServer.listen(3000, () => {
    console.log('Server is running on port 3000')
});

myServer.close()