import * as anchor from "@project-serum/anchor";
import * as fs from "fs";

describe("beacon-server", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const idl = JSON.parse(
    fs.readFileSync("./target/idl/beacon_server.json", "utf8")
  );
  const programId = new anchor.web3.PublicKey("FRoo7m8Sf6ZAirGgnn3KopQymDtujWx818kcnRxzi23b");
  const program = new anchor.Program(idl, programId);

  const testAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const beaconIdRaw = "0384392".padEnd(64, "0");
    const beaconID = Buffer.from(beaconIdRaw, "hex");
    const templateID = Buffer.allocUnsafe(32);
    const timestamp = Buffer.allocUnsafe(32);
    const data = Buffer.from(anchor.utils.bytes.utf8.encode("random-test-data"));
    const signature = Buffer.from(anchor.utils.bytes.utf8.encode("random-test-signature"));

    const [beaconIdPDA, beaconIdBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("beacon-id")),
        beaconID
      ],
      program.programId
    )

    const tx = await program.rpc.newBeacon(
      beaconID,
      templateID,
      timestamp,
      data,
      signature,
      {
        accounts: {
          beacon: beaconIdPDA,
          user: anchor.getProvider().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
      }
    );
    console.log("Your transaction signature", tx);
  });
});
