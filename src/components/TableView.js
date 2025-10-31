import React, { useMemo } from 'react';
import './TableView.css';

function TableView({ data, sortOrder }) {
  // Get all column names from the first data row
  const columns = useMemo(() => {
    if (data.length === 0) return [];
    return Object.keys(data[0]);
  }, [data]);
  
  // Sort data based on sort order (using the first numeric column for sorting)
  const sortedData = useMemo(() => {
    if (data.length === 0) return [];
    
    const numericColumns = columns.filter(col => 
      typeof data[0][col] === 'number'
    );
    
    if (numericColumns.length === 0) return data;
    
    const sortColumn = numericColumns[0]; // Sort by first numeric column
    
    return [...data].sort((a, b) => {
      const aVal = a[sortColumn] || 0;
      const bVal = b[sortColumn] || 0;
      return sortOrder === 'ascending' ? aVal - bVal : bVal - aVal;
    });
  }, [data, sortOrder, columns]);
  
  // Format cell value
  const formatValue = (value) => {
    if (typeof value === 'number') {
      return value.toFixed(1);
    }
    return value;
  };
  
  if (data.length === 0) {
    return (
      <div className="TableView-empty">
        <p>No data to display. Please select tools from the sidebar.</p>
      </div>
    );
  }
  
  return (
    <div className="TableView">
      
      <div className="TableView-container">
        <table className="TableView-table">
          <thead>
            <tr>
              {columns.map((col, idx) => (
                <th key={idx} className={`col-${col.replace(/\s+/g, '-').toLowerCase()}`}>
                  {col}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {sortedData.map((row, rowIdx) => (
              <tr key={rowIdx}>
                {columns.map((col, colIdx) => (
                  <td 
                    key={colIdx}
                    className={`col-${col.replace(/\s+/g, '-').toLowerCase()}`}
                  >
                    {formatValue(row[col])}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default TableView;
