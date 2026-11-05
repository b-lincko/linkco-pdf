#!/bin/sh
# Regenerate the README's screenshots, at a size that stays sharp.
#
# A screenshot holds only the pixels that were captured: it is sharp when those are the pixels
# the reader's screen shows, and soft when it holds fewer. So capture at twice the size the
# image is displayed at (2 × a 1440 × 900 window) and do not downscale it — a HiDPI display then
# gets one image pixel per screen pixel, and a 1× display gets an exact 2:1 reduction.
#
# Do not pass `--width`: it resamples the image and softens the text, which is what made the
# older README screenshots look blurry (1280 × 800, downscaled from 2880 × 1800, then rescaled
# again by the browser to whatever width the page had).
#
# `shot` renders the real shell headlessly, so it needs a working wgpu adapter and the crates
# built; it is a desktop-only tool (not part of any package). The second image needs a document
# and uses the showcase PDF, which `cargo xtask demo-pdf` builds (it needs Chrome or Chromium).
set -eu
cd "$(dirname "$0")/../.."

shot() {
    cargo run --release -p pdfcraft-ui-egui --example shot -- "$@"
}

shot docs/images/linkco-home-dashboard.png --size 1440x900 --scale 2

cargo xtask demo-pdf
shot docs/images/linkco-organize-pages.png dist/demo/pdfcraft-showcase.pdf \
    --size 1440x900 --scale 2 --page 3 --panel pages
