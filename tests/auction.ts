import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Auction } from "../target/types/auction";
import { Keypair, PublicKey } from '@solana/web3.js'; 
import { sha256 } from 'js-sha256';
import { Buffer } from 'buffer'

import testCaller from "/home/f/wallet/test1.json";

const HASH_PREFIX = "WEB3 Name Service";

export const WEB3_NAME_SERVICE_ID = new PublicKey("9WykwriEQGT1RjzJvAa7a31AQ8ZtHGnvmXRaeQ47oQLk");


export function getHashedName(name: string){
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

export function getSeedAndKey(
  programid: PublicKey, hashedName: Uint8Array, rootOpt: null | PublicKey ){
  
  let seeds = new Uint8Array([...hashedName]);
  
  const rootDomain = rootOpt || PublicKey.default;
  seeds = new Uint8Array([...seeds, ...rootDomain.toBytes()]);

  const seedChunks = [];
  for (let i = 0; i < seeds.length; i += 32) {
      const chunk = seeds.slice(i, i + 32);
      seedChunks.push(chunk);
  }

  const [nameAccountKey, bump] = PublicKey.findProgramAddressSync(
      seedChunks,
      programid
  );

  seeds = new Uint8Array([...seeds, bump]);

  return {nameAccountKey, seeds};
}

describe("auction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Auction as Program<Auction>;

  //prepare for the accounts

  const willCreateName = "web3"
  
  //will create root
  const {nameAccountKey: willCreateRoot} = getSeedAndKey(
    WEB3_NAME_SERVICE_ID, getHashedName(willCreateName), null
  )
  console.log("will create root:", willCreateRoot.toBase58())

  const {nameAccountKey: willCreateRecord} = getSeedAndKey(
    WEB3_NAME_SERVICE_ID, getHashedName(program.programId.toBase58()), null
  )
  console.log("auction record:", willCreateRecord.toBase58())


  const payerSecretKey = Uint8Array.from(testCaller)
  const payer = Keypair.fromSecretKey(payerSecretKey);

  console.log("caller:", payer.publicKey.toBase58())

  it("start call the auction create root account", async () => {
    
    try {
        const tx = await program.methods
            .createFunding(willCreateName)
            .accounts({
              willCreateRoot: willCreateRoot,
              caller: payer.publicKey,
              
            })
            .signers([payer])
            .rpc();
        console.log('Transaction successful:', tx);
    } catch (err) {
        console.error('Error creating name:', err);
    }
  });
});


