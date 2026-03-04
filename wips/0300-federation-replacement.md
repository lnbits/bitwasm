# WIP-0300: Federation Replacement Rules

* **Title:** Federation Replacement Rules
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

## Abstract

This proposal defines mechanisms for replacing the federation responsible for executing a container.

Federation replacement ensures that contracts remain operational even if the original federation becomes unreliable or unavailable.

## Motivation

Federations must not become permanent authorities.

Users must be able to migrate contracts to new federations over time.

## Specification

Containers may transition to a new federation by executing a **federation upgrade transaction**.

Example transition:

```
container(federation_A)
        ↓
container(federation_B)
```

Conditions for federation replacement:

1. A valid execution receipt authorizing the transition
2. Signatures from the current federation
3. Updated container commitment referencing the new federation_id

Future WIPs may introduce **forced federation replacement** mechanisms triggered by timeout or equivocation.

## Security Considerations

Federation upgrades must ensure:

* state continuity
* receipt validity
* correct policy inheritance