import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConsensusGauge } from "../target/types/consensus_gauge";
import { expect } from "chai";

describe("consensus_gauge", () => {
  // --- Setup ---
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ConsensusGauge as Program<ConsensusGauge>;
  
  const gaugeState = anchor.web3.Keypair.generate();
  
  const user = provider.wallet as anchor.Wallet;

  // --- Test Cases ---

  it("Is initialized!", async () => {
    // Test Instruction 1: create_gauge
    const tx = await program.methods
      .createGauge()
      .accounts({
        gaugeState: gaugeState.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([gaugeState])
      .rpc();

    console.log("Your transaction signature", tx);

    const account = await program.account.gaugeState.fetch(
      gaugeState.publicKey
    );

    expect(account.score.toNumber()).to.equal(0);
    expect(account.authority.equals(user.publicKey)).to.be.true;
  });

  it("Signals agree", async () => {
    // Test Instruction 2: signal_agree
    const tx = await program.methods
      .signalAgree()
      .accounts({
        gaugeState: gaugeState.publicKey,
        user: user.publicKey,
      })
      .rpc();

    const account = await program.account.gaugeState.fetch(
      gaugeState.publicKey
    );

    expect(account.score.toNumber()).to.equal(1);
  });

  it("Signals disagree", async () => {
    // Test Instruction 3: signal_disagree
    const tx = await program.methods
      .signalDisagree()
      .accounts({
        gaugeState: gaugeState.publicKey,
        user: user.publicKey,
      })
      .rpc();

    const account = await program.account.gaugeState.fetch(
      gaugeState.publicKey
    );

    expect(account.score.toNumber()).to.equal(0);
  });
});