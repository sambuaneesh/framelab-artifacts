SETUP

rustup target add wasm32-unknown-unknown
cargo install trunk

DEVELOPMENT

trunk serve

Opens at http://127.0.0.1:8080

PRODUCTION BUILD

trunk build --release

Output in dist/

DEPLOYMENT

Deploy dist/ to GitHub Pages.

In GitHub repo settings:
- Pages -> Source: Deploy from branch
- Branch: main/gh-pages -> /root or /docs
- Or use GitHub Actions

ARCHITECTURE

App: Root, fetches data, manages state
Sidebar: Tool/variant checkboxes
GlobalControls: Metric selector, sort order
ChartGrid: 2x2 layout
BarChart: SVG horizontal bars

Flow: JSON -> Rust structs -> filter -> sort -> SVG

Zero JavaScript. Pure Rust/Wasm. Minimal size.
