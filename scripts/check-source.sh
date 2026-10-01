#!/bin/sh
set -eu
export PYTHONDONTWRITEBYTECODE=1
python3 scripts/test-check-committed-tree.py
python3 scripts/test-public-tree-policy.py
python3 scripts/test-package-legal-policy.py
python3 scripts/test-workspace-dependency-policy.py
python3 scripts/test-documentation-checks.py
python3 scripts/test-verification-routing.py
python3 scripts/documentation-route/test_documentation_route.py
python3 scripts/documentation-route/test_adopt.py
python3 scripts/audit-public-tree.py
python3 scripts/generate-binary-dependency-notices.py --check
