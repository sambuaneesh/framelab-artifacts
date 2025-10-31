use yew::prelude::*;
use std::collections::HashSet;
use crate::colors::ColorGenerator;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub tools: Vec<String>,
    pub active_tools: HashSet<String>,
    pub on_toggle: Callback<String>,
    #[prop_or_default]
    pub color_generator: Option<ColorGenerator>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class="sidebar">
            <h3>{"Tools & Variants"}</h3>
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
    }
}
