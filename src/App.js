import React, { useState, useEffect } from 'react';
import './App.css';
import Sidebar from './components/Sidebar';
import GlobalControls from './components/GlobalControls';
import ChartGrid from './components/ChartGrid';
import TableView from './components/TableView';

function App() {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  // State for user controls
  const [selectedMetric, setSelectedMetric] = useState('CMod');
  const [sortOrder, setSortOrder] = useState('descending');
  const [selectedTools, setSelectedTools] = useState(new Set());
  const [activeTab, setActiveTab] = useState('charts'); // 'charts' or 'table'
  const [sidebarOpen, setSidebarOpen] = useState(true);
  
  // Available metrics (exclude non-numeric columns)
  const [availableMetrics, setAvailableMetrics] = useState([]);
  
  // Load data on mount
  useEffect(() => {
    fetch('/web_data.json')
      .then(response => {
        if (!response.ok) {
          throw new Error('Failed to load data');
        }
        return response.json();
      })
      .then(jsonData => {
        setData(jsonData);
        
        // Extract available metrics (numeric columns only, excluding metadata)
        if (jsonData.length > 0) {
          const excludeColumns = ['Case Study', 'Tool', 'Variant', '# Obs. Ents. (%)'];
          const metrics = Object.keys(jsonData[0]).filter(key => 
            !excludeColumns.includes(key) && 
            typeof jsonData[0][key] === 'number'
          );
          setAvailableMetrics(metrics);
          
          // Set default metric if CMod exists
          if (metrics.includes('CMod')) {
            setSelectedMetric('CMod');
          } else if (metrics.length > 0) {
            setSelectedMetric(metrics[0]);
          }
          
          // Initialize all tools as selected
          const allTools = new Set(
            jsonData.map(row => `${row.Tool} (${row.Variant})`)
          );
          setSelectedTools(allTools);
        }
        
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);
  
  // Get unique tool/variant combinations
  const getUniqueTools = () => {
    const tools = new Map();
    data.forEach(row => {
      const key = `${row.Tool} (${row.Variant})`;
      if (!tools.has(key)) {
        tools.set(key, { tool: row.Tool, variant: row.Variant });
      }
    });
    return Array.from(tools.keys()).sort();
  };
  
  // Filter data based on selected tools
  const getFilteredData = () => {
    return data.filter(row => 
      selectedTools.has(`${row.Tool} (${row.Variant})`)
    );
  };
  
  if (loading) {
    return (
      <div className="App loading">
        <div className="loading-spinner">Loading data...</div>
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="App error">
        <div className="error-message">
          <h2>Error Loading Data</h2>
          <p>{error}</p>
          <p>Please ensure web_data.json is in the public folder.</p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="App">
      <div className="App-tabs">
        <button 
          className={`tab-button ${activeTab === 'charts' ? 'active' : ''}`}
          onClick={() => setActiveTab('charts')}
        >
          Charts View
        </button>
        <button 
          className={`tab-button ${activeTab === 'table' ? 'active' : ''}`}
          onClick={() => setActiveTab('table')}
        >
          Table View
        </button>
      </div>
      
      <div className="App-container">
        <Sidebar
          uniqueTools={getUniqueTools()}
          selectedTools={selectedTools}
          setSelectedTools={setSelectedTools}
          isOpen={sidebarOpen}
          onToggle={() => setSidebarOpen(!sidebarOpen)}
        />
        
        <main className={`App-main ${!sidebarOpen ? 'sidebar-closed' : ''}`}>
          <button 
            className="sidebar-toggle-btn"
            onClick={() => setSidebarOpen(!sidebarOpen)}
            title={sidebarOpen ? 'Hide sidebar' : 'Show sidebar'}
          >
            {sidebarOpen ? '◀' : '▶'}
          </button>
          
          {activeTab === 'charts' ? (
            <>
              <GlobalControls
                availableMetrics={availableMetrics}
                selectedMetric={selectedMetric}
                setSelectedMetric={setSelectedMetric}
                sortOrder={sortOrder}
                setSortOrder={setSortOrder}
              />
              
              <ChartGrid
                data={getFilteredData()}
                selectedMetric={selectedMetric}
                sortOrder={sortOrder}
              />
            </>
          ) : (
            <TableView
              data={getFilteredData()}
              sortOrder={sortOrder}
            />
          )}
        </main>
      </div>
    </div>
  );
}

export default App;
