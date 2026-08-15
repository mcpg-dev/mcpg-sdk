//! MCPG SDK — plugin-authoring umbrella.
//!
//! This crate is a thin façade over the plugin-authoring surface: it
//! re-exports `mcpg-plugin-protocol` (traits + types + manifest
//! schema) and `mcpg-plugin-sdk` (macros + testing helpers). Most plugin
//! authors should import from here instead of picking between the
//! sub-crates by hand.
//!
//! ## Quickstart
//!
//! Option A — use the SDK under its real name:
//!
//! ```toml
//! [dependencies]
//! mcpg-sdk = "1"
//! ```
//!
//! ```ignore
//! use mcpg_sdk::prelude::*;
//! ```
//!
//! Option B — rename at the Cargo level so imports read `mcpg::…`
//! (recommended; same pattern Tokio users apply to `tokio-util`):
//!
//! ```toml
//! [dependencies]
//! mcpg = { package = "mcpg-sdk", version = "1" }
//! ```
//!
//! ```ignore
//! use mcpg::prelude::*;
//!
//! struct MyGate;
//!
//! #[async_trait]
//! impl ToolGatePlugin for MyGate {
//!     fn manifest(&self) -> &PluginManifest { /* ... */ }
//!     async fn evaluate(
//!         &self,
//!         _ctx: &PluginContext,
//!         _tool: &str,
//!         _args: &serde_json::Value,
//!     ) -> GateDecision {
//!         GateDecision::Allow
//!     }
//! }
//! ```
//!
//! ## What this crate is, and isn't
//!
//! * **Is**: a curated façade for plugin authors.
//! * **Isn't**: the gateway binary (that's the [`mcpg`] crate — the
//!   flagship install target), the plugin-host runtime
//!   ([`mcpg-plugin-host`]), or the plugin-authoring CLI
//!   ([`mcpg-plugin`]). Those ship as their own crates and are
//!   intentionally NOT re-exported here — an umbrella over a binary
//!   would just drag in runtime deps nobody writing a plugin needs.
//!
//! [`mcpg`]: https://crates.io/crates/mcpg
//! [`mcpg-plugin-host`]: https://crates.io/crates/mcpg-plugin-host
//! [`mcpg-plugin`]: https://crates.io/crates/mcpg-plugin

/// Re-exported plugin API crate. Prefer [`prelude`] for day-to-day use;
/// this module-level alias is here for the occasional type that the
/// prelude doesn't surface.
pub use mcpg_plugin_protocol as api;

/// Re-exported plugin SDK crate (macros, testing harness, config
/// helpers). Same rationale as [`api`].
pub use mcpg_plugin_sdk as sdk;

/// The common set of items a typical plugin author needs. Glob-import
/// this at the top of every plugin file:
///
/// ```ignore
/// use mcpg::prelude::*;
/// ```
pub mod prelude {
    // Traits — one of these is what a plugin actually implements.
    pub use mcpg_plugin_protocol::{
        BackendPlugin, IdentityProviderPlugin, ToolGatePlugin, TransformPlugin, WatchEventSink,
        WatchHandle, WatchStrategyPlugin,
    };

    // Manifest — every plugin declares one of these.
    pub use mcpg_plugin_protocol::{PluginClass, PluginManifest, PluginTier};

    // Request / response + context types that flow through plugin calls.
    pub use mcpg_plugin_protocol::{
        BackendError, BackendRequest, BackendResponse, GateDecision, IdentityResolution,
        ListedResource, PluginContext, PluginIdentity, ResourcePage, TransformResult, WatchEvent,
    };

    // Async-trait macro — needed to implement any of the plugin traits.
    pub use mcpg_plugin_protocol::async_trait;
}
