# TON Read-only Probe (v1)

A deliberately small first-stage probe for the TON recovery inventory. It uses the existing `tonlibjson-client` implementation in this repository.

## Scope

- Validate a public TON raw or user-friendly address locally.
- Connect using the repository default TON mainnet global configuration.
- Check lite-server readiness.
- Fetch one account-state snapshot.
- Print the response for diagnostic inspection.

## Run

From the repository root:

```sh
TON_ACCOUNT_ADDRESS='0:<64-hex-character-address>' cargo run -p ton-readonly-probe --release
```

A user-friendly TON address may also be supplied.

## Safety boundary

This probe only calls `ready()` and `get_account_state()`. It does not call message submission, signing, wallet transfer, or contract mutation methods. It does not request or read seed phrases, private keys, or signing material.

The printed Rust debug representation is diagnostic output, not yet a normalized recovery observation. Do not import it as verified asset evidence without parsing, source metadata, acquisition timestamps, and reconciliation.

## Known limitations

- This first probe checks a single account-state snapshot only.
- Jetton master discovery, Jetton balances, NFT discovery, transaction pagination, retries with completeness accounting, and Supabase persistence are intentionally separate milestones.
- The `tonlibjson-sys` native build requirements have not yet been validated on Android Termux. Run the compile/test gate before using it there.
- The probe currently uses the default mainnet configuration. No testnet or custom endpoint is selected.

## Acceptance gate

1. `cargo test -p ton-readonly-probe` passes.
2. `cargo run -p ton-readonly-probe` validates an address and completes the read-only account-state call.
3. The report records whether the account is active, uninitialized, or otherwise represented by the upstream response; no status is inferred from a failed request.
4. No write/signing API is invoked.
