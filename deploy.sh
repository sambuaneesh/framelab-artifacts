#!/bin/sh
set -e

echo "Building release..."
trunk build --release

echo ""
echo "Build complete. Deploy dist/ to GitHub Pages."
echo ""
echo "Quick deploy with GitHub CLI:"
echo "  gh workflow run pages-deploy"
echo ""
echo "Or commit dist/ and push:"
echo "  git add dist/"
echo "  git commit -m 'Build artifacts'"
echo "  git push"
