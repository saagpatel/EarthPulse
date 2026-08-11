#!/usr/bin/env bash
set -euo pipefail

# codex-os-managed
if ! command -v gitleaks >/dev/null 2>&1; then
  # CI runs this guard through the verify bundle on a runner that does not
  # install gitleaks, so refusing here would fail every pull request. Secret
  # scanning in CI belongs to the git-hygiene workflow.
  if [[ -n "${CI:-}" ]]; then
    echo "gitleaks not found in CI. Skipping local secret guard because dedicated CI scanning covers it."
    exit 0
  fi
  # Outside CI, refuse. A missing scanner is not a clean scan, and this hook is
  # the only secret check a commit reaches when it never becomes a pull request.
  echo "gitleaks not found. Install gitleaks to enforce secret scanning."
  exit 1
fi

gitleaks protect --staged --redact
