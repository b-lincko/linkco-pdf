#!/usr/bin/env bash
# Runs a command and, if it fails, publishes the end of its output as GitHub Actions error
# annotations (the checks API returns annotations, so a failure can be read without the logs).
# Usage: run-annotated.sh "<title>" <command> [args...]
title="$1"
shift
export CARGO_TERM_COLOR=never
log="$(mktemp)"
set -o pipefail
"$@" 2>&1 | tee "$log"
rc=${PIPESTATUS[0]}
if [ "$rc" -ne 0 ]; then
  # The last ~27 KB, as up to 10 annotations of under 3000 characters (GitHub shows 10 per step).
  tail -c 27000 "$log" | awk -v t="$title" '
    { gsub(/\r/, ""); gsub(/%/, "%25"); line = $0 "%0A"
      if (buf != "" && length(buf) + length(line) > 2900) { out[++n] = buf; buf = "" }
      buf = buf line }
    END { if (buf != "") out[++n] = buf
          for (i = 1; i <= n; i++) printf "::error title=%s (%d of %d)::%s\n", t, i, n, out[i] }'
fi
rm -f "$log"
exit "$rc"
