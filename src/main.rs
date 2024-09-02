use containerd_shim_wasm::sandbox::cli::{revision, shim_main, version};
use crate::instance::WamrInstance;  

fn main() {
    shim_main::<WamrInstance>("wamr", version!(), revision!(), "v1", None);
}
