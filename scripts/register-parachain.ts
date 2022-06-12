import fs from "fs";
import "@polkadot/api-augment";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { Bytes } from "@polkadot/types";

import {
  AddressOrPair,
  ApiTypes,
  SubmittableExtrinsic,
} from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";
import { TypeRegistry } from "@polkadot/types/create";

const PARACHAIN_ID = 2000;

export function signAndSend(
  tx: SubmittableExtrinsic<ApiTypes>,
  account: AddressOrPair
) {
  return new Promise<{ block: string }>(async (resolve, reject) => {
    await tx.signAndSend(account, (result) => {
      console.log(`Current status is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transaction included at blockHash ${result.status.asInBlock}`
        );
      } else if (result.status.isFinalized) {
        console.log(
          `Transaction finalized at blockHash ${result.status.asFinalized}`
        );
        resolve({
          block: result.status.asFinalized.toString(),
        });
      } else if (result.status.isInvalid) {
        reject(`Transaction is ${result.status}`);
      }
    });
  });
}

async function registerParachain(api: ApiPromise) {
  // Get keyring of Alice, who is also the sudo in dev chain spec
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const genesisHeadBytes = fs.readFileSync("tmp/genesis-state", "utf8");
  const validationCodeBytes = fs.readFileSync("tmp/genesis-wasm", "utf8");

  const registry = new TypeRegistry();

  const tx = api.tx.sudo.sudo(
    api.tx.parasSudoWrapper.sudoScheduleParaInitialize(
      PARACHAIN_ID,
      {
        genesisHead: new Bytes(registry, genesisHeadBytes),
        validationCode: new Bytes(registry, validationCodeBytes),
        parachain: true,
      }
    )
  );

  console.log(`Parachain registration tx Sent!`);
  return signAndSend(tx, alice);
}

(async () => {
  console.log("Register parachain ...");

  const provider = new WsProvider("ws://localhost:9943");
  const api = await ApiPromise.create({
    provider: provider,
  });

  await registerParachain(api);
  await api.disconnect();
  provider.on("disconnected", () => {
    console.log("Disconnect from relaychain");
    process.exit(0);
  });
})();
