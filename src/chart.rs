use yew::prelude::*;
use crate::data::DataPoint;
use crate::colors::ColorGenerator;

#[derive(Properties, PartialEq)]
pub struct BarChartProps {
    pub data: Vec<DataPoint>,
    pub metric: String,
    #[prop_or_default]
    pub color_generator: Option<ColorGenerator>,
    #[prop_or_default]
    pub highlighted_variant: Option<String>,
    pub on_highlight: Callback<Option<String>>,
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
                        let variant = d.tool_variant();
                        let color = props.color_generator.as_ref()
                            .map(|cg| cg.get_color(&variant))
                            .unwrap_or_else(|| "#64748b".to_string());
                        
                        // Determine highlighting level
                        let (opacity, stroke, stroke_width) = if let Some(highlighted) = &props.highlighted_variant {
                            let is_exact_match = highlighted == &variant;
                            
                            let is_same_family = props.color_generator.as_ref()
                                .map(|cg| {
                                    let highlighted_family = cg.get_tool_family(highlighted);
                                    let current_family = cg.get_tool_family(&variant);
                                    highlighted_family == current_family
                                })
                                .unwrap_or(false);
                            
                            if is_exact_match {
                                // Level 1: Exact match - full highlight with white border
                                ("1.0", "#ffffff", "3")
                            } else if is_same_family {
                                // Level 2: Same family - medium highlight with subtle border
                                ("0.7", "#ffffff", "1.5")
                            } else {
                                // Level 3: Other - dimmed
                                ("0.25", "none", "0")
                            }
                        } else {
                            // No highlight - normal state
                            ("1.0", "none", "0")
                        };
                        
                        let variant_clone = variant.clone();
                        let on_highlight = props.on_highlight.clone();
                        let onmouseenter = Callback::from(move |_| {
                            on_highlight.emit(Some(variant_clone.clone()));
                        });
                        
                        let on_highlight_leave = props.on_highlight.clone();
                        let onmouseleave = Callback::from(move |_| {
                            on_highlight_leave.emit(None);
                        });
                        
                        html! {
                            <g 
                                class="bar-group" 
                                style={format!("cursor: pointer; opacity: {}", opacity)}
                                onmouseenter={onmouseenter}
                                onmouseleave={onmouseleave}
                            >
                                <text
                                    x={format!("{}", margin_left - 5.0)}
                                    y={format!("{}", y + bar_height / 2.0 + font_size * 0.35)}
                                    text-anchor="end"
                                    class="bar-label"
                                    fill="#9ca3af"
                                    font-size={format!("{}px", font_size)}
                                >
                                    {variant}
                                </text>
                                <rect
                                    x={format!("{}", margin_left)}
                                    y={format!("{}", y)}
                                    width={format!("{}", bar_width)}
                                    height={format!("{}", bar_height)}
                                    fill={color}
                                    rx="2"
                                    class="bar"
                                    stroke={stroke}
                                    stroke-width={stroke_width}
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
