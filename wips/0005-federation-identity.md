# WIP-0004: Federation Identity and Threshold Signature Scheme

* **Title:** Federation Identity and Threshold Signature Scheme
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

---

# Abstract

This proposal defines the **federation identity model** and the **threshold signature scheme** used in the bitWASM protocol.

Federations are responsible for executing WASM contracts, producing execution receipts, and authorizing settlement transactions.

A federation is identified by a deterministic identifier derived from its member public keys and threshold parameters. Federation nodes cooperate to produce **threshold signatures** over receipt hashes and settlement transactions.

---

# Motivation

bitWASM relies on federated execution rather than global mining or proof-of-stake.

Federations provide:

* deterministic contract execution
* receipt verification
* settlement authorization
* execution fee distribution

To ensure interoperability and verifiability, federations must be identified in a deterministic way and must use a standardized threshold signing scheme.

---

# Specification

## Federation Overview

A **federation** is a group of nodes that collectively execute contracts and sign receipts.

Federation parameters include:

* member public keys
* signature threshold
* federation identifier

Example:

```id="eqo7s1"
members = [pk1, pk2, pk3, pk4, pk5]
threshold = 3
```

Any subset of three members can jointly produce a valid federation signature.

---

# Federation Identity

Each federation is assigned a deterministic identifier.

The `federation_id` is defined as:

```id="c87cmq"
federation_id = SHA256(
    threshold ||
    member_pubkeys
)
```

Where:

* `threshold` is encoded as a 16-bit integer
* `member_pubkeys` is the ordered list of public keys

Public keys must be sorted lexicographically before hashing.

This ensures all nodes compute the same identifier.

---

# Federation Member Keys

Federation members must use **Schnorr public keys** compatible with Bitcoin Taproot.

Key format:

```id="5lkkpf"
32 byte x-only public key
```

This allows direct compatibility with Taproot signing and aggregation.

---

# Threshold Signature Scheme

Federations must use a **threshold Schnorr signature scheme**.

Supported schemes include:

* MuSig2
* FROST
* ROAST-compatible threshold signing

The exact implementation may vary as long as the resulting signature is a valid Schnorr signature over the message.

---

# Signature Target

Federation signatures are created over the **receipt hash**.

Example:

```id="1y1ycd"
message = receipt_hash
signature = threshold_sign(message)
```

The signature proves that the federation has agreed on the execution result.

---

# Signature Aggregation

Threshold signatures must produce a **single aggregated signature**.

Example output:

```id="njd4x3"
signature = 64 bytes
```

This allows the signature to be directly used in Bitcoin Taproot transactions.

---

# Federation Quorum

A signature is valid if at least `threshold` members participated in the signing process.

Example:

```id="dqqjpw"
members = 10
threshold = 7
```

At least 7 federation members must cooperate to produce the signature.

---

# Federation Membership Changes

Federations may evolve over time.

Membership changes must result in a **new federation identity**.

Example:

```id="yjgajy"
old federation_id
    ↓
new federation_id
```

Contracts referencing a federation must explicitly reference the correct `federation_id`.

Federation upgrade mechanisms may be defined in future WIPs.

---

# Receipt Signing

The federation signs the receipt hash defined in WIP-0002.

Example:

```id="9uhtqr"
receipt_hash = SHA256(serialized_receipt)
signature = threshold_sign(receipt_hash)
```

The resulting signature is attached to the receipt.

---

# Settlement Authorization

The same federation threshold key may also authorize settlement transactions.

Example:

```id="7z32k6"
bitcoin_tx_signature = threshold_sign(tx_hash)
```

This enables federations to sign Bitcoin transactions that settle contract execution results.

---

# Federation Node Responsibilities

Each federation node must:

* validate WASM execution
* verify deterministic results
* compute receipt hashes
* participate in threshold signing
* broadcast execution receipts

Nodes must refuse to sign receipts that do not match the canonical specification.

---

# Federation Communication

Federation nodes must communicate to coordinate signing rounds.

Communication protocols are implementation-specific but may include:

* peer-to-peer gossip
* threshold signing coordination channels
* execution result propagation

Future WIPs may standardize federation communication protocols.

---

# Backwards Compatibility

Federation identity and signature schemes are tied to the `federation_id`.

If a federation changes:

* members
* threshold
* signature scheme

a new `federation_id` must be created.

Existing contracts must explicitly migrate to the new federation.

---

# Security Considerations

Federations represent a trust assumption in the bitWASM system.

Potential risks include:

* federation collusion
* key compromise
* denial of service

To mitigate these risks:

* thresholds should be high enough to tolerate faults
* keys should be stored securely
* nodes should operate independently

Future WIPs may introduce additional safeguards such as:

* federation rotation
* equivocation proofs
* slashing mechanisms

---

# Reference Implementation

Reference implementations should include:

* federation configuration format
* deterministic federation ID generation
* threshold signature integration
* receipt signing workflow

Test vectors should verify that federation IDs and signatures are computed consistently.

---

# Conclusion

This proposal defines how federations are identified and how they produce signatures in the bitWASM protocol.

By standardizing federation identity and threshold signatures, nodes can verify execution results and settlement authorization in a consistent and interoperable way.
