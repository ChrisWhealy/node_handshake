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

* `mx.jamestrev.com`
* `mail.saxrag.com`
* `seed.btc.petertodd.org`
* `seed.bitcoin.sipa.be`
* `dnsseed.bluematt.me`
* `seed.bitcoinstats.com`
* `seed.bitcoin.jonasschnelli.ch`

If a port number is not supplied, the default is `8333`.

If a timeout is not supplied, the default is 5 seconds.
This timeout value applies to all network requests including DNS name resolution.

## Possible Output

The supplied DNS name will, mostly likely, resolve to multiple IP addresses.
However, the availability of a responsive P2P node at each of these IP addresses is uncertain; therefore each time you run this program against the same DNS seed node, you may receive differing results.

```shell
$ export RUST_LOG=info
$ cargo run seed.bitcoin.jonasschnelli.ch
   Compiling node-handshake v0.1.0 (/Users/chris/Developer/Eiger/node-handshake)
    Finished dev [unoptimized + debuginfo] target(s) in 3.12s
     Running `target/debug/node-handshake seed.bitcoin.jonasschnelli.ch`
2024-03-15T09:04:43.726311Z  INFO node_handshake::dns_name_resolver: seed.bitcoin.jonasschnelli.ch resolves to 24 IP addresses

2024-03-15T09:04:43.726417Z  INFO node_handshake::handshake: Attempting handshake with 209.177.138.245:8333
2024-03-15T09:04:43.726446Z  INFO node_handshake::handshake::bitcoin: Connecting to 209.177.138.245:8333
2024-03-15T09:04:43.947861Z  INFO node_handshake::handshake::send_message: VERSION: Sending 70001 (127 bytes) to target node 209.177.138.245:8333
2024-03-15T09:04:43.947985Z  INFO node_handshake::handshake::send_message: VERSION: Sent
2024-03-15T09:04:44.180276Z  INFO node_handshake::handshake::bitcoin: VERSION: Target node accepts messages up to version 70016
2024-03-15T09:04:44.180375Z  INFO node_handshake::handshake::send_message: VERACK: Sending 24 bytes to 209.177.138.245:8333
2024-03-15T09:04:44.180476Z  INFO node_handshake::handshake::send_message: VERACK: Sent
2024-03-15T09:04:44.180547Z  INFO node_handshake::handshake::bitcoin: VERACK received
2024-03-15T09:04:44.180652Z  INFO node_handshake::handshake: Handshake with 209.177.138.245:8333 succeeded

2024-03-15T09:04:44.180681Z  INFO node_handshake::handshake: Attempting handshake with 45.44.213.123:8333
2024-03-15T09:04:44.180698Z  INFO node_handshake::handshake::bitcoin: Connecting to 45.44.213.123:8333
2024-03-15T09:04:44.295680Z  INFO node_handshake::handshake::send_message: VERSION: Sending 70001 (127 bytes) to target node 45.44.213.123:8333
2024-03-15T09:04:44.295782Z  INFO node_handshake::handshake::send_message: VERSION: Sent
2024-03-15T09:05:04.137053Z  WARN node_handshake::handshake::bitcoin: VERSION: Message took 19841 ms to arrive
2024-03-15T09:05:04.137094Z  INFO node_handshake::handshake::bitcoin: VERSION: Target node accepts messages up to version 70016
2024-03-15T09:05:04.137131Z  INFO node_handshake::handshake::send_message: VERACK: Sending 24 bytes to 45.44.213.123:8333
2024-03-15T09:05:04.137219Z  INFO node_handshake::handshake::send_message: VERACK: Sent
2024-03-15T09:05:04.137261Z  INFO node_handshake::handshake::bitcoin: VERACK received
2024-03-15T09:05:04.137320Z  INFO node_handshake::handshake: Handshake with 45.44.213.123:8333 succeeded

2024-03-15T09:05:04.137339Z  INFO node_handshake::handshake: Attempting handshake with 114.216.118.251:8333
2024-03-15T09:05:04.137351Z  INFO node_handshake::handshake::bitcoin: Connecting to 114.216.118.251:8333
2024-03-15T09:05:04.398325Z ERROR node_handshake::handshake: Handshake with 114.216.118.251:8333 failed: IO ERROR: Connection refused (os error 61)

2024-03-15T09:05:04.399417Z  INFO node_handshake::handshake: Attempting handshake with 92.249.179.185:8333
2024-03-15T09:05:04.399439Z  INFO node_handshake::handshake::bitcoin: Connecting to 92.249.179.185:8333
2024-03-15T09:05:04.627841Z  INFO node_handshake::handshake::send_message: VERSION: Sending 70001 (127 bytes) to target node 92.249.179.185:8333
2024-03-15T09:05:04.628000Z  INFO node_handshake::handshake::send_message: VERSION: Sent
2024-03-15T09:05:04.673873Z  INFO node_handshake::handshake::bitcoin: VERSION: Target node accepts messages up to version 70016
2024-03-15T09:05:04.673935Z  INFO node_handshake::handshake::send_message: VERACK: Sending 24 bytes to 92.249.179.185:8333
2024-03-15T09:05:04.674000Z  INFO node_handshake::handshake::send_message: VERACK: Sent
2024-03-15T09:05:04.674041Z  INFO node_handshake::handshake::bitcoin: VERACK received
2024-03-15T09:05:04.674095Z  INFO node_handshake::handshake: Handshake with 92.249.179.185:8333 succeeded

...
```

## Testing

A test run is available in `main.rs` that attempts to handshake with a set preconfigured DNS seed nodes.

However, since the results returned by a P2P handshake are entirely variable, it is not possible to use `assert!()` to check for an expected result.
Hence, this is not a `#[test]` in the traditional Rust sense.

To see the output of this test run, you must tell `cargo` not to capture IO written to `stdout`:

```shell
$ cargo test -- --nocapture
```
