#!/usr/bin/env bash
set -euo pipefail

echo "==> Step 1: Building Rust WASM engine and Vite frontend..."
bun run build

echo "==> Step 2: Adding .nojekyll flag..."
touch dist/.nojekyll

REMOTE_URL=$(git remote get-url origin)
CURRENT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "manual")

echo "==> Step 3: Publishing dist/ to gh-pages branch on origin..."
(
  cd dist
  rm -rf .git
  git init -b gh-pages
  git config user.name "$(git config user.name 2>/dev/null || echo 'Grapher Deployer')"
  git config user.email "$(git config user.email 2>/dev/null || echo 'deployer@grapher.local')"
  git add -A
  git commit -m "deploy: ${CURRENT_COMMIT} ($(date -u +'%Y-%m-%dT%H:%M:%SZ'))"
  git push "${REMOTE_URL}" gh-pages --force
  rm -rf .git
)

echo "==> Successfully deployed to GitHub Pages (gh-pages branch)!"
