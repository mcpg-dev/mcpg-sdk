# mcpg-sdk

> Umbrella façade over the MCPG plugin-authoring API — one dependency, one prelude.

A thin, library-only façade that re-exports `mcpg-plugin-protocol` (the plugin
traits, request/response types, and manifest schema) and `mcpg-plugin-sdk`
(macros, testing harness, config helpers), so a plugin crate writes
`use mcpg_sdk::prelude::*;` instead of juggling the sub-crates and `async_trait`
by hand. Reach for it when you are writing an MCPG plugin and want a single
line in `[dependencies]`. It is deliberately *not* the gateway binary, the
plugin-host runtime, or the plugin-authoring CLI — those ship as their own
crates, and an umbrella over a binary would drag in runtime dependencies no
plugin author needs.

## What's here

- `mcpg_sdk::api` — the whole `mcpg-plugin-protocol` crate, for the occasional
  type the prelude does not surface.
- `mcpg_sdk::sdk` — the whole `mcpg-plugin-sdk` crate: registration macros, the
  test harness, and config helpers.
- `mcpg_sdk::prelude` — the curated glob import:
  - plugin traits — `BackendPlugin`, `IdentityProviderPlugin`, `ToolGatePlugin`,
    `TransformPlugin`, `WatchStrategyPlugin`, plus `WatchEventSink` and
    `WatchHandle`;
  - manifest types — `PluginManifest`, `PluginClass`, `PluginTier`;
  - call-path types — `PluginContext`, `PluginIdentity`, `BackendRequest`,
    `BackendResponse`, `BackendError`, `GateDecision`, `IdentityResolution`,
    `TransformResult`, `ListedResource`, `ResourcePage`, `WatchEvent`;
  - the `async_trait` attribute macro, needed to implement any of the traits.

The crate carries no logic of its own: every item above is a re-export, so a
type named here is the same type the gateway's plugin host loads.

## Who it is for

Plugin crates authored outside this workspace, which want a single dependency
and the `prelude` glob import. Plugin crates inside the workspace name
`mcpg-plugin-protocol` and `mcpg-plugin-sdk` directly, because they also reach
for the feature flags (`cdylib-export`, `static-firstparty`) that the façade
does not forward.

## Usage

```toml
[dependencies]
mcpg-sdk = "<version>"
```

Renaming the dependency at the Cargo level keeps imports short — the same trick
Tokio users apply to `tokio-util`:

```toml
[dependencies]
mcpg = { package = "mcpg-sdk", version = "<version>" }
```

```rust
use mcpg::prelude::*;

struct AllowAll {
    manifest: PluginManifest,
}

#[async_trait]
impl ToolGatePlugin for AllowAll {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn evaluate_pre_dispatch(
        &self,
        _ctx: &PluginContext,
        _arguments: &serde_json::Value,
        _meta: Option<&serde_json::Value>,
        _config: &serde_json::Value,
    ) -> GateDecision {
        GateDecision::allow()
    }
}
```

`serde_json` is not re-exported by the prelude — plugin crates depend on it
directly, since it is the currency of every argument, result, and config value
crossing the plugin boundary.

The crate targets Rust edition 2024.

## Build / test

```bash
cargo build -p mcpg-sdk
cargo test  -p mcpg-sdk
```

## Licence

Apache-2.0.

## See also

- [Plugins and the plugin protocol](https://mcpg.dev/docs/plugins/plugins-and-protocol)
- [Writing a plugin](https://mcpg.dev/docs/plugins/plugin-authoring)
- `libs/plugin-protocol` and `libs/plugin-sdk` — the two crates re-exported here.
