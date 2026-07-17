```
██╗  ██╗██╗   ██╗██████╗ ███████╗ ██████╗████████╗██╗     
██║ ██╔╝██║   ██║██╔══██╗██╔════╝██╔════╝╚══██╔══╝██║     
█████╔╝ ██║   ██║██████╔╝█████╗  ██║        ██║   ██║     
██╔═██╗ ██║   ██║██╔══██╗██╔══╝  ██║        ██║   ██║     
██║  ██╗╚██████╔╝██████╔╝███████╗╚██████╗   ██║   ███████╗
╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝ ╚═════╝   ╚═╝   ╚══════╝
                                                          
```

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![zshrs plugin](https://img.shields.io/badge/zshrs-native%20plugin-blue.svg)](https://github.com/MenkeTechnologies/zshrs)

### `[LIVE KUBECTL COMPLETION — COMPILED]`

> *"delegates to kubectl __complete — always in sync with your kubectl."*

## `[NATIVE ZSHRS PLUGIN]`

`kubectl` completion as a native [zshrs](https://github.com/MenkeTechnologies/zshrs) plugin. Instead of a large static, version-pinned `_kubectl` completion function, this delegates to **cobra's built-in completion protocol** — `kubectl __complete <args…> <partial>` — which every modern kubectl exposes. It returns candidates for *the kubectl version you actually have installed*, so there is nothing to keep in sync with kubectl releases.

### [`zshrs`](https://github.com/MenkeTechnologies/zshrs) &middot; [`znative`](https://github.com/MenkeTechnologies/zshrs/blob/main/docs/ZPM.md)

---

## Table of Contents

- [\[0x00\] Overview](#0x00-overview)
- [\[0x01\] Install](#0x01-install)
- [\[0x02\] Usage](#0x02-usage)
- [\[0x03\] How it works](#0x03-how-it-works)
- [\[0xFF\] License](#0xff-license)

---

## [0x00] OVERVIEW

```text
kubectl <TAB>             → annotate api-resources apply attach auth …
kubectl g<TAB>            → get
kubectl config <TAB>      → current-context get-contexts use-context …
kubectl get -<TAB>        → --namespace --output --selector …
kubectl get pod <TAB>     → (live pod names, when a cluster is reachable)
```

Requires `kubectl` on `PATH`. Subcommand/flag completion works offline; resource completion queries the cluster (exactly as a `_kubectl` completion's `kubectl get` calls do).

---

## [0x01] INSTALL

```sh
znative load MenkeTechnologies/zshrs-kubectl-completion
```

Put that one line in your `.zshrc`. [znative](https://github.com/MenkeTechnologies/zshrs/blob/main/docs/ZPM.md), zshrs's package manager, installs the plugin on the first shell start — clones it, runs `cargo build --release`, and `zmodload -R`s the resulting `libkubectl_completion` — then loads it from the store, zero-network, on every start after. No separate install step; then `kubectl <TAB>` completes.

### Manual build

```sh
cargo build --release
zmodload -R ./target/release/libkubectl_completion.dylib   # .so on Linux
# (after compinit)
kubectl get po<TAB>
```

---

## [0x02] USAGE

After loading, `kubectl <TAB>` completes subcommands, flags, and — when a cluster is reachable — live resource names, for whatever kubectl version is on `PATH`.

---

## [0x03] HOW IT WORKS

A `completions:` generator in the `znative` SDK receives the current command line, runs `kubectl __complete` with those words, and emits the candidate column of cobra's `name\tdescription` output (stopping at the trailing `:<directive>` line). See the zshrs plugin porting guide: [docs/PORTING_ZSH_PLUGIN.md](https://github.com/MenkeTechnologies/zshrs/blob/main/docs/PORTING_ZSH_PLUGIN.md).

---

## [0xFF] LICENSE

MIT. An independent implementation (delegates to `kubectl __complete`); inspired by [nnao45/zsh-kubectl-completion](https://github.com/nnao45/zsh-kubectl-completion) but shares no code with it. See [LICENSE](LICENSE).
