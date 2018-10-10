#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod wasm_imp;
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use self::wasm_imp::*;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
mod std_imp;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
pub use self::std_imp::*;
