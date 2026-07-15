# Multi-stage Dockerfile for KodeVibe

# Build stage
FROM golang:1.26-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git ca-certificates tzdata

# Set working directory
WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download && go mod verify

# Copy source code
COPY . .

# Build the CLI and server binaries
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -ldflags '-extldflags "-static" -s -w' -o kodevibe ./cmd/cli
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -ldflags '-extldflags "-static" -s -w' -o kodevibe-server ./cmd/server

# Final stage
FROM alpine:latest

# Install runtime dependencies
RUN apk --no-cache add ca-certificates git curl bash

# Create non-root user
RUN addgroup -g 1001 -S kodevibe && \
    adduser -u 1001 -S kodevibe -G kodevibe

# Set working directory
WORKDIR /app

# Copy binaries from builder stage
COPY --from=builder /app/kodevibe /usr/local/bin/kodevibe
COPY --from=builder /app/kodevibe-server /usr/local/bin/kodevibe-server

# Copy configuration files (if any)
COPY --from=builder /app/.kodevibe.yaml /app/.kodevibe.yaml

# Change ownership to non-root user
RUN chown -R kodevibe:kodevibe /app

# Switch to non-root user
USER kodevibe

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose port
EXPOSE 8080

# Default command (can be overridden)
CMD ["kodevibe-server"]

# Labels for metadata
LABEL maintainer="KodeVibe Team"
LABEL version="1.0.0"
LABEL description="KodeVibe - Advanced Code Quality Scanner"
LABEL org.opencontainers.image.source="https://github.com/KooshaPari/KodeVibe-Go"
LABEL org.opencontainers.image.documentation="https://github.com/KooshaPari/KodeVibe-Go/blob/main/README.md"
LABEL org.opencontainers.image.licenses="MIT"