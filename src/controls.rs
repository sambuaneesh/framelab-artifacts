use yew::prelude::*;
use crate::data::METRICS;

#[derive(Properties, PartialEq)]
pub struct GlobalControlsProps {
    pub selected_metric: String,
    pub sort_ascending: bool,
    pub on_metric_change: Callback<String>,
    pub on_sort_change: Callback<bool>,
}

#[function_component(GlobalControls)]
pub fn global_controls(props: &GlobalControlsProps) -> Html {
    let on_metric_change = props.on_metric_change.clone();
    let metric_change = Callback::from(move |e: Event| {
        e.stop_propagation(); // Prevent clearing selection when changing metric
        let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
        if let Some(select) = target {
            on_metric_change.emit(select.value());
        }
    });

    let on_sort_change = props.on_sort_change.clone();
    let sort_asc = on_sort_change.clone();
    let sort_desc = on_sort_change.clone();

    html! {
        <div class="controls" onclick={Callback::from(|e: MouseEvent| {
            e.stop_propagation(); // Prevent clearing selection when using controls
        })}>
            <div class="control-group">
                <label>{"Metric:"}</label>
                <select onchange={metric_change} value={props.selected_metric.clone()}>
                    {
                        METRICS.iter().map(|(key, label)| {
                            html! {
                                <option value={*key}>{*label}</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
            </div>
            <div class="control-group">
                <label>{"Sort:"}</label>
                <label>
                    <input
                        type="radio"
                        name="sort"
                        checked={props.sort_ascending}
                        onchange={move |_| sort_asc.emit(true)}
                    />
                    {"Ascending"}
                </label>
                <label>
                    <input
                        type="radio"
                        name="sort"
                        checked={!props.sort_ascending}
                        onchange={move |_| sort_desc.emit(false)}
                    />
                    {"Descending"}
                </label>
            </div>
        </div>
    }
}
