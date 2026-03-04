# WIP-0100: Escape Hatch and Timeout Mechanism

* **Title:** Escape Hatch and Timeout Mechanism
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Consensus
* **Created:** 2026-03-04

## Abstract

This proposal introduces an **escape hatch mechanism** that allows users to recover funds from a container if the federation becomes unavailable, malicious, or refuses to cooperate.

The mechanism relies on **timelocked Bitcoin script paths** that allow unilateral recovery after a predefined timeout.

## Motivation

Federations provide execution and settlement services, but they must not be able to permanently lock funds.

Users must always have a **liveness guarantee** ensuring that funds can be recovered without federation cooperation.

## Specification

Each container must include an **alternative spend path** allowing withdrawal after a timeout.

Example conceptual script:

```
IF federation_signature
    spend normally
ELSE IF blockheight >= timeout_height
    user_key spends
ENDIF
```

The timeout value must be defined by the container's policy.

Typical values may range from:

```
timeout = 144 blocks (≈1 day)
timeout = 1008 blocks (≈1 week)
```

## Security Considerations

Timeout values must balance:

* user safety
* federation operational flexibility

Timeouts that are too short may allow premature withdrawal attempts.
