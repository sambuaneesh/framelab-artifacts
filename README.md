# Microservice Decomposition Metrics Visualizer

An interactive web application for visualizing and comparing microservice decomposition tool performance across multiple case studies.

## Features

- **2x2 Grid Layout**: Compare metrics across 4 case studies simultaneously
- **Interactive Bar Charts**: Powered by Plotly.js for smooth interactions
- **Dynamic Metric Selection**: Switch between 19 different metrics instantly
- **Tool Filtering**: Show/hide specific tools and variants
- **Flexible Sorting**: Sort bars in ascending or descending order
- **Responsive Design**: Clean, modern interface

## Installation

1. **Install Dependencies**:
   ```bash
   npm install
   ```

2. **Copy Data File**:
   Copy your `web_data.json` file to the `public` folder:
   ```bash
   cp ../metrics/scripts/data/metrics/web_data.json public/
   ```

## Running the Application

Start the development server:

```bash
npm start
```

The application will automatically open in your browser at `http://localhost:3000`.

## Building for Production

Create an optimized production build:

```bash
npm run build
```

The build files will be in the `build/` directory, ready to deploy to any static hosting service.

## Data Format

The application expects a JSON file (`web_data.json`) with the following structure:

```json
[
  {
    "Case Study": "7ep-demo",
    "Tool": "CHUNKING",
    "Variant": "30k_ai",
    "#Partitions": 4,
    "CMod": 74.8,
    "BCP": 80.0,
    "MoJoFM": 23.3,
    ...
  }
]
```

## Available Metrics

The application supports visualization of the following metrics:

### Structural Quality
- **CMod**: Cohesion and Coupling Modularization Quality
- **BCP**: Best Coverage Percentage
- **CiD**: Cyclic Independence (100 - CDP)

### Evolutionary Metrics
- **TC**: Team Cohesion (based on contributors)
- **LC**: Logical Coupling (based on commits)

### Entropy Metrics
- **DI**: Domain Independence (Use Case Entropy)
- **DTP**: Data Table Partitioning (Database Entropy)

### Comparison Metrics
- **MoJoFM**: Similarity to Ground Truth (0-100, higher is better)
- **c2c_cvg 10%/33%/50%**: Convergence thresholds

### Partition Statistics
- **#Partitions**: Number of microservices
- **Partition Size Min/Med/Max**: Service size distribution
- **# Obs. Ents. (%)**: Completeness percentage

## Technology Stack

- **React 18**: Component-based UI framework
- **Plotly.js**: Interactive charting library
- **Create React App**: Build tooling and development server

## Project Structure

```
web-visualization/
├── public/
│   ├── index.html
│   └── web_data.json          # Your metrics data
├── src/
│   ├── App.js                 # Main application component
│   ├── App.css                # Application styles
│   ├── components/
│   │   ├── ChartGrid.js       # 2x2 grid of charts
│   │   ├── GlobalControls.js  # Metric & sort controls
│   │   └── Sidebar.js         # Tool filter controls
│   ├── index.js               # React entry point
│   └── index.css              # Global styles
├── package.json
└── README.md
```

## Usage Guide

### 1. Select a Metric
Use the dropdown at the top to choose which metric to visualize across all case studies.

### 2. Sort Results
Choose "Ascending" or "Descending" to sort bars within each chart.

### 3. Filter Tools
Use the sidebar checkboxes to show/hide specific tools and variants. All tools are shown by default.

### 4. Analyze Comparisons
Compare how different tools perform on the same metric across different case studies:
- Which tool achieves the highest CMod score in JPetStore?
- How does the chunking strategy (30k vs 50k vs 80k) affect MoJoFM?
- Which case study shows the most consistent tool performance?

## Development Notes

- The application fetches `web_data.json` from the public folder on load
- All filtering and sorting is done client-side for instant updates
- Charts automatically resize to fill available space
- Tool/variant combinations are extracted dynamically from the data

## Troubleshooting

**Q: Charts not displaying?**
- Ensure `web_data.json` is in the `public/` folder
- Check browser console for fetch errors
- Verify JSON file has valid structure

**Q: Missing metrics in dropdown?**
- The dropdown only shows numeric columns
- Non-metric fields (Case Study, Tool, Variant) are excluded automatically

**Q: Application won't start?**
- Run `npm install` to ensure dependencies are installed
- Check that you have Node.js 14+ installed

## License

This project is part of the ICSA 2024 research artifact for comparing microservice decomposition approaches.

## Citation

If you use this visualization tool in your research, please cite the original paper:
[Citation information to be added]
