#!/bin/bash

# Test script for all Last.fm CLI commands
# This script tests every command to ensure they work with the production worker

CLI="./target/release/lastfm-cli"

echo "Testing Last.fm CLI - All Commands"
echo "=================================="

# Artist commands
echo -e "\n=== ARTIST COMMANDS ==="
echo "1. artist info"
$CLI artist info "The Beatles" -o table | head -5

echo -e "\n2. artist similar"
$CLI artist similar "Radiohead" --limit 3 -o table

echo -e "\n3. artist search"
$CLI artist search "Beatles" --limit 3 -o table

echo -e "\n4. artist top-albums"
$CLI artist top-albums "Nirvana" --limit 3 -o table

echo -e "\n5. artist top-tracks"
$CLI artist top-tracks "Queen" --limit 3 -o table

# Album commands
echo -e "\n=== ALBUM COMMANDS ==="
echo "6. album info"
$CLI album info "The Beatles" "Abbey Road" -o table | head -10

echo -e "\n7. album search"
$CLI album search "Dark Side" --limit 3 -o table

# Track commands
echo -e "\n=== TRACK COMMANDS ==="
echo "8. track info"
$CLI track info "The Beatles" "Hey Jude" -o table | head -5

echo -e "\n9. track similar"
$CLI track similar "Radiohead" "Creep" --limit 3 -o table

echo -e "\n10. track search"
$CLI track search "Imagine" --limit 3 -o table

# Chart commands
echo -e "\n=== CHART COMMANDS ==="
echo "11. chart top-artists"
$CLI chart top-artists --limit 3 -o table

echo -e "\n12. chart top-tracks"
$CLI chart top-tracks --limit 3 -o table

echo -e "\n13. chart top-tags"
$CLI chart top-tags --limit 3 -o table

# Geo commands
echo -e "\n=== GEO COMMANDS ==="
echo "14. geo top-artists"
$CLI geo top-artists "United States" --limit 3 -o table

echo -e "\n15. geo top-tracks"
$CLI geo top-tracks "Japan" --limit 3 -o table

# Tag commands
echo -e "\n=== TAG COMMANDS ==="
echo "16. tag info"
$CLI tag info "rock" -o table | head -5

echo -e "\n17. tag top-artists"
$CLI tag top-artists "shoegaze" --limit 3 -o table

echo -e "\n18. tag top-albums"
$CLI tag top-albums "rock" --limit 3 -o table

echo -e "\n19. tag top-tracks"
$CLI tag top-tracks "electronic" --limit 3 -o table

# User commands
echo -e "\n=== USER COMMANDS ==="
echo "20. user info"
$CLI user info "rj" -o table | head -10

echo -e "\n21. user recent-tracks"
$CLI user recent-tracks "rj" --limit 3 -o table

echo -e "\n22. user top-artists"
$CLI user top-artists "rj" --period 7day --limit 3 -o table

# Library commands
echo -e "\n=== LIBRARY COMMANDS ==="
echo "23. library artists"
$CLI library artists "rj" -o table

# Authenticated 'my' commands
echo -e "\n=== AUTHENTICATED 'MY' COMMANDS ==="
if $CLI auth status | grep -q '"authenticated": true'; then
    echo "24. my recent-tracks"
    $CLI my recent-tracks --limit 2 -o table
    
    echo -e "\n25. my top-artists"
    $CLI my top-artists --limit 2 -o table
    
    echo -e "\n26. my top-tracks"
    $CLI my top-tracks --limit 2 -o table
    
    echo -e "\n27. my top-albums"
    $CLI my top-albums --limit 2 -o table
else
    echo "Not authenticated - skipping 'my' commands"
fi

echo -e "\n=== ALL TESTS COMPLETE ==="