import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PixelStaking } from "../target/types/pixel_staking";

describe("pixel-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PixelStaking as Program<PixelStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
