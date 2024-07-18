use wasm_bindgen::prelude::*;
use gloo_console::log;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

// pub fn get_stake_address(addr: &str) -> JsValue {
#[wasm_bindgen]
pub fn get_stake_address() -> JsValue {
  
  // async function getStakeAddress(wallet: any) {
//   // if (network === AddressFormat.mainnet) {}
//   const networkId = await wallet.getNetworkId();
//   const changeAddrHex = await wallet.getChangeAddress();

//   // derive the stakeAddress from the change address to be sure we are getting
//   // the stake address of the currently active account.
//   const changeAddress: Address = Address.from_hex(changeAddrHex);
//   console.log("changeAddress", changeAddress);
//   // const changeAddress = Address.from_bytes(Buffer.from(changeAddrHex, "hex"));
//   const messageHex = new TextEncoder().encode(changeAddrHex).toString();
//   console.log("messageHex", messageHex);
//   const baseAddr = BaseAddress.from_address(changeAddress);
//   if (baseAddr === undefined) {
//     throw new Error("Can't conver changeAddress to BaseAddress!");
//   }
//   const stakeCredential = baseAddr?.stake();

//   // const stakeCredential = BaseAddress.from_address(changeAddress).stake_cred();
//   const stakeAddress = RewardAddress.new(networkId, stakeCredential)
//     .to_address();

//   return [stakeAddress.to_hex(), stakeAddress.to_bech32()];
// }
  // let outcome = format!("{addr}_some_stake_address");
  let outcome = format!("_some_stake_address");
  log!("outcome", &outcome);
  JsValue::from(outcome) 
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }
}
