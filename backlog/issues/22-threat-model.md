# Threat model document mapped to tests

**Complexity:** Medium (150 points)
**Labels:** `security`, `documentation`, `complexity: medium`
**Dependencies:** None

## Problem
SECURITY.md lists the security properties, but there's no structured threat model showing who the attackers are, what they can do, and which test proves each defence.

## Scope
- `docs/threat-model.md`: actors (chairperson, voter, outsider, indexer), assets (tallies, eligibility, lifecycle), threats (STRIDE or similar), and for each: mitigation, the test name that covers it, and any residual risk.
- Write up any gaps you find as **new issues** (link them). Don't fix them in this PR.

## Acceptance criteria
- [ ] Every property in SECURITY.md "Security model" is mapped to at least one existing test.
- [ ] Residual risks match the "Known trust assumptions" section, or that section is updated.

## Testing requirements
- None (docs), but the reviewer checks that the referenced tests exist.
