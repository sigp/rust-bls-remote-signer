# Remote BLS Signer

## Overview

Simple HTTP BLS signer service.

This service is designed to be consumed by Ethereum 2.0 clients, looking for a more secure avenue to store their BLS12-381 private keys, while running their validators in more permisive and/or scalable environments.

One goal of this package is to be standard compliant, that is, following an API definition published on an Ethereum Improvement Proposal. Please refer to the [wishlist](#wishlist--roadmap) in this very document for a list of advanced features.

## API

### Standard

#### `/upcheck`

* Response
  * Returns `200` if the service is running.

#### `/publicKeys`

* Response
  * Returns `200` and a JSON containing a list of the BLS public keys available.
  * Returns `404` and an empty payload if there are no keys available.

#### `/sign/{public-key}`

* Request
  * A JSON payload with the parameters
    * `signingRoot`: [REQUIRED] A string representation of the hexadecimal value to be signed.
    * `message`: [OPTIONAL] A string contained a serialization of the message to be signed. This field will be ignored by the signer, however, it can be useful in the implementation of middlewares and filters.

* Response
  * Returns `200` and a JSON containing the `signature` field, as a string representation of an hexadecimal value.
  * Returns `404` if there is no private key matching the given public key.
  * Returns `400` on malformed JSON payloads.

## Build instructions

1. [Get Rust](https://www.rust-lang.org/learn/get-started)
2. `make signer`

### Testing

```bash
make test
```
## Running the signer

### Storing the private keys as raw files

* (TODO)

### Command line flags

```
USAGE:
    bls-remote-signer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --debug-level <LEVEL>      The verbosity level for emitting logs. [default: info]  [possible values:
                                   info, debug, trace, warn, error, crit]
        --log-format <FORMAT>      Specifies the format used for logging. [possible values: JSON]
        --logfile <FILE>           File path where output will be written.
        --storage-raw-dir <DIR>    Data directory for private keys in raw files.
```

## TODO

- [ ] Basic implementation
  - [x] Executable boilerplate
  - [ ] Actual HTTP API server
    - [x] Server boilerplate
      - [ ] Allow for configuration of `listen_address` and `port`
    - [x] Implement `/upcheck`
    - [ ] Implement `/publicKeys` (respond NOT IMPLEMENTED)
    - [ ] Implement `/sign/{public-key}` (respond NOT IMPLEMENTED)
  - [ ] Backend
    - [ ] Generic crate
      - [ ] CLI option for `--storage-raw-dir` and pass it to the client
    - [ ] Raw files
      - [ ] Retrieving key from storage
      - [ ] Document how to prepare the raw files in this very README
      - [ ] Implement `/publicKeys` response
      - [ ] Implement `/sign/{public-key}`
        - [ ] Just dump the contents of the key
        - [ ] Perform signature
  - [ ] Complete testing of features
  - [ ] CI/CD pipeline
- [ ] Produce an EIP for the API
  - [ ] Write EIP draft into this repository
  - [ ] Publish EIP github issue
- [ ] Intermediate implementation
  - [ ] Allow to opt between milagro and blst libraries
  - [ ] Zeroize private key after use
  - [ ] Metrics
  - [ ] Benchmarking & Profiling
  - [ ] Release management & Arch builds

## Wishlist / Roadmap

- [ ] EIP standard compliant
- [ ] Support EIP-2335, BLS12-381 keystore
- [ ] Support storage in AWS Cloud HSM
- [ ] Filter by the `message` field
  - [ ] Middleware REST API
  - [ ] Built-in middleware
  - [ ] Flag to enforce the `message` field and compare it to the signing root.
- [ ] TLS/SSL support for requests
- [ ] Authentication by HTTP Header support
- [ ] Confidential computing support (e.g. Intel SGX)

## LICENSE

* Apache 2.0.
