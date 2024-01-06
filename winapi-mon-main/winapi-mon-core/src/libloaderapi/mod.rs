mod get_proc_address;
mod load_library;
pub use get_proc_address::{hook_GetProcAddress, GetProcAddressDetour};
pub use load_library::{
    hook_LoadLibraryA, hook_LoadLibraryW, LoadLibraryADetour, LoadLibraryWDetour,
};
