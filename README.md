# P2P node handshake

_Spec v3 (2023-03-30)_

Pick a publicly available P2P node (e.g. a blockchain one) implementation - which itself doesn't need to be written in Rust - and write a [network handshake](https://en.wikipedia.org/wiki/Handshaking) for it in Rust, and instructions on how to test it.

## Requirements

- Both the target node and the handshake code should compile at least on Linux.
- The solution has to perform a full **protocol-level** (post-TCP/etc.) handshake with the target node.
- The provided **instructions** should include information on how to verify that the handshake has concluded.
- The solution can not depend on the code of the target node (but it can share some of its dependencies).
- The submitted code can not reuse entire preexisting handshake implementations like `libp2p_noise/XX`.

### Non-requirements

- A lot of parameters can potentially be exchanged during the handshake, but only the mandatory ones need to be included.
- The solution can ignore any post-handshake traffic from the target node, and it doesn't have to keep the connection alive.

## Evaluation

- **Quality**: the solution should be idiomatic and adhere to Rust coding conventions.
- **Performance**: the solution should be as fast as the handshake protocol allows, and it shouldn't block resources.
- **Security**: the network is an inherently untrusted environment, and it should be taken into account.
- **Minimalism**: any number of dependencies can be used, but they should be tailored to the task.
- **Versatility**: the more broadly applicable the solution is (bi-directional, unfixed values, etc.), the better.
- **Uniqueness**: bonus points for non-Bitcoin implementations.

## Usage

```shell
$ cargo run <dns_seed_name> <port number> <timeout in secs>
```

Suggested DNS seed names include:

* ` mx.jamestrev.com`
* `mail.saxrag.com`
* `showy-toys.aeza.network`
* `nickjlock.plus.com`
* `seed.btc.petertodd.org`
* `seed.bitcoin.sipa.be`
* `dnsseed.bluematt.me`
* `seed.bitcoinstats.com`
* `seed.bitcoin.jonasschnelli.ch`

If a port number is not supplied, the default is `8333`.

If a timeout is not supplied, the default network request timeout is 5 seconds.

## Possible Output

The supplied DNS name will, mostly likely, resolve to multiple IP addresses.
However, the availability of a responsive P2P node at each of these IP addresses is uncertain; therefore each time you run this program against the same DNS seed node, it is very likely that you will receive differing results.

```shell
$ export RUST_LOG=info
$ cargo run seed.bitcoin.jonasschnelli.ch
   Compiling node-handshake v0.1.0 (/Users/chris/Developer/Eiger/node-handshake)
    Finished dev [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/node-handshake seed.bitcoin.jonasschnelli.ch`
2024-03-15T14:46:46.035849Z  INFO node_handshake::dns_name_resolver: seed.bitcoin.jonasschnelli.ch resolves to 24 IP addresses

2024-03-15T14:46:46.036019Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (184.96.188.234) -> NOT STARTED
2024-03-15T14:46:46.036054Z  INFO node_handshake::handshake::bitcoin: Handshake with seed.bitcoin.jonasschnelli.ch (184.96.188.234) -> STARTED
2024-03-15T14:46:51.036891Z ERROR node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (184.96.188.234) -> FAILED: TCP connection timed out

2024-03-15T14:46:51.036940Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (123.30.249.234) -> NOT STARTED
2024-03-15T14:46:51.036955Z  INFO node_handshake::handshake::bitcoin: Handshake with seed.bitcoin.jonasschnelli.ch (123.30.249.234) -> STARTED
2024-03-15T14:46:56.037021Z ERROR node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (123.30.249.234) -> FAILED: TCP connection timed out

2024-03-15T14:46:56.037065Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (143.110.147.13) -> NOT STARTED
2024-03-15T14:46:56.037081Z  INFO node_handshake::handshake::bitcoin: Handshake with seed.bitcoin.jonasschnelli.ch (143.110.147.13) -> STARTED
2024-03-15T14:46:56.197124Z  INFO node_handshake::handshake::send_message: version: Sending 70001 (127 bytes) to target node 143.110.147.13:8333
2024-03-15T14:46:56.197289Z  INFO node_handshake::handshake::send_message: version: Sent
2024-03-15T14:46:56.378334Z  INFO node_handshake::handshake::bitcoin: version: Target node accepts messages up to version 70015
2024-03-15T14:46:56.378403Z  INFO node_handshake::handshake::send_message: verack: Sending 24 bytes to 143.110.147.13:8333
2024-03-15T14:46:56.378458Z  INFO node_handshake::handshake::send_message: verack: Sent
2024-03-15T14:46:56.378553Z  INFO node_handshake::handshake::bitcoin: verack received
2024-03-15T14:46:56.378658Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (143.110.147.13) -> SUCCESS

...snip...

2024-03-15T14:47:44.498947Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (186.137.174.198) -> NOT STARTED
2024-03-15T14:47:44.498961Z  INFO node_handshake::handshake::bitcoin: Handshake with seed.bitcoin.jonasschnelli.ch (186.137.174.198) -> STARTED
2024-03-15T14:47:44.753985Z ERROR node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (186.137.174.198) -> FAILED: TCP Connection refused (os error 61)

2024-03-15T14:47:44.754051Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (65.35.192.46) -> NOT STARTED
2024-03-15T14:47:44.754074Z  INFO node_handshake::handshake::bitcoin: Handshake with seed.bitcoin.jonasschnelli.ch (65.35.192.46) -> STARTED
2024-03-15T14:47:49.754653Z ERROR node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (65.35.192.46) -> FAILED: TCP connection timed out

2024-03-15T14:47:49.754745Z  INFO node_handshake::handshake: Summary of handshakes to seed.bitcoin.jonasschnelli.ch
2024-03-15T14:47:49.754769Z  INFO node_handshake::handshake:    Success = 11
2024-03-15T14:47:49.754786Z  INFO node_handshake::handshake:    Partial = 0
2024-03-15T14:47:49.754802Z  INFO node_handshake::handshake:    Failed  = 13
```

## Testing

A test run is available in `main.rs` that attempts to handshake with a set preconfigured DNS seed nodes.

However, since the results returned by a P2P handshake are entirely variable, it is not possible to use `assert!()` to check for an expected result.
Hence, this is not a `#[test]` in the traditional Rust sense.

To see the output of this test run, you must tell `cargo test` not to capture IO written to `stdout`:

```shell
$ cargo test -- --nocapture
```
