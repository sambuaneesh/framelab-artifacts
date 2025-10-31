SETUP

rustup target add wasm32-unknown-unknown
cargo install trunk

DEVELOPMENT

trunk serve

Opens at http://127.0.0.1:8080

PRODUCTION BUILD

trunk build --release --public-url /web-visualization/

Output in dist/

DEPLOYMENT TO GITHUB PAGES

Method 1: Direct push to gh-pages branch
  trunk build --release --public-url /web-visualization/
  git add dist/
  git commit -m "Build for GitHub Pages"
  git subtree push --prefix dist origin gh-pages

Method 2: GitHub Actions (recommended)
  Push to main branch. GitHub Actions will build and deploy automatically.
  Ensure .github/workflows/deploy.yml exists.

Method 3: Manual dist copy
  trunk build --release --public-url /web-visualization/
  cd dist
  git init
  git add .
  git commit -m "Deploy"
  git remote add origin <your-repo-url>
  git push -f origin main:gh-pages

After deployment, enable GitHub Pages:
  Settings -> Pages -> Source: gh-pages branch -> Save

URL: https://sambuaneesh.github.io/framelab-artifacts/

ARCHITECTURE

App: Root, fetches data, manages state
Sidebar: Tool/variant checkboxes (popup overlay)
GlobalControls: Metric selector, sort order
ChartGrid: 2x2 layout, viewport-fitted
BarChart: SVG horizontal bars, dynamic scaling

Flow: JSON -> Rust structs -> filter -> sort -> SVG

Zero JavaScript. Pure Rust/Wasm. ~340KB total.
