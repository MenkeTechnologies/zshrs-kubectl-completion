# zshrs-kubectl-completion

`kubectl` completion as a native
[zshrs](https://github.com/MenkeTechnologies/zshrs) plugin.

Instead of a large static, version-pinned `_kubectl` completion function,
this delegates to **cobra's built-in completion protocol** —
`kubectl __complete <args…> <partial>` — which every modern kubectl exposes.
It returns candidates (subcommands, flags, and live resources) for *the
kubectl version you actually have installed*, so there is nothing to keep in
sync with kubectl releases.

```text
kubectl <TAB>              → annotate api-resources apply attach auth …
kubectl g<TAB>            → get
kubectl config <TAB>      → current-context get-contexts use-context …
kubectl get -<TAB>        → --namespace --output --selector …
kubectl get pod <TAB>     → (live pod names, when a cluster is reachable)
```

Requires `kubectl` on `PATH`. Subcommand/flag completion works offline;
resource completion queries the cluster (exactly as a `_kubectl` completion's
`kubectl get` calls do).

## Install

```sh
zpm load MenkeTechnologies/zshrs-kubectl-completion
```

Put that one line in your `.zshrc`.
[zpm](https://github.com/MenkeTechnologies/zshrs/blob/main/docs/ZPM.md),
zshrs's package manager, installs the plugin on the first shell start — clones
it, runs `cargo build --release`, and `zmodload -R`s the resulting
`libkubectl_completion` — then loads it from the store, zero-network, on every
start after. No separate install step; then `kubectl <TAB>` completes.

### Manual build

```sh
cargo build --release
zmodload -R ./target/release/libkubectl_completion.dylib   # .so on Linux
# (after compinit)
kubectl get po<TAB>
```

## How it works

A `completions:` generator in the `zshrs-plugin` SDK receives the current
command line, runs `kubectl __complete` with those words, and emits the
candidate column of cobra's `name\tdescription` output (stopping at the
trailing `:<directive>` line). See the zshrs plugin porting guide:
[docs/PORTING_ZSH_PLUGIN.md](https://github.com/MenkeTechnologies/zshrs/blob/main/docs/PORTING_ZSH_PLUGIN.md).

## License

MIT. An independent implementation (delegates to `kubectl __complete`);
inspired by [nnao45/zsh-kubectl-completion](https://github.com/nnao45/zsh-kubectl-completion)
but shares no code with it. See [LICENSE](LICENSE).
