# WIP-0001: WIP Process and Repository Structure

* **Title:** WIP Process and Repository Structure
* **Author:** bitWASM Contributors
* **Status:** Draft
* **Type:** Process
* **Created:** 2026-03-04

---

# Abstract

This document defines the **WIP (bitWASM Improvement Proposal)** process and the structure of the bitWASM specification repository.

WIPs are the primary mechanism for proposing, discussing, and documenting changes to the bitWASM protocol. This process is modeled after the Bitcoin Improvement Proposal (BIP) process and is intended to provide a clear and transparent framework for protocol evolution.

---

# Motivation

bitWASM is designed to be an **evolvable protocol**.

To ensure that improvements are made in a structured and transparent way, changes must be documented and discussed through a standardized proposal process.

The WIP process provides:

* a canonical place to document protocol changes
* a structured discussion process
* clear versioning of protocol improvements
* a historical record of design decisions

---

# Specification

## WIP Numbering

WIPs are assigned a unique number once accepted into the repository.

Example:

```
WIP-0001
WIP-0002
WIP-0003
```

Numbers are assigned sequentially.

---

## WIP Types

Each WIP must specify a **Type**.

### Consensus

Changes that affect protocol consensus rules.

Examples:

* receipt format changes
* WASM execution rules
* federation signing rules
* container format changes

These proposals require broad review because they affect compatibility between nodes.

---

### Interface

Changes that affect APIs or interaction surfaces.

Examples:

* RPC interfaces
* message formats
* client protocols
* developer tooling standards

---

### Process

Changes that affect governance, development workflow, or the WIP process itself.

Examples:

* repository structure
* proposal lifecycle
* review procedures

---

### Informational

General documentation, guidelines, or explanatory material that does not change the protocol.

Examples:

* threat models
* architectural explanations
* best practices

---

## WIP Status

Each WIP has a **Status** field.

Possible values:

### Draft

The proposal is being developed and discussed.

### Proposed

The proposal is considered mature and ready for review.

### Accepted

The proposal has been accepted for implementation.

### Final

The proposal has been implemented and is considered stable.

### Rejected

The proposal was reviewed but not accepted.

### Superseded

A newer WIP replaces this proposal.

---

# WIP Lifecycle

The lifecycle of a WIP follows these stages.

### 1. Idea

An author proposes a new idea for improving bitWASM.

Discussion may occur in issues, forums, or chat channels.

---

### 2. Draft

The author writes a WIP using the standard template and submits a pull request.

Editors review the proposal for clarity and formatting.

Once merged, the proposal becomes **Draft**.

---

### 3. Discussion

The proposal is discussed by the community.

Authors may revise the document based on feedback.

---

### 4. Proposed

When the proposal appears technically complete and has community support, it may be marked **Proposed**.

---

### 5. Accepted

If maintainers and implementers agree that the proposal should become part of the protocol, it is marked **Accepted**.

---

### 6. Final

After implementations exist and the change is considered stable, the WIP becomes **Final**.

---

# Repository Structure

The bitWASM repository follows a simple structure.

```
bitwasm/
├── README.md
├── LICENSE
├── wips/
│   ├── 0001-process.md
│   ├── 0002-receipt-format.md
│   ├── 0003-wasm-profile.md
│   └── ...
├── reference/
│   ├── node/
│   ├── wasm-runtime/
│   └── tools/
└── docs/
```

---

## Directory Descriptions

### `/wips`

Contains all WIP documents.

Each document is named:

```
0001-process.md
0002-receipt-format.md
0003-wasm-profile.md
```

This directory represents the canonical specification of the protocol.

---

### `/reference`

Reference implementations of core components.

Possible subprojects include:

* reference node
* deterministic WASM runtime
* federation tooling
* test vectors

These implementations are intended to demonstrate the protocol and assist developers building compatible software.

---

### `/docs`

Supporting documentation including:

* architecture guides
* protocol overviews
* developer tutorials

---

# WIP Format

Each WIP should follow a consistent structure:

```
Title
Author
Status
Type
Created

Abstract
Motivation
Specification
Rationale
Backwards Compatibility
Security Considerations
Reference Implementation
```

Not all sections are required for every proposal.

---

# Editors

The bitWASM project maintains a small group of **WIP editors** responsible for:

* assigning WIP numbers
* reviewing formatting
* merging proposals into the repository

Editors do **not** decide whether a proposal is technically correct or should be adopted. Their role is administrative.

Protocol acceptance depends on **community and implementer consensus**.

---

# Backwards Compatibility

The WIP process itself does not define upgrade mechanisms.

Individual consensus WIPs must specify how upgrades affect existing containers, federations, and clients.

---

# Security Considerations

The WIP process ensures that protocol changes are publicly documented and reviewed.

However, acceptance of a WIP does not guarantee security.

Implementers should review each proposal carefully before deploying it in production environments.

---

# Conclusion

The WIP process provides a structured and transparent method for evolving the bitWASM protocol.

By documenting changes through WIPs, the project maintains:

* technical clarity
* historical record
* community participation

This process enables bitWASM to evolve carefully while maintaining a stable and understandable protocol foundation.
