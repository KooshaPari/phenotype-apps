# Kwality Docker Issue Report

**Date:** 2026-05-28
**Status:** Identified - OrbStack Docker Stopped
**Impact:** Integration tests fail at container initialization

## Error

```
ERROR kwality_runtime_validator::container: Failed to ping Docker daemon: error trying to connect: No such file or directory (os error 2)
Error: Failed to initialize container manager
Caused by: Docker daemon not accessible
```

## Root Cause

OrbStack Docker service is stopped. The Docker socket is not available:
- Expected: `/var/run/docker.sock` or `~/.orbstack/run/docker.sock`
- Actual: Socket does not exist

## Verification

```bash
# Check OrbStack status
orb list
# Output: headscale  stopped  ubuntu  noble  arm64  1.3 GB

# Check Docker socket
ls -la /var/run/docker.sock
# Output: No such file or directory
```

## Resolution

Start OrbStack Docker service:

```bash
# Option 1: Via OrbStack UI
# Open OrbStack app and start Docker

# Option 2: Via CLI (if available)
orb docker start

# Option 3: Restart OrbStack
# System Settings > General > Login Items > OrbStack > Restart
```

After Docker is running, re-run tests:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/kwality
go test ./tests/integration/... -v -count=1
```

## Related

- Issue affects: `TestRuntimeValidatorTestSuite` integration tests
- Container manager: `engines/runtime-validator/src/container.rs`
- Test file: `tests/integration/runtime_validator_test.go`
