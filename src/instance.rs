use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use anyhow::{bail, Context, Result};
use containerd_shim_wasm::container::{
    Engine, Entrypoint, Instance, RuntimeContext, Stdio, WasmBinaryType,
};
use containerd_shim_wasm::sandbox::WasmLayer;

// Replace this with your custom LinuxRlimit struct:
use crate::utils::rlimits::LinuxRlimit;


// WAMR imports
use wamr_runtime::{self as wasm_runtime, WasmModule, WasmExecEnv, WasmFunction};  // these are the external crates

// Define the WAMR Instance type
pub type WamrInstance = Instance<WamrEngine<DefaultConfig>>;

// Define the WAMR Engine struct
#[derive(Clone)]
pub struct WamrEngine<T: WamrConfig> {
    engine: wasm_runtime::WasmRuntime, // WAMR runtime engine
    config_type: PhantomData<T>,
}

// Define the default configuration
#[derive(Clone)]
pub struct DefaultConfig {}

impl WamrConfig for DefaultConfig {
    fn new_config() -> wasm_runtime::WasmRuntimeConfig {
        wasm_runtime::WasmRuntimeConfig::default()
    }
}

// Define the WAMR configuration trait
pub trait WamrConfig: Clone + Sync + Send + 'static {
    fn new_config() -> wasm_runtime::WasmRuntimeConfig;
}

// Implement the Default trait for WamrEngine
impl<T: WamrConfig> Default for WamrEngine<T> {
    fn default() -> Self {
        wasm_runtime::init();
        let config = T::new_config();
        let engine = wasm_runtime::WasmRuntime::new(config);
        Self {
            engine,
            config_type: PhantomData,
        }
    }
}

// Define the LinuxRlimit struct
#[derive(Debug, Clone)]
pub struct LinuxRlimit {
    pub r#type: String,
    pub hard: u64,
    pub soft: u64,
}

// Implement the `Engine` trait for `WamrEngine`.
impl<T: WamrConfig> Engine for WamrEngine<T> {
    fn name() -> &'static str {
        "wamr"
    }

    fn run_wasi(&self, ctx: &impl RuntimeContext, stdio: Stdio) -> Result<i32> {
        log::info!("setting up wasi");
        let envs: Vec<_> = std::env::vars().collect();
        let Entrypoint {
            source,
            func,
            arg0: _,
            name: _,
        } = ctx.entrypoint();

        log::info!("building wasi context");
        let wasi_ctx = prepare_wasi_ctx(ctx, envs)?;
        let module = self.load_module(&source)?;
        let status = self.execute(&module, wasi_ctx, &func, stdio)?;

        Ok(status.unwrap_or_else(|err| {
            log::error!("Execution error: {:?}", err);
        }))
    }

    fn precompile(&self, layers: &[WasmLayer]) -> Result<Vec<Option<Vec<u8>>>> {
        log::warn!("Precompilation not supported in WAMR");
        Ok(layers.iter().map(|_| None).collect())
    }

    fn can_precompile(&self) -> Option<String> {
        let mut hasher = DefaultHasher::new();
        self.engine
            .precompile_compatibility_hash()
            .hash(&mut hasher);
        Some(hasher.finish().to_string())
    }
}

// Implement additional methods for `WamrEngine`
impl<T: std::clone::Clone + Sync + WamrConfig + Send + 'static> WamrEngine<T> {
    fn load_module(&self, wasm_binary: &[u8]) -> Result<WasmModule> {
        let module = wasm_runtime::load_module(wasm_binary)
            .context("Failed to load wasm module")?;
        Ok(module)
    }

    fn execute(
        &self,
        module: &WasmModule,
        wasi_ctx: WasiCtx,
        func: &String,
        stdio: Stdio,
    ) -> Result<std::prelude::v1::Result<(), anyhow::Error>, anyhow::Error> {
        log::debug!("Loading WASM function");
        let exec_env = wasm_runtime::create_exec_env(module)?;

        let function = wasm_runtime::lookup_function(exec_env, func)
            .context("Failed to find wasm function")?;

        log::debug!("Executing WASM function");
        stdio.redirect()?;
        wasm_runtime::call_wasm_function(exec_env, function, &[])?;

        Ok(Ok(()))
    }
}

// Prepare WAMR WASI contexts
fn prepare_wasi_ctx(
    ctx: &impl RuntimeContext,
    _envs: Vec<(String, String)>,
) -> Result<WasiCtx, anyhow::Error> {
    // Implement WAMR WASI context preparation if needed
    Ok(WasiCtx {
        // Placeholder fields for WASI contexts
    })
}

// The stub implementation for ScmpFilterContext with the get_notify_fd method
pub struct ScmpFilterContext;

impl ScmpFilterContext {
    pub fn get_notify_fd(&self) -> i32 {
        // Return a default or dummy value
        -1
    }
}
