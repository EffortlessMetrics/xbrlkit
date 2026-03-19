---
name: Scenario Activation
about: Activate a BDD scenario for alpha testing
title: '[Activate] '
labels: ['scenario-activation']
assignees: ''

---

## Scenario

- **ID:** 
- **Feature:** 
- **AC ID:** 

## Pre-Activation Checklist

- [ ] `@alpha-active` tag added to feature file
- [ ] `ac_id` added to meta.yaml
- [ ] `profile_pack` added to meta.yaml (if needed)
- [ ] Step handlers implemented in `xbrlkit-bdd-steps`
- [ ] AC assertion added to `scenario-runner`
- [ ] AC added to `ACTIVE_ALPHA_ACS` in `xtask/src/alpha_check.rs`
- [ ] `cargo test --workspace` passes
- [ ] `cargo xtask alpha-check` passes locally

## Post-Merge

- [ ] CI green on main
- [ ] Update HEARTBEAT.md active scenario count

