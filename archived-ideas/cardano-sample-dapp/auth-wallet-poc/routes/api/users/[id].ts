import { Handlers } from "$fresh/server.ts";

type User = {
  id: string;
  name: string;
};

// const kv = await Deno.openKv("file::memory:?cache=shared");
const kv = await Deno.openKv();

export const handler: Handlers<User | null> = {
  async GET(_req, ctx) {
    const id = ctx.params.id;
    console.log(id);

    await logAllEntries();

    const key = ["users", id];
    console.log(key);
    const data = await kv.get<User>(key);

    console.log(data);
    const user = data.value!;
    console.log(user);
    return new Response(JSON.stringify(user));
  },
};

async function logAllEntries() {
  console.log("--logAllEntries(): BEGIN--");
  const entries = kv.list({ prefix: ["users"] });
  let n = 0;

  for await (const entry of entries) {
    n += 1;
    console.log(`entry: ${n}`, entry);
  }
  console.log(`Total entries: ${n}`);
  console.log("--logAllEntries(): END--");
}
