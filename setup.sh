#!/bin/bash

echo "================================================"
echo "Microservice Decomposition Visualizer Setup"
echo "================================================"
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed!"
    echo "Please install Node.js (version 14 or higher) from https://nodejs.org/"
    exit 1
fi

NODE_VERSION=$(node -v)
echo "✅ Node.js $NODE_VERSION detected"

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed!"
    exit 1
fi

NPM_VERSION=$(npm -v)
echo "✅ npm $NPM_VERSION detected"
echo ""

# Check if web_data.json exists
if [ ! -f "public/web_data.json" ]; then
    echo "⚠️  Warning: web_data.json not found in public/ folder"
    echo ""
    echo "To copy the data file, run:"
    echo "  cp ../metrics/scripts/data/metrics/web_data.json public/"
    echo ""
    read -p "Continue without data file? (y/N): " confirm
    if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
        exit 1
    fi
else
    echo "✅ web_data.json found in public/ folder"
fi

echo ""
echo "📦 Installing dependencies..."
echo ""

npm install

if [ $? -eq 0 ]; then
    echo ""
    echo "================================================"
    echo "✅ Setup complete!"
    echo "================================================"
    echo ""
    echo "To start the development server, run:"
    echo "  npm start"
    echo ""
    echo "The application will open automatically at:"
    echo "  http://localhost:3000"
    echo ""
    echo "To build for production, run:"
    echo "  npm run build"
    echo ""
else
    echo ""
    echo "❌ Installation failed!"
    echo "Please check the error messages above."
    exit 1
fi
