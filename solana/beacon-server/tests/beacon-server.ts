import * as anchor from "@project-serum/anchor";
import { expect } from "chai";
import nacl from 'tweetnacl';
import * as fs from "fs";

const delay = ms => new Promise(resolve => setTimeout(resolve, ms))

describe("beacon-server", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const idl = JSON.parse(
    fs.readFileSync("./target/idl/beacon_server.json", "utf8")
  );
  const programId = new anchor.web3.PublicKey("FRoo7m8Sf6ZAirGgnn3KopQymDtujWx818kcnRxzi23b");
  const program = new anchor.Program(idl, programId);

  const airnode = anchor.web3.Keypair.generate();
  const messageRelayer = anchor.web3.Keypair.generate();

  const beaconID = Buffer.from("0384392".padEnd(64, "0"), "hex");

  before(async () => {
    // fund the accounts one shot
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(airnode.publicKey, anchor.web3.LAMPORTS_PER_SOL));
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(messageRelayer.publicKey, anchor.web3.LAMPORTS_PER_SOL));
  })
  
  it("updateBeaconWithSignedData", async () => {
    const templateID = Buffer.allocUnsafe(32);
    const timestamp = Buffer.allocUnsafe(32);
    const data = Buffer.from(anchor.utils.bytes.utf8.encode("random-test-data"));

    const [beaconIdPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("datapoint")),
        beaconID
      ],
      program.programId
    );

    const method = program.instruction.updateBeaconWithSignedData(
      beaconID,
      templateID,
      timestamp,
      data,
      {
        accounts: {
          datapoint: beaconIdPDA,
          user: airnode.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
      }
    );

    const tx = new anchor.web3.Transaction().add(method);
    tx.recentBlockhash = (await program.provider.connection.getLatestBlockhash()).blockhash;
    tx.feePayer = messageRelayer.publicKey;
    const realDataNeedToSign = tx.serializeMessage();

    // 2. Sign Transaction
    const feePayerSignature = nacl.sign.detached(realDataNeedToSign, messageRelayer.secretKey);
    const airnodeSignature = nacl.sign.detached(realDataNeedToSign, airnode.secretKey);

    // 3. Recover transaction
    let recoverTx = anchor.web3.Transaction.populate(anchor.web3.Message.from(realDataNeedToSign));
    recoverTx.addSignature(messageRelayer.publicKey, Buffer.from(feePayerSignature));
    recoverTx.addSignature(airnode.publicKey, Buffer.from(airnodeSignature));

    // 4. Send transaction
    await provider.connection.sendRawTransaction(recoverTx.serialize());

    // wait a bit for the transaction to take effect
    await delay(1000);

    const wrappedDataPoint = await program.account.wrappedDataPoint.fetch(beaconIdPDA);
    expect(wrappedDataPoint.rawDatapoint).to.deep.eq(data);
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
});
