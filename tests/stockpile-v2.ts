import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StockpileV2 } from "../target/types/stockpile_v2";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";

describe("stockpile-v2", () => {

    anchor.setProvider(anchor.AnchorProvider.env());

    const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"));
  
    const program = anchor.workspace.StockpileV2 as Program<StockpileV2>;
  
    it("createProject", async () => {
      // Generate keypairs for payer, and admins
      const payer = anchor.web3.Keypair.generate();
      let adminKp1 = anchor.web3.Keypair.generate();
      let adminKp2 = anchor.web3.Keypair.generate();
  
      // Fund payer account
      await connection.requestAirdrop(payer.publicKey, 2);
  
      // Generate a beneficiary keypair and random projectId
      let beneficiary = anchor.web3.Keypair.generate().publicKey;
      let projectId = Math.floor(10000 + Math.random() * 90000)
  
      // Find PDA address
      const [fundraiserPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
          [utf8.encode("fundraiser"), new anchor.BN(projectId).toArrayLike(Buffer, "le", 8),],
          program.programId
      );
  
      // Define dummy values
      let name = "Nautilus";
      let admins = [adminKp1.publicKey, adminKp2.publicKey];
      let goal = 100;
    
      // Let it fly
      const tx = await program.methods.createProject(new anchor.BN(projectId), name, admins, beneficiary, new anchor.BN(goal))
      .accounts({
        payer: payer.publicKey,
        project: fundraiserPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
  
      // If it passes, we get a friendly message
      console.log(`🚀 Project "${name}" Created! Transaction Hash:`, tx);
    });
  
}