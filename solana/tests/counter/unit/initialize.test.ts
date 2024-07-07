// import {
//   Connection,
//   Keypair,
//   Transaction,
//   TransactionInstruction,
//   sendAndConfirmTransaction,
//   LAMPORTS_PER_SOL,
//   PublicKey,
//   AccountMeta,
//   SystemProgram,
// } from "@solana/web3.js";
// import { Buffer } from "buffer";
// import { getCounterAccountData } from "../../../client";
//
// import dotenv from "dotenv";
// dotenv.config({ path: ".env" });
//
// import TEST_KEYPAIR from "../../test-keypair.json";
//
// const RPC_URL = process.env.TEST_RPC_URL as string;
// const PROGRAM_ID = new PublicKey(process.env.TEST_COUNTER_PROGRAM_ID as string);
// const WALLET = Keypair.fromSecretKey(new Uint8Array(TEST_KEYPAIR));
//
// const INSTRUCTIONS = {
//   Initialize: 0,
//   Increment: 1,
//   Decrement: 2,
// };
//
// jest.setTimeout(100000);
//
// test("initialize counter", async () => {
//   const connection = new Connection(RPC_URL);
//   const payer = WALLET;
//
//   // get airdrop
//   await connection.requestAirdrop(payer.publicKey, 5 * LAMPORTS_PER_SOL);
//
//   const [counterAccountPda, canonicalBump] = PublicKey.findProgramAddressSync(
//     [payer.publicKey.toBuffer(), Buffer.from("counter_account")],
//     PROGRAM_ID,
//   );
//
//   const accounts: AccountMeta[] = [
//     {
//       pubkey: payer.publicKey,
//       isSigner: true,
//       isWritable: true,
//     },
//     {
//       pubkey: counterAccountPda,
//       isSigner: false,
//       isWritable: true,
//     },
//     {
//       pubkey: SystemProgram.programId,
//       isSigner: false,
//       isWritable: true,
//     },
//   ];
//
//   const CODE_SIZE = 1; // byte (octet)
//   const BUMP_SIZE = 1; // byte (octet)
//
//   const DATA_SIZE = CODE_SIZE + BUMP_SIZE;
//
//   // instruction data
//   const instructionData = Buffer.alloc(DATA_SIZE);
//   instructionData.writeUInt8(INSTRUCTIONS.Initialize, 0); // InitializeCounter(bump)
//   instructionData.writeUInt8(canonicalBump, CODE_SIZE); // bump
//
//   const instruction = new TransactionInstruction({
//     keys: accounts,
//     programId: PROGRAM_ID,
//     data: instructionData,
//   });
//
//   const tx = new Transaction().add(instruction);
//
//   try {
//     const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
//   } catch (err) {
//     console.error(err);
//   }
//
//   const counterData = await getCounterAccountData(
//     connection,
//     counterAccountPda,
//   );
//
//   expect(counterData?.count).toBe(BigInt(0));
//   expect(counterData?.bump).toBe(canonicalBump);
// });
