#!/bin/bash

echo "🚀 Starting GMX Solana Parser Test Server..."
echo ""
echo "📂 Working directory: $(pwd)"
echo "🌐 Server URL: http://localhost:3001/test_gmx.html"
echo ""
echo "⚠️  Make sure you have built the WASM module first:"
echo "   wasm-pack build --target web --out-dir pkg"
echo ""
echo "🛑 To stop the server: Ctrl+C or run 'pkill -f \"python3 -m http.server\"'"
echo ""

# Start Python HTTP server
python3 -m http.server 3001 