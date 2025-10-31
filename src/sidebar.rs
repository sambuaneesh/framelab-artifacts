use yew::prelude::*;
use std::collections::HashSet;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub tools: Vec<String>,
    pub active_tools: HashSet<String>,
    pub on_toggle: Callback<String>,
}

fn get_tool_color(tool_variant: &str) -> &'static str {
    let tool = tool_variant.split(" (").next().unwrap_or("");
    match tool {
        "CHUNKING" => "#3b82f6",
        "DATACENTRIC" => "#8b5cf6",
        "GROUND_TRUTH" => "#10b981",
        "HyDec" => "#f59e0b",
        "LOG2MS" => "#ef4444",
        "MEM-CMT" => "#ec4899",
        "MEM-CNTR" => "#14b8a6",
        "MONO2MICRO" => "#f97316",
        "ZEROSHOT" => "#6366f1",
        _ => "#64748b",
    }
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
                    let color = get_tool_color(tool);
                    
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
