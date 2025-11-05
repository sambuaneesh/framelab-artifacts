use yew::prelude::*;
use std::collections::HashSet;
use crate::colors::ColorGenerator;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub tools: Vec<String>,
    pub active_tools: HashSet<String>,
    pub on_toggle: Callback<String>,
    pub on_close: Callback<()>,
    pub is_open: bool,
    #[prop_or_default]
    pub color_generator: Option<ColorGenerator>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let sidebar_class = if props.is_open { "sidebar open" } else { "sidebar" };
    let overlay_class = if props.is_open { "sidebar-overlay open" } else { "sidebar-overlay" };
    
    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    
    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    
    html! {
        <>
            <div class={overlay_class} onclick={on_overlay_click}></div>
            <div class={sidebar_class} onclick={Callback::from(|e: MouseEvent| {
                e.stop_propagation(); // Prevent clearing selection when using sidebar
            })}>
                <div class="sidebar-header">
                    <h3>{"Tools & Variants"}</h3>
                    <button class="sidebar-close" onclick={on_close_click}>{"×"}</button>
                </div>
                {
                    props.tools.iter().map(|tool| {
                        let tool_clone = tool.clone();
                        let is_checked = props.active_tools.contains(tool);
                        let on_toggle = props.on_toggle.clone();
                        let color = props.color_generator.as_ref()
                            .map(|cg| cg.get_color(tool))
                            .unwrap_or_else(|| "#64748b".to_string());
                        
                        html! {
                            <label class="filter-item">
                                <input
                                    type="checkbox"
                                    checked={is_checked}
                                    onchange={move |_| on_toggle.emit(tool_clone.clone())}
                                />
                                <span style={format!("display: inline-block; width: 12px; height: 12px; background: {}; border-radius: 2px; margin-right: 6px;", color)}></span>
                                {tool}
                            </label>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
