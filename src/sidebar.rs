use yew::prelude::*;
use std::collections::HashSet;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub tools: Vec<String>,
    pub active_tools: HashSet<String>,
    pub on_toggle: Callback<String>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class="sidebar">
            <h3>{"Tools"}</h3>
            {
                props.tools.iter().map(|tool| {
                    let tool_clone = tool.clone();
                    let is_checked = props.active_tools.contains(tool);
                    let on_toggle = props.on_toggle.clone();
                    
                    html! {
                        <label class="filter-item">
                            <input
                                type="checkbox"
                                checked={is_checked}
                                onchange={move |_| on_toggle.emit(tool_clone.clone())}
                            />
                            {tool}
                        </label>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
