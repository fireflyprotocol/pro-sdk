# --- Code Generation ---
.PHONY: generate
generate: generate-ts generate-py generate-rs ## Generate all SDK clients

.PHONY: generate-ts
generate-ts: ## Generate TypeScript client
	apigen -l ts

.PHONY: generate-py
generate-py: ## Generate Python client
	apigen -l python

.PHONY: generate-rs
generate-rs: ## Generate Rust client
	rm -rf rust/gen/bluefin_api
	apigen -l rust

# --- Help ---
.PHONY: help
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
