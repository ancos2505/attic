import { Handlers } from "$fresh/server.ts";

type Token = {
  id: string;
  name: string;
};
const kv = await Deno.openKv();

export const handler: Handlers<Token | null> = {
  async GET(_req, ctx) {
    const id = ctx.params.id;
    console.log(id);

    await logAllEntries();

    const key = ["tokens", id];
    console.log(key);
    const data = await kv.get<Token>(key);

    console.log(data);
    const token = data.value!;
    console.log(token);
    return new Response(JSON.stringify(token));
  },
};

async function logAllEntries() {
  console.log("--logAllEntries(): BEGIN--");
  const entries = kv.list({ prefix: ["tokens"] });
  let n = 0;

  for await (const entry of entries) {
    n += 1;
    console.log(`entry: ${n}`, entry);
  }
  console.log(`Total entries: ${n}`);
  console.log("--logAllEntries(): END--");
}
