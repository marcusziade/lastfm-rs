#!/bin/bash

# Preview script for the landing page
# This script starts a local HTTP server to preview the landing page

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCS_DIR="$PROJECT_ROOT/docs"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🎵 Last.fm-rs Landing Page Preview${NC}"
echo -e "${BLUE}===================================${NC}"

# Check if Python is installed
if ! command -v python3 &> /dev/null; then
    echo -e "${YELLOW}Python 3 is not installed. Trying python...${NC}"
    if ! command -v python &> /dev/null; then
        echo "Error: Python is not installed. Please install Python to use this script."
        exit 1
    fi
    PYTHON_CMD="python"
else
    PYTHON_CMD="python3"
fi

# Navigate to docs directory
cd "$DOCS_DIR"

# Detect available port
PORT=8000
while lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1; do
    ((PORT++))
done

echo -e "${GREEN}Starting local server on port $PORT...${NC}"
echo -e "${GREEN}Landing page will be available at:${NC}"
echo -e "${BLUE}  http://localhost:$PORT${NC}"
echo ""
echo -e "${YELLOW}Note: The API Docs button will link to the deployed worker URL.${NC}"
echo -e "${YELLOW}Update 'your-account' in the HTML to your actual Cloudflare subdomain.${NC}"
echo ""
echo -e "${GREEN}Press Ctrl+C to stop the server${NC}"
echo ""

# Start the server
if [[ "$PYTHON_CMD" == "python3" ]]; then
    $PYTHON_CMD -m http.server $PORT
else
    $PYTHON_CMD -m SimpleHTTPServer $PORT
fi