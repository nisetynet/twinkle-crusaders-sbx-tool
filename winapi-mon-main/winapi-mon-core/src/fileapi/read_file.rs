use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::sync::{Arc, OnceLock, RwLock};
use tracing::{event, Level};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, TRUE};
use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::winnt::HANDLE;
type FnReadFile = extern "system" fn(HANDLE, LPVOID, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;

pub static ReadFileDetour: OnceLock<Arc<RwLock<GenericDetour<FnReadFile>>>> = OnceLock::new();
declare_init_hook!(
    hook_ReadFile,
    FnReadFile,
    ReadFileDetour,
    "kernel32",
    name_of!(ReadFile),
    __hook__ReadFile
);

//tfw no decltype
pub extern "system" fn __hook__ReadFile(
    hFile: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfBytesToRead: DWORD,
    lpNumberOfBytesRead: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL {
    event!(
        Level::INFO,
        "[{}] {} {:?}, {} {}",
        name_of!(ReadFile),
        name_of!(lpBuffer),
        lpBuffer,
        name_of!(nNumberOfBytesToRead),
        nNumberOfBytesToRead
    );

    // call trampoline
    let f = get_detour!(ReadFileDetour);

    unsafe {
        f.call(
            hFile,
            lpBuffer,
            nNumberOfBytesToRead,
            lpNumberOfBytesRead,
            lpOverlapped,
        )
    }
}
