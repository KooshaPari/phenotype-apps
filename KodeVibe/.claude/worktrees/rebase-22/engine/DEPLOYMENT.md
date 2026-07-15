# 🚀 KodeVibe Production Deployment Guide

## 📋 Prerequisites

### System Requirements
- **Go 1.21+** (for building from source)
- **Linux/macOS/Windows** (64-bit)
- **Memory**: Minimum 512MB RAM, Recommended 2GB+
- **Storage**: 100MB+ available disk space
- **Network**: HTTP/HTTPS access for integrations

### Dependencies
- Git (for version control integration)
- Optional: Docker (for containerized deployment)
- Optional: Nginx/Apache (for reverse proxy)

## 🔧 Installation Methods

### Method 1: Binary Installation (Recommended)
```bash
# Download latest release binary
curl -sSL https://github.com/KooshaPari/KodeVibe-Go/releases/latest/download/kodevibe-$(uname -s)-$(uname -m) -o kodevibe
chmod +x kodevibe
sudo mv kodevibe /usr/local/bin/

# Verify installation
kodevibe --version
```

### Method 2: Build from Source
```bash
# Clone repository
git clone https://github.com/KooshaPari/KodeVibe-Go.git
cd KodeVibe-Go

# Build binaries
make build

# Install system-wide
sudo make install

# Or install locally
make install-local
```

### Method 3: Docker Deployment
```bash
# Build Docker image
docker build -t kodevibe:latest .

# Run container
docker run -d \
  --name kodevibe \
  -p 8080:8080 \
  -v /path/to/projects:/workspace \
  kodevibe:latest server
```

## ⚙️ Configuration

### Production Configuration File
Create `/etc/kodevibe/config.yaml`:

```yaml
# Production configuration
project:
  name: "Production KodeVibe"
  type: "enterprise"
  environment: "production"

# Server settings
server:
  host: "0.0.0.0"
  port: 8080
  tls: true
  cert_file: "/etc/ssl/certs/kodevibe.crt"
  key_file: "/etc/ssl/private/kodevibe.key"
  
  # Security settings
  auth:
    enabled: true
    jwt_secret: "${JWT_SECRET}"
    session_timeout: "24h"
  
  # Rate limiting
  rate_limit:
    enabled: true
    requests_per_second: 100
    burst_size: 200

# Performance settings
performance:
  max_concurrency: 20
  cache_enabled: true
  cache_ttl: "1h"
  cache_size: "256MB"
  timeout: "5m"

# Logging
logging:
  level: "info"
  format: "json"
  file: "/var/log/kodevibe/app.log"
  max_size: "100MB"
  max_backups: 10
  max_age: 30

# Monitoring
monitoring:
  enabled: true
  metrics_port: 9090
  health_check_port: 8081
  prometheus: true

# Database (if using persistent storage)
database:
  type: "postgres"
  host: "${DB_HOST}"
  port: 5432
  database: "kodevibe"
  username: "${DB_USER}"
  password: "${DB_PASSWORD}"
  ssl_mode: "require"

# Integrations
integrations:
  slack:
    enabled: true
    webhook_url: "${SLACK_WEBHOOK_URL}"
  
  github:
    enabled: true
    token: "${GITHUB_TOKEN}"
    webhook_secret: "${GITHUB_WEBHOOK_SECRET}"
  
  teams:
    enabled: false
    webhook_url: "${TEAMS_WEBHOOK_URL}"

# Security settings
security:
  secret_detection:
    entropy_threshold: 4.5
    pattern_matching: true
  
  vulnerability_scanning:
    enabled: true
    update_interval: "24h"
  
  compliance:
    pci_dss: false
    hipaa: false
    gdpr: true
```

### Environment Variables
Create `/etc/kodevibe/environment`:

```bash
# Database credentials
export DB_HOST="localhost"
export DB_USER="kodevibe"
export DB_PASSWORD="secure_password_here"

# Authentication
export JWT_SECRET="your_jwt_secret_here"

# Integration tokens
export GITHUB_TOKEN="ghp_your_token_here"
export SLACK_WEBHOOK_URL="https://hooks.slack.com/services/..."
export GITHUB_WEBHOOK_SECRET="your_webhook_secret"

# SSL certificates
export SSL_CERT_PATH="/etc/ssl/certs/kodevibe.crt"
export SSL_KEY_PATH="/etc/ssl/private/kodevibe.key"
```

## 🐳 Docker Production Setup

### Dockerfile Optimization
```dockerfile
FROM golang:1.21-alpine AS builder

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o kodevibe-server ./cmd/server

FROM alpine:latest
RUN apk --no-cache add ca-certificates git
WORKDIR /root/

COPY --from=builder /app/kodevibe-server .
COPY --from=builder /app/config/production.yaml ./config/

EXPOSE 8080 9090 8081
CMD ["./kodevibe-server", "--config", "config/production.yaml"]
```

### Docker Compose Setup
```yaml
version: '3.8'

services:
  kodevibe:
    build: .
    ports:
      - "8080:8080"   # Main application
      - "9090:9090"   # Metrics
      - "8081:8081"   # Health checks
    volumes:
      - ./config:/app/config
      - ./logs:/var/log/kodevibe
      - ./data:/app/data
    environment:
      - CONFIG_PATH=/app/config/production.yaml
    depends_on:
      - postgres
      - redis
    restart: unless-stopped

  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: kodevibe
      POSTGRES_USER: kodevibe
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/ssl
    depends_on:
      - kodevibe
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
```

## 🔒 Security Hardening

### SSL/TLS Configuration
```bash
# Generate self-signed certificate (for testing)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout /etc/ssl/private/kodevibe.key \
  -out /etc/ssl/certs/kodevibe.crt

# Set proper permissions
chmod 600 /etc/ssl/private/kodevibe.key
chmod 644 /etc/ssl/certs/kodevibe.crt
```

### Firewall Configuration
```bash
# UFW setup
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw allow 8080/tcp  # KodeVibe (if direct access needed)
sudo ufw enable
```

### User and Permissions
```bash
# Create dedicated user
sudo useradd -r -s /bin/false kodevibe

# Create directories
sudo mkdir -p /var/log/kodevibe
sudo mkdir -p /var/lib/kodevibe
sudo mkdir -p /etc/kodevibe

# Set ownership
sudo chown -R kodevibe:kodevibe /var/log/kodevibe
sudo chown -R kodevibe:kodevibe /var/lib/kodevibe
sudo chown -R kodevibe:kodevibe /etc/kodevibe
```

## 📊 Monitoring and Logging

### Systemd Service
Create `/etc/systemd/system/kodevibe.service`:

```ini
[Unit]
Description=KodeVibe Code Quality Server
After=network.target

[Service]
Type=simple
User=kodevibe
Group=kodevibe
WorkingDirectory=/opt/kodevibe
ExecStart=/usr/local/bin/kodevibe server --config /etc/kodevibe/config.yaml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=kodevibe

# Security measures
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/kodevibe /var/lib/kodevibe

# Environment
EnvironmentFile=/etc/kodevibe/environment

[Install]
WantedBy=multi-user.target
```

### Log Rotation
Create `/etc/logrotate.d/kodevibe`:

```
/var/log/kodevibe/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 kodevibe kodevibe
    postrotate
        systemctl reload kodevibe
    endscript
}
```

### Health Checks
```bash
# Simple health check script
#!/bin/bash
curl -f http://localhost:8081/health || exit 1
```

## 🚀 Deployment Process

### 1. Pre-deployment Checklist
- [ ] System requirements verified
- [ ] Configuration files prepared
- [ ] SSL certificates configured
- [ ] Database setup completed
- [ ] Firewall rules applied
- [ ] Monitoring tools configured

### 2. Deployment Steps
```bash
# 1. Stop existing service (if upgrading)
sudo systemctl stop kodevibe

# 2. Backup current installation
sudo cp /usr/local/bin/kodevibe /usr/local/bin/kodevibe.backup

# 3. Install new binary
sudo cp kodevibe /usr/local/bin/
sudo chmod +x /usr/local/bin/kodevibe

# 4. Update configuration if needed
sudo cp config.yaml /etc/kodevibe/

# 5. Reload systemd and start service
sudo systemctl daemon-reload
sudo systemctl start kodevibe
sudo systemctl enable kodevibe

# 6. Verify deployment
sudo systemctl status kodevibe
curl -f http://localhost:8081/health
```

### 3. Post-deployment Verification
```bash
# Check service status
sudo systemctl status kodevibe

# Check logs
sudo journalctl -u kodevibe -f

# Test API endpoints
curl -k https://localhost:8080/api/v1/health
curl -k https://localhost:8080/api/v1/vibes

# Verify metrics endpoint
curl http://localhost:9090/metrics
```

## 🔄 Backup and Recovery

### Backup Strategy
```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/var/backups/kodevibe"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p $BACKUP_DIR/$DATE

# Backup configuration
cp -r /etc/kodevibe $BACKUP_DIR/$DATE/

# Backup data
cp -r /var/lib/kodevibe $BACKUP_DIR/$DATE/

# Backup database (if using)
pg_dump kodevibe > $BACKUP_DIR/$DATE/database.sql

# Compress backup
tar -czf $BACKUP_DIR/kodevibe_backup_$DATE.tar.gz -C $BACKUP_DIR $DATE
rm -rf $BACKUP_DIR/$DATE

echo "Backup completed: $BACKUP_DIR/kodevibe_backup_$DATE.tar.gz"
```

### Recovery Process
```bash
#!/bin/bash
# restore.sh

BACKUP_FILE=$1

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file>"
    exit 1
fi

# Stop service
sudo systemctl stop kodevibe

# Extract backup
tar -xzf $BACKUP_FILE -C /tmp/

# Restore configuration
sudo cp -r /tmp/kodevibe/etc/kodevibe/* /etc/kodevibe/

# Restore data
sudo cp -r /tmp/kodevibe/var/lib/kodevibe/* /var/lib/kodevibe/

# Restore database (if using)
psql kodevibe < /tmp/kodevibe/database.sql

# Start service
sudo systemctl start kodevibe

echo "Recovery completed"
```

## 📈 Performance Tuning

### System Optimization
```bash
# Increase file descriptor limits
echo "kodevibe soft nofile 65535" >> /etc/security/limits.conf
echo "kodevibe hard nofile 65535" >> /etc/security/limits.conf

# Kernel parameter tuning
echo "net.core.somaxconn = 65535" >> /etc/sysctl.conf
echo "net.ipv4.tcp_max_syn_backlog = 65535" >> /etc/sysctl.conf
sysctl -p
```

### Application Tuning
```yaml
# In config.yaml
performance:
  max_concurrency: 50        # Increase for more CPU cores
  worker_pool_size: 20       # Adjust based on workload
  cache_size: "512MB"        # Increase for better performance
  request_timeout: "10m"     # Adjust based on analysis complexity
```

## 🆘 Troubleshooting

### Common Issues

#### Service Won't Start
```bash
# Check logs
sudo journalctl -u kodevibe --no-pager -l

# Verify configuration
kodevibe config validate --config /etc/kodevibe/config.yaml

# Check permissions
sudo chown -R kodevibe:kodevibe /var/log/kodevibe /var/lib/kodevibe
```

#### High Memory Usage
```bash
# Monitor memory usage
top -p $(pgrep kodevibe)

# Adjust cache settings in config.yaml
performance:
  cache_size: "128MB"  # Reduce if needed
  max_concurrency: 10  # Reduce concurrent operations
```

#### SSL Certificate Issues
```bash
# Check certificate validity
openssl x509 -in /etc/ssl/certs/kodevibe.crt -text -noout

# Verify certificate chain
openssl verify -CAfile /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/kodevibe.crt
```

### Log Analysis
```bash
# Common log patterns
sudo grep -i error /var/log/kodevibe/app.log
sudo grep -i "connection refused" /var/log/kodevibe/app.log
sudo grep -i "timeout" /var/log/kodevibe/app.log
```

## 📞 Support

### Support Channels
- **GitHub Issues**: https://github.com/KooshaPari/KodeVibe-Go/issues
- **Documentation**: https://kodevibe.dev/docs
- **Community**: https://discord.gg/kodevibe

### Emergency Contacts
- **Critical Issues**: Create GitHub issue with "critical" label
- **Security Issues**: Email security@kodevibe.dev

---

## ✅ Production Deployment Checklist

- [ ] System requirements verified
- [ ] Configuration files prepared and validated
- [ ] SSL certificates installed and configured
- [ ] Database setup and migrations completed
- [ ] Firewall rules configured
- [ ] Service user created with proper permissions
- [ ] Systemd service file created and enabled
- [ ] Log rotation configured
- [ ] Monitoring and health checks configured
- [ ] Backup strategy implemented
- [ ] Performance tuning applied
- [ ] Security hardening completed
- [ ] Post-deployment verification passed

---

**KodeVibe is now production-ready! 🚀**

*For additional support and advanced configuration options, refer to the full documentation at https://kodevibe.dev/docs*