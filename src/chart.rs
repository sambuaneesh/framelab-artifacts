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
    #[prop_or_default]
    pub selected_variants: Vec<String>,
    pub on_highlight: Callback<Option<String>>,
    pub on_select: Callback<String>,
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
                // Define diagonal stripe pattern for highlighted bars
                <defs>
                    <pattern id="diagonalStripes" patternUnits="userSpaceOnUse" width="8" height="8" patternTransform="rotate(45)">
                        <line x1="0" y1="0" x2="0" y2="8" stroke="rgba(255, 255, 255, 0.6)" stroke-width="4"/>
                    </pattern>
                </defs>
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
                        
                        // Check if this variant is selected
                        let selection_index = props.selected_variants.iter().position(|v| v == &variant);
                        let is_selected = selection_index.is_some();
                        
                        // Determine highlighting level
                        let (opacity, stroke, stroke_width, stroke_color) = if is_selected {
                            // Selected bars get contrasting colors based on selection order
                            let idx = selection_index.unwrap();
                            let contrast_colors = ["#ff6b6b", "#4ecdc4", "#ffe66d", "#a8e6cf", "#ff8b94", "#c7ceea"];
                            let contrast_color = contrast_colors[idx % contrast_colors.len()];
                            ("1.0", contrast_color, "4", contrast_color)
                        } else if !props.selected_variants.is_empty() {
                            // If any bars are selected, non-selected bars should stay dimmed
                            // (maintain focus mode even without hovering)
                            if let Some(highlighted) = &props.highlighted_variant {
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
                                    ("1.0", "#ffffff", "3", "#ffffff")
                                } else if is_same_family {
                                    // Level 2: Same family - medium highlight with subtle border
                                    ("0.7", "#ffffff", "1.5", "#ffffff")
                                } else {
                                    // Level 3: Other - dimmed (stay dimmed in focus mode)
                                    ("0.25", "none", "0", "none")
                                }
                            } else {
                                // No hover but in focus mode - stay dimmed
                                ("0.25", "none", "0", "none")
                            }
                        } else if let Some(highlighted) = &props.highlighted_variant {
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
                                ("1.0", "#ffffff", "3", "#ffffff")
                            } else if is_same_family {
                                // Level 2: Same family - medium highlight with subtle border
                                ("0.7", "#ffffff", "1.5", "#ffffff")
                            } else {
                                // Level 3: Other - dimmed
                                ("0.25", "none", "0", "none")
                            }
                        } else {
                            // No highlight - normal state
                            ("1.0", "none", "0", "none")
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
                        
                        let variant_clone_click = variant.clone();
                        let on_select = props.on_select.clone();
                        let onclick = Callback::from(move |e: MouseEvent| {
                            e.stop_propagation();
                            on_select.emit(variant_clone_click.clone());
                        });
                        
                        // Check if this is the exact match being hovered
                        let is_exact_match = props.highlighted_variant.as_ref()
                            .map(|hv| hv == &variant)
                            .unwrap_or(false);
                        
                        // Add scale and smooth transition for exact match or selected
                        let transform = if is_exact_match || is_selected {
                            let scale = if is_selected { 1.03 } else { 1.02 };
                            let center_x = margin_left + bar_width / 2.0;
                            let center_y = y + bar_height / 2.0;
                            format!("translate({} {}) scale({}, 1.05) translate({} {})", 
                                center_x, center_y, scale, -center_x, -center_y)
                        } else {
                            "".to_string()
                        };
                        
                        let transition = "transform 0.2s ease-out, opacity 0.2s ease-out";
                        
                        html! {
                            <g 
                                class="bar-group" 
                                style={format!("cursor: pointer; opacity: {}; transition: {}", opacity, transition)}
                                onmouseenter={onmouseenter}
                                onmouseleave={onmouseleave}
                                onclick={onclick}
                                transform={transform}
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
                                // Add diagonal stripe pattern overlay for exact match or selected
                                if is_exact_match || is_selected {
                                    <rect
                                        x={format!("{}", margin_left)}
                                        y={format!("{}", y)}
                                        width={format!("{}", bar_width)}
                                        height={format!("{}", bar_height)}
                                        fill="url(#diagonalStripes)"
                                        rx="2"
                                        style="pointer-events: none;"
                                    />
                                }
                                // Add selection number indicator (positioned on left side to avoid overlap)
                                if let Some(idx) = selection_index {
                                    <g>
                                        <circle
                                            cx={format!("{}", margin_left - 20.0)}
                                            cy={format!("{}", y + bar_height / 2.0)}
                                            r={format!("{}", (bar_height * 0.35).max(8.0).min(12.0))}
                                            fill={stroke_color}
                                            stroke="#1e293b"
                                            stroke-width="2"
                                        />
                                        <text
                                            x={format!("{}", margin_left - 20.0)}
                                            y={format!("{}", y + bar_height / 2.0 + font_size * 0.35)}
                                            text-anchor="middle"
                                            fill="#1e293b"
                                            font-size={format!("{}px", font_size * 0.85)}
                                            font-weight="bold"
                                            style="pointer-events: none;"
                                        >
                                            {idx + 1}
                                        </text>
                                    </g>
                                }
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
