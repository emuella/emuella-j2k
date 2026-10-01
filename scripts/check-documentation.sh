#!/bin/sh
set -eu
sh scripts/check-source.sh
python3 scripts/documentation_checks.py --package-inventory
