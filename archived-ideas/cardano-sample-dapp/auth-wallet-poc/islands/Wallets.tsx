import type { Signal } from "@preact/signals";
import { Button } from "@/components/Button.tsx";
import { C } from "lucid";
import { convertBytesToHex, convertHexToBytes } from "@/helpers/hex.ts";
// import { useEffect } from "preact/hooks";
// import { useState } from "preact/hooks";

type SignatureData = {
  key: string;
  signature: string;
};

interface WalletsProps {
  count: Signal<number>;
}

export default function Wallets(props: WalletsProps) {
  async function authenticateTyphon() {
    const wallet = await (window as any).cardano.typhoncip30.enable();
    // console.log("wallet", wallet);

    const [stakeAddrHex, stakeAddrBech32] = await getStakeAddress(wallet);
    const messageUtf = `account: ${stakeAddrBech32}`;
    // console.log("messageUtf", messageUtf);
    const messageBytes = new TextEncoder().encode(messageUtf);
    const messageHex = convertBytesToHex(messageBytes);
    // console.log("messageHex", messageHex);
    const sigData = await wallet.signData(stakeAddrHex, messageHex);
    const result = await submitToBackend(sigData);
    console.log("result", result);
    localStorage.setItem("session", JSON.stringify(result));
    // console.log('result["access"]["tokens"]', result["access"]["tokens"]);

    alert(result.message);
  }
  return (
    <div class="flex gap-8 py-6">
      <h2>Wallets</h2>
      <Button onClick={() => authenticateTyphon()}>Typhon</Button>
      {
        /* <p class="text-3xl tabular-nums">{props.count}</p>
      <Button onClick={authenticateTyphon}>Typhon</Button> */
      }
    </div>
  );
}

async function getStakeAddress(wallet: any) {
  const networkId = await wallet.getNetworkId();
  // console.log("networkId", networkId);
  const changeAddrHex = await wallet.getChangeAddress();
  // console.log("typeof(changeAddrHex)", typeof changeAddrHex);
  // console.log("changeAddrHex", changeAddrHex);
  // derive the stakeAddress from the change address to be sure we are getting
  // the stake address of the currently active account.
  const changeAddrBytes = convertHexToBytes(changeAddrHex);
  // console.log("typeof(changeAddrBytes)", typeof changeAddrBytes);
  // console.log("changeAddrBytes", changeAddrBytes);
  const changeAddress = C.Address.from_bytes(changeAddrBytes);
  // console.log("typeof(changeAddress)", typeof changeAddress);
  // console.log("changeAddress", changeAddress);
  const baseAddress = C.BaseAddress.from_address(changeAddress);
  if (baseAddress === undefined) {
    throw new Error("Error on converting Address to BaseAddress");
  }
  // console.log("typeof(baseAddress)", typeof baseAddress);
  // console.log("baseAddress", baseAddress);

  const stakeCredential = baseAddress.stake_cred();
  // console.log("typeof(stakeCredential)", typeof stakeCredential);
  // console.log("stakeCredential", stakeCredential);
  const stakeAddress = C.RewardAddress.new(networkId, stakeCredential)
    .to_address();
  // console.log("typeof(stakeAddress)", typeof stakeAddress);
  // console.log("stakeAddress", stakeAddress);

  const outcome = [
    convertBytesToHex(stakeAddress.to_bytes()),
    stakeAddress.to_bech32("stake_test"),
  ];

  // console.log(
  //   'outcome: [convertBytesToHex(stakeAddress.to_bytes()), stakeAddress.to_bech32("stake_test1")]: ',
  //   outcome,
  // );

  return outcome;
}

async function submitToBackend(sigData: SignatureData) {
  // console.log("sigData", sigData);
  const result = await fetch(`/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(sigData),
  });
  const responseBody = await result.json();
  // console.log("responseBody", responseBody);
  return responseBody;
}
