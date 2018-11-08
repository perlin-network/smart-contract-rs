# smart-contract-rs

[Documentation](https://docs.rs/smart-contract)

Write smart contracts for Wavelet in Rust.

### What's a smart contract?

A smart contract is an account operated by a piece of code instead of a private key. Upon activation, a smart contract's code is executed to perform transactions from the contract and modify the state of the contract.

### A simple smart contract on Wavelet

Let's create an example smart contract that always transfers back half the received amount of PERLs to the sender.

Define the transaction & activation payloads:

```rust
// `TransferActivation` is what gets passed into the contract when a `transfer` transaction occurs.
#[derive(Deserialize)]
pub struct TransferActivation {
    pub sender: String,
    pub amount: u64,
}

// `TransferTx` is the payload of a `transfer` transaction.
#[derive(Serialize)]
pub struct TransferTx {
    pub amount: u64,
    pub recipient: String,
}
```

Write the entry point:

```rust
fn handle_activation() {
    // The activation reason can be seen as an immutable global state.
    // We load and decode it in json as `TransferActivation` here.
    let reason: Reason<TransferActivation> = match Reason::load() {
        Some(v) => v,
        None => return, // if the incoming transaction isn't a transfer, do nothing.
    };

    // Create and send a `transfer` transaction back to the sender.
    Transaction::new_json(
        "transfer",
        TransferTx {
            amount: (reason.details.amount + 1) / 2,
            recipient: reason.details.sender,
        },
    )
    .send();
}
```

Register the entry point:

```rust
contract_entry!(handle_activation);
```

### Build

Make sure you have the latest stable Rust toolchain with the `wasm32-unknown-unknown` target installed. If you don't
have the target installed yet, install it with:

```
rustup target add wasm32-unknown-unknown
```

Then, run in your project directory:

```
cargo build --release --target wasm32-unknown-unknown
```

## Contributions

We at Perlin love reaching out to the open-source community and are open to
accepting issues and pull-requests.

For all code contributions, please ensure they adhere as close as possible to
the following guidelines:

1. Fix all warnings from the Rust compiler, unless in some very special cases.
2. Commit messages are in the format `module_name: Change typed down as a sentence.`
   This allows our maintainers and everyone else to know what specific code
   changes you wish to address.
3. Consider backwards compatibility. New methods are perfectly fine, though
   changing the existing public API for example should only be
   done should there be a good reason.

If you...

1. love the work we are doing,
2. want to work full-time with us,
3. or are interested in getting paid for working on open-source projects

... **we're hiring**.

To grab our attention, just make a PR and start contributing.
