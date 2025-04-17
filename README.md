# Bluefin Pro SDK

Software Development Kit to interact with the Bluefin Pro API and contracts.

We are actively working on the docs.

For now please use the readme that is located inside each supported language directory.

## Generate OpenAPI spec
1. Bundle the openapi specs into one
```bash
docker run --rm -v $PWD:/spec redocly/cli bundle -o /spec/out/bluefin.bundle.yaml /spec/resources/v1/bluefin-api.yaml
```
2. View the spec
```bash
docker run --rm -v $PWD/out:/spec -p 4000:4000 redocly/cli preview
```
