import * as anchor from "@project-serum/anchor";
import { AnchorError, LangErrorCode, Program } from "@project-serum/anchor";
import { expect } from "chai";
import { Voting } from "../target/types/voting";


const logTx = async (provider, tx) => {
  await provider.connection.confirmTransaction(tx, "confirmed");
  console.log(
    (await provider.connection.getConfirmedTransaction(tx, "confirmed")).meta
      .logMessages
  );
};

describe("voting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voting as Program<Voting>;
  const LAMPORTS_PER_SOL = 1000000000;
  console.log(program.programId.toBase58());
  const mint_auth = anchor.web3.Keypair.generate();
  const gobernance_account = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  const Candidate1 = anchor.web3.Keypair.generate();
  const Candidate2 = anchor.web3.Keypair.generate();
  
  it("Initialize start state", async () => {
    // Airdropping tokens to a payer.
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        mint_auth.publicKey,
        2*LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        gobernance_account.publicKey,
        2*LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        user.publicKey,
        0.02*LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        Candidate1.publicKey,
        0.02*LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        Candidate2.publicKey,
        0.02*LAMPORTS_PER_SOL
      ),
      "confirmed"
    );
  });

  it("Voting System Test", async () => {
    // Add your test here.
    let mint = (await anchor.web3.PublicKey.findProgramAddress(
      [mint_auth.publicKey.toBuffer()],
      program.programId,
    ))[0];
    let txid = await program.rpc.initializeMint({
      accounts: {
        mint: mint,
        payer: mint_auth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [mint_auth]
    });
    await logTx(program.provider, txid);
    let gobernance_accountTA = (await anchor.web3.PublicKey.findProgramAddress(
      [gobernance_account.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];
    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: gobernance_accountTA,
        mint: mint,
        payer: gobernance_account.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [gobernance_account],
    });
    await logTx(program.provider, txid);
    let userTA = (await anchor.web3.PublicKey.findProgramAddress(
      [user.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];

    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: userTA,
        mint: mint,
        payer: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [user],
    });
    // FIRST CANDIDATE 
    
    let Candidate1TA = (await anchor.web3.PublicKey.findProgramAddress(
      [Candidate1.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];
    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: Candidate1TA,
        mint: mint,
        payer: Candidate1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [Candidate1],
    });

    // SECOND CANDIDATE

    let Candidate2TA = (await anchor.web3.PublicKey.findProgramAddress(
      [Candidate2.publicKey.toBuffer(), mint.toBuffer()], program.programId
    ))[0];

    txid = await program.rpc.initializeTokenAccount({
      accounts: {
        tokenAccount: Candidate2TA,
        mint: mint,
        payer: Candidate2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [Candidate2],
    });
    

    await logTx(program.provider, txid);

    txid = await program.rpc.mint(new anchor.BN(25), {
      accounts: {
        dst: gobernance_accountTA,
        mint: mint,
        authority: mint_auth.publicKey,
      },
      signers: [mint_auth],
    });
    await logTx(program.provider, txid);

    //txid = await program.rpc.mint(new anchor.BN(1), {
    //  accounts: {
    //    dst: userTA,
    //    mint: mint,
    //    authority: mint_auth.publicKey,
    //  },
    //  signers: [mint_auth],
    //});

    txid = await program.rpc.transfer(new anchor.BN(2), {
      accounts: {
        src: gobernance_accountTA,
        dst: userTA,
        owner: gobernance_account.publicKey,
      },
      signers: [gobernance_account],
    });
    await logTx(program.provider, txid);

    txid = await program.rpc.transfer(new anchor.BN(1), {
      accounts: {
        src: userTA,
        dst: Candidate1TA,
        owner: user.publicKey,
      },
      signers: [user],
    });
    await logTx(program.provider, txid);
    txid = await program.rpc.transfer(new anchor.BN(1), {
      accounts: {
        src: userTA,
        dst: Candidate2TA,
        owner: user.publicKey,
      },
      signers: [user],
    });
    await logTx(program.provider, txid);
    

  });



  
  
});
