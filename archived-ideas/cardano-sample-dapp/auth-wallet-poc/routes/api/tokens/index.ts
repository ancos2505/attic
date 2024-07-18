import { Handlers } from "$fresh/server.ts";
import { encodeHex } from "$std/encoding/hex.ts";
import { AlgorandIndexer, Network, TatumSDK } from "@tatumio/tatum";

// fake call to prevent error
window.location = { hostname: "fake" };

type UserInput = {
  name: string;
};

type User = {
  id: string;
  name: string;
};

// const kv = await Deno.openKv(serverUri);
const kv = await Deno.openKv();

const tatumApiKey = Deno.env.get("TATUM_API_KEY");

export const handler: Handlers<UserInput | null> = {
  async GET(_req, _ctx) {
    // const rawUserInput = (await req.json()) as UserInput;
    {
      // Initialize the Tatum SDK for Algorand
      const tatum = await TatumSDK.init<AlgorandIndexer>({
        network: Network.ALGORAND_INDEXER_TESTNET,
        apiKey: {
          v4: tatumApiKey,
        },
      });

      // Check the health status of the Algorand node
      const healthStatus = await tatum.rpc.getHealth();

      // Log the health status
      console.log("Algorand Node Health Status:", healthStatus);

      // Always destroy the Tatum SDK instance when done to stop any background processes
      await tatum.destroy();
    }

    const newId = await genId({});
    // if (!(typeof rawUserInput.name === "string")) {
    //   throw new Error("Invalid Input");
    // }
    const user = {
      id: newId,
      // name: rawUserInput["name"],
    };

    const userKey = ["users", newId];
    const ok = await kv.atomic().set(userKey, user).commit();
    if (!ok) throw new Error("Something went wrong.");
    const data = await kv.get<User>(userKey);
    console.log("data", data);
    const retrievedUser = data.value!;
    console.log(retrievedUser);
    return new Response(JSON.stringify(retrievedUser));
  },
};

async function genId(req: {}): Promise<string> {
  const now = new Date().getTime();
  const message = JSON.stringify(req) + `${now}`;
  const messageBuffer = new TextEncoder().encode(message);
  const hashBuffer = await crypto.subtle.digest("SHA-256", messageBuffer);
  const hash = encodeHex(hashBuffer);
  return hash;
}
