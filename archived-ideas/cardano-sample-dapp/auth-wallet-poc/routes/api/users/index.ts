import { Handlers } from "$fresh/server.ts";
import { encodeHex } from "$std/encoding/hex.ts";
import { jwtGetPayloadFromToken } from "@/helpers/jwt.ts";

type UserInput = {
  name: string;
};

type User = {
  id: string;
  name: string;
};

// const kv = await Deno.openKv(serverUri);
// const kv = await Deno.openKv("file::memory:?cache=shared");
const kv = await Deno.openKv();

export const handler: Handlers<UserInput | null> = {
  async POST(req, _ctx) {
    // const req.headers;
    const auth = req.headers.get("Authorization");
    if (auth == undefined) {
      throw new Error("Can't get value on HTTP Header 'Authorization'");
    }
    const receivedToken = auth.split(" ").at(1);
    if (receivedToken == undefined) {
      throw new Error("Can't get Bearer token");
    }
    const payloadFromJwtToken = await jwtGetPayloadFromToken(receivedToken);

    console.log("payloadFromJwtToken", payloadFromJwtToken);

    const rawUserInput = (await req.json()) as UserInput;
    const newId = await genId(req);
    if (!(typeof rawUserInput.name === "string")) {
      throw new Error("Invalid Input");
    }
    const user = {
      id: newId,
      name: rawUserInput["name"],
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

async function genId(req: Request): Promise<string> {
  const now = new Date().getTime();
  const message = JSON.stringify(req) + `${now}`;
  const messageBuffer = new TextEncoder().encode(message);
  const hashBuffer = await crypto.subtle.digest("SHA-256", messageBuffer);
  const hash = encodeHex(hashBuffer);
  return hash;
}
