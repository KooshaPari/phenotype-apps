# KDesktopVirt Deployment Guide

This guide covers production deployment of KDesktopVirt across various environments.

## 🐳 Docker Deployment

### Quick Start

```bash
# Pull and run the latest image
docker pull ghcr.io/kooshapari/kdesktopvirt:latest

# Run with default configuration
docker run -d \
  --name kdesktopvirt \
  -p 3000:3000 \
  -p 6080:6080 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v kdesktopvirt-data:/var/lib/kdesktopvirt \
  ghcr.io/kooshapari/kdesktopvirt:latest
```

### Production Configuration

```yaml
# docker-compose.yml
version: '3.8'
services:
  kdesktopvirt:
    image: ghcr.io/kooshapari/kdesktopvirt:latest
    ports:
      - "3000:3000"
      - "6080:6080"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - ./config:/etc/kdesktopvirt
      - kdesktopvirt-data:/var/lib/kdesktopvirt
      - kdesktopvirt-logs:/var/log/kdesktopvirt
    environment:
      - KDESKTOPVIRT_LOG_LEVEL=info
      - KDESKTOPVIRT_ENABLE_WEB_UI=true
      - KDESKTOPVIRT_MAX_SESSIONS=10
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      
  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data
    restart: unless-stopped
    
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    restart: unless-stopped
    
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./monitoring/grafana:/etc/grafana/provisioning
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=changeme
    restart: unless-stopped

volumes:
  kdesktopvirt-data:
  kdesktopvirt-logs:
  redis-data:
  prometheus-data:
  grafana-data:
```

## ☸️ Kubernetes Deployment

### Namespace Setup

```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: kdesktopvirt
  labels:
    name: kdesktopvirt
```

### ConfigMap

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: kdesktopvirt-config
  namespace: kdesktopvirt
data:
  config.toml: |
    [general]
    container_runtime = "docker"
    default_desktop = "kubuntu"
    log_level = "info"
    
    [resources]
    default_memory_mb = 2048
    default_cpu_cores = 2
    max_sessions = 10
    
    [security]
    enable_encryption = true
    enable_mfa = false
    
    [mcp]
    server_port = 3001
    enable_tools = true
```

### Deployment

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kdesktopvirt
  namespace: kdesktopvirt
spec:
  replicas: 3
  selector:
    matchLabels:
      app: kdesktopvirt
  template:
    metadata:
      labels:
        app: kdesktopvirt
    spec:
      containers:
      - name: kdesktopvirt
        image: ghcr.io/kooshapari/kdesktopvirt:latest
        ports:
        - containerPort: 3000
          name: http
        - containerPort: 6080
          name: vnc
        env:
        - name: KDESKTOPVIRT_CONFIG_PATH
          value: "/etc/kdesktopvirt/config.toml"
        - name: KDESKTOPVIRT_LOG_LEVEL
          value: "info"
        volumeMounts:
        - name: config
          mountPath: /etc/kdesktopvirt
          readOnly: true
        - name: docker-sock
          mountPath: /var/run/docker.sock
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "2"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 10
      volumes:
      - name: config
        configMap:
          name: kdesktopvirt-config
      - name: docker-sock
        hostPath:
          path: /var/run/docker.sock
          type: Socket
```

### Service

```yaml
# service.yaml
apiVersion: v1
kind: Service
metadata:
  name: kdesktopvirt
  namespace: kdesktopvirt
spec:
  selector:
    app: kdesktopvirt
  ports:
  - name: http
    port: 80
    targetPort: 3000
  - name: vnc
    port: 6080
    targetPort: 6080
  type: ClusterIP
```

### Ingress

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: kdesktopvirt
  namespace: kdesktopvirt
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/proxy-body-size: "100m"
    nginx.ingress.kubernetes.io/proxy-read-timeout: "300"
    nginx.ingress.kubernetes.io/proxy-send-timeout: "300"
spec:
  tls:
  - hosts:
    - kdesktopvirt.yourdomain.com
    secretName: kdesktopvirt-tls
  rules:
  - host: kdesktopvirt.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: kdesktopvirt
            port:
              number: 80
```

### Deploy to Kubernetes

```bash
# Apply all manifests
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml

# Check deployment status
kubectl get pods -n kdesktopvirt
kubectl logs -f deployment/kdesktopvirt -n kdesktopvirt
```

## 🚀 Cloud Deployment

### AWS ECS

```json
{
  "family": "kdesktopvirt",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::account:role/kdesktopvirtTaskRole",
  "containerDefinitions": [
    {
      "name": "kdesktopvirt",
      "image": "ghcr.io/kooshapari/kdesktopvirt:latest",
      "portMappings": [
        {
          "containerPort": 3000,
          "protocol": "tcp"
        },
        {
          "containerPort": 6080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "KDESKTOPVIRT_LOG_LEVEL",
          "value": "info"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/kdesktopvirt",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

### Google Cloud Run

```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: kdesktopvirt
  annotations:
    run.googleapis.com/ingress: all
    run.googleapis.com/execution-environment: gen2
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/maxScale: "10"
        run.googleapis.com/cpu-throttling: "false"
        run.googleapis.com/memory: "2Gi"
        run.googleapis.com/cpu: "1"
    spec:
      containers:
      - image: ghcr.io/kooshapari/kdesktopvirt:latest
        ports:
        - containerPort: 3000
        env:
        - name: KDESKTOPVIRT_LOG_LEVEL
          value: "info"
        resources:
          limits:
            memory: "2Gi"
            cpu: "1"
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `KDESKTOPVIRT_CONFIG_PATH` | Path to config file | `~/.kdesktopvirt/config.toml` |
| `KDESKTOPVIRT_LOG_LEVEL` | Log level | `info` |
| `KDESKTOPVIRT_DOCKER_HOST` | Docker daemon socket | `unix:///var/run/docker.sock` |
| `KDESKTOPVIRT_DATA_DIR` | Data directory | `/var/lib/kdesktopvirt` |
| `KDESKTOPVIRT_ENABLE_WEB_UI` | Enable web interface | `true` |
| `KDESKTOPVIRT_MAX_SESSIONS` | Maximum concurrent sessions | `10` |

### Security Configuration

```toml
[security]
enable_encryption = true
enable_mfa = true
vault_path = "/etc/kdesktopvirt/vault"
allowed_origins = ["https://yourdomain.com"]

[auth]
jwt_secret = "your-jwt-secret"
session_timeout = 3600
enable_oauth = true

[oauth.google]
client_id = "your-client-id"
client_secret = "your-client-secret"
```

## 📊 Monitoring

### Prometheus Metrics

KDesktopVirt exposes metrics at `/metrics`:

- `kdesktopvirt_sessions_total` - Total active sessions
- `kdesktopvirt_requests_total` - HTTP requests by endpoint
- `kdesktopvirt_session_duration_seconds` - Session duration histogram
- `kdesktopvirt_memory_usage_bytes` - Memory usage
- `kdesktopvirt_cpu_usage_percent` - CPU usage

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "KDesktopVirt Monitoring",
    "panels": [
      {
        "title": "Active Sessions",
        "type": "graph",
        "targets": [
          {
            "expr": "kdesktopvirt_sessions_total"
          }
        ]
      },
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(kdesktopvirt_requests_total[5m])"
          }
        ]
      }
    ]
  }
}
```

## 🔒 Security Best Practices

### Network Security

1. **Firewall Rules**: Restrict access to management ports
2. **TLS/SSL**: Enable HTTPS for all connections
3. **VPN**: Use VPN for administrative access
4. **Network Segmentation**: Isolate KDesktopVirt network

### Container Security

1. **Non-root User**: Run containers as non-root
2. **Read-only Filesystem**: Mount filesystems as read-only
3. **Resource Limits**: Set CPU and memory limits
4. **Security Scanning**: Regular vulnerability scans

### Data Protection

1. **Encryption at Rest**: Encrypt persistent volumes
2. **Encryption in Transit**: TLS for all communications
3. **Backup Strategy**: Regular automated backups
4. **Access Controls**: RBAC and least privilege

## 🔄 Backup and Recovery

### Backup Script

```bash
#!/bin/bash
# backup-kdesktopvirt.sh

BACKUP_DIR="/backups/kdesktopvirt"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup configuration
cp -r /etc/kdesktopvirt "$BACKUP_DIR/config_$DATE"

# Backup data
tar -czf "$BACKUP_DIR/data_$DATE.tar.gz" /var/lib/kdesktopvirt

# Backup database
docker exec kdesktopvirt-db pg_dump -U postgres kdesktopvirt > "$BACKUP_DIR/db_$DATE.sql"

# Cleanup old backups (keep 7 days)
find "$BACKUP_DIR" -type f -mtime +7 -delete

echo "Backup completed: $BACKUP_DIR"
```

### Recovery Process

```bash
#!/bin/bash
# restore-kdesktopvirt.sh

BACKUP_FILE="$1"
RESTORE_DIR="/tmp/kdesktopvirt-restore"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup-file>"
    exit 1
fi

# Stop services
docker-compose down

# Extract backup
mkdir -p "$RESTORE_DIR"
tar -xzf "$BACKUP_FILE" -C "$RESTORE_DIR"

# Restore configuration
cp -r "$RESTORE_DIR/config"/* /etc/kdesktopvirt/

# Restore data
cp -r "$RESTORE_DIR/data"/* /var/lib/kdesktopvirt/

# Start services
docker-compose up -d

echo "Restore completed"
```

## 📈 Scaling

### Horizontal Scaling

1. **Load Balancer**: Distribute requests across instances
2. **Session Affinity**: Route users to same instance
3. **Shared Storage**: Use distributed storage for sessions
4. **Database Clustering**: Scale database layer

### Vertical Scaling

1. **Resource Monitoring**: Monitor CPU/memory usage
2. **Auto-scaling**: Configure HPA in Kubernetes
3. **Resource Limits**: Adjust container limits
4. **Performance Tuning**: Optimize configuration

## 🛠️ Troubleshooting

### Common Issues

**Sessions not starting:**
```bash
# Check Docker daemon
docker version
docker info

# Check logs
docker logs kdesktopvirt
kubectl logs -f deployment/kdesktopvirt -n kdesktopvirt
```

**High memory usage:**
```bash
# Monitor resource usage
docker stats
kubectl top pods -n kdesktopvirt

# Check session limits
curl http://localhost:3000/api/sessions
```

**Network connectivity:**
```bash
# Test connectivity
curl -I http://localhost:3000/health
telnet localhost 6080

# Check firewall
iptables -L
ufw status
```

### Performance Optimization

1. **Resource Allocation**: Increase CPU/memory limits
2. **Session Cleanup**: Configure session timeouts
3. **Cache Configuration**: Optimize Redis settings
4. **Network Optimization**: Tune network parameters

---

For additional support, see:
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Performance Tuning](PERFORMANCE.md)
- [Security Guide](SECURITY.md)