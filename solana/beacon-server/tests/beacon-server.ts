import * as anchor from "@project-serum/anchor";
import { assert, expect } from "chai";
import nacl from 'tweetnacl';
import * as fs from "fs";
import { bufferU64BE, deriveBeaconId, deriveDApiId, deriveDatapointPDA, encodeData, prepareMessage } from "./utils";
import { createInstructionWithPublicKey } from "./sig";

const delay = ms => new Promise(resolve => setTimeout(resolve, ms))

/**
 * Create a new offline UpdateBeaconWithSignedData transasction
 * 
 * @param beaconID 
 * @param templateID 
 * @param timestamp 
 * @param data 
 * @param program 
 * @param beaconIdPDA 
 * @param storageFunderKey 
 * @param txnRelayerKey 
 * @returns The serialized offline transaction buffer
 */
async function newUpdateBeaconWithSignedDataTxn(
  beaconID: Buffer,
  templateID: number,
  timestamp: number,
  data: number,
  program: anchor.Program,
  beaconIdPDA: anchor.web3.PublicKey,
  storageFunder: anchor.web3.Keypair,
  txnRelayerKey: anchor.web3.PublicKey,
): Promise<[Uint8Array, Buffer]> {
  // const bufferedTimestamp = Buffer.allocUnsafe(32);
  // bufferedTimestamp.writeBigInt64BE(BigInt(0), 0);
  // bufferedTimestamp.writeBigInt64BE(BigInt(0), 8);
  // bufferedTimestamp.writeBigInt64BE(BigInt(0), 16);
  // bufferedTimestamp.writeBigInt64BE(BigInt(timestamp), 24);
  const bufferedTimestamp = bufferU64BE(timestamp);
  const encodedData = encodeData(data);

  const method = program.instruction.updateBeaconWithSignedData(
    beaconID,
    bufferU64BE(templateID),
    bufferedTimestamp,
    encodedData,
    {
      accounts: {
        datapoint: beaconIdPDA,
        user: storageFunder.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    }
  );
  
  const tx = new anchor.web3.Transaction().add(method);
  tx.recentBlockhash = (await program.provider.connection.getLatestBlockhash()).blockhash;
  tx.feePayer = txnRelayerKey;

  const rawTxn = tx.serializeMessage();
  const signature = nacl.sign.detached(rawTxn, storageFunder.secretKey);
  return [signature, rawTxn];
}

async function relayTxn(
  rawTxn: Buffer,
  storageSignature: Uint8Array,
  storageFunderKey: anchor.web3.PublicKey,
  relayer: anchor.web3.Keypair,
): Promise<Buffer> {
  const relayerSignature = nacl.sign.detached(rawTxn, relayer.secretKey);
  let recoverTx = anchor.web3.Transaction.populate(anchor.web3.Message.from(rawTxn));
  recoverTx.addSignature(relayer.publicKey, Buffer.from(relayerSignature));
  recoverTx.addSignature(storageFunderKey, Buffer.from(storageSignature));

  return recoverTx.serialize();
}

describe("beacon-server", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const idl = JSON.parse(
    fs.readFileSync("./target/idl/beacon_server.json", "utf8")
  );
  const programId = new anchor.web3.PublicKey("FRoo7m8Sf6ZAirGgnn3KopQymDtujWx818kcnRxzi23b");
  const program = new anchor.Program(idl, programId);

  const airnode = anchor.web3.Keypair.generate();
  const airnode1 = anchor.web3.Keypair.generate();
  const airnode2 = anchor.web3.Keypair.generate();
  const airnode3 = anchor.web3.Keypair.generate();
  const messageRelayer = anchor.web3.Keypair.generate();

  before(async () => {
    // fund the accounts one shot
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(airnode.publicKey, anchor.web3.LAMPORTS_PER_SOL));
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(airnode1.publicKey, anchor.web3.LAMPORTS_PER_SOL));
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(airnode2.publicKey, anchor.web3.LAMPORTS_PER_SOL));
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(airnode3.publicKey, anchor.web3.LAMPORTS_PER_SOL));
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(messageRelayer.publicKey, anchor.web3.LAMPORTS_PER_SOL));
  })

  it("updateBeaconWithSignedData", async () => {
    // 1. Airnode create the txn
    const templateID = 1;
    const timestamp = 1649133996;    
    const data = 123;

    const beaconId = deriveBeaconId(airnode.publicKey.toBytes(), templateID);
    const beaconIdPDA = await deriveDatapointPDA(beaconId, program.programId);
    console.log("raw beaconId with length", beaconId.length, "and value", beaconId.toString("hex"), "pda", beaconIdPDA.toString());

    const [airnodeSignature, airnodeTxn] = await newUpdateBeaconWithSignedDataTxn(
      beaconId,
      templateID,
      timestamp,
      data,
      program,
      beaconIdPDA,
      airnode,
      messageRelayer.publicKey
    );
    
    // 2. Relay the transaction
    const offlineTxn = await relayTxn(airnodeTxn, airnodeSignature, airnode.publicKey, messageRelayer);

    // 3. Send transaction
    await provider.connection.sendRawTransaction(offlineTxn);

    // wait a bit for the transaction to take effect
    await delay(1000);

    const wrappedDataPoint = await program.account.wrappedDataPoint.fetch(beaconIdPDA);

    // construct expected
    const expected = Buffer.allocUnsafe(36);
    expected.writeBigInt64BE(BigInt(0), 0);
    expected.writeBigInt64BE(BigInt(0), 8);
    expected.writeBigInt64BE(BigInt(0), 16);
    expected.writeBigInt64BE(BigInt(data), 24);
    expected.writeUInt32BE(timestamp, 32);

    expect(wrappedDataPoint.rawDatapoint).to.deep.eq(expected);
  });

  // it("updateDapiWithBeacons", async () => {
  //   const [beaconIdPDA] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from(anchor.utils.bytes.utf8.encode("datapoint")),
  //       beaconID
  //     ],
  //     program.programId
  //   )

  //   const tempDAPIId = Buffer.from("1".padEnd(64, "0"), "hex");
  //   const [dapiPDA] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from(anchor.utils.bytes.utf8.encode("datapoint")),
  //       tempDAPIId
  //     ],
  //     program.programId
  //   )

  //   const tx = await program.rpc.updateDapiWithBeacons(
  //     tempDAPIId,
  //     [beaconID],
  //     {
  //       accounts: {
  //         dapi: dapiPDA,
  //         user: anchor.getProvider().wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       },
  //       remainingAccounts: [
  //         { isSigner: false, isWritable: false, pubkey: beaconIdPDA }
  //       ],
  //     }
  //   );

  //   const wrappedDataPoint = await program.account.wrappedDataPoint.fetch(dapiPDA);
  //   console.log(JSON.stringify(wrappedDataPoint));
  //   // expect(wrappedDataPoint.rawDatapoint).to.deep.eq(data);
  // });

  it("updateDapiWithSignedData", async () => {
    // Step 1. Airnode1 create the data
    const templateId1 = 1;
    const timestamp1 = 1649133996;    
    const data1 = 123;
    const message1 = prepareMessage(templateId1, timestamp1, data1);
    const sig1 = nacl.sign.detached(message1, airnode1.secretKey);

    // Step 2. Airnode2 create the data
    const templateId2 = 2;
    const timestamp2 = 1649133997;    
    const data2 = 124;
    const message2 = prepareMessage(templateId2, timestamp2, data2);
    const sig2 = nacl.sign.detached(message2, airnode2.secretKey);
    
    // Step 3. Airnode3 no data
    const templateId3 = 1;
    const timestamp3 = 1649133997; 

    // Step 4. Create the transaction call
    const beaconId1 = deriveBeaconId(airnode1.publicKey.toBytes(), templateId1);
    const beaconId2 = deriveBeaconId(airnode2.publicKey.toBytes(), templateId2);
    const beaconId3 = deriveBeaconId(airnode.publicKey.toBytes(), templateId3);
    console.log("beaconId3", beaconId3.toString("hex"));
    const beaconIds = [beaconId1, beaconId2, beaconId3];

    const dataPointId = deriveDApiId(beaconIds);

    const sigVerify = createInstructionWithPublicKey(
      [
        { publicKey: airnode1.publicKey.toBytes(), message: message1, signature: sig1 },
        { publicKey: airnode2.publicKey.toBytes(), message: message2, signature: sig2 }
      ],
      0
    );

    const dapiPDA = await deriveDatapointPDA(dataPointId, program.programId);
    const remainingAccounts = [{ isSigner: false, isWritable: false, pubkey: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY }];
    for (const id of [beaconId3]) {
      const pda = await deriveDatapointPDA(id, program.programId);
      const wrappedDataPoint = await program.account.wrappedDataPoint.fetch(pda);
      console.log(wrappedDataPoint);
      console.log("pda", id.toString("hex"), pda.toString(), airnode.publicKey.toString());
      remainingAccounts.push({ isSigner: false, isWritable: false, pubkey: pda });
    }

    console.log([airnode1, airnode2, airnode3].map(t => t.publicKey.toBytes()));
    const updateInstruction = program.instruction.updateDapiWithSignedData(
      dataPointId,
      [airnode1, airnode2, airnode3].map(t => t.publicKey.toBytes()),
      beaconIds,
      [templateId1, templateId2, templateId3].map(t => bufferU64BE(t)),
      [timestamp1, timestamp2, timestamp3].map(t => bufferU64BE(t)),
      [data1, data2, data1].map(t => encodeData(t)),
      {
        accounts: {
          datapoint: dapiPDA,
          user: messageRelayer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        remainingAccounts,
      }
    );

    const tx = new anchor.web3.Transaction();
    tx.add(sigVerify);
    tx.add(updateInstruction);

    await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [messageRelayer],
    );
  });
});
