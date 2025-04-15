import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Auction } from "../target/types/auction";
import { Connection, Keypair, PublicKey } from '@solana/web3.js'; 
import { sha256 } from 'js-sha256';
import { Buffer } from 'buffer'

import testCaller from "/home/f/wallet/test1.json";
import { BN } from "bn.js";
import { assert } from "console";

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

export  function checkFundingStateAccount(
  programId: PublicKey, rootDomainPda: PublicKey){
  
  const seeds = [
      Buffer.from("web3 Auction"),
      rootDomainPda.toBuffer(),
  ];

  try{
      const [address, _] =  PublicKey.findProgramAddressSync(seeds, programId);

      return address
  }catch(err){
      console.log(err)
  }
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

  const connection = new Connection("http://localhost:8899", "confirmed"); 

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

  it("find", async () => {
      const SEED = Buffer.from("web3 auction account list");
  
      const [crowdingAccountPubkey, bump] = await PublicKey.findProgramAddressSync(
          [SEED],
          program.programId,
      );

    const accountInfo = await connection.getAccountInfo(payer.publicKey);

    if (accountInfo === null) {
      console.log("Account not found");
    }

    console.log("Account Info:", accountInfo);

  })

  const willAddAmount = 500000;

  const addAmount = new BN(willAddAmount);

  const {nameAccountKey: willCreateRootPda} = getSeedAndKey(
      WEB3_NAME_SERVICE_ID, getHashedName(willCreateName), null
  )

  const {nameAccountKey: allRootRecordPda} = getSeedAndKey(
    WEB3_NAME_SERVICE_ID, getHashedName(program.programId.toBase58()), null
  )

  const rootFundStateAccount =  checkFundingStateAccount(program.programId, willCreateRootPda);

  // it("add", async () => {
  //     try{
  //       if(rootFundStateAccount){
  //           const addTx = await program.methods
  //               .addFunding(addAmount, willCreateName)
  //               .accounts({
  //                   willCreateRoot: willCreateRootPda,
  //                   allRootRecordAccount: allRootRecordPda,
  //                   fundraisingStateAccount: rootFundStateAccount,
  //                   payer: payer.publicKey,
  //               })
  //               .rpc()

  //           console.log('add:', addTx);

  //           return addTx;
  //       }else{
  //           throw new Error("no this account");
  //       }
  //   }catch(err){
  //     console.error('Error creating name:', err);
  //   }
  // })

        
});


