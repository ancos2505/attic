# svelte5-spa


## Clean cached npm and libraries
```sh
rm -rf dist node_modules package-lock.json
npm cache clean --target --force
```

## Getting Started
```sh
npm i
npm run build
python3 -mhttp.server -d ./dist
```