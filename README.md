# avm-plugin-node

avm's native Node.js provider: version install/switching plus automatic
`package.json` script aliases. Works with [avm](https://github.com/PrajaNova/avm)
via avm's plugin marketplace — install with:

```bash
avm plugin add node
```

That fetches this repo's latest compiled release for your platform from
GitHub — no Rust toolchain or network access needed beyond the one
download. See [Release process](#release-process) below if you're
building/publishing this repo itself.

## Features

- **Live version index** — `avm node versions` reads
  [`nodejs.org/dist/index.json`](https://nodejs.org/dist/index.json)
  directly, so every real Node release is available immediately, no
  hardcoded/stale list.
- **Version install & switching** — per-project (local) and machine-wide
  (global) pins, same model as nvm/fnm/asdf, resolved through avm's config.
- **`package.json` script aliases** — if a project has a `package.json`
  with a `scripts` block, avm exposes each script as a runnable alias
  (`avm <script-name>`) automatically, with the right package manager
  (`npm`/`pnpm`/`yarn`/`bun`) chosen from the lockfile present. This part
  runs inside `avm-cli` itself (not through the plugin protocol), so it
  works even before you've run `avm plugin add node`.
- **Global packages resolve across local version switches** — `npm install
  -g <pkg>` under your global Node version stays reachable even when a
  different local Node version is active in a project (avm's shim
  resolution tries the local pin's bin dir first, then falls back to the
  global pin's).

## Commands

Once installed (`avm plugin add node`), everything is under `avm node`:

| Command | What it does |
| --- | --- |
| `avm node` | Interactive menu (list / browse versions / install latest / uninstall / help) |
| `avm node list` | Show the selected and installed versions |
| `avm node versions` | Browse the 10 most recent releases |
| `avm node <major> versions` | e.g. `avm node 20 versions` — every release on that major line |
| `avm node latest versions` | Just the newest release |
| `avm node use <version> [-g\|--global]` | Select an installed version, locally (default) or globally |
| `avm node set <version> [-g\|--global]` | Alias for `use` |
| `avm node install <version\|latest\|N>` | Install (if missing) + auto-pin: local, and global too if nothing's pinned globally yet |
| `avm node install <version> --global` | Install + pin globally only |
| `avm node install <version> --no-pin` | Install without touching any pin |
| `avm node uninstall <version>` | Remove a managed version |

`<version>` accepts an exact version (`20.11.1`), a bare major (`20` —
resolves to that major's latest), or `latest`.

Package.json scripts work without any extra command — from a directory
with a `package.json`:

```bash
avm which start     # shows where the "start" alias resolves to
avm start            # runs it (npm/pnpm/yarn/bun run start, manager auto-detected)
```

## Environment

No special env vars — Node doesn't need one (unlike `JAVA_HOME` or
`ANDROID_HOME`). avm just puts the selected version's `bin/` on `PATH`.

## Release process

Tag-triggered (`vX.Y.Z`) GitHub Actions workflow builds
`avm-plugin-node_<os>_<arch>.tar.gz` for `linux_amd64`, `linux_arm64`, and
`darwin_arm64`, and publishes them as a GitHub Release — that's what `avm
plugin add node` downloads. See
[`avm-marketplace`](https://github.com/PrajaNova/avm-marketplace) for the
registry entry that points at this repo, and the main
[avm repo](https://github.com/PrajaNova/avm)'s
`docs/migration/PLUGIN_PROTOCOL.md` for the full wire protocol this
executable speaks (`manifest`, `versions`, `is-installed`,
`installed-versions`, `executable-path`, `env-vars`, `install`,
`uninstall`).

```bash
cargo build --release
# binary at target/release/avm-plugin-node
```
