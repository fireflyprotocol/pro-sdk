OPENAPI_GEN_VERSION := 7.13.0
OPENAPI_GEN_IMAGE   := openapitools/openapi-generator-cli:v$(OPENAPI_GEN_VERSION)

SPEC := resources/bluefin-api.yaml

DOCKER_RUN := docker run --rm -v $(CURDIR):/work -w /work $(OPENAPI_GEN_IMAGE) generate \
	--input-spec /work/$(SPEC)

.PHONY: generate generate-ts generate-py generate-rs version-bump help

# --- Code Generation ---

version-bump: ## Detect spec changes and bump SDK versions
	python3 scripts/version_bump.py

generate: generate-ts generate-py generate-rs ## Generate all SDK clients

generate-ts: ## Generate TypeScript client
	$(DOCKER_RUN) \
		--config /work/ts/sdk/openapitools.json \
		--generator-name typescript-axios \
		--output /work/ts/sdk/src

generate-py: ## Generate Python client
	$(DOCKER_RUN) \
		--config /work/python/sdk/config.yaml \
		--generator-name python \
		--output /work/python/sdk/src

generate-rs: ## Generate Rust client
	rm -rf rust/gen/bluefin_api
	$(DOCKER_RUN) \
		--config /work/rust/gen/config.yaml \
		--generator-name rust \
		--output /work/rust/gen/bluefin_api

generate-all: version-bump generate ## Generate all SDK clients with version bump

# --- Help ---

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
