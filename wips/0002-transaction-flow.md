# WIP-0006: Transaction Flow

* **Title:** Transaction Flow
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Informational
* **Created:** 2026-03-04

---

User
 │
 │ deploy
 ▼
Container UTXO
 │
 │ invoke
 ▼
Federation executes WASM
 │
 │ produce receipt
 ▼
Threshold signature
 │
 ▼
Bitcoin settlement tx
 │
 ▼
New container state

# Abstract

This document describes the **end-to-end transaction flow** of the bitWASM protocol.

It explains how containers, WASM execution, receipts, and Bitcoin settlement interact during the lifecycle of a contract.

The transaction flow connects the primitives defined in:

* WIP-0002: Receipt Format
* WIP-0003: VM Profile (`wasm32-bitwasm-v0`)
* WIP-0004: Federation Identity and Threshold Signatures
* WIP-0005: Container Format

This document provides a high-level operational overview for implementers.

---

# Overview

The bitWASM protocol follows a simple lifecycle:

1. Contract deployment
2. Container creation
3. Contract invocation
4. WASM execution
5. Receipt generation
6. Federation signing
7. Bitcoin settlement
8. Container update

Conceptually:

```
Bitcoin
   │
   │ deposit
   ▼
Container (state_root_0)
   │
   │ contract call
   ▼
Federation executes WASM
   │
   │ deterministic result
   ▼
Execution Receipt
   │
   │ threshold signature
   ▼
Bitcoin settlement
   │
   ▼
Container (state_root_1)
```

---

# Step 1: Contract Deployment

A contract begins when a user publishes a WASM program and deploys a container.

Deployment includes:

* WASM bytecode
* `wasm_hash`
* initial contract state
* selected federation
* initial funding in satoshis

Example:

```
deploy(
  wasm_code,
  initial_state,
  federation_id,
  funding_sats
)
```

The WASM code must conform to the VM profile defined in **WIP-0003**.

---

# Step 2: Container Creation

A Bitcoin transaction creates the initial container UTXO.

The container commits to:

* `bitwasm_version`
* `vm_id`
* `wasm_hash`
* `state_root`
* `federation_id`
* `policy_id`

Example:

```
Bitcoin TX
   input → user funds
   output → container UTXO
```

The container becomes the settlement anchor for the contract.

---

# Step 3: Contract Invocation

A user submits a contract invocation request.

Invocation includes:

* container identifier
* input parameters
* gas limit
* maximum fee

Example:

```
invoke(
  container_id,
  input_data,
  gas_limit
)
```

The invocation request is broadcast to federation nodes.

---

# Step 4: WASM Execution

Federation nodes execute the contract deterministically using the VM profile defined in **WIP-0003**.

Execution input:

```
execute(
  state_bytes,
  input_bytes
)
```

Execution produces:

```
state_bytes_new
outputs
gas_used
```

All nodes must produce identical results.

---

# Step 5: Receipt Generation

After execution, nodes construct an **execution receipt**.

Receipt fields are defined in **WIP-0002**.

Example receipt:

```
receipt = {
  bitwasm_version
  vm_id
  wasm_hash
  state_root_before
  input_hash
  state_root_after
  outputs_hash
  gas_used
  fee_sats
  expiry_height
  federation_id
}
```

The receipt is serialized and hashed:

```
receipt_hash = SHA256(serialized_receipt)
```

---

# Step 6: Federation Signing

Federation nodes verify that their execution result matches the receipt.

Nodes then participate in **threshold signing** as defined in **WIP-0004**.

Example:

```
signature = threshold_sign(receipt_hash)
```

The result is a single aggregated signature proving federation agreement.

---

# Step 7: Bitcoin Settlement

The container UTXO is spent in a Bitcoin transaction.

Settlement outputs must match the `outputs_hash` committed in the receipt.

Example settlement transaction:

```
container UTXO
   │
   ├─ user payment output
   ├─ federation fee output
   └─ new container UTXO
```

The new container contains the updated state commitment.

---

# Step 8: Container Update

The settlement transaction creates the next container state.

```
container(state_root_0)
     │
     ▼
container(state_root_1)
```

This new container becomes the active contract state.

Future invocations repeat the same process.

---

# Execution Failure

Execution fails if:

* WASM execution fails
* gas limit is exceeded
* federation nodes disagree on results
* receipt validation fails

In this case:

* no receipt is produced
* no settlement transaction occurs
* the container remains unchanged

---

# Determinism Requirements

All nodes must produce identical results.

Determinism is guaranteed through:

* the VM profile defined in **WIP-0003**
* canonical receipt hashing defined in **WIP-0002**
* deterministic serialization rules

If nodes produce different results, execution must abort.

---

# Security Model

The transaction flow relies on the following assumptions:

* federation nodes execute contracts honestly
* threshold signatures represent federation consensus
* container commitments correctly anchor state to Bitcoin

Additional protections are introduced in later WIPs:

* WIP-0100: Escape hatches
* WIP-0200: Public logs
* WIP-0300: Federation replacement
* WIP-0400: Pseudo-covenants
* WIP-0500: Optimistic challenges

---

# Conclusion

This document defines the operational lifecycle of a bitWASM contract.

By combining deterministic WASM execution with Bitcoin-settled containers and federated consensus, bitWASM enables programmable systems that remain anchored to Bitcoin's security and settlement guarantees.
