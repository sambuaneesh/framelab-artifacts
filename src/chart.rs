use yew::prelude::*;
use crate::data::DataPoint;
use crate::colors::ColorGenerator;

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub data: Vec<DataPoint>,
    pub metric: String,
    #[prop_or_default]
    pub color_generator: Option<ColorGenerator>,
}

#[function_component(BarChart)]
pub fn bar_chart(props: &BarChartProps) -> Html {
    if props.data.is_empty() {
        return html! {
            <div class="chart-container">
                <svg viewBox="0 0 800 600" preserveAspectRatio="xMidYMid meet">
                    <text x="400" y="300" text-anchor="middle" class="bar-label" fill="#9ca3af">{"No data"}</text>
                </svg>
            </div>
        };
    }

    let max_value = props.data.iter()
        .map(|d| d.get_metric(&props.metric))
        .fold(0.0f64, |a, b| a.max(b));

    let margin_left = 200.0;
    let margin_right = 50.0;
    let margin_top = 15.0;
    let margin_bottom = 15.0;
    let chart_width = 800.0;
    let chart_height = 600.0;
    let available_width = chart_width - margin_left - margin_right;
    let available_height = chart_height - margin_top - margin_bottom;
    
    let num_bars = props.data.len() as f64;
    let bar_height = available_height / num_bars * 0.88;
    let bar_gap = available_height / num_bars * 0.12;
    let font_size = (bar_height * 0.45).max(10.0).min(14.0);

    html! {
        <div class="chart-container">
            <svg viewBox={format!("0 0 {} {}", chart_width, chart_height)} preserveAspectRatio="xMidYMid meet">
                {
                    props.data.iter().enumerate().map(|(i, d)| {
                        let value = d.get_metric(&props.metric);
                        let bar_width = if max_value > 0.0 {
                            (value / max_value) * available_width
                        } else {
                            0.0
                        };
                        let y = margin_top + i as f64 * (bar_height + bar_gap);
                        let color = props.color_generator.as_ref()
                            .map(|cg| cg.get_color(&d.tool_variant()))
                            .unwrap_or_else(|| "#64748b".to_string());
                        
                        html! {
                            <g>
                                <text
                                    x={format!("{}", margin_left - 5.0)}
                                    y={format!("{}", y + bar_height / 2.0 + font_size * 0.35)}
                                    text-anchor="end"
                                    class="bar-label"
                                    fill="#9ca3af"
                                    font-size={format!("{}px", font_size)}
                                >
                                    {d.tool_variant()}
                                </text>
                                <rect
                                    x={format!("{}", margin_left)}
                                    y={format!("{}", y)}
                                    width={format!("{}", bar_width)}
                                    height={format!("{}", bar_height)}
                                    fill={color}
                                    rx="2"
                                    class="bar"
                                />
                                <text
                                    x={format!("{}", margin_left + bar_width + 5.0)}
                                    y={format!("{}", y + bar_height / 2.0 + font_size * 0.35)}
                                    class="bar-value"
                                    fill="#f9fafb"
                                    font-size={format!("{}px", font_size)}
                                >
                                    {format!("{:.1}", value)}
                                </text>
                            </g>
                        }
                    }).collect::<Html>()
                }
            </svg>
        </div>
    }
}
