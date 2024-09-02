fn main() {
  cc::Build::new()
      .include("wasm-micro-runtime/core/iwasm/include")
      .include("wasm-micro-runtime/core/shared/include")
      .file("wasm-micro-runtime/core/iwasm/aot/aot_runtime.c")
      .file("wasm-micro-runtime/core/iwasm/common/arch/arch_base.c")
      .file("wasm-micro-runtime/core/iwasm/common/arch/arch_config.c")
      .compile("libwamr.a");
}
