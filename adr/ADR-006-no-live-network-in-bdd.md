# ADR-006: No live network in BDD

## Decision

BDD and focused acceptance runs use locked local corpora and synthetic fixtures only. Live SEC network access is reserved for explicit sync or oracle lanes.
