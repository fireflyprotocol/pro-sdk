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

Please install the following tools locally:

- the rust toolchain: https://rustup.rs
- the OpenAPI Generator CLI using npm: https://openapi-generator.tech/docs/installation#npm

### ApiGen

Tool to programmatically generate OpenAPI bindings. To call the appropriate `openapi-generator-cli` command for all target languages, run the following from the `tools/` folder:

```bash
cargo run --bin apigen
```

Or, from anywhere in the repository:

```bash
cargo run --bin apigen --manifest-path $(git rev-parse --show-toplevel)/tools/Cargo.toml
```

Use `--lang` to restrict generation to a single language:

```bash
# --lang python|rust|typescript; or, -l py|rs|ts
cargo run --bin apigen -- --lang python
```

If OpenAPI specs have changed since the last run of `apigen`, the tool will automatically increment the minor version of all SDKs (resetting the patch level to 0), and store hashes of the updated specs in [a versioned file](./.apigen-state) for future reference.
