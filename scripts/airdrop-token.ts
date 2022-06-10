import { mintTo }  from "@solana/spl-token";
import { tokenMintKeypair, connection, randomPayer } from "./config";
import { TokenHelper } from "../tests/token_helper";
import { User } from "../tests/user";


const airdropToken = async () => {
    const user = new User()
    await user.getOrCreateTokenBag();

    await mintTo(
        connection,
        await randomPayer(),
        tokenMintKeypair.publicKey,
        user.tokenBag,
        tokenMintKeypair,
        1_000_000_000,
        []
    );

    const balance = await (new TokenHelper(tokenMintKeypair.publicKey)).balance(user.tokenBag);
    console.log(`Token Account '${user.tokenBag.toString()}' balance: ${balance}`);
}

export {
    airdropToken,
}