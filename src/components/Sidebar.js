import React from 'react';
import './Sidebar.css';

function Sidebar({ uniqueTools, selectedTools, setSelectedTools, isOpen }) {
  const handleToggle = (tool) => {
    const newSelected = new Set(selectedTools);
    if (newSelected.has(tool)) {
      newSelected.delete(tool);
    } else {
      newSelected.add(tool);
    }
    setSelectedTools(newSelected);
  };
  
  const handleSelectAll = () => {
    setSelectedTools(new Set(uniqueTools));
  };
  
  const handleDeselectAll = () => {
    setSelectedTools(new Set());
  };
  
  return (
    <aside className={`Sidebar ${!isOpen ? 'closed' : ''}`}>
      <div className="Sidebar-header">
        <h2>Filter</h2>
        <div className="Sidebar-actions">
          <button onClick={handleSelectAll} className="btn-small">
            Select All
          </button>
          <button onClick={handleDeselectAll} className="btn-small">
            Clear All
          </button>
        </div>
      </div>
      
      <div className="Sidebar-content">
        <div className="tool-count">
          {selectedTools.size} of {uniqueTools.length} tools selected
        </div>
        
        {uniqueTools.map(tool => (
          <label key={tool} className="tool-checkbox">
            <input
              type="checkbox"
              checked={selectedTools.has(tool)}
              onChange={() => handleToggle(tool)}
            />
            <span className="tool-label">{tool}</span>
          </label>
        ))}
      </div>
    </aside>
  );
}

export default Sidebar;
