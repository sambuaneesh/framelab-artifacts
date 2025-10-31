mod data;
mod colors;
mod sidebar;
mod controls;
mod chart;
mod grid;
mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
