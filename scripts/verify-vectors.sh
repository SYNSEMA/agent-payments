#!/bin/sh
# agent-payments — runs the Synsema reference verifier over the whole vector set.
#
#   sh scripts/verify-vectors.sh
#
# testvectors/valid/   every file must VERIFY
# testvectors/invalid/ every file must be REJECTED
#
# Exit code 0 = every expectation held.

set -eu

VERIFIER="src/verify.syn"
DIRS="testvectors/valid testvectors/invalid"
fail=0

for dir in $DIRS; do
  echo "=== $dir ==="
  for f in "$dir"/*.json; do
    [ -e "$f" ] || continue
    if synsema run --cap-set "stdout,file.read" "$VERIFIER" -- "$f" >/dev/null 2>&1; then
      result="VERIFIED"
    else
      result="REJECTED"
    fi
    case "$dir" in
      testvectors/valid)   want="VERIFIED" ;;
      testvectors/invalid) want="REJECTED" ;;
      *) want="VERIFIED" ;;
    esac
    if [ "$result" = "$want" ]; then
      echo "  [ok] $f -> $result"
    else
      echo "  [XX] $f -> $result (expected $want)"
      fail=1
    fi
  done
done

echo ""
if [ "$fail" -eq 0 ]; then
  echo "every vector behaved as the specification requires"
else
  echo "at least one vector did NOT behave as the specification requires"
fi
exit "$fail"
