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
  
    it("createPool", async () => {
      // Generate payer keypair, and random poolId
      const payer = anchor.web3.Keypair.generate();
      const admin1 = anchor.web3.Keypair.generate();
      const admin2 = anchor.web3.Keypair.generate();
      const admin3 = anchor.web3.Keypair.generate();
      let poolId = Math.floor(1 + Math.random() * 9)
  
      // Fund payer account
      await connection.requestAirdrop(payer.publicKey, 2);
  
      // Find PDA address
      const [poolPDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
          [utf8.encode("pool"), new anchor.BN(poolId).toArrayLike(Buffer, "le", 8)],
          program.programId
      );
  
      // Define dummy values
      let name = "Money Laundering Machine";
      let start = new anchor.BN(Math.floor(Date.now() / 1000));
      let end = new anchor.BN(Math.floor(Date.now() / 1000) + 30000);
      let admins = [admin1.publicKey, admin2.publicKey, admin3.publicKey];
    
      // Alea iacta est
      const tx = await program.methods.createPool(
        new anchor.BN(poolId), 
        name, 
        new anchor.BN(start), 
        new anchor.BN(end), 
        admins
      )
      .accounts({
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        pool: poolPDA,
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
  
      // If it passes, we get a friendly message
      console.log(`👾 Funding Round "${name}" Initialized! Transaction Hash:`, tx);
    });
  
    it("createSource", async () => {
      // Generate keypairs for payer, and admins
      const payer = anchor.web3.Keypair.generate();
  
      // Fund payer account
      await connection.requestAirdrop(payer.publicKey, 2);
  
      // Find PDA address
      const [sourcePDA, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
          [utf8.encode("source"), payer.publicKey.toBuffer()],
          program.programId
      );
  
      // Define dummy value
      let name = "Buffalo Joe";
    
      // Run it up
      const tx = await program.methods.createSource(name)
      .accounts({
        payer: payer.publicKey,
        source: sourcePDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
  
      // If it passes, we get a friendly message
      console.log(`✨ Source "${name}" Created! Transaction Hash:`, tx);
    });
  
    it("joinPool", async () => {
      // Generate keypairs for payer, and admins
      const payer = anchor.web3.Keypair.generate();
      let adminKp1 = anchor.web3.Keypair.generate();
      let adminKp2 = anchor.web3.Keypair.generate();
  
      // Fund payer account
      await connection.requestAirdrop(payer.publicKey, 2);
  
      let beneficiary = anchor.web3.Keypair.generate().publicKey;
      let projectId = Math.floor(10000 + Math.random() * 90000)
  
      // Find project PDA address
      const [fundraiserPDA, fundraiserBump] = await anchor.web3.PublicKey.findProgramAddressSync(
          [utf8.encode("fundraiser"), new anchor.BN(projectId).toArrayLike(Buffer, "le", 8),],
          program.programId
      );
  
      // Define dummy values
      let projectName = "Motherfuckin' Demons from the planet Jupiter";
      let admins = [adminKp1.publicKey, adminKp2.publicKey];
      let goal = 100;
    
      // Create project
      const projectTx = await program.methods.createProject(new anchor.BN(projectId), projectName, admins, beneficiary, new anchor.BN(goal))
      .accounts({
        payer: payer.publicKey,
        project: fundraiserPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
  
      let poolId = Math.floor(1 + Math.random() * 9);
  
      // Find pool PDA address
      const [poolPDA, poolBump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [utf8.encode("pool"), new anchor.BN(poolId).toArrayLike(Buffer, "le", 8)],
        program.programId
      );
  
      // Define more dummy values
      let poolName = "Dill Clyntin";
      let start = new anchor.BN(Math.floor(Date.now() / 1000));
      let end = new anchor.BN(Math.floor(Date.now() / 1000) + 30000);
  
      // Create a pool
      const poolTx = await program.methods.createPool(
        new anchor.BN(poolId), 
        poolName, 
        new anchor.BN(start), 
        new anchor.BN(end), 
        admins
      )
      .accounts({
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        pool: poolPDA,
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
    
      // Fire when ready captain
      const tx = await program.methods.joinPool(new anchor.BN(projectId), new anchor.BN(poolId))
      .accounts({
        payer: payer.publicKey,
        pool: poolPDA,
        project: fundraiserPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([ payer ])
      .rpc({
        skipPreflight: true
      });
  
      // If it passes, we get a friendly message
      console.log(`
        ✨ Pool "${poolName}" Joined w/ Project "${projectName}"! 
        Project Tx Hash: ${projectTx}, 
        Pool Tx Hash: ${poolTx}, 
        Join Tx Hash: ${tx}
      `);
    });
  });
  
    
  
}