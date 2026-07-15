package security

import (
	"os"
	"path/filepath"
)

// writeFileBytes / mkdirAll are test-only filesystem helpers shared by the
// security_test.go suite. They live in their own _test.go file so the
// production build never includes them.

func writeFileBytes(path string, data []byte) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

func mkdirAll(path string) error {
	return os.MkdirAll(path, 0o755)
}
