use yew::prelude::*;
use crate::data::DataPoint;

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub data: Vec<DataPoint>,
    pub metric: String,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    if props.data.is_empty() {
        return html! {
            <svg viewBox="0 0 800 400">
                <text x="400" y="200" text-anchor="middle" class="bar-label">{"No data"}</text>
            </svg>
        };
    }

    let max_value = props.data.iter()
        .map(|d| d.get_metric(&props.metric))
        .fold(0.0f64, |a, b| a.max(b));

    let bar_height = 25.0;
    let bar_gap = 5.0;
    let margin_left = 180.0;
    let margin_right = 60.0;
    let margin_top = 10.0;
    let chart_width = 800.0;
    let available_width = chart_width - margin_left - margin_right;
    
    let chart_height = (bar_height + bar_gap) * props.data.len() as f64 + margin_top * 2.0;

    html! {
        <svg viewBox={format!("0 0 {} {}", chart_width, chart_height)}>
            {
                props.data.iter().enumerate().map(|(i, d)| {
                    let value = d.get_metric(&props.metric);
                    let bar_width = if max_value > 0.0 {
                        (value / max_value) * available_width
                    } else {
                        0.0
                    };
                    let y = margin_top + i as f64 * (bar_height + bar_gap);
                    
                    html! {
                        <g>
                            <text
                                x={format!("{}", margin_left - 5.0)}
                                y={format!("{}", y + bar_height / 2.0 + 4.0)}
                                text-anchor="end"
                                class="bar-label"
                            >
                                {d.tool_variant()}
                            </text>
                            <rect
                                x={format!("{}", margin_left)}
                                y={format!("{}", y)}
                                width={format!("{}", bar_width)}
                                height={format!("{}", bar_height)}
                                class="bar"
                            />
                            <text
                                x={format!("{}", margin_left + bar_width + 5.0)}
                                y={format!("{}", y + bar_height / 2.0 + 4.0)}
                                class="bar-value"
                            >
                                {format!("{:.1}", value)}
                            </text>
                        </g>
                    }
                }).collect::<Html>()
            }
        </svg>
    }
}
