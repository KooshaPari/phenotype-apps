# Jarvis SWE Agent Containerization - Technical Implementation

## 1. Multi-Container Architecture

The containerization strategy uses a multi-container approach with specialized containers for different components of the Jarvis SWE Agent.

### 1.1 Core Agent Container

```dockerfile
# Dockerfile for Jarvis Core Agent
FROM node:18-alpine

# Set working directory
WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY . .

# Set environment variables
ENV NODE_ENV=production
ENV PORT=3000

# Expose port
EXPOSE 3000

# Set non-root user
USER node

# Start application
CMD ["node", "src/index.js"]
```

**Key Components:**
- **Base Image**: `node:18-alpine` for minimal footprint
- **Runtime Dependencies**: Express.js, Socket.IO, MongoDB client, Redis client
- **Configuration**: Environment variables for service discovery, authentication, and logging
- **Resource Limits**: 2 CPU cores, 4GB memory per instance
- **Network Configuration**: Internal service mesh with controlled external access

### 1.2 Terminal Container

```dockerfile
# Dockerfile for Terminal Container
FROM ubuntu:22.04

# Install required packages
RUN apt-get update && apt-get install -y \
    nodejs \
    npm \
    python3 \
    python3-pip \
    git \
    curl \
    wget \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Install node-pty and other dependencies
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY . .

# Create non-root user
RUN useradd -m jarvis
USER jarvis

# Expose port
EXPOSE 3001

# Start terminal service
CMD ["node", "src/terminal-service.js"]
```

**Key Components:**
- **Base Image**: Custom Ubuntu minimal with essential development tools
- **Key Components**: node-pty for pseudo-terminal management
- **Security Hardening**: No root execution, seccomp profiles, capability restrictions
- **Isolation**: Separate network namespace, limited filesystem access
- **Resource Limits**: Dynamic allocation based on active sessions

### 1.3 Browser Container

```dockerfile
# Dockerfile for Browser Container
FROM mcr.microsoft.com/playwright:v1.32.0-focal

# Set working directory
WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY . .

# Create non-root user
RUN useradd -m jarvis
USER jarvis

# Expose port
EXPOSE 3002

# Start browser service
CMD ["node", "src/browser-service.js"]
```

**Key Components:**
- **Base Image**: Playwright-enabled container with Chromium, Firefox, WebKit
- **Security Considerations**: No persistent storage, network isolation, content security policies
- **Performance Optimization**: Shared browser instances with isolated contexts
- **Resource Management**: GPU acceleration when available, memory limits per session

## 2. Container Orchestration

### 2.1 Docker Compose for Development

```yaml
# docker-compose.yml
version: '3.8'

services:
  jarvis-core:
    build:
      context: ./core
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
    environment:
      - MONGODB_URI=mongodb://mongodb:27017/jarvis
      - REDIS_URI=redis://redis:6379
      - TERMINAL_SERVICE_URL=http://jarvis-terminal:3001
      - BROWSER_SERVICE_URL=http://jarvis-browser:3002
    depends_on:
      - mongodb
      - redis
    networks:
      - jarvis-network
    restart: unless-stopped
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G

  jarvis-terminal:
    build:
      context: ./terminal
      dockerfile: Dockerfile
    ports:
      - "3001:3001"
    environment:
      - REDIS_URI=redis://redis:6379
    depends_on:
      - redis
    networks:
      - jarvis-network
    restart: unless-stopped
    security_opt:
      - seccomp=terminal-seccomp.json
    cap_drop:
      - ALL
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 2G

  jarvis-browser:
    build:
      context: ./browser
      dockerfile: Dockerfile
    ports:
      - "3002:3002"
    environment:
      - REDIS_URI=redis://redis:6379
    depends_on:
      - redis
    networks:
      - jarvis-network
    restart: unless-stopped
    security_opt:
      - seccomp=browser-seccomp.json
    cap_drop:
      - ALL
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G

  mongodb:
    image: mongo:6
    volumes:
      - mongodb-data:/data/db
    networks:
      - jarvis-network
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data
    networks:
      - jarvis-network
    restart: unless-stopped

networks:
  jarvis-network:
    driver: bridge

volumes:
  mongodb-data:
  redis-data:
```

### 2.2 Kubernetes for Production

```yaml
# jarvis-core-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jarvis-core
  labels:
    app: jarvis-core
spec:
  replicas: 3
  selector:
    matchLabels:
      app: jarvis-core
  template:
    metadata:
      labels:
        app: jarvis-core
    spec:
      containers:
      - name: jarvis-core
        image: jarvis/core:latest
        resources:
          limits:
            cpu: "2"
            memory: "4Gi"
          requests:
            cpu: "1"
            memory: "2Gi"
        env:
        - name: MONGODB_URI
          valueFrom:
            secretKeyRef:
              name: jarvis-secrets
              key: mongodb-uri
        - name: REDIS_URI
          valueFrom:
            secretKeyRef:
              name: jarvis-secrets
              key: redis-uri
        - name: TERMINAL_SERVICE_URL
          value: "http://jarvis-terminal:3001"
        - name: BROWSER_SERVICE_URL
          value: "http://jarvis-browser:3002"
        ports:
        - containerPort: 3000
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
```

## 3. Inter-Container Communication

### 3.1 Communication Protocol

The communication between containers uses a standardized message format:

```typescript
interface ContainerMessage {
  messageId: string;          // Unique message identifier
  messageType: string;        // Message type for routing
  source: {                   // Source information
    containerId: string;      // Container identifier
    serviceId: string;        // Service identifier
  };
  destination: {              // Destination information
    containerId: string;      // Container identifier
    serviceId: string;        // Service identifier
  };
  timestamp: number;          // Message creation timestamp
  payload: any;               // Message payload
  metadata: {                 // Additional metadata
    priority: number;         // Message priority
    ttl: number;              // Time-to-live in seconds
    correlationId?: string;   // For request-response correlation
  };
}
```

### 3.2 Communication Methods

1. **REST APIs**: For synchronous, request-response interactions
2. **WebSockets**: For real-time, bidirectional communication
3. **Message Queue**: RabbitMQ for asynchronous, reliable messaging

## 4. Security Hardening

### 4.1 Container Security Measures

- **Non-root Users**: All containers run as non-root users
- **Read-only Filesystem**: Root filesystem mounted as read-only where possible
- **Capability Dropping**: Unnecessary Linux capabilities are dropped
- **Seccomp Profiles**: Custom seccomp profiles to restrict system calls
- **Resource Limits**: CPU, memory, and I/O limits to prevent resource exhaustion
- **Network Policies**: Strict network policies to control container communication

### 4.2 Example Seccomp Profile for Terminal Container

```json
{
  "defaultAction": "SCMP_ACT_ERRNO",
  "architectures": ["SCMP_ARCH_X86_64"],
  "syscalls": [
    {
      "names": [
        "accept", "accept4", "access", "arch_prctl", "bind", "brk",
        "capget", "capset", "chdir", "chmod", "chown", "clock_getres",
        "clock_gettime", "clock_nanosleep", "close", "connect",
        "dup", "dup2", "dup3", "epoll_create", "epoll_create1",
        "epoll_ctl", "epoll_pwait", "epoll_wait", "eventfd", "eventfd2",
        "execve", "exit", "exit_group", "faccessat", "fadvise64",
        "fallocate", "fchdir", "fchmod", "fchmodat", "fchown",
        "fchownat", "fcntl", "fdatasync", "fgetxattr", "flistxattr",
        "flock", "fork", "fremovexattr", "fsetxattr", "fstat",
        "fstatfs", "fsync", "ftruncate", "futex", "getcwd",
        "getdents", "getdents64", "getegid", "geteuid", "getgid",
        "getgroups", "getitimer", "getpeername", "getpgid", "getpgrp",
        "getpid", "getppid", "getpriority", "getrandom", "getresgid",
        "getresuid", "getrlimit", "getrusage", "getsid", "getsockname",
        "getsockopt", "gettid", "gettimeofday", "getuid", "getxattr",
        "inotify_add_watch", "inotify_init", "inotify_init1",
        "inotify_rm_watch", "ioctl", "kill", "lchown", "lgetxattr",
        "link", "linkat", "listen", "listxattr", "llistxattr",
        "lremovexattr", "lseek", "lsetxattr", "lstat", "madvise",
        "memfd_create", "mkdir", "mkdirat", "mknod", "mknodat",
        "mlock", "mmap", "mount", "mprotect", "mremap", "msync",
        "munlock", "munmap", "nanosleep", "newfstatat", "open",
        "openat", "pause", "pipe", "pipe2", "poll", "ppoll",
        "prctl", "pread64", "preadv", "prlimit64", "pselect6",
        "pwrite64", "pwritev", "read", "readlink", "readlinkat",
        "readv", "recvfrom", "recvmmsg", "recvmsg", "rename",
        "renameat", "renameat2", "restart_syscall", "rmdir", "rt_sigaction",
        "rt_sigprocmask", "rt_sigreturn", "rt_sigsuspend", "sched_getaffinity",
        "sched_getparam", "sched_getscheduler", "sched_yield",
        "select", "semctl", "semget", "semop", "semtimedop", "sendfile",
        "sendmmsg", "sendmsg", "sendto", "setgid", "setgroups",
        "setitimer", "setpgid", "setpriority", "setregid", "setresgid",
        "setresuid", "setreuid", "setrlimit", "setsid", "setsockopt",
        "setuid", "shutdown", "sigaltstack", "socket", "socketpair",
        "stat", "statfs", "symlink", "symlinkat", "sync", "syncfs",
        "sysinfo", "tgkill", "time", "timer_create", "timer_delete",
        "timer_getoverrun", "timer_gettime", "timer_settime", "timerfd_create",
        "timerfd_gettime", "timerfd_settime", "times", "tkill", "truncate",
        "umask", "uname", "unlink", "unlinkat", "utime", "utimensat",
        "utimes", "vfork", "wait4", "waitid", "write", "writev"
      ],
      "action": "SCMP_ACT_ALLOW"
    }
  ]
}
```

## 5. Resource Management

### 5.1 Resource Allocation Strategies

- **Static Allocation**: Fixed resource limits for predictable workloads
- **Dynamic Allocation**: Adjustable limits based on workload
- **Burstable Resources**: Allow temporary resource spikes
- **Resource Quotas**: Namespace-level resource constraints
- **Priority Classes**: Different priority levels for critical services

### 5.2 Monitoring and Scaling

- **Metrics Collection**: Prometheus for resource usage metrics
- **Autoscaling**: Horizontal Pod Autoscaler for automatic scaling
- **Resource Optimization**: Vertical Pod Autoscaler for resource recommendations
- **Load Balancing**: Service mesh for intelligent traffic routing
