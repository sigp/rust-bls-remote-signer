# Remote BLS Signer

## Overview

Simple HTTP BLS signer service.

This service is designed to be consumed by Ethereum 2.0 clients, looking for a more secure avenue to store their BLS12-381 secret keys, while running their validators in more permisive and/or scalable environments.

One goal of this package is to be standard compliant, that is, following an API definition published on an Ethereum Improvement Proposal. Please refer to the [wishlist](#wishlist--roadmap) in this very document for a list of advanced features.

## API

### Standard

#### `GET /upcheck`

* Response
  * Returns `200` and a JSON payload containing `{"status": "OK"}` if the service is running.

#### `GET /publicKeys`

* Response
  * Returns `200` and a JSON payload containing a list of the BLS public keys available.
  * Returns `404` and an empty JSON payload if there are no keys available.
  * Returns `500` on server errors.

#### `POST /sign/:public_key`

* Request
  * A JSON payload with the parameters
    * `signingRoot`: [REQUIRED] A string representation of the hexadecimal value to be signed.
    * Any other field will be **ignored**
      * To allow for the installing of filter enhancements and/or plugins.
      * A limit on the size of the payload will be implemented as a control.

* Response
  * Returns `200` and a JSON containing the `signature` field, as a string representation of an hexadecimal value.
  * Returns `404` if there is no secret key matching the given public key.
  * Returns `400` on bad requests:
    * Malformed JSON requests.
    * Missing or invalid field `signingRoot`.
    * Invalid request path.
    * Invalid `:public_key` parameter.
  * Returns `500` on server errors:
    * Storage errors.
    * Invalid secret key retrieved.
    * Key pair mismatch.

## Build instructions

1. [Get Rust](https://www.rust-lang.org/learn/get-started).
2. Execute `make`.
3. The binary `bls-remote-signer` will most likely be found in `./target/release`.

### Testing

```bash
make test
```
## Running the signer

### Storing the secret keys as raw files

* Steps to store a secret key
  * Choose an empty directory, as the backend will parse every file looking for keys.
  * Create a file named after the **hex representation of the public key without 0x**.
  * Write the **hex representation of the secret key without 0x**.
  * Store the file in your chosen directory.
  * Use this directory as a command line parameter (`--storage-raw-dir`)

### Command line flags

```
USAGE:
    bls-remote-signer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --debug-level <LEVEL>         The verbosity level for emitting logs. [default: info]  [possible values:
                                      info, debug, trace, warn, error, crit]
        --listen-address <ADDRESS>    The address to listen for TCP connections. [default: 0.0.0.0]
        --log-format <FORMAT>         Specifies the format used for logging. [possible values: JSON]
        --logfile <FILE>              File path where output will be written.
        --port <PORT>                 The TCP port to listen on. [default: 9000]
        --storage-raw-dir <DIR>       Data directory for secret keys in raw files.
```

## TODO

Please, check this repository's issue for the [Implementation Tracking](https://github.com/sigp/rust-bls-remote-signer/issues/1)

## Wishlist / Roadmap

- [ ] EIP standard compliant
- [ ] Support EIP-2335, BLS12-381 keystore
- [ ] Support storage in AWS Cloud HSM
- [ ] Route with the `warp` library
- [ ] Filter by the `message` field
  - [ ] Middleware REST API
  - [ ] Built-in middleware
  - [ ] Flag to enforce the `message` field and compare it to the signing root
- [ ] TLS/SSL support for requests
- [ ] Authentication by HTTP Header support
- [ ] Confidential computing support (e.g. Intel SGX)

## LICENSE

* Apache 2.0.
