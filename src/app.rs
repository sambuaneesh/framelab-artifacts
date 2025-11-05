use yew::prelude::*;
use gloo_net::http::Request;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use crate::data::DataPoint;
use crate::colors::ColorGenerator;
use crate::sidebar::Sidebar;
use crate::controls::GlobalControls;
use crate::grid::ChartGrid;

pub enum Msg {
    DataLoaded(Vec<DataPoint>),
    LoadError(String),
    ToggleTool(String),
    ChangeMetric(String),
    ChangeSortOrder(bool),
    ToggleSidebar,
    CloseSidebar,
    HighlightVariant(Option<String>),
    ToggleSelectVariant(String),
    ClearSelection,
}

pub struct App {
    data: Option<Vec<DataPoint>>,
    error: Option<String>,
    active_tools: HashSet<String>,
    all_tools: Vec<String>,
    selected_metric: String,
    sort_ascending: bool,
    color_generator: ColorGenerator,
    sidebar_open: bool,
    highlighted_variant: Option<String>,
    selected_variants: Vec<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match Request::get("web_data.json").send().await {
                Ok(response) => {
                    match response.json::<Vec<DataPoint>>().await {
                        Ok(data) => Msg::DataLoaded(data),
                        Err(e) => Msg::LoadError(format!("Parse error: {}", e)),
                    }
                }
                Err(e) => Msg::LoadError(format!("Network error: {}", e)),
            }
        });

        Self {
            data: None,
            error: None,
            active_tools: HashSet::new(),
            all_tools: Vec::new(),
            selected_metric: "cid".to_string(),
            sort_ascending: false,
            color_generator: ColorGenerator::new(),
            sidebar_open: false,
            highlighted_variant: None,
            selected_variants: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DataLoaded(data) => {
                let mut tools: Vec<String> = data.iter()
                    .map(|d| d.tool_variant())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                tools.sort();
                
                self.color_generator.generate_tool_colors(&tools);
                
                self.active_tools = tools.iter().cloned().collect();
                self.all_tools = tools;
                self.data = Some(data);
                self.error = None;
                true
            }
            Msg::LoadError(error) => {
                self.error = Some(error);
                true
            }
            Msg::ToggleTool(tool) => {
                if self.active_tools.contains(&tool) {
                    self.active_tools.remove(&tool);
                } else {
                    self.active_tools.insert(tool);
                }
                true
            }
            Msg::ChangeMetric(metric) => {
                self.selected_metric = metric;
                true
            }
            Msg::ChangeSortOrder(ascending) => {
                self.sort_ascending = ascending;
                true
            }
            Msg::ToggleSidebar => {
                self.sidebar_open = !self.sidebar_open;
                true
            }
            Msg::CloseSidebar => {
                self.sidebar_open = false;
                true
            }
            Msg::HighlightVariant(variant) => {
                self.highlighted_variant = variant;
                true
            }
            Msg::ToggleSelectVariant(variant) => {
                if let Some(pos) = self.selected_variants.iter().position(|v| v == &variant) {
                    // Already selected, remove it
                    self.selected_variants.remove(pos);
                } else {
                    // Not selected, add it
                    self.selected_variants.push(variant);
                }
                true
            }
            Msg::ClearSelection => {
                self.selected_variants.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(error) = &self.error {
            return html! {
                <div class="error">{error}</div>
            };
        }

        if let Some(data) = &self.data {
            let mut filtered: Vec<DataPoint> = data.iter()
                .filter(|d| self.active_tools.contains(&d.tool_variant()))
                .cloned()
                .collect();

            filtered.sort_by(|a, b| {
                let val_a = a.get_metric(&self.selected_metric);
                let val_b = b.get_metric(&self.selected_metric);
                if self.sort_ascending {
                    val_a.partial_cmp(&val_b).unwrap()
                } else {
                    val_b.partial_cmp(&val_a).unwrap()
                }
            });

            html! {
                <>
                    <button 
                        class="toggle-sidebar-btn"
                        onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                    >
                        {"☰ Filters"}
                    </button>
                    <Sidebar
                        tools={self.all_tools.clone()}
                        active_tools={self.active_tools.clone()}
                        on_toggle={ctx.link().callback(Msg::ToggleTool)}
                        on_close={ctx.link().callback(|_| Msg::CloseSidebar)}
                        is_open={self.sidebar_open}
                        color_generator={Some(self.color_generator.clone())}
                    />
                    <div 
                        class="main-content"
                        onclick={ctx.link().callback(|e: MouseEvent| {
                            // Check if click was on the background (not a bar)
                            if let Some(target) = e.target() {
                                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                                    let class_name = element.class_name();
                                    // Clear selection if clicked on main-content or chart-grid
                                    if class_name.contains("main-content") || 
                                       class_name.contains("chart-grid") ||
                                       class_name.contains("chart-cell") ||
                                       class_name.contains("chart-container") {
                                        return Msg::ClearSelection;
                                    }
                                }
                            }
                            Msg::ClearSelection
                        })}
                    >
                        <GlobalControls
                            selected_metric={self.selected_metric.clone()}
                            sort_ascending={self.sort_ascending}
                            on_metric_change={ctx.link().callback(Msg::ChangeMetric)}
                            on_sort_change={ctx.link().callback(Msg::ChangeSortOrder)}
                        />
                        <ChartGrid
                            data={filtered}
                            metric={self.selected_metric.clone()}
                            color_generator={Some(self.color_generator.clone())}
                            highlighted_variant={self.highlighted_variant.clone()}
                            selected_variants={self.selected_variants.clone()}
                            on_highlight={ctx.link().callback(Msg::HighlightVariant)}
                            on_select={ctx.link().callback(Msg::ToggleSelectVariant)}
                        />
                    </div>
                </>
            }
        } else {
            html! {
                <div class="loading">{"Loading..."}</div>
            }
        }
    }
}
