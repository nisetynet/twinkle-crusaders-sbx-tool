use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use lazy_static::lazy_static;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::iter::Once;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, HINSTANCE, HMODULE, LPDWORD, LPVOID, PDWORD, TRUE,
};
use winapi::shared::ntdef::NULL;
use winapi::um::libloaderapi::{LoadLibraryA, LoadLibraryW};
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::winnt::{LPCSTR, LPCWSTR};

type FnLoadLibraryA = extern "system" fn(LPCSTR) -> HMODULE;
type FnLoadLibraryW = extern "system" fn(LPCWSTR) -> HMODULE;

pub static LoadLibraryADetour: OnceLock<Arc<RwLock<GenericDetour<FnLoadLibraryA>>>> =
    OnceLock::new();
pub static LoadLibraryWDetour: OnceLock<Arc<RwLock<GenericDetour<FnLoadLibraryW>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_LoadLibraryA,
    FnLoadLibraryA,
    LoadLibraryADetour,
    "kernel32",
    name_of!(LoadLibraryA),
    __hook__LoadLibraryA
);

pub extern "system" fn __hook__LoadLibraryA(lpFileName: LPCSTR) -> HMODULE {
    let file_name = unsafe { std::ffi::CStr::from_ptr(lpFileName) };
    event!(
        Level::INFO,
        "[{}] {} {:?}",
        name_of!(LoadLibraryA),
        name_of!(lpFileName),
        file_name
    );
    // call trampoline

    let f = get_detour!(LoadLibraryADetour);

    unsafe { f.call(lpFileName) }
}

declare_init_hook!(
    hook_LoadLibraryW,
    FnLoadLibraryW,
    LoadLibraryWDetour,
    "kernel32",
    name_of!(LoadLibraryW),
    __hook__LoadLibraryW
);

pub extern "system" fn __hook__LoadLibraryW(lpFileName: LPCWSTR) -> HMODULE {
    use widestring::{U16Str, U16String};
    use winapi::um::winbase::lstrlenW;
    event!(
        Level::INFO,
        "[{}] {} {:?}",
        name_of!(LoadLibraryW),
        name_of!(lpFileName),
        unsafe { U16Str::from_ptr(lpFileName, lstrlenW(lpFileName) as usize) }
    );
    // call trampoline
    let f = get_detour!(LoadLibraryWDetour);

    unsafe { f.call(lpFileName) }
}
