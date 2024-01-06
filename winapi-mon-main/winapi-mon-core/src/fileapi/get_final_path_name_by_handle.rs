use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, TRUE};
use winapi::um::fileapi::{GetFinalPathNameByHandleA, GetFinalPathNameByHandleW};
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};

/*
DWORD GetFinalPathNameByHandleA(
  [in]  HANDLE hFile,
  [out] LPSTR  lpszFilePath,
  [in]  DWORD  cchFilePath,
  [in]  DWORD  dwFlags
);
*/
/// https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlea
type FnGetFinalPathNameByHandleA = extern "system" fn(HANDLE, LPSTR, DWORD, DWORD) -> DWORD;

pub static GetFinalPathNameByHandleADetour: OnceLock<
    Arc<RwLock<GenericDetour<FnGetFinalPathNameByHandleA>>>,
> = OnceLock::new();

declare_init_hook!(
    hook_GetFinalPathNameByHandleA,
    FnGetFinalPathNameByHandleA,
    GetFinalPathNameByHandleADetour,
    "kernel32",
    name_of!(GetFinalPathNameByHandleA),
    __hook__GetFinalPathNameByHandleA
);

pub extern "system" fn __hook__GetFinalPathNameByHandleA(
    hFile: HANDLE,
    lpszFilePath: LPSTR,
    cchFilePath: DWORD,
    dwFlags: DWORD,
) -> DWORD {
    // call trampoline first
    let f = get_detour!(GetFinalPathNameByHandleADetour);

    let result = unsafe { f.call(hFile, lpszFilePath, cchFilePath, dwFlags) };

    //LPSTR -> CStr
    let lpszFilePath = unsafe { std::ffi::CStr::from_ptr(lpszFilePath) };

    event!(
        Level::INFO,
        "[{}] {} {:?}, ret = {}",
        name_of!(GetFinalPathNameByHandleA),
        name_of!(lpszFilePath),
        lpszFilePath,
        result
    );
    result
}
