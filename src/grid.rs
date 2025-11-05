use yew::prelude::*;
use crate::data::{DataPoint, CASE_STUDIES};
use crate::colors::ColorGenerator;
use crate::chart::BarChart;

#[derive(Properties, PartialEq)]
pub struct ChartGridProps {
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

#[function_component(ChartGrid)]
pub fn chart_grid(props: &ChartGridProps) -> Html {
    html! {
        <div class="chart-grid">
            {
                CASE_STUDIES.iter().map(|case_study| {
                    let case_data: Vec<DataPoint> = props.data.iter()
                        .filter(|d| d.case_study == *case_study)
                        .cloned()
                        .collect();
                    
                    html! {
                        <div class="chart-cell">
                            <div class="chart-title">{*case_study}</div>
                            <BarChart 
                                data={case_data} 
                                metric={props.metric.clone()}
                                color_generator={props.color_generator.clone()}
                                highlighted_variant={props.highlighted_variant.clone()}
                                selected_variants={props.selected_variants.clone()}
                                on_highlight={props.on_highlight.clone()}
                                on_select={props.on_select.clone()}
                            />
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
