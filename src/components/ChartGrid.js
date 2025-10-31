import React from 'react';
import Plot from 'react-plotly.js';
import './ChartGrid.css';

function ChartGrid({ data, selectedMetric, sortOrder }) {
  const caseStudies = ['7ep-demo', 'JPetStore', 'PartsUnlimitedMRP', 'Spring-PetClinic'];
  
  const createChartData = (caseStudy) => {
    // Filter data for this case study
    const caseData = data.filter(row => row['Case Study'] === caseStudy);
    
    if (caseData.length === 0) {
      return null;
    }
    
    // Sort data based on sort order
    const sortedData = [...caseData].sort((a, b) => {
      const aValue = a[selectedMetric] || 0;
      const bValue = b[selectedMetric] || 0;
      return sortOrder === 'ascending' ? aValue - bValue : bValue - aValue;
    });
    
    // Create labels and values for the bar chart
    const labels = sortedData.map(row => `${row.Tool} (${row.Variant})`);
    const values = sortedData.map(row => row[selectedMetric] || 0);
    
    // Color palette for different tools
    const colors = sortedData.map(row => {
      const toolColors = {
        'CHUNKING': '#3498db',
        'DATACENTRIC': '#e74c3c',
        'GROUND_TRUTH': '#2ecc71',
        'HyDec': '#f39c12',
        'LOG2MS': '#9b59b6',
        'MEM-CMT': '#1abc9c',
        'MEM-CNTR': '#16a085',
        'MONO2MICRO': '#e67e22',
        'TOMICROSERVICES': '#95a5a6',
        'MOSAIC': '#34495e',
        'CARGO': '#d35400'
      };
      return toolColors[row.Tool] || '#7f8c8d';
    });
    
    return {
      x: values,
      y: labels,
      type: 'bar',
      orientation: 'h',
      marker: {
        color: colors,
        line: {
          color: 'rgba(0,0,0,0.1)',
          width: 1
        }
      },
      hovertemplate: '<b>%{y}</b><br>' +
                     selectedMetric + ': %{x:.1f}<br>' +
                     '<extra></extra>'
    };
  };
  
  const createLayout = (caseStudy) => ({
    title: {
      text: `<b>${caseStudy}</b>`,
      font: {
        size: 16,
        color: '#333'
      }
    },
    xaxis: {
      title: selectedMetric,
      gridcolor: '#e0e0e0',
      zeroline: true,
      zerolinecolor: '#999',
      zerolinewidth: 1
    },
    yaxis: {
      automargin: true,
      tickfont: {
        size: 11
      }
    },
    margin: {
      l: 180,
      r: 20,
      t: 60,
      b: 60
    },
    paper_bgcolor: 'white',
    plot_bgcolor: '#fafafa',
    hovermode: 'closest',
    hoverlabel: {
      bgcolor: 'white',
      bordercolor: '#333',
      font: {
        size: 12
      }
    }
  });
  
  const config = {
    responsive: true,
    displayModeBar: true,
    displaylogo: false,
    modeBarButtonsToRemove: ['select2d', 'lasso2d']
  };
  
  return (
    <div className="ChartGrid">
      {caseStudies.map(caseStudy => {
        const chartData = createChartData(caseStudy);
        
        return (
          <div key={caseStudy} className="chart-container">
            {chartData ? (
              <Plot
                data={[chartData]}
                layout={createLayout(caseStudy)}
                config={config}
                style={{ width: '100%', height: '100%' }}
                useResizeHandler={true}
              />
            ) : (
              <div className="no-data">
                <h3>{caseStudy}</h3>
                <p>No data available for selected filters</p>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}

export default ChartGrid;
