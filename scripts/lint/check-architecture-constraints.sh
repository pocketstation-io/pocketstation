#!/usr/bin/env bash
# check-architecture-constraints.sh — enforces CRATE_OWNERSHIP.md rules.
#
# Run: bash scripts/lint/check-architecture-constraints.sh
# Wired into: .github/workflows/ci.yml
#
# Five hard checks (exit 1 on any failure):
#   1. No first-party AI connector crate directories
#   2. No closed graph enums in pks-graph / pks-runtime
#   3. No provider names leaking into core crates
#   4. No Rust TypeId in pks-graph (breaks cross-language contract stability)
#   5. Endpoint lifecycle dependency direction remains acyclic
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CORE="${REPO_ROOT}/crates"

fail=0

# 1. Forbid first-party AI connector crate directories.
# These must never be created: pks-ai, pks-ai-*, model-connectors.
if find "${REPO_ROOT}" -maxdepth 4 -type d 2>/dev/null \
     | grep -qE '/(pks-ai(-|$)|model-connectors)'; then
  echo "FAIL [arch-1]: Forbidden first-party AI connector crate directory found." >&2
  echo "  Allowed: examples/, community crates. Forbidden: pks-ai-*, model-connectors." >&2
  fail=$((fail + 1))
fi

# 2. Forbid closed graph enums in pks-graph and pks-runtime.
# Core must not have enum ModelNode, enum PolicyNode, or enum SinkNode.
for dir in "${CORE}/pks-graph" "${CORE}/pks-runtime"; do
  if [[ -d "${dir}" ]]; then
    if grep -rq "enum ModelNode\|enum PolicyNode\|enum SinkNode" "${dir}/src" 2>/dev/null; then
      echo "FAIL [arch-2]: Closed graph enum (ModelNode/PolicyNode/SinkNode) in ${dir##"${REPO_ROOT}"/}." >&2
      echo "  Use OperatorSpec + OperatorId instead. See CRATE_OWNERSHIP.md." >&2
      fail=$((fail + 1))
    fi
  fi
done

# 3. Forbid provider names leaking into core crates.
# pks-frame, pks-graph, pks-runtime must not mention AI provider names.
PROVIDER_RE='[Ww]hisper|[Oo]llama|[Dd]eepgram|[Ee]leven[Ll]abs|[Oo]pen[Aa][Ii]'
for dir in "${CORE}/pks-frame" "${CORE}/pks-graph" "${CORE}/pks-runtime" "${CORE}/pks-endpoint"; do
  if [[ -d "${dir}" ]]; then
    if grep -rqE "${PROVIDER_RE}" "${dir}/src" 2>/dev/null; then
      echo "FAIL [arch-3]: AI provider name leaked into ${dir##"${REPO_ROOT}"/}/src." >&2
      echo "  Providers belong in examples/ or community crates only." >&2
      fail=$((fail + 1))
    fi
  fi
done

# 4. Forbid Rust TypeId in pks-graph (breaks cross-language contract stability).
# Use SignalId(Cow<'static, str>) instead.
if [[ -d "${CORE}/pks-graph" ]]; then
  if grep -rqE "std::any::TypeId|core::any::TypeId" "${CORE}/pks-graph/src" 2>/dev/null; then
    echo "FAIL [arch-4]: TypeId found in pks-graph/src." >&2
    echo "  Use SignalId(Cow<'static, str>) for cross-language stable signal identity." >&2
    fail=$((fail + 1))
  fi
fi

# 5. Keep endpoint semantics above runtime and below Session/concrete drivers.
if grep -qE 'pks-endpoint' "${CORE}/pks-runtime/Cargo.toml"; then
  echo "FAIL [arch-5]: pks-runtime must not depend on pks-endpoint." >&2
  fail=$((fail + 1))
fi
if grep -qE 'pks-session|pks-nodes|pks-capture-(macos|windows|linux)' \
     "${CORE}/pks-endpoint/Cargo.toml"; then
  echo "FAIL [arch-5]: pks-endpoint depends upward on Session, nodes, or target capture." >&2
  fail=$((fail + 1))
fi

echo "---"
if [[ "${fail}" -eq 0 ]]; then
  echo "Architecture constraint checks: PASS (${fail} failures)"
else
  echo "Architecture constraint checks: FAIL (${fail} failure(s))" >&2
  exit 1
fi
