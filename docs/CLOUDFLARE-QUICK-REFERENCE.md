# Cloudflare Quick Reference

## 🚀 Deployment Commands

### One-Command Deployment

```bash
npm run deploy
```

### Manual Deployment

```bash
npm run build:cf
npm run db:migrate
wrangler deploy
```

### Quick Deploy (Skip Tests)

```bash
npm run deploy:quick
```

## 📊 Monitoring Commands

### View Logs

```bash
npm run logs                    # Real-time logs
wrangler tail --format json     # JSON format
wrangler tail | grep "ERROR"    # Filter errors
```

### Database Management

```bash
npm run db:shell               # Interactive database shell
wrangler d1 info connect-four-db
wrangler d1 execute connect-four-db --command "SELECT * FROM games LIMIT 5;"
```

### Status Checks

```bash
wrangler status                # Deployment status
wrangler whoami                # Authentication status
wrangler d1 info connect-four-db
```

## 🔧 Troubleshooting

### Build Issues

```bash
npm run nuke                   # Clean everything
npm run build:cf              # Rebuild
```

### Database Issues

```bash
wrangler d1 info connect-four-db
wrangler d1 execute connect-four-db --command "SELECT 1;"
```

### Authentication Issues

```bash
wrangler login
wrangler whoami
```

### WASM Issues

```bash
npm run build:wasm-assets
ls -la .open-next/assets/
```

## 🌐 Domain & DNS

### Current Configuration

- **Domain**: connect-4.tre.systems
- **Worker**: connect-four-main
- **Database**: connect-four-db

### DNS Records

```
Type: CNAME
Name: connect-4
Target: connect-four-main.your-subdomain.workers.dev
Proxy: Enabled (orange cloud)
```

## 📋 Environment Variables

### Required Secrets

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

### Set Secrets

```bash
wrangler secret put SECRET_NAME
```

## 🔄 CI/CD

### GitHub Actions

- **Trigger**: Push to main branch
- **Workflow**: `.github/workflows/deploy.yml`
- **Auto-deploys**: Yes
- **Runs tests**: Yes
- **Database migrations**: Yes

### Manual Trigger

```bash
git push origin main
```

## 📈 Performance

### Bundle Size

- **Worker**: ~60KB
- **WASM**: ~1MB
- **Total**: ~161KB first load

### Database Stats

- **Size**: ~618KB
- **Tables**: 2
- **Region**: WEUR

## 🆘 Emergency Commands

### Rollback

```bash
wrangler rollback
```

### Emergency Deploy

```bash
npm run deploy:quick
```

### Check Health

```bash
curl https://connect-4.tre.systems/health
```

## 📞 Support

- **Cloudflare Status**: https://www.cloudflarestatus.com/
- **Wrangler Docs**: https://developers.cloudflare.com/workers/wrangler/
- **D1 Docs**: https://developers.cloudflare.com/d1/
