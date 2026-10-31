#!/usr/bin/env bash
# Creates GitHub labels and issues from backlog/issues/*.md using the gh CLI.
# Default is a dry run. Pass --publish to actually create them.
set -euo pipefail

REPO="${REPO:-milah-247/votecontract}"
PUBLISH=false
[[ "${1:-}" == "--publish" ]] && PUBLISH=true

cd "$(dirname "$0")/../backlog/issues"

run() {
  if $PUBLISH; then "$@"; else printf '[dry-run]'; printf ' %q' "$@"; echo; fi
}

# label|color|description
labels=(
  "bug|d73a4a|Something isn't working"
  "enhancement|a2eeef|New feature or request"
  "testing|0e8a16|Tests"
  "documentation|0075ca|Docs"
  "security|b60205|Security hardening"
  "ci|5319e7|CI / automation"
  "dx|c5def5|Developer experience"
  "performance|fbca04|Resource usage"
  "good first issue|7057ff|Good for newcomers"
  "area: contract|bfdadc|Contract logic"
  "area: storage|bfdadc|Storage / TTL"
  "area: auth|bfdadc|Authorization"
  "complexity: trivial|c2e0c6|Drips Wave Trivial (100 pts)"
  "complexity: medium|fef2c0|Drips Wave Medium (150 pts)"
  "complexity: high|f9d0c4|Drips Wave High (200 pts)"
)
for entry in "${labels[@]}"; do
  IFS='|' read -r name color desc <<<"$entry"
  run gh label create "$name" --repo "$REPO" --color "$color" --description "$desc" --force
done

for file in *.md; do
  title="$(head -n1 "$file" | sed 's/^# //')"
  label_line="$(grep -m1 '^\*\*Labels:\*\*' "$file")"
  label_args=()
  while IFS= read -r l; do
    [[ -n "$l" ]] && label_args+=(--label "$l")
  done < <(grep -o '`[^`]*`' <<<"$label_line" | tr -d '`')
  body="$(mktemp)"
  tail -n +3 "$file" >"$body"
  run gh issue create --repo "$REPO" --title "$title" --body-file "$body" "${label_args[@]}"
  rm -f "$body"
done
