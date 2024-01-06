use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::ffi::CString;
use std::sync::OnceLock;
use tracing::{event, Level};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, TRUE};
use winapi::um::fileapi::{GetFinalPathNameByHandleA, GetFinalPathNameByHandleW};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
mod msg;
pub use msg::MSG_to_string;
//https://doc.rust-lang.org/book/ch19-06-macros.html

#[macro_export]
macro_rules! declare_init_hook {
    ($func_name:ident,$target_func_type:ty, $sync_once_cell_detour:expr,$module_name:expr,$func_symbol:expr,$hook_func:expr) => {
        /// hook: Optional hook. You can overwrite default hook by providing Some(your_hook)
        /// enable: If true, enables hook right after the hooking.
        pub fn $func_name(
            hook: Option<$target_func_type>,
            enable: bool,
        ) -> anyhow::Result<Arc<RwLock<GenericDetour<$target_func_type>>>> {
            use crate::utility::get_module_proc_address;
            event!(
                Level::INFO,
                "Trying to find function {}::{}",
                $module_name,
                $func_symbol
            );

            let opt = get_module_proc_address($module_name, $func_symbol)?;
            if opt.is_none() {
                event!(Level::INFO, "Not found!");
                return Err(anyhow::Error::msg(format!(
                    "{}::{} not found!",
                    $module_name, $func_symbol
                )));
            }
            let address = opt.unwrap();
            event!(Level::INFO, "Found at {:#16x}", address);

            let target: $target_func_type = unsafe { std::mem::transmute(address) };

            let detour;
            if hook.is_some() {
                event!(
                    Level::INFO,
                    "Use your hook(at {:x}) for {}",
                    hook.unwrap() as usize,
                    $func_symbol
                );
                detour = unsafe { GenericDetour::<$target_func_type>::new(target, hook.unwrap()) }?;
            } else {
                event!(Level::INFO, "Use default hook for {}", $func_symbol);
                detour = unsafe { GenericDetour::<$target_func_type>::new(target, $hook_func) }?;
            }

            if enable {
                unsafe { detour.enable() }?;
            }

            let detour = Arc::new(RwLock::new(detour));

            let set_result = $sync_once_cell_detour.set(detour.clone());
            if set_result.is_err() {
                event!(Level::INFO, "OnceLock error!");
                return Err(anyhow::Error::msg("Failed to initialize once cell."));
            }
            assert!($sync_once_cell_detour.get().is_some()); //must

            event!(Level::INFO, "Hooked {}", $func_symbol);
            Ok(detour)
        }
    };
}

#[macro_export]
macro_rules! get_detour {
    ($detour_sync_once_cell:expr) => {
        match &$detour_sync_once_cell.get() {
            Some(detour) => detour.read().unwrap(),
            None => {
                tracing::event!(tracing::Level::ERROR, "Should not happen");
                unreachable!()
            }
        }
    };
}

/// Get module::symbol's address
//wchar_t == u16
#[must_use]
pub fn get_module_proc_address(module: &str, symbol: &str) -> Result<Option<usize>> {
    let symbol = CString::new(symbol)?;

    //call GetModuleHandleW
    let handle = get_module_handle(module)?;

    match unsafe { GetProcAddress(handle, symbol.as_ptr()) } as usize {
        0 => Ok(None),
        n => Ok(Some(n)),
    }
}

#[must_use]
fn get_module_handle(module: &str) -> Result<HINSTANCE> {
    //str to LPCWSTR
    use std::iter;
    let module_str = module
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();

    let handle = unsafe { GetModuleHandleW(module_str.as_ptr()) };
    if handle.is_null() {
        return Err(anyhow::Error::msg(format!(
            "module {} not found! ({})",
            module,
            name_of!(GetModuleHandleW)
        )));
    }
    Ok(handle)
}
