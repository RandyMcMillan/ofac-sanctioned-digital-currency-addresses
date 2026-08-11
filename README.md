# OFAC Sanctioned Digital Currency Addresses

Tool to extract the 'sanctioned' Bitcoin (and other Digital Currency assets)
addresses from the [Specially Designated Nationals (SDN) list][1] of the US
Office of Foreign Asset Control. An XML version of this list can be downloaded
here: [`sdn_advanced.xml`][2].

[1]: https://home.treasury.gov/policy-issues/financial-sanctions/specially-designated-nationals-and-blocked-persons-list-sdn-human-readable-lists
[2]: https://www.treasury.gov/ofac/downloads/sanctions/1.0/sdn_advanced.xml

As of December 2023 the tool covers the following assets. There might be assets
on the SDN list that aren't covered yet. These can be found by grepping for
"Digital Currency Address" on the `sdn_advanced.xml` file. Feel free to submit
an issue or pull-request adding assets.

- XBT (Bitcoin)
- ETH (Ethereum)
- XMR (Monero)
- LTC (Litecoin)
- ZEC (ZCash)
- DASH (Dash)
- BTG (Bitcoin Gold)
- ETC (Ethereum Classic)
- BSV (Bitcoin Satoshi Vision)
- BCH (Bitcoin Cash)
- XVG (Verge)
- USDC (USD Coin)
- USDT (USD Tether)
- XRP (Ripple)
- TRX (Tron)
- ARB (Arbitrum)
- BSC (Binance Smart Chain)
- SOL (Solana)

The sanctioned addresses can be extracted with this tool from the
[`sdn_advanced.xml`][2] file. The tool supports the following output formats:
- `TXT` file format (one address per line)
- `JSON` file containing a list of addresses

## Automatically Updated Lists

The [`lists`](/tree/lists) branch of this repository contains automatically
updated lists of sanctioned addresses for each covered asset. These are
generated each night at 0 UTC by a GitHub Actions workflow.

## Usage Examples

### Rust CLI

``` console
$ cargo build
$ cargo build --release
$ cargo run -- fetch -o sdn_advanced.xml
$ cargo run -- XBT ETH -sdn sdn_advanced.xml -f TXT JSON -path ./out
$ cargo doc --open
```

`cargo run` uses the Rust implementation in `src/lib.rs` and `src/main.rs`.
This README covers the Rust CLI and project overview. See [PYTHON.md](PYTHON.md)
for Python usage examples and script-specific documentation.

## License and Warranty

This software is provided under the MIT License. For details see [LICENSE](LICENSE).

The author does not provide warranty of any kind. Especially not on the
completeness and correctness of the produced lists.
