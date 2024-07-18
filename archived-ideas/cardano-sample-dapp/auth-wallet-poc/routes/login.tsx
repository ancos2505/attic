import { Handlers } from "$fresh/server.ts";
import { encodeHex } from "$std/encoding/hex.ts";
import { convertBytesToHex, convertHexToBytes } from "@/helpers/hex.ts";
import {
  BigNum,
  COSEKey,
  COSESign1,
  Int,
  Label,
} from "cardano-message-signing-nodejs";
import { C } from "lucid";
import { jwtGenerateToken } from "@/helpers/jwt.ts";

type UserInput = {
  key: string;
  signature: string;
};

type User = {
  id: string;
  key: string;
  signature: string;
};

const registeredUsers = [
  "stake_test1urr4pgfpzzer5f0524tcwvgc5uqjxv44g66g4rktwdg926g33c8j7",
];

// const kv = await Deno.openKv(serverUri);
// const kv = await Deno.openKv("file::memory:?cache=shared");
const kv = await Deno.openKv();

export const handler: Handlers<UserInput | null> = {
  async POST(req, _ctx) {
    const rawUserInput = (await req.json()) as UserInput;

    if (!(typeof rawUserInput.key === "string")) {
      throw new Error("Invalid Input");
    }
    if (!(typeof rawUserInput.signature === "string")) {
      throw new Error("Invalid Input");
    }

    console.log("rawUserInput", rawUserInput);
    const messageBytes = convertHexToBytes(rawUserInput.signature);
    const decoded = COSESign1.from_bytes(messageBytes);
    // console.log("decoded", decoded);
    const headerMap = decoded.headers().protected().deserialized_headers();
    // console.log("headerMap", headerMap);
    const addressfromHeaderMap = headerMap.header(Label.new_text("address"));

    if (addressfromHeaderMap === undefined) {
      throw new Error("Can't get addressfromHeaderMap from headerMap");
    }
    // console.log("addressfromHeaderMap", addressfromHeaderMap);
    const addressHex = convertBytesToHex(addressfromHeaderMap.to_bytes())
      .substring(4);
    console.log("addressHex", addressHex);

    const address = C.Address.from_bytes(convertHexToBytes(addressHex));

    const key = COSEKey.from_bytes(convertHexToBytes(rawUserInput.key));
    const pubKeyCbor = key.header(
      Label.new_int(Int.new_negative(BigNum.from_str("2"))),
    );
    if (pubKeyCbor === undefined) {
      throw new Error("Can't get pubKeyCbor from rawUserInput.key");
    }
    const pubKeyBytes = pubKeyCbor.as_bytes();
    if (pubKeyBytes === undefined) {
      throw new Error("Can't convert pubKeyCbor int pubKeyBytes");
    }
    const publicKey = C.PublicKey.from_bytes(pubKeyBytes);

    const payload = decoded.payload();
    if (payload === undefined) {
      throw new Error("Can't get payload from decoded COSESign1");
    }
    const signature = C.Ed25519Signature.from_bytes(decoded.signature());
    // console.log("signature", signature);
    const receivedData = decoded.signed_data().to_bytes();
    // console.log("receivedData", receivedData);
    const rewardAddress = C.RewardAddress.from_address(address);
    if (rewardAddress === undefined) {
      throw new Error("Can't convert Address int RewardAddress");
    }
    const signerStakeAddrBech32 = rewardAddress
      .to_address().to_bech32("stake_test");
    console.log("signerStakeAddrBech32", signerStakeAddrBech32);
    const messageHex = convertBytesToHex(payload);
    console.log("messageHex", messageHex);
    const receivedMessageUtf = new TextDecoder().decode(payload);
    console.log('receivedMessageUtf "', receivedMessageUtf, '"');
    const expectedMessageUtf = `account: ${signerStakeAddrBech32}`; // reconstructed message
    console.log('expectedMessageUtf "', expectedMessageUtf, '"');

    // verify:
    const isVerified = publicKey.verify(receivedData, signature);

    console.log("isVerified", isVerified);
    const payloadAsExpected = receivedMessageUtf == expectedMessageUtf;
    console.log("payloadAsExpected", payloadAsExpected);
    const signerIsRegistered = registeredUsers.includes(
      signerStakeAddrBech32,
    );
    console.log("signerIsRegistered", signerIsRegistered);

    const isAuthSuccess = isVerified && payloadAsExpected &&
      signerIsRegistered;

    const jwtAccessExpiration = 3600;
    const jwtRefreshExpiration = 1800;

    const errorToSend = {
      success: false,
      message: "❌ Authentication failed.",
    };
    if (isAuthSuccess) {
      const userId = signerStakeAddrBech32;
      const now = Date.now(); // in millis
      const accessTokenExpires = now + (jwtAccessExpiration * 1000);
      const accessToken = await jwtGenerateToken(accessTokenExpires, userId);
      const refreshTokenExpires = now + (jwtRefreshExpiration * 1000);
      const refreshToken = await jwtGenerateToken(refreshTokenExpires, userId);

      const tokenToBeSaved = {
        token: refreshToken,
        user: userId,
        expires: new Date(refreshTokenExpires),
        type: "refresh",
        blacklisted: false,
      };
      console.log("tokenToBeSaved", tokenToBeSaved);
      const tokens = {
        access: {
          token: accessToken,
          expires: new Date(accessTokenExpires),
        },
        refresh: {
          token: refreshToken,
          expires: new Date(refreshTokenExpires),
        },
      };
      const bodyToSend = {
        tokens,
        user: {
          id: userId,
          displayName: "",
          role: "ADMIN",
          isDisabled: false,
          createdAt: "",
          updatedAt: "",
        },
        success: isAuthSuccess,
        message: isAuthSuccess
          ? "✅ Authentication success!"
          : "❌ Authentication failed.",
      };

      console.log("bodyToSend", bodyToSend);

      return new Response(JSON.stringify(bodyToSend));
    } else {
      return new Response(JSON.stringify(errorToSend));
    }
  },
  // const bodyStr = isAuthSuccess
  //   ? await getToken()
  //   : JSON.stringify(errorToSend);

  // const newId = await genId(req);
  // const user: User = {
  //   id: newId,
  //   key: rawUserInput["key"],
  //   signature: rawUserInput["signature"],
  // };
  // const userKey = ["users", newId];
  // const ok = await kv.atomic().set(userKey, user).commit();
  // if (!ok) throw new Error("Something went wrong.");
  // const data = await kv.get<User>(userKey);
  // console.log("data", data);
  // const retrievedUser = data.value!;
  // console.log(retrievedUser);
  // return new Response(JSON.stringify(retrievedUser));
};

async function genId(req: Request): Promise<string> {
  const now = new Date().getTime();
  const message = JSON.stringify(req) + `${now}`;
  const messageBuffer = new TextEncoder().encode(message);
  const hashBuffer = await crypto.subtle.digest("SHA-256", messageBuffer);
  const hash = encodeHex(hashBuffer);
  return hash;
}
