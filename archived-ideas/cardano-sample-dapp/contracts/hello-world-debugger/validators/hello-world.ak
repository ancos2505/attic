use aiken/bytearray
use aiken/cbor
use aiken/hash.{Blake2b_224, Hash}
use aiken/list
use aiken/string
use aiken/transaction.{
  OutputReference, ScriptContext, Spend, Transaction, TransactionId,
}
use aiken/transaction/credential.{VerificationKey}

// Program
type Datum {
  owner: Hash<Blake2b_224, VerificationKey>,
}

type Redeemer {
  msg: ByteArray,
}

validator {
  fn hello_world(
    datum: Datum,
    redeemer: Redeemer,
    context: ScriptContext,
  ) -> Bool {
    // Debug Helpers
    let extra_signatories: String =
      cbor.diagnostic(context.transaction.extra_signatories)
    // Tracing
    trace string.concat(
      @"TRACE:",
      @"\n",
    )
      |> string.concat(@"- redeemer.msg: ")
      |> string.concat(string.from_bytearray(redeemer.msg))
      |> string.concat(
          @" , \n",
        )
      |> string.concat(@"- datum.owner: ")
      |> string.concat(bytearray.to_hex(datum.owner))
      |> string.concat(
          @" , \n",
        )
      |> string.concat(@"- extra_signatories: ")
      |> string.concat(extra_signatories)
    // Check Redeemer.msg
    let must_say_hello = (redeemer.msg == "Hello, World!")?
    // Check Datum.owner
    let must_be_signed =
      list.has(context.transaction.extra_signatories, datum.owner)?
    // * All checks must be true
    must_say_hello && must_be_signed
  }
}

// Tests
test hello_world_example() {
  let datum =
    Datum { owner: #"00000000000000000000000000000000000000000000000000000000" }
  // TODO: ` If you do want to enforce some side-effects, use expect with a
  // TODO: discard _other_datum instead of let.`
  let unknown =
    #"e7aaf0cb183fc92fed6ebdc9bf49d40899499406e6bb3598735e1dbe373a2817"
  let redeemer = Redeemer { msg: "Hello, World!" }
  let placeholder_utxo =
    OutputReference { transaction_id: TransactionId(""), output_index: 0 }
  let context =
    ScriptContext {
      purpose: Spend(placeholder_utxo),
      transaction: transaction.placeholder()
        |> fn(transaction) {
            Transaction {
              ..transaction,
              extra_signatories: [datum.owner, unknown],
            }
          },
    }
  hello_world(datum, redeemer, context)
}
