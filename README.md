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
$ cargo run <dns_seed_name> <port number> <timeout in milliseconds>
```

Suggested DNS seed names include:

* `mx.jamestrev.com`
* `mail.saxrag.com`
* `showy-toys.aeza.network`
* `nickjlock.plus.com`
* `seed.btc.petertodd.org`
* `seed.bitcoin.sipa.be`
* `dnsseed.bluematt.me`
* `seed.bitcoinstats.com`
* `seed.bitcoin.jonasschnelli.ch`

If a port number is not supplied, the default is `8333`.

If a timeout is not supplied, the default network request timeout is 5000 milliseconds.

## Possible Output

The supplied DNS name will, mostly likely, resolve to multiple IP addresses.
However, the availability of a responsive P2P node at each of these IP addresses is uncertain; therefore each time you run this program against the same DNS seed node, it is very likely that you will receive differing results.

```shell
$ export RUST_LOG=info
$ cargo run seed.bitcoin.jonasschnelli.ch
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/node-handshake seed.bitcoin.jonasschnelli.ch`
2024-03-18T15:13:47.213902Z  INFO node_handshake: Attempting handshake(s) with seed.bitcoin.jonasschnelli.ch:8333  Timeout = 5000 ms
2024-03-18T15:13:47.287732Z  INFO node_handshake::dns_name_resolver: seed.bitcoin.jonasschnelli.ch resolves to 24 IP addresses

2024-03-18T15:13:47.287853Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (79.204.1.107) -> STARTED
2024-03-18T15:13:47.336974Z  INFO node_handshake::handshake::btc_message: version: Sending 127 bytes to 79.204.1.107:8333
2024-03-18T15:13:47.337144Z  INFO node_handshake::handshake::btc_message: version: Sent
2024-03-18T15:13:47.394395Z  INFO node_handshake::handshake::btc_message: version: Target node accepts messages up to version 70016
2024-03-18T15:13:47.394474Z  INFO node_handshake::handshake::btc_message: verack: Sending 24 bytes to 79.204.1.107:8333
2024-03-18T15:13:47.394553Z  INFO node_handshake::handshake::btc_message: verack: Sent
2024-03-18T15:13:47.398562Z  WARN node_handshake::handshake::btc_message: Target node skipped verack.  Instead got Alert([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 127, 0, 0, 0, 0, 255, 255, 255, 127, 254, 255, 255, 127, 1, 255, 255, 255, 127, 0, 0, 0, 0, 255, 255, 255, 127, 0, 255, 255, 255, 127, 0, 47, 85, 82, 71, 69, 78, 84, 58, 32, 65, 108, 101, 114, 116, 32, 107, 101, 121, 32, 99, 111, 109, 112, 114, 111, 109, 105, 115, 101, 100, 44, 32, 117, 112, 103, 114, 97, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 0])
2024-03-18T15:13:47.398643Z  WARN node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (79.204.1.107) -> PARTIAL

2024-03-18T15:13:47.398671Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (91.190.121.45) -> STARTED
2024-03-18T15:13:52.399450Z ERROR node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (91.190.121.45) -> FAILED: TCP connection timed out

...snip...

2024-03-18T15:14:12.404300Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (37.60.239.26) -> STARTED
2024-03-18T15:14:12.462803Z  INFO node_handshake::handshake::btc_message: version: Sending 127 bytes to 37.60.239.26:8333
2024-03-18T15:14:12.462905Z  INFO node_handshake::handshake::btc_message: version: Sent
2024-03-18T15:14:12.485318Z  INFO node_handshake::handshake::btc_message: version: Target node accepts messages up to version 70016
2024-03-18T15:14:12.485367Z  INFO node_handshake::handshake::btc_message: verack: Sending 24 bytes to 37.60.239.26:8333
2024-03-18T15:14:12.485419Z  INFO node_handshake::handshake::btc_message: verack: Sent
2024-03-18T15:14:12.486083Z  INFO node_handshake::handshake::btc_message: verack received
2024-03-18T15:14:12.486148Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (37.60.239.26) -> SUCCESS

2024-03-18T15:14:12.486174Z  INFO node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (37.60.225.80) -> STARTED
2024-03-18T15:14:12.511101Z  INFO node_handshake::handshake::btc_message: version: Sending 127 bytes to 37.60.225.80:8333
2024-03-18T15:14:12.511195Z  INFO node_handshake::handshake::btc_message: version: Sent
2024-03-18T15:14:12.531556Z  INFO node_handshake::handshake::btc_message: version: Target node accepts messages up to version 70016
2024-03-18T15:14:12.531606Z  INFO node_handshake::handshake::btc_message: verack: Sending 24 bytes to 37.60.225.80:8333
2024-03-18T15:14:12.531667Z  INFO node_handshake::handshake::btc_message: verack: Sent
2024-03-18T15:14:12.551808Z  WARN node_handshake::handshake::btc_message: Target node skipped verack.  Instead got Ping(2736944585018940862)
2024-03-18T15:14:12.551883Z  WARN node_handshake::handshake: Handshake with seed.bitcoin.jonasschnelli.ch (37.60.225.80) -> PARTIAL

...snip...

2024-03-18T15:14:47.318848Z  INFO node_handshake::handshake: Summary of handshakes with seed.bitcoin.jonasschnelli.ch
2024-03-18T15:14:47.318889Z  INFO node_handshake::handshake:    Success = 2
2024-03-18T15:14:47.318908Z  INFO node_handshake::handshake:    Partial = 9
2024-03-18T15:14:47.318925Z  INFO node_handshake::handshake:    Failed  = 13
```

## DNS Name Resolution Failure

If DNS is unable to resolve the name of a seed node, you will see output similar to the following:

```shell
cargo run showy-toys.aeza.network
    Finished dev [unoptimized + debuginfo] target(s) in 1.43s
     Running `target/debug/node-handshake showy-toys.aeza.network`
2024-03-18T15:24:31.936206Z  INFO node_handshake: Attempting handshake(s) with showy-toys.aeza.network:8333  Timeout = 5000 ms
2024-03-18T15:24:32.460327Z ERROR node_handshake::dns_name_resolver: DNS Name resolution error: no record found for Query { name: Name("showy-toys.aeza.network."), query_type: AAAA, query_class: IN }
2024-03-18T15:24:32.460397Z ERROR node_handshake::handshake: Hand shake(s) with showy-toys.aeza.network not possible
```

## Testing

An integration test can be run against a set of DNS Seed Node names that in total, resolve to around 120 IP addresses.
However, since the results returned by a P2P handshake are entirely variable, it is not possible to `assert!()` that any particular value should or should not be received.

To see the output of this test run, you must tell `cargo test` not to capture IO written to `stdout`:

```shell
$ cargo test -- --nocapture
```
