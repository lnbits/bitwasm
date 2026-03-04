# bitWASM 

Statechain + WASM = bitcoin smart contracts 🧡

**bitWASM** is a protocol for running deterministic **WASM programs secured by Bitcoin settlement**.

It allows developers to publish a piece of WASM code, attach some sats, and have a federation of nodes execute the program in a deterministic environment. The result is agreed by the federation and settled using Bitcoin transactions.

bitWASM is designed as a **minimal, modular protocol** that can evolve over time through **WIPs (bitWASM Improvement Proposals)**.

The goal is to create a new programmable layer around Bitcoin that is:

* simple
* deterministic
* modular
* upgradeable
* Bitcoin-native

---

# Why bitWASM?

Bitcoin is the most secure monetary settlement system ever created, but it intentionally keeps its scripting system limited.

At the same time, developers increasingly want programmable systems for things like:

* marketplaces
* escrow services
* auctions
* prediction markets
* automated settlement
* distributed applications

Most existing smart contract systems solve this by creating entirely new blockchains.

bitWASM takes a different approach.

Instead of competing with Bitcoin as a new base layer, bitWASM treats Bitcoin as the **money layer**, while computation happens in a **deterministic WASM environment executed by a federation**.

Bitcoin remains the final settlement layer.

---

# The Core Idea

A bitWASM contract consists of three simple parts:

1. **WASM Code**

   A deterministic WebAssembly program that defines how the contract behaves.

2. **Execution**

   A federation of nodes runs the same WASM program with the same inputs.
   Because the environment is deterministic, all nodes produce the same result.

3. **Settlement**

   The federation signs the execution result and settles the outcome using Bitcoin transactions.

In short:

```
Bitcoin = settlement layer
WASM = execution layer
Federation = consensus layer
```

This separation keeps the system simple while enabling powerful functionality.

---

# Why WASM?

WebAssembly is an ideal execution environment because it is:

* deterministic
* portable
* sandboxed
* efficient
* supported by many programming languages

Developers can write contracts in languages such as:

* Rust
* Go
* AssemblyScript
* C/C++

and compile them into WASM.

bitWASM defines a strict deterministic WASM profile so every node produces identical execution results.

---

# The Role of Federations

bitWASM uses federated execution.

Federation nodes:

* execute WASM contracts
* verify deterministic results
* sign execution receipts
* share execution fees

Federations provide fast and efficient execution while Bitcoin provides settlement security.

The protocol is designed so federations can be **replaced or upgraded over time**.

---

# WIPs – bitWASM Improvement Proposals

The protocol evolves through **WIPs**.

WIPs define:

* protocol changes
* VM updates
* execution rules
* security improvements
* new features

This allows bitWASM to grow in a transparent and collaborative way, similar to how Bitcoin evolves through BIPs.

---

# Design Principles

bitWASM is guided by several core principles.

### Minimal base protocol

The initial protocol focuses on a small, stable core that can be extended through WIPs.

### Deterministic execution

All contracts must produce identical results on every node.

### Bitcoin settlement

Bitcoin remains the final source of truth for funds.

### Modular architecture

Execution, settlement, and consensus are separate layers.

### Evolvable protocol

Future improvements can be introduced without breaking existing contracts.

---

# What bitWASM Enables

bitWASM opens the door to a new class of Bitcoin-native applications:

* decentralized marketplaces
* programmable escrow
* auctions and trading systems
* prediction markets
* distributed services
* automation paid in sats

You can think of bitWASM as something like:

**"serverless computing for Bitcoin."**

Publish a program.
Attach some sats.
Let the network execute it.

---

# Current Status

bitWASM is currently in the early design phase.

The project aims to produce:

* a reference specification
* a reference node implementation
* a deterministic WASM runtime
* federation tooling
* a WIP standards process

---

# Contributing

Contributions are welcome.

The project will develop openly through WIPs and reference implementations.

If you are interested in:

* Bitcoin infrastructure
* distributed systems
* WASM runtimes
* protocol design

we encourage you to participate.

---

# License

MIT
