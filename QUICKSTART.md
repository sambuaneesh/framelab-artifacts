# 🚀 Quick Start Guide

## Prerequisites

- Node.js 14+ and npm installed
- Generated `web_data.json` from the metrics pipeline

## Setup (3 steps)

### 1. Navigate to the project directory

```bash
cd /home/stealthspectre/temp/icsa/artifact/web-visualization
```

### 2. Run the setup script

```bash
./setup.sh
```

Or manually:

```bash
npm install
```

### 3. Start the development server

```bash
npm start
```

The application will automatically open in your browser at `http://localhost:3000`.

## Updating Data

When you regenerate metrics with new decompositions:

```bash
# 1. Regenerate metrics
cd /home/stealthspectre/temp/icsa/artifact/metrics/scripts/data/metrics
python table.py

# 2. Copy updated data to web app
cp web_data.json /home/stealthspectre/temp/icsa/artifact/web-visualization/public/

# 3. Refresh browser (the app will auto-reload)
```

## Features Overview

### 📊 Metric Selection
- Choose from 19 different metrics
- CMod, BCP, MoJoFM, TC, LC, DI, DTP, and more
- Instant updates across all charts

### 🔧 Tool Filtering
- Show/hide specific tools and variants
- Compare chunking strategies (30k vs 50k vs 80k vs full_repo)
- Compare AI vs concatenated summaries

### 🔀 Sort Control
- Ascending: Low to High
- Descending: High to Low (default)
- Helps identify best/worst performers

### 📈 Interactive Charts
- Hover for detailed values
- Zoom and pan capabilities
- Download as PNG images
- Powered by Plotly.js

## Use Cases

### Compare Chunking Strategies

1. **Select Metric**: Choose "CMod" (structural quality)
2. **Filter Tools**: Check only CHUNKING variants
3. **Analyze**: Compare 30k_ai vs 30k_concat vs 50k_ai, etc.

**Question**: Does AI summarization improve over simple concatenation?

### Find Best Tool per Case Study

1. **Select Metric**: Choose "MoJoFM" (similarity to ground truth)
2. **Sort**: Set to "Descending"
3. **Compare**: Top tool in each case study is best match

**Question**: Which tool produces decompositions closest to expert design?

### Evaluate Full Repo Baseline

1. **Filter Tools**: Select only "full_repo" variants
2. **Select Metric**: Choose any metric
3. **Compare**: See baseline performance without chunking

**Question**: How much quality do we lose by chunking?

### Cross-Application Analysis

1. **Select Metric**: Choose "TC" (team cohesion)
2. **Compare**: Look across all 4 case studies
3. **Identify**: Which apps have consistent tool performance?

**Question**: Do certain apps benefit more from specific strategies?

## Troubleshooting

### Charts not displaying?

**Check 1**: Ensure `web_data.json` is in `public/` folder
```bash
ls -lh public/web_data.json
```

**Check 2**: Check browser console for errors (F12)

**Fix**: Regenerate and copy data file

### Empty charts?

**Check**: Tool filters - you may have unchecked all tools

**Fix**: Click "Select All" in the sidebar

### Application won't start?

**Check**: Dependencies installed
```bash
npm install
```

**Check**: Port 3000 available
```bash
lsof -i :3000
```

**Fix**: Kill process or use different port
```bash
PORT=3001 npm start
```

## Production Build

To create an optimized production build:

```bash
npm run build
```

This creates a `build/` directory with static files you can deploy to:
- GitHub Pages
- Netlify
- Vercel
- Any static hosting service

### Deploy to GitHub Pages

```bash
# Install gh-pages
npm install --save-dev gh-pages

# Add to package.json scripts:
# "predeploy": "npm run build",
# "deploy": "gh-pages -d build"

# Deploy
npm run deploy
```

## Tips for Analysis

1. **Start with MoJoFM**: Shows similarity to expert decompositions
2. **Check CMod**: Indicates structural quality (cohesion/coupling)
3. **Review BCP**: Shows coverage of business capabilities
4. **Compare variants**: Look at all 7 chunking variants together
5. **Export insights**: Use Plotly's download feature to save charts

## Next Steps

- Analyze which chunk size gives best quality/cost tradeoff
- Compare AI vs concat summarization effectiveness
- Identify application-specific patterns
- Generate paper figures from the charts
- Share interactive dashboard with team

---

For detailed documentation, see [README.md](README.md)
