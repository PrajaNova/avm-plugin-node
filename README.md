# avm-plugin-node

avm's native Node.js provider — version listing (live from
`nodejs.org/dist/index.json`), install, and env resolution — plus
package.json script detection used by [avm](https://github.com/PrajaNova/avm)
itself.

Speaks avm's plugin protocol, documented in the main repo. Built as both a
library (consumed directly by `avm-cli` for package.json script parsing) and
a standalone `avm-plugin-node` executable that `avm-bin` discovers and
drives for all version/install/env operations.

```bash
cargo build --release
# binary at target/release/avm-plugin-node
```
