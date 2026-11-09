#!/usr/bin/env bash
# Same build path as Cloudflare's `npm run build`; output directory: dist.
set -euo pipefail
cd "$(dirname "$0")"
node tools/build.mjs
