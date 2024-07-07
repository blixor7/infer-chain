import { Connection, PublicKey } from "@solana/web3.js";
import * as borsh from "borsh";

interface IProgramSchema {
  [key: string]: string;
}

interface IProgramState {
  [key: string]: any;
}

export interface IProgram {
  readonly schema: IProgramSchema;
  getState: () => IProgramState;
}

export enum CounterInstructions {
  InitializeCounter = 0,
  IncrementCounter = 1,
  DecrementCounter = 2,
}

export const CounterSchema = {
  struct: { count: "i64", bump: "u8" },
};

// TODO: explore WASM for annotating state
// export class CounterAccountData {
//   public readonly count;
//   public readonly bump;
//
//   constructor(fields: { count: number; bump: number }) {
//     if (fields) {
//       this.count = fields.count;
//       this.bump = fields.bump;
//     }
//   }
// }

export async function getCounterAccountData(
  connection: Connection,
  publicKey: PublicKey,
): Promise<CounterAccountData | undefined> {
  try {
    // get data
    const accountInfo = await connection.getAccountInfo(publicKey);

    if (accountInfo) {
      const fetchedData = accountInfo.data;
      const counterData = borsh.deserialize(
        CounterSchema,
        fetchedData,
      ) as CounterAccountData;
      return counterData;
    }
  } catch (err) {
    console.error(err);
  }
}

export interface CounterAccountData {
  count: number;
  bump: number;
}
