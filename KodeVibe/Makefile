.PHONY: engine-build engine-test engine-server engine-clean

ENGINE_DIR := engine
ENGINE_BUILD := $(ENGINE_DIR)/build

engine-build:
	@mkdir -p $(ENGINE_BUILD)
	cd $(ENGINE_DIR) && go build -o build/kodevibe ./cmd/cli
	cd $(ENGINE_DIR) && go build -o build/kodevibe-server ./cmd/server

engine-server: engine-build

engine-test:
	cd $(ENGINE_DIR) && go test ./...

engine-clean:
	rm -rf $(ENGINE_BUILD)
