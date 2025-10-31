import React from 'react';
import './GlobalControls.css';

function GlobalControls({ 
  availableMetrics, 
  selectedMetric, 
  setSelectedMetric,
  sortOrder,
  setSortOrder 
}) {
  return (
    <div className="GlobalControls">
      <div className="control-group">
        <label htmlFor="metric-selector">
          Metric:
        </label>
        <select
          id="metric-selector"
          value={selectedMetric}
          onChange={(e) => setSelectedMetric(e.target.value)}
          className="metric-selector"
        >
          {availableMetrics.map(metric => (
            <option key={metric} value={metric}>
              {metric}
            </option>
          ))}
        </select>
      </div>
      
      <div className="control-group">
        <label>Sort Order:</label>
        <div className="radio-group">
          <label className="radio-label">
            <input
              type="radio"
              value="descending"
              checked={sortOrder === 'descending'}
              onChange={(e) => setSortOrder(e.target.value)}
            />
            <span>Descending</span>
          </label>
          <label className="radio-label">
            <input
              type="radio"
              value="ascending"
              checked={sortOrder === 'ascending'}
              onChange={(e) => setSortOrder(e.target.value)}
            />
            <span>Ascending</span>
          </label>
        </div>
      </div>
    </div>
  );
}

export default GlobalControls;
