# WIP-0003: VM Profile `wasm32-bitwasm-v0`

* **Title:** VM Profile `wasm32-bitwasm-v0`
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

---

# Abstract

This proposal defines the **`wasm32-bitwasm-v0` virtual machine profile** used by the bitWASM protocol.

The profile specifies a **deterministic subset of WebAssembly** along with execution rules, gas metering, memory limits, and host functions.

All federation nodes must execute WASM contracts using this profile to guarantee that identical inputs produce identical execution results across all nodes.

---

# Motivation

bitWASM relies on deterministic execution to achieve consensus between federation nodes.

Standard WebAssembly environments include features that may introduce non-determinism such as:

* floating point behavior
* system calls
* external state
* timing
* randomness

This proposal defines a restricted execution environment that eliminates sources of non-determinism and provides predictable resource usage through gas metering.

---

# Specification

## VM Identifier

The execution environment defined in this proposal is identified by:

```
vm_id = "wasm32-bitwasm-v0"
```

Receipts referencing this VM profile must include this identifier.

---

# WASM Module Requirements

WASM modules executed under this profile must meet the following constraints:

* Target architecture: **wasm32**
* Deterministic instruction execution
* No access to external environment
* No floating point instructions
* No non-deterministic host functions

Modules must be validated before execution.

---

# Disallowed Features

The following WebAssembly features are **not permitted** in this profile:

### Floating Point

The following instructions are disallowed:

```
f32.*
f64.*
```

Floating point behavior can vary across implementations and therefore must not be used.

---

### Threads

WASM threading features are not allowed.

This includes:

```
atomic.*
memory.atomic.*
```

Execution must remain strictly single-threaded.

---

### WASI System Calls

Standard WASI APIs are not available.

Programs cannot access:

* filesystem
* networking
* system time
* environment variables
* randomness

---

# Deterministic Execution Rules

The VM must enforce the following rules:

1. Execution is **single-threaded**
2. Memory behavior must be deterministic
3. Host functions must produce deterministic outputs
4. Execution must terminate when gas is exhausted

---

# Memory Limits

The VM must enforce a strict memory limit.

Initial limits for v0:

```
max_memory_pages = 256
```

Each WASM page is 64 KiB.

Maximum memory:

```
256 * 64 KiB = 16 MiB
```

Memory growth beyond this limit must cause execution failure.

---

# Stack Limits

The VM must enforce a maximum stack depth.

Suggested limit:

```
max_stack_depth = 1024
```

Exceeding this limit results in execution failure.

---

# Gas Metering

Execution cost is measured in **gas units**.

Each instruction consumes gas.

Example baseline cost model:

| Instruction Type   | Gas Cost |
| ------------------ | -------- |
| Integer arithmetic | 1        |
| Memory load/store  | 3        |
| Control flow       | 1        |
| Function call      | 5        |
| Memory growth      | 50       |

Gas metering must be deterministic.

If gas consumption exceeds the limit provided in the execution request, execution must halt and fail.

---

# Execution Inputs

Contracts receive input through a deterministic byte array.

Example interface:

```
execute(state_bytes, input_bytes)
```

Where:

```
state_bytes = previous contract state
input_bytes = invocation input
```

Both values must be identical across all nodes.

---

# Execution Outputs

Execution must produce:

```
(state_bytes_new, outputs, gas_used)
```

Where:

* `state_bytes_new` is the updated contract state
* `outputs` is a deterministic representation of settlement outputs
* `gas_used` is the gas consumed during execution

These values are included in the execution receipt.

---

# Host Functions

The VM provides a minimal set of deterministic host functions.

### SHA256

```
sha256(data: bytes) -> bytes32
```

Computes the SHA256 hash of the provided data.

---

### Verify Signature

```
verify_signature(pubkey, message, signature) -> bool
```

Verifies a Schnorr signature.

---

### Read Input

```
read_input() -> bytes
```

Returns the invocation input provided during execution.

---

### Read State

```
read_state() -> bytes
```

Returns the previous contract state.

---

### Write State

```
write_state(bytes)
```

Sets the new contract state.

---

# Deterministic Serialization

State and output data must use canonical serialization.

Recommended encoding:

```
CBOR
```

or another deterministic encoding defined in future WIPs.

All nodes must use identical encoding rules.

---

# Failure Conditions

Execution fails if any of the following occur:

* invalid WASM module
* use of disallowed instructions
* gas limit exceeded
* memory limit exceeded
* stack limit exceeded
* host function misuse

Failed executions must not produce a valid receipt.

---

# Backwards Compatibility

This proposal defines the **initial VM profile**.

Future WIPs may define additional profiles such as:

```
wasm32-bitwasm-v1
wasm64-bitwasm-v1
```

Existing contracts referencing `wasm32-bitwasm-v0` must continue to operate unchanged.

---

# Security Considerations

Deterministic execution is critical for consensus.

Implementations must ensure:

* strict WASM validation
* consistent instruction cost model
* identical serialization behavior

VM implementations should be carefully audited.

---

# Reference Implementation

A reference VM should include:

* WASM validation
* deterministic runtime
* gas metering instrumentation
* host function interface

Reference implementations may be based on existing WASM runtimes adapted for deterministic execution.

---

# Conclusion

The `wasm32-bitwasm-v0` profile defines a deterministic WebAssembly environment suitable for consensus-critical execution within the bitWASM protocol.

By restricting WebAssembly features and enforcing strict execution rules, federation nodes can independently execute contracts and produce identical results.
