use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, TRUE};
use winapi::um::fileapi::{
    CreateFileA, CreateFileW, CREATE_ALWAYS, CREATE_NEW, OPEN_ALWAYS, OPEN_EXISTING,
    TRUNCATE_EXISTING,
};
use winapi::um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES};
use winapi::um::winnt::{HANDLE, LPCSTR};

type FnCreateFileA =
    extern "system" fn(LPCSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE) -> HANDLE;

pub static CreateFileADetour: OnceLock<Arc<RwLock<GenericDetour<FnCreateFileA>>>> = OnceLock::new();

declare_init_hook!(
    hook_CreateFileA,
    FnCreateFileA,
    CreateFileADetour,
    "kernel32",
    name_of!(CreateFileA),
    __hook__CreateFileA
);

pub extern "system" fn __hook__CreateFileA(
    lpFileName: LPCSTR,
    dwDesiredAccess: DWORD,
    dwShareMode: DWORD,
    lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    dwCreationDisposition: DWORD,
    dwFlagsAndAttributes: DWORD,
    hTemplateFile: HANDLE,
) -> HANDLE {
    let file_name = unsafe { std::ffi::CStr::from_ptr(lpFileName) };

    let creation_disposition = match dwCreationDisposition {
        CREATE_ALWAYS => {
            name_of!(CREATE_ALWAYS)
        }
        CREATE_NEW => {
            name_of!(CREATE_NEW)
        }
        OPEN_ALWAYS => {
            name_of!(OPEN_ALWAYS)
        }
        OPEN_EXISTING => {
            name_of!(OPEN_EXISTING)
        }
        TRUNCATE_EXISTING => {
            name_of!(TRUNCATE_EXISTING)
        }
        _ => "Unknown",
    };

    // https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    //todo,  maybe need to do '&' one by one
    let flags_and_atributes = match dwFlagsAndAttributes {
        __ => "TODO",
    };
    event!(
        Level::INFO,
        "[{}] {} {:?}, {} {}, {} {}",
        name_of!(CreateFileA),
        name_of!(lpFileName),
        file_name,
        name_of!(dwCreationDisposition),
        creation_disposition,
        name_of!(dwFlagsAndAttributes),
        flags_and_atributes
    );

    // call trampoline
    let f = get_detour!(CreateFileADetour);
    unsafe {
        f.call(
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        )
    }
}
