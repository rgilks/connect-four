# Cloudflare Deployment Guide

This guide covers deploying and managing the Connect Four application on Cloudflare Workers, D1 Database, and custom domains.

## 🚀 Quick Deployment

### One-Command Deployment

```bash
./scripts/deploy.sh
```

This script handles:

- Building WebAssembly assets
- Building for Cloudflare
- Running database migrations
- Deploying to Cloudflare Workers

### Manual Deployment Steps

If you prefer manual control:

```bash
# 1. Build WASM assets
npm run build:wasm-assets

# 2. Build for Cloudflare
npm run build:cf

# 3. Run database migrations
wrangler d1 migrations apply connect-four-db --remote

# 4. Deploy
wrangler deploy
```

## 📋 Prerequisites

### Required Tools

1. **Node.js 20+** and npm
2. **Rust** and Cargo
3. **wasm-pack**: `cargo install wasm-pack --version 0.12.1 --locked`
4. **Wrangler CLI**: `npm install -g wrangler`

### Cloudflare Account Setup

1. **Create Cloudflare Account**: [Sign up here](https://dash.cloudflare.com/sign-up)
2. **Get Account ID**: Found in Cloudflare dashboard
3. **Create D1 Database**:
   ```bash
   wrangler d1 create connect-four-db
   ```
4. **Add Custom Domain**: Configure DNS for your domain

## ⚙️ Configuration

### wrangler.toml

Your current configuration includes:

```toml
name = "connect-four-main"
main = ".open-next/worker.js"
compatibility_date = "2025-06-14"
compatibility_flags = ["nodejs_compat", "global_fetch_strictly_public"]

[assets]
directory = ".open-next/assets"
binding = "ASSETS"

[vars]
ENVIRONMENT = "production"

[observability]
enabled = true
head_sampling_rate = 1

[[routes]]
pattern = "connect-4.tre.systems/*"
zone_name = "tre.systems"

[[d1_databases]]
binding = "DB"
database_name = "connect-four-db"
database_id = "f3b432c2-1e6e-48ce-8bc2-d81b4c3f5b12"
preview_database_id = "connect-four-db-preview"
migrations_dir = "migrations"
```

### Environment Variables

Add any environment variables to `wrangler.toml`:

```toml
[vars]
ENVIRONMENT = "production"
API_KEY = "your-api-key"
```

For secrets, use:

```bash
wrangler secret put SECRET_NAME
```

## 🗄️ Database Management

### D1 Database Commands

```bash
# View database info
wrangler d1 info connect-four-db

# Run migrations
wrangler d1 migrations apply connect-four-db --remote

# Execute SQL commands
wrangler d1 execute connect-four-db --command "SELECT * FROM games LIMIT 10;"

# Open database shell
wrangler d1 execute connect-four-db --interactive

# Backup database
wrangler d1 export connect-four-db --output backup.sql

# Restore database
wrangler d1 execute connect-four-db --file backup.sql
```

### Local Development Database

```bash
# Reset local database
npm run db:local:reset

# Run local migrations
npm run migrate:local
```

## 🔍 Monitoring and Debugging

### View Logs

```bash
# Real-time logs
wrangler tail --format pretty

# Recent logs
wrangler tail --format json

# Filter logs
wrangler tail --format pretty | grep "ERROR"
```

### Performance Monitoring

```bash
# Check deployment status
wrangler status

# View analytics
wrangler analytics

# Check worker performance
wrangler tail --format pretty | grep "duration"
```

### Debugging

```bash
# Test locally
wrangler dev

# Test specific routes
wrangler dev --test-scheduled

# Check configuration
wrangler config
```

## 🔄 CI/CD Integration

### GitHub Actions

Your project includes a GitHub Actions workflow for automatic deployment:

```yaml
# .github/workflows/deploy.yml
name: Deploy to Cloudflare
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - uses: actions/setup-rust@v1
      - run: cargo install wasm-pack --version 0.12.1 --locked
      - run: npm ci
      - run: npm run build:cf
      - run: wrangler d1 migrations apply connect-four-db --remote
      - run: wrangler deploy
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
```

### Environment Secrets

Set up these secrets in your GitHub repository:

- `CLOUDFLARE_API_TOKEN`: Your Cloudflare API token
- `CLOUDFLARE_ACCOUNT_ID`: Your Cloudflare account ID

## 🌐 Custom Domain Setup

### DNS Configuration

1. **Add Domain to Cloudflare**:
   - Go to Cloudflare dashboard
   - Add your domain
   - Update nameservers at your registrar

2. **Configure DNS Records**:

   ```
   Type: CNAME
   Name: connect-4
   Target: connect-four-main.your-subdomain.workers.dev
   Proxy: Enabled (orange cloud)
   ```

3. **SSL/TLS Settings**:
   - Set SSL/TLS mode to "Full (strict)"
   - Enable "Always Use HTTPS"

### Route Configuration

Update `wrangler.toml` with your domain:

```toml
[[routes]]
pattern = "your-domain.com/*"
zone_name = "your-domain.com"
```

## 🔧 Troubleshooting

### Common Issues

1. **Build Failures**:

   ```bash
   # Clean and rebuild
   npm run nuke
   npm run build:cf
   ```

2. **Database Connection Issues**:

   ```bash
   # Check database status
   wrangler d1 info connect-four-db

   # Test connection
   wrangler d1 execute connect-four-db --command "SELECT 1;"
   ```

3. **WASM Loading Issues**:

   ```bash
   # Rebuild WASM assets
   npm run build:wasm-assets

   # Check asset paths
   ls -la .open-next/assets/
   ```

4. **Deployment Failures**:

   ```bash
   # Check wrangler version
   wrangler --version

   # Update wrangler
   npm install -g wrangler@latest

   # Check authentication
   wrangler whoami
   ```

### Performance Optimization

1. **Asset Optimization**:
   - Ensure WASM files are properly cached
   - Use appropriate cache headers
   - Optimize image assets

2. **Database Optimization**:
   - Add indexes for frequently queried columns
   - Use connection pooling
   - Monitor query performance

3. **Worker Optimization**:
   - Minimize bundle size
   - Use appropriate compatibility flags
   - Monitor cold start times

## 📊 Analytics and Monitoring

### Cloudflare Analytics

- **Web Analytics**: Built into Cloudflare dashboard
- **Workers Analytics**: Monitor function execution
- **D1 Analytics**: Database performance metrics

### Custom Monitoring

```bash
# Set up custom metrics
wrangler tail --format json | jq '.metrics'

# Monitor specific endpoints
wrangler tail --format pretty | grep "/api/"
```

## 🔐 Security

### Best Practices

1. **Environment Variables**: Use secrets for sensitive data
2. **CORS Configuration**: Restrict origins appropriately
3. **Rate Limiting**: Implement rate limiting for API endpoints
4. **Input Validation**: Validate all user inputs
5. **HTTPS Only**: Force HTTPS for all requests

### Security Headers

Configure security headers in your application:

```typescript
// Add to your Next.js config
const securityHeaders = [
  {
    key: 'X-Frame-Options',
    value: 'DENY',
  },
  {
    key: 'X-Content-Type-Options',
    value: 'nosniff',
  },
  {
    key: 'Referrer-Policy',
    value: 'origin-when-cross-origin',
  },
];
```

## 📚 Additional Resources

- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [D1 Database Documentation](https://developers.cloudflare.com/d1/)
- [OpenNext Documentation](https://open-next.js.org/)
- [Wrangler CLI Reference](https://developers.cloudflare.com/workers/wrangler/)

## 🆘 Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review Cloudflare Workers logs
3. Check the GitHub Issues for known problems
4. Consult the Cloudflare community forums
