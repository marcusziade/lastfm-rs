# lastfm-rs 🎵

A blazing-fast Rust-based Last.fm API proxy for Cloudflare Workers and a feature-rich CLI that lets you access both public and personal Last.fm data.

## ✨ Key Features

- **🔐 Full Authentication Support** - Access your personal Last.fm data (loved tracks, recent plays, top artists)
- **🚀 High Performance** - Built with Rust for optimal speed and efficiency
- **💾 Smart Caching** - KV-based caching with intelligent TTL management
- **🛡️ Rate Limiting** - Built-in protection against API abuse
- **📊 31+ Commands** - Comprehensive coverage across 10 categories
- **🎨 Multiple Output Formats** - JSON, table, pretty-print, and compact modes

## 🚀 Quick Start

### CLI Installation

```bash
# Install from source
cargo install --path . --bin lastfm-cli

# Or download pre-built binary (if available)
curl -L https://github.com/yourusername/lastfm-rs/releases/latest/download/lastfm-cli-linux-x64 -o lastfm-cli
chmod +x lastfm-cli
```

### Worker Deployment

```bash
# Clone and setup
git clone https://github.com/yourusername/lastfm-rs
cd lastfm-rs

# Configure KV namespaces
wrangler kv:namespace create "CACHE"
wrangler kv:namespace create "RATE_LIMIT"

# Add your Last.fm API key
wrangler secret put LASTFM_API_KEY

# Deploy to Cloudflare
wrangler deploy
```

## 🎯 Example Commands

### Personal Data (Authenticated)

```bash
# Login to Last.fm
lastfm-cli auth login

# Get your recent tracks
lastfm-cli my recent-tracks --limit 10

# See your top artists this month
lastfm-cli my top-artists --period 1month

# Check your loved tracks
lastfm-cli my loved-tracks
```

### Public Data

```bash
# Get artist info with beautiful formatting
lastfm-cli artist info "Taylor Swift" -o pretty

# Search for similar tracks
lastfm-cli track similar "The Beatles" "Hey Jude" -o table

# Discover music by country
lastfm-cli geo top-tracks "Japan" --limit 20

# Explore genres
lastfm-cli tag top-artists "shoegaze" -o json | jq '.topartists.artist[0:5]'
```

### Advanced Usage

```bash
# Compare your music taste with a friend
lastfm-cli user compare "YourUsername" "FriendUsername"

# Export your library to CSV
lastfm-cli my top-tracks --period overall --limit 1000 -o json | \
  jq -r '.toptracks.track[] | [.name, .artist.name, .playcount] | @csv' > my_music.csv

# Track listening habits over time
lastfm-cli my weekly-chart-list | \
  lastfm-cli my weekly-track-chart --from $(date -d '1 month ago' +%s)
```

## 📚 Documentation

- **[CLI User Manual](docs/CLI_USER_MANUAL.md)** - Complete guide to all CLI features
- **[CLI Command Showcase](docs/CLI_SHOWCASE.md)** - Live examples of every command
- **[Testing Summary](docs/TESTING_SUMMARY.md)** - Quality assurance details

## 🛠️ Configuration

```bash
# Set your worker URL
lastfm-cli config set worker_url https://your-worker.workers.dev

# Use direct Last.fm API (optional)
lastfm-cli config set api_key YOUR_API_KEY

# View all settings
lastfm-cli config list
```

## 🏗️ Architecture

- **Worker**: Rust-based Cloudflare Worker with caching, rate limiting, and CORS support
- **CLI**: Modern command-line interface with authentication, multiple output formats, and comprehensive API coverage
- **Shared Core**: Common types and utilities for consistent behavior

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.
