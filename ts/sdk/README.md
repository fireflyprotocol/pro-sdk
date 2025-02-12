## @bluefin/bluefin-pro-sdk


### Building

To build and compile the typescript sources to javascript use:
```
npm install
npm run build
```

### Publishing

First build the package then run `npm publish`

### Consuming

navigate to the folder of your consuming project and run one of the following commands.

_published:_

```
npm install @bluefin/api-client@1.0.0 --save
```

_unPublished (not recommended):_

```
npm install PATH_TO_GENERATED_PACKAGE --save
```



Generated via 
```
openapi-generator generate -i ../resources/bluefin-api.yaml -c ./sdk/openapitools.json -g typescript-axios -o sdk/src --skip-validate-spec
```