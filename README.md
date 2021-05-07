# GlobalKey

## Building

```shell
cargo install nj-cli

nj-cli build --release
```

## Calling from node

```node
const globalkey = require("./dist");

function keyCallBack(keys) {
  console.log(keys);
}

globalkey.start(keyCallBack);
```
