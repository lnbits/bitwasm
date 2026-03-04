# WIP-0200: Public Execution Log and Equivocation Proofs

* **Title:** Public Execution Log and Equivocation Proofs
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

## Abstract

This proposal introduces a **public execution log** for bitWASM receipts and defines mechanisms for detecting and proving **federation equivocation**.

Equivocation occurs when federation members sign conflicting receipts for the same container state.

## Motivation

Federations must not be able to produce conflicting execution results.

A public log allows:

* transparency
* conflict detection
* reputation tracking
* future slashing mechanisms

## Specification

Federation nodes must broadcast signed receipts to a **public log network**.

The log must allow anyone to verify:

```
container_id
state_root_before
receipt_hash
signatures
```

If two receipts exist such that:

```
state_root_before is identical
AND
state_root_after differs
```

then equivocation has occurred.

Proof of equivocation consists of:

```
receipt_A
receipt_B
conflicting signatures
```

Such proofs can be publicly verified.

## Security Considerations

Equivocation proofs allow detection of malicious behavior and may be used by future WIPs for:

* slashing
* federation removal
* reputation penalties