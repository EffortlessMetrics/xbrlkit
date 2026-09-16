# Hosted CLA Assistant governance

`EffortlessMetrics/xbrlkit` uses **CLA Assistant from `cla-assistant.io`**, the hosted GitHub App. It does not use CLA Assistant Lite or repository-hosted CLA workflow code.

## Activation state

This migration is staged until the hosted service is linked and proven on its pull request. Before merge, maintainers must create the public Gist from the reviewed agreement and metadata payloads, install and link the hosted App, observe a successful `license/cla` status from that App, replace the pending Gist fields in [`cla-assistant-source.json`](cla-assistant-source.json), and require the status on the default branch with CLA Assistant pinned as its expected source.

## Agreement source

The App is linked to a public Gist containing `CLA.md` and `metadata`. The Gist `CLA.md` and repository [`CLA.md`](../../CLA.md) must be byte-identical. The staged custom-field payload is retained in [`cla-assistant-metadata.json`](cla-assistant-metadata.json). The activation state, exact Gist URL and revision, and content hashes are recorded in [`cla-assistant-source.json`](cla-assistant-source.json).

The required custom fields are full legal name, email address, and an acknowledgement that the signer is acting in an individual capacity and has authority to grant the stated rights.

## Corporate contributions

The hosted form is the individual flow. A contributor whose employer or another entity owns or controls the relevant rights must not sign on the entity's behalf through that form. Corporate contributions require a separate written agreement and authorization process handled privately by the maintainers.

A corporate contribution does not create a false individual signature and does not put a human account or employer organization on the CLA Assistant allowlist. After a Corporate CLA is executed, the private authorization record must identify the entity, agreement version, covered GitHub usernames, scope, and effective date.

The corporate merge path is an audited exception to the CLA rule only:

1. enforce the App-pinned `license/cla` check in a dedicated default-branch CLA ruleset, separate from the repository's baseline branch ruleset;
2. make a dedicated `cla-corporate-approvers` team the only `pull_request`-mode bypass actor on that CLA-only ruleset;
3. require an approver to verify the private Corporate CLA record before bypassing the CLA ruleset for a specific pull request; and
4. retain a private receipt containing the entity, agreement version, covered usernames, pull request, approver, timestamp, and reason.

All ordinary pull-request, review, CI, deletion, and non-fast-forward rules remain enforced. Until the Corporate CLA and the narrowly scoped exception path are both configured, an entity-owned contribution cannot merge.

## Enforcement

The migration pull request is the initial test pull request. It must first receive a successful `license/cla` status from the hosted App. A dedicated default-branch CLA ruleset must then require that context and pin its expected source to CLA Assistant. Keep the existing baseline ruleset unchanged and without new bypass actors. The CLA-only ruleset starts with no bypass actor; add only the dedicated corporate-approval team if the corporate process described above is activated.

The CLA Assistant allowlist starts empty. A bot may be exempted only after its contribution path is shown to be controlled and attributable to the project. Do not use that allowlist for collaborators, organization members, or corporate contributors.

## Evidence retention

Maintainers export the signature register privately when the CLA changes and at release or governance-freeze checkpoints. The retained evidence set includes the exported register, Gist payload and revision, repository source record, ruleset JSON, and checksums. Personal signature data is not committed to this repository.

Changing the agreement or `metadata` creates a new version. Update the repository and Gist in one governed change, update the source record, expect contributors to re-sign, and take a fresh private export.

See [`cla-privacy.md`](cla-privacy.md) for the contributor privacy notice.

## CLA Assistant Lite predecessor

Before this migration, `.github/workflows/cla.yml` ran `contributor-assistant/github-action@v2.6.1` with `pull_request_target` and write permissions. Its public predecessor record is captured in [`cla-lite-predecessor.json`](cla-lite-predecessor.json). The `cla-signatures` branch and its ledger commit remain retained as historical evidence; they are not imported as equivalent hosted signatures because that record did not collect the new required fields.
