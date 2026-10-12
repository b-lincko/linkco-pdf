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
  # First a summary of every failure line in the whole log (a run with several failures would
  # otherwise only show the last one), then the last ~24 KB without cargo's progress lines; up to
  # 10 annotations of under 3000 characters in all (GitHub shows 10 per step).
  summary="$(grep -E "panicked at|FAILED|^error(\[|:)|^warning: unused|^failures:|^    [A-Za-z0-9_:]+$" "$log" | grep -v "^error: could not compile" | tr -d "\r" | head -c 2800 | sed -e 's/%/%25/g' | awk '{ printf "%s%%0A", $0 }')"
  [ -n "$summary" ] && printf '::error title=%s (summary)::%s\n' "$title" "$summary"
  grep -Ev '^ *(Compiling|Checking|Downloaded|Downloading|Fresh|Blocking|Updating|Locking|Adding) ' "$log" | tail -c 24000 | awk -v t="$title" '
    { gsub(/\r/, ""); gsub(/%/, "%25"); line = $0 "%0A"
      if (buf != "" && length(buf) + length(line) > 2900) { out[++n] = buf; buf = "" }
      buf = buf line }
    END { if (buf != "") out[++n] = buf
          for (i = 1; i <= n; i++) printf "::error title=%s (%d of %d)::%s\n", t, i, n, out[i] }'
fi
rm -f "$log"
exit "$rc"
