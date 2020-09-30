---
eip: <to be assigned>
title: BLS remote signer HTTP API standard.
author: Herman Junge <herman@sigmaprime.io>
discussions-to: <URL>
status: Draft
type: Standards Track
category: ERC
created: 2020-09-30
---

## Simple Summary
This EIP defines the HTTP API standard of the BLS remote signer, consumed by validator clients to sign proposals and attestations of blocks.

## Abstract
A validator client contributes to the consensus of the blockchain by signing proposals and attestations of blocks, using a BLS secret key which must be available to this client.

The BLS remote signer API is designed to be consumed by validator clients, looking for a more secure avenue to store their BLS12-381 secret keys, enabling them to run in more permissive and scalable environments.

## Motivation
Ethereum 2.0 utilizes BLS12-381 signatures.

Consensus in the blockchain is created by the proposal and attestation of blocks from the validator clients, using a BLS secret key which must be available each time a signature is made: That is, at least once every epoch (6.4 minutes), during a small window of time within this epoch, given by the duty of the validator at random.

While there is not a directive on the Ethereum 2.0 specification on where this BLS secret key should be, leaving this detail to the client implementer, it is inferred that such assets must be in the same node where the validator client is executed.

This assumption must be sufficient in the use case where the validator client is run in a physically secure network (i.e. nobody, but the operator, has a chance to log-in into the node), as such configuration would only allow _outbound_ calls from the validator client. In this situation, only a physical break-in will allow an attacker to either have arbitrary access to the storage or to the memory of the device.

There are, however, use cases where it is required by the operator to run a validator node in less strict security constraints, as the ones given by a cloud provider. Notwithstanding any security expectation, nothing prevents a rogue operator from gaining arbitrary access to the assets running inside a node.

The situation is not better when the requirement is to execute the validators by leveraging a container orchestration solution. The handling of secret keys across nodes can become a burden both from an operational as well as a security perspective.

The proposed solution comprises running a specialized node with exclusive access to the secret keys, listening to a simple API (to be defined at the [Specification](#specification) section), and returning the requested signatures. Operators working under this schema must utilize clients with the adequate enhancement supporting the consumption of this API.

The focus of this specification is the supply of BLS signatures on demand. The aspects of authentication, key management (creation, update, and deletion), pre-image validation, and transport encryption are discussed in the [Rationale](#rationale) of this document. Moreover, there is further recognition of these matters at the threat model at the [Security](#security-considerations) section.

## Specification

### `GET /upcheck`

_**Responses**_

Success | <br>
--- | ---
Code | `200`
Content | `{"status": "OK"}`

---

### `GET /publicKeys`

_**Responses**_

Success | <br>
--- | ---
Code | `200`
Content | `{"public_keys": "[public_key_hex_string_without_0x]"}`

_or_

Error | <br>
--- | ---
Code | `404`
Content | `{"error": "No keys found in storage."}`

_or_

Error | <br>
--- | ---
Code |  `500`
Content|  `{"error": "<Server Error Message>"}`

---

### `POST /sign/:public_key`

URL Parameter | <br>
--- | ---
`:public_key` | `public_key_hex_string_without_0x`

_**Request**_

JSON Body | <br> | <br>
--- | --- | ---
`signingRoot` | **Required** | A string representation of the hexadecimal value to be signed
<br> | Optional | Any other field will be ignored by the signer

_**Responses**_

Success | <br>
--- | ---
Code |  `200`
Content | `{"signature": "<signature_hex_string>"}`

_or_

Error | <br>
--- | ---
Code |  `400`
Content | `{"error": "<Bad Request Error Message>"}`

_or_

Error | <br>
--- | ---
Code |  `404`
Content | `{"error": "Key not found: <public_key_hex_string_without_0x>"}`

_or_

Error | <br>
--- | ---
Code |  `500`
Content | `{"error": "<Server Error Message>"}`

---

## Rationale

### URL parameter `:public-key` without `0x`

By relaxing the constraint of using an hexadecimal, clients can opt-in to using custom ids for their private keys, and perform the adequate retrieving and matching with the secret key within the signer. This is, in the measure the remote signer implementer allows for this feature.

### Unix philosophy: Simple API

This API specification contains only three methods: One for **status**, one for **listing the available keys**, and one to **perform a signature**. There are no methods for authentication, pre-image validation, key management, nor encryption in-transit.

The following subsections discuss aspects to being considered by the client implementers relative to these subjects.

#### Authentication

Can be accomplished by either prepending to the API, or adding a middleware to the client implementation the validation of an HTTP Request Header.

There are several ways to negotiate and issue a valid token to communicate the validator client with the remote signer, each of them not without drawbacks, from replay attacks, to the problem of distributing the credential to the validator client. In general, any method of authentication should be combined with encryption in transit to be succesful.

The operator can also implement ACL rules between the networks of the validator client and the remote signer, mitigating the threat to the one where the adversary needs to be in the actual client network to perform an attack.

#### Pre-image validation

A key feature for a remote signer, pre-image validation implies that not only the `signingRoot`, but all the required elements needed to perform complete validation of the message, are sent through the wire to obtain a signature

Can be accomplished by either prepending to the API, or adding a middleware to the client implementation a control that parses the message. There is no breaking of this document API specification, as any other field different from `signingRoot` will be ignored by the remote signer.

Implementers should address the additional requirements emerging for each specific validation, such as, slashing protection, as this entails the needs to manage a database and the mechanisms to update it. Also new threats need to be addressed and controlled, among them, attackers looking into tamper the source of data.

#### Key management

There are several ways to store secret keys, namely Hardware security modules (HSM), Secrets management applications (e.g. Hashicorp Vault), cloud storage with tight private network ACL rules, or even raw files in a directory. In general the remote signer implementers will abstract the HTTP API from the storage medium.

Is in this perspective that any procedure to create, update, or delete keys should be worked apart from the client implementation.

#### Encription in-transit

This feature can be accomplished by either prepending to the API, or adding a middleware to the client implementation.

If the operator is working with self-signed certificates, it is required that the client enhancement consuming the remote signer allows this option.

## Test Cases

### Test Data

* BLS Pair
  * Public key: `0xb7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a`.
  * Secret key: `0x68081afeb7ad3e8d469f87010804c3e8d53ef77d393059a55132637206cc59ec`.
* Signing root: `0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c`.
* Expected signature: `0xb5d0c01cef3b028e2c5f357c2d4b886f8e374d09dd660cd7dd14680d4f956778808b4d3b2ab743e890fc1a77ae62c3c90d613561b23c6adaeb5b0e288832304fddc08c7415080be73e556e8862a1b4d0f6aa8084e34a901544d5bb6aeed3a612`.


### `GET /upcheck`

```bash
# Success

## Request
curl -v localhost:9000/upcheck

## Response
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET /upcheck HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 15
< date: Wed, 30 Sep 2020 02:25:08 GMT
<
* Connection #0 to host localhost left intact
{"status":"OK"}

```

### `GET /publicKeys`

```bash
# Success

## Request
curl -v localhost:9000/publicKeys

## Response
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET /publicKeys HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 116
< date: Wed, 30 Sep 2020 02:25:36 GMT
<
* Connection #0 to host localhost left intact
{"public_keys":["b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a"]}

# No Keys Available

## Preparation
## Delete the file

## Request
curl -v localhost:9000/publicKeys

## Response
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET /publicKeys HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 404 Not Found
< content-length: 37
< date: Wed, 30 Sep 2020 02:27:05 GMT
<
* Connection #0 to host localhost left intact
{"error":"No keys found in storage."}


# Server Error

## Preparation
## `chmod` keys directory to the octal 311 (-wx--x--x).

## Request
curl -v localhost:9000/publicKeys

## Response
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> GET /publicKeys HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 500 Internal Server Error
< content-length: 43
< date: Wed, 30 Sep 2020 02:26:09 GMT
<
* Connection #0 to host localhost left intact
{"error":"Storage error: PermissionDenied"}


```

### `POST /sign/:public_key`

```bash
# Success

## Request
curl -v -X POST -d '{"signingRoot":"0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c"}' -H 'Content-Type: application/json' localhost:9000/sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a

## Response
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> POST /sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 84
>
* upload completely sent off: 84 out of 84 bytes
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 210
< date: Wed, 30 Sep 2020 02:16:02 GMT
<
* Connection #0 to host localhost left intact
{"signature":"0xb5d0c01cef3b028e2c5f357c2d4b886f8e374d09dd660cd7dd14680d4f956778808b4d3b2ab743e890fc1a77ae62c3c90d613561b23c6adaeb5b0e288832304fddc08c7415080be73e556e8862a1b4d0f6aa8084e34a901544d5bb6aeed3a612"}

# Bad Request Error

## Request
curl -v -X POST -d '{"signingRoot":"0xaa1"}' -H 'Content-Type: application/json' localhost:9000/sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a

## Response
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> POST /sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 23
>
* upload completely sent off: 23 out of 23 bytes
* Mark bundle as not supporting multiuse
< HTTP/1.1 400 Bad Request
< content-length: 38
< date: Wed, 30 Sep 2020 02:15:05 GMT
<
* Connection #0 to host localhost left intact
{"error":"Invalid signingRoot: 0xaa1"}

# No Keys Available

## Request
curl -v -X POST -d '{"signingRoot":"0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c"}' -H 'Content-Type: application/json' localhost:9000/sign/000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

## Response
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> POST /sign/000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 84
>
* upload completely sent off: 84 out of 84 bytes
* Mark bundle as not supporting multiuse
< HTTP/1.1 404 Not Found
< content-length: 123
< date: Wed, 30 Sep 2020 02:18:53 GMT
<
* Connection #0 to host localhost left intact
{"error":"Key not found: 000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"}

# Server Error

## Preparation
## `chmod` both keys directory and file to the octal 311 (-wx--x--x).
## `chmod` back to 755 to delete them afterwards.

## Request
curl -v -X POST -d '{"signingRoot":"0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c"}' -H 'Content-Type: application/json' localhost:9000/sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a

## Response
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:9000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 9000 (#0)
> POST /sign/b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a HTTP/1.1
> Host: localhost:9000
> User-Agent: curl/7.68.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 84
>
* upload completely sent off: 84 out of 84 bytes
* Mark bundle as not supporting multiuse
< HTTP/1.1 500 Internal Server Error
< content-length: 43
< date: Wed, 30 Sep 2020 02:21:08 GMT
<
* Connection #0 to host localhost left intact
{"error":"Storage error: PermissionDenied"}
```

## Implementation

Repository Url | Language | Organization | Commentary
--- | --- | --- | ---
[BLS Remote Signer](https://github.com/sigp/rust-bls-remote-signer) | Rust | Sigma Prime | Supports proposed specification.
[Web3signer](https://github.com/PegaSysEng/web3signer) | Java | PegaSys | Supports proposed specification, although with [slightly different methods](https://pegasyseng.github.io/web3signer/web3signer-eth2.html):<br>{`/sign` => `/api/v1/eth2/sign`, `/publicKeys` => `/api/v1/eth2/publicKeys`}.

The Prysm client supports a [Remote Signing Wallet](https://docs.prylabs.network/docs/wallet/remote/), however its API requires using gRPC as transport.

## Security Considerations

### Threat model

Let's consider the following threats and their mitigations:

Threat | Mitigation(s)
--- | ---
An attacker can spoof the validator client. | See the discussion at [Authentication](#authentication).
An attacker can send a crafted message to the signer. | See discussion at  [Pre-image validation](#pre-image-validation).
An attacker can create, update, or delete secret keys. | Keys are not to be writable via any interface of the remote signer.
An attacker can repudiate a sent message. | Implement logging in the signer. Enhance it by sending logs to a syslog box.
An attacker can disclose the contents of a private key by retrieving the key from storage. | Storage in Hardware security module (HSM).<br>_or_<br>Storage in Secrets management applications (e.g. Hashicorp Vault).
An attacker can eavesdrop on the uploading of a secret key. | Upload the keys using a secure channel, based on each storage specification.
An attacker can eavesdrop on the retrieval of a key from the remote signer. | Always pass the data between storage and remote signer node using a secure channel.
An attacker can dump the memory in the remote signer to disclose a secret key. |  Prevent physical access to the node running the remote signer.<br>_or_<br>Prevent access to the terminal of the node runnig the remote signer: Logs being sent to a syslog box. Deployments triggered by a simple, non-parameterized API.<br>_or_<br>Implement zeroization of the secret key at memory.<br>_or_<br>Explore the compilation and running of the remote signer in a Trusted execution environment (TEE).
An attacker can DoS the remote signer. | Mitigation left to the implementer.

## Copyright
Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
