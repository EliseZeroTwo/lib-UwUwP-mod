use detour::{Function, StaticDetour};

use crate::utils::get_module_symbol_address;

pub fn init_enable_static_hook<T, D>(
    module: &str,
    symbol: &str,
    hook: &StaticDetour<T>,
    closure: D,
) -> Result<(), String>
where
    T: Function,
    D: Fn<T::Arguments, Output = T::Output> + Send + 'static,
{
    let addr = if let Some(addr) = get_module_symbol_address("kernel32.dll", "CreateFileW") {
        addr
    } else {
        return Err(format!("Could not find {} in {}", symbol, module));
    };

    unsafe {
        let target: T = T::from_ptr(std::mem::transmute(addr));
        if hook.initialize(target, closure).is_err() {
            return Err(format!("Could not enable hook of {} in {}", symbol, module));
        }

        if hook.enable().is_err() {
            return Err(format!("Could not enable hook of {} in {}", symbol, module));
        }
    }

    Ok(())
}
