import {
  create,
  getNumericDate,
  type Header,
  type Payload,
  verify,
} from "djwt";

const key = await crypto.subtle.generateKey(
  { name: "HMAC", hash: "SHA-512" },
  true,
  ["sign", "verify"],
);

const header: Header = {
  alg: "HS512",
  typ: "JWT",
};

// export async function newJwtSession(): Promise<string> {
//   const verifiedDataBeforeSend = await verify(bodyToSend, key);
//   console.log("newJwtEnvelope::verifiedDataBeforeSend", verifiedDataBeforeSend);
//   return bodyToSend;
// }

export async function jwtGenerateToken(exp: number, id: string): Promise<string> {
  console.log("exp", exp);
  console.log("id", id);
  const now = Date.now(); // in millis
  console.log("now", now);
  const payload: Payload = {
    iss: "deno_rest",
    iat: now,
    id,
    exp,
  };
  console.log("header", header);
  console.log("payload", payload);
  console.log("key", key);
  const tokenToSend = await create(header, payload, key);
  console.log("newJwtEnvelope::tokenToSend", tokenToSend);
  return tokenToSend;
}

export async function jwtGetPayloadFromToken(token: string): Promise<Payload> {
  const verifiedPayload = await verify(token, key);
  console.log("jwtVerifyToken::verifiedPayload", verifiedPayload);
  return verifiedPayload;
}
