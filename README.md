# Bluefin Pro SDK

Software Development Kit to interact with the Bluefin Pro API and contracts.

We are actively working on the docs.

For now please use the readme that is located inside each supported language directory.

## Generate OpenAPI spec
1. Bundle the openapi specs into one
```bash
docker run --rm -v $PWD:/spec redocly/cli bundle -o /spec/out/bluefin.bundle.yaml /spec/resources/bluefin-api.yaml
```
2. View the spec
```bash
docker run --rm -v $PWD/out:/spec -p 4000:4000 redocly/cli preview
```

## Dev Tools

### ApiGen

Tool to programmatically generate OpenAPI code without explicitely running the openapi-generator-cli command.

Navigate to the root of the repository.
```bash
cargo install --path tools
```

This will install the dev tools to your cargo toolchain.

Now you can call apigen to generate OpenAPI code for all 3 languages by calling:

Rust:
```bash
apigen -l rust
```

Python:
```bash
apigen -l python
```

Typescript:
```bash
apigen -l ts
```

In case of any updates, periodically rebuild the dev tools by running the following command in the the root of the repository:
```bash
cargo install --path tools
```
