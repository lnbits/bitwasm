# WIP-0500: Optimistic Withdrawal Challenges

* **Title:** Optimistic Withdrawal Challenges
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

## Abstract

This proposal introduces **optimistic challenge mechanisms** for validating withdrawals.

Withdrawals may be proposed optimistically by the federation but remain challengeable for a fixed time window.

This mechanism is inspired by **optimistic rollups and BitVM-style verification games**.

## Motivation

Federations may attempt to submit invalid withdrawals.

Allowing anyone to challenge withdrawals improves security without requiring full on-chain verification.

## Specification

Withdrawal process:

1. Federation proposes withdrawal transaction.
2. Transaction enters **challenge period**.
3. Any participant may submit a challenge if the withdrawal violates the receipt.

Example timeline:

```
T0 withdrawal proposed
T0 + N blocks challenge window
T0 + N blocks withdrawal finalizes
```

If a valid challenge is submitted:

```
withdrawal is cancelled
proof is recorded
```

Challenge proofs must demonstrate:

```
receipt mismatch
invalid state transition
invalid settlement outputs
```

Future WIPs may define interactive dispute protocols.

## Security Considerations

Challenge windows must be long enough to allow independent verification.

Watchtowers and monitoring services are recommended to detect invalid withdrawals.