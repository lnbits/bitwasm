# WIP-0400: Pseudo-Covenant Transaction Trees

* **Title:** Pseudo-Covenant Transaction Trees
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

## Abstract

This proposal introduces **pseudo-covenant transaction trees** to constrain possible state transitions without requiring new Bitcoin opcodes.

Pseudo-covenants are implemented using **pre-signed transaction structures**.

## Motivation

Bitcoin currently lacks native covenant functionality.

Pseudo-covenants allow limited covenant-like behavior by pre-committing future transaction paths.

## Specification

At container creation time, a set of **pre-signed transactions** may be generated representing valid future state transitions.

Example structure:

```
container_0
   ├─ tx_1 → container_1
   ├─ tx_2 → withdrawal
   └─ tx_3 → federation upgrade
```

Each transaction is pre-signed by required parties.

Nodes may only follow pre-committed paths.

## Advantages

Pseudo-covenants allow:

* restricted settlement paths
* improved safety against theft
* enforceable contract logic

## Limitations

Transaction trees must be generated in advance.

Complex contract logic may require large trees.

Future Bitcoin covenant proposals may simplify this design.