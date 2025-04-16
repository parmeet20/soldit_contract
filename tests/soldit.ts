import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Soldit } from "../target/types/soldit";
import fs from 'fs';
import { assert } from "chai";
const { SystemProgram, PublicKey } = anchor.web3;
const idl = require("../target/idl/soldit.json");

describe("soldit", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = new Program<Soldit>(idl, provider);

  it("It initializes the user", async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    try {
      await program.methods.initializeUser("usrname").accountsPartial({
        authority: creator.publicKey,
        user: userPda,
        systemProgram: SystemProgram.programId,
      }).rpc();
      console.log("usr initiaized");
    } catch (error) {
      const user = await program.account.user.fetch(userPda);
      assert.equal(user.initialized, true);
      assert.equal(user.username, "usrname");
      console.log("usr already initiaized", user);
    }
  });
  it('Creates new thread', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    const user = await program.account.user.fetch(userPda);
    const tid = new anchor.BN(user.threadCount).add(new anchor.BN(1));
    const [threadPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), creator.publicKey.toBuffer(), tid.toArrayLike(Buffer, 'le', 8)], program.programId,
    )
    const TITLE: string = "DEMO TITLE";
    const DESCRIPTION: string = "DEMO DESCRIPTION";
    const IMAGE: string = "https://images.pexels.com/photos/326055/pexels-photo-326055.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2";
    await program.methods.createThread(TITLE, DESCRIPTION, IMAGE)
      .accountsPartial({
        thread: threadPda,
        user: userPda,
        authority: creator.publicKey,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const thread = await program.account.thread.fetch(threadPda);
    console.log(thread);
  })
  it('Updates the thread', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    const user = await program.account.user.fetch(userPda);
    const tid = new anchor.BN(user.threadCount);
    const [threadPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), creator.publicKey.toBuffer(), tid.toArrayLike(Buffer, 'le', 8)], program.programId,
    )
    const TITLE: string = "UPDATED TITLE";
    const DESCRIPTION: string = "UPDATED DESCRIPTION";
    const IMAGE: string = "https://images.pexels.com/photos/326055/pexels-photo-326055.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2";
    await program.methods.updateThread(TITLE, DESCRIPTION, IMAGE)
      .accountsPartial({
        thread: threadPda,
        user: userPda,
        authority: creator.publicKey,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const thread = await program.account.thread.fetch(threadPda);
    console.log(thread);
  })
  it('upvote thread', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    const user = await program.account.user.fetch(userPda);
    const tid = new anchor.BN(user.threadCount);
    const [threadPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), creator.publicKey.toBuffer(), tid.toArrayLike(Buffer, 'le', 8)], program.programId,
    )
    const [upvotePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("upvote_thread"), tid.toArrayLike(Buffer, 'le', 8), creator.publicKey.toBuffer()],
      program.programId,
    )
    await program.methods.upvoteThread(tid)
      .accountsPartial({
        authority: creator.publicKey,
        voter: creator.publicKey,
        thread: threadPda,
        upvote: upvotePda,
        user: userPda,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const th = await program.account.thread.fetch(threadPda);
    console.log(th);
  })
  it('creates new comment on thread', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    const user = await program.account.user.fetch(userPda);
    const tid = new anchor.BN(user.threadCount);
    const [threadPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), creator.publicKey.toBuffer(), tid.toArrayLike(Buffer, 'le', 8)], program.programId,
    )
    const thread = await program.account.thread.fetch(threadPda);
    const comment_count = thread.commentCount.add(new anchor.BN(1));
    const [commentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("comment"), tid.toArrayLike(Buffer, "le", 8), comment_count.toArrayLike(Buffer, "le", 8), creator.publicKey.toBuffer()],
      program.programId,
    );
    const COMMENT = "NEW COMMENT ON THIS THREAD";
    await program.methods.createComment(tid, COMMENT).accountsPartial({
      authority: creator.publicKey,
      commentAuthority: creator.publicKey,
      thread: threadPda,
      comment: commentPda,
      user: userPda,
      systemProgram: SystemProgram.programId,
    }).rpc();
    const comment = await program.account.comment.fetch(commentPda);
    console.log(comment);
    const usr = await program.account.user.fetch(userPda);
    console.log(usr);
    const thre = await program.account.thread.fetch(threadPda);
    console.log(thre);
  })
  it('it updates comment on thread', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    )
    const user = await program.account.user.fetch(userPda);
    const tid = new anchor.BN(user.threadCount);
    const [threadPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("thread"), creator.publicKey.toBuffer(), tid.toArrayLike(Buffer, 'le', 8)], program.programId,
    )
    const thread = await program.account.thread.fetch(threadPda);
    const comment_count = thread.commentCount;
    const [commentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("comment"), tid.toArrayLike(Buffer, "le", 8), comment_count.toArrayLike(Buffer, "le", 8), creator.publicKey.toBuffer()],
      program.programId,
    );
    const NEW_COMMENT = "NEW COMMENT ON THIS THREAD";
    await program.methods.updateComment(tid, NEW_COMMENT).accountsPartial({
      authority: creator.publicKey,
      commentAuthority: creator.publicKey,
      thread: threadPda,
      comment: commentPda,
      user: userPda,
      systemProgram: SystemProgram.programId,
    }).rpc();
    const comment = await program.account.comment.fetch(commentPda);
    console.log(comment);
    const usr = await program.account.user.fetch(userPda);
    console.log(usr);
    const thre = await program.account.thread.fetch(threadPda);
    console.log(thre);
  })
  it('it handles voting comment', async () => {
    const creator = provider.wallet;
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), creator.publicKey.toBuffer()],
      program.programId,
    );
    const usr = await program.account.user.fetch(userPda);
    const tid = usr.threadCount;
    const [threadPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("thread"),
        creator.publicKey.toBuffer(),
        tid.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )
    const thread = await program.account.thread.fetch(threadPda);
    const comment_count = thread.commentCount;
    const [commentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("comment"), tid.toArrayLike(Buffer, "le", 8), comment_count.toArrayLike(Buffer, "le", 8), creator.publicKey.toBuffer()]
      , program.programId);
    const [voteCommentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("comment_vote"),
      commentPda.toBuffer(),
      creator.publicKey.toBuffer(),
      ],
      program.programId,
    )
    const tx = await program.methods.voteComment(tid, false)
      .accountsPartial({
        authority: creator.publicKey,
        commentAuthority: creator.publicKey,
        voter: creator.publicKey,
        thread: threadPda,
        comment: commentPda,
        voteComment: voteCommentPda,
        user: userPda,
        systemProgram: SystemProgram.programId,
      }).rpc();
    const keypairPath = './id.json'
    const keyPairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
    const wallet = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(keyPairData));
    const [voteCommentPda2] = PublicKey.findProgramAddressSync(
      [Buffer.from("comment_vote"),
      commentPda.toBuffer(),
      wallet.publicKey.toBuffer(),
      ],
      program.programId,
    )
    const tx2 = await program.methods.voteComment(tid, false)
      .accountsPartial({
        authority: creator.publicKey,
        commentAuthority: creator.publicKey,
        voter: wallet.publicKey,
        thread: threadPda,
        comment: commentPda,
        voteComment: voteCommentPda2,
        user: userPda,
        systemProgram: SystemProgram.programId,
      }).signers([wallet]).rpc();

    console.log(tx);
    console.log(tx2);
    console.log(await program.account.voteComment.fetch(voteCommentPda));
    console.log(await program.account.comment.fetch(commentPda));
  })
});
