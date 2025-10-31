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
                            />
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
