![npm](https://img.shields.io/npm/v/globalkey) ![downloads](https://img.shields.io/npm/dm/globalkey)

# GlobalKey

## Building

```shell
cargo install nj-cli

nj-cli build --release
```

## Calling from node

```shell
npm i globalkey

# or

yarn add globalkey
```

```node
const globalkey = require("globalkey");

function keyCallBack(keys) {
  console.log(keys);
}

globalkey.start(keyCallBack);
```
