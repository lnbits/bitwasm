# WIP-0005: Container Format v0

* **Title:** Container Format v0
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

---

# Abstract

This proposal defines the **bitWASM container format (v0)**.

A container represents the **Bitcoin-settled state holder for a bitWASM contract**. Containers are implemented as Bitcoin UTXOs that commit to the current contract state and execution policy.

Each container acts as the settlement anchor for a contract. When a contract executes, the container is spent and replaced with a new container reflecting the updated state.

---

# Motivation

bitWASM requires a mechanism to anchor contract state and funds to Bitcoin.

Containers provide:

* a Bitcoin-native settlement anchor
* a verifiable commitment to contract state
* a mechanism for state transitions
* a way to enforce federation authorization

By representing contracts as UTXOs, bitWASM inherits the security and finality properties of Bitcoin.

---

# Specification

## Container Overview

A container is a Bitcoin UTXO that commits to a specific contract state.

Conceptually:

```id="n96tk1"
container = {
    value_sats
    contract_commitment
    federation_id
    policy_id
}
```

The container is spent when the contract executes and replaced with a new container reflecting the updated state.

---

# Container Fields

The container commitment contains the following fields.

| Field             | Type    | Description                                            |
| ----------------- | ------- | ------------------------------------------------------ |
| `bitwasm_version` | uint16  | Protocol version                                       |
| `vm_id`           | string  | Execution environment identifier                       |
| `wasm_hash`       | bytes32 | Hash of the contract WASM code                         |
| `state_root`      | bytes32 | Current contract state root                            |
| `federation_id`   | bytes32 | Identifier of the federation responsible for execution |
| `policy_id`       | bytes32 | Identifier for execution policy parameters             |

---

# Container Commitment

The container commitment is serialized and hashed.

```id="sod4jq"
container_commitment = SHA256(serialized_container)
```

This commitment represents the current contract state.

---

# Bitcoin Representation

Containers are implemented as **Bitcoin Taproot outputs**.

Example structure:

```id="6gypov"
scriptPubKey = P2TR(federation_threshold_key)
```

The Taproot key represents the aggregated federation signing key.

The container commitment is included in the **Taproot tweak or associated metadata**.

Implementation options include:

* Taproot key tweak
* OP_RETURN commitment
* witness metadata

Future WIPs may standardize the exact encoding.

---

# Container Lifecycle

## Container Creation

A container is created when a contract is deployed.

Example:

```id="rbb0ip"
Bitcoin TX
    output → container UTXO
```

The container includes:

* contract WASM hash
* initial state root
* federation identifier

---

## Container Execution

When a contract executes:

1. Federation nodes run the WASM program.
2. An execution receipt is produced.
3. The container UTXO is spent.
4. A new container UTXO is created with the updated state root.

Example transition:

```id="knj1ho"
container(state_root_0)
    ↓ execution
container(state_root_1)
```

---

# Settlement Outputs

Execution may produce settlement outputs.

Example:

```id="zqzbjd"
container UTXO
    ↓
recipient payment
federation fee
new container
```

All settlement outputs must match the `outputs_hash` defined in the execution receipt.

---

# Policy Identifier

The `policy_id` field references a policy configuration.

Policies may define:

* gas pricing
* execution limits
* federation upgrade rules
* timeout parameters

Policies are defined in future WIPs.

---

# Container Serialization

Container fields must be serialized in the following order.

```id="sx3cbd"
bitwasm_version
vm_id
wasm_hash
state_root
federation_id
policy_id
```

Integers must be encoded as **big-endian**.

Strings must be encoded as:

```id="0ehq2a"
length || bytes
```

Binary fields are fixed-length byte arrays.

---

# Container Identification

Containers may be referenced using the Bitcoin UTXO identifier.

Example:

```id="30ox7g"
container_id = txid:vout
```

This identifier uniquely locates the container on the Bitcoin blockchain.

---

# State Transitions

A valid container transition must satisfy:

1. The container UTXO is spent
2. The execution receipt is valid
3. Federation signatures are valid
4. The new container commitment matches the receipt state root

Nodes must reject invalid transitions.

---

# Backwards Compatibility

This proposal defines **Container Format v0**.

Future WIPs may introduce:

* new container fields
* covenant-style constraints
* enhanced settlement commitments

The `bitwasm_version` field allows nodes to interpret containers correctly.

---

# Security Considerations

Containers hold contract funds and state commitments.

Potential risks include:

* federation collusion
* invalid state transitions
* incorrect commitment encoding

Implementations must ensure:

* strict container validation
* correct receipt verification
* deterministic serialization

Future WIPs may introduce additional protections such as:

* escape hatches
* challenge mechanisms
* covenant-based state enforcement

---

# Reference Implementation

Reference implementations should include:

* container serialization library
* Taproot output construction
* container validation logic
* state transition verification

Test vectors should ensure consistent container commitments across implementations.

---

# Conclusion

Containers provide the Bitcoin settlement anchor for bitWASM contracts.

By representing contract state as Bitcoin UTXOs, bitWASM inherits the security, immutability, and settlement guarantees of the Bitcoin blockchain.
