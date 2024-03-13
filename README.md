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
$ cargo run mx.jamestrev.com
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/node-handshake mx.jamestrev.com`
2024-03-13T12:01:39.868791Z  INFO node_handshake::dns_name_resolver: mx.jamestrev.com resolves to 1 IP address

2024-03-13T12:01:39.868845Z  INFO node_handshake::handshake: Attempting handshake with 217.169.14.90:8333
2024-03-13T12:01:39.868858Z  INFO node_handshake::handshake::bitcoin: Connecting to 217.169.14.90:8333
2024-03-13T12:01:39.895768Z  INFO node_handshake::handshake::send_message: VERSION: Sending 70001 (127 bytes) to target node 217.169.14.90:8333
2024-03-13T12:01:39.895825Z  INFO node_handshake::handshake::send_message: VERSION: Sent
2024-03-13T12:01:39.927494Z  INFO node_handshake::handshake::bitcoin: VERSION: Target node accepts messages up to version 70016
2024-03-13T12:01:39.927528Z  INFO node_handshake::handshake::send_message: VERACK: Sending 24 bytes to 217.169.14.90:8333
2024-03-13T12:01:39.927561Z  INFO node_handshake::handshake::send_message: VERACK: Sent
2024-03-13T12:01:39.927600Z  INFO node_handshake::handshake::bitcoin: VERACK received
2024-03-13T12:01:39.927628Z  INFO node_handshake::handshake: Handshake with 217.169.14.90:8333 succeeded
```
