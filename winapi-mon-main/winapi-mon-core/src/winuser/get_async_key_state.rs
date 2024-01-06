use crate::{declare_init_hook, get_detour, utility::MSG_to_string};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::ops::BitAnd;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, HINSTANCE, INT, LPDWORD, LPVOID, LRESULT, PDWORD, TRUE, UINT,
};
use winapi::shared::ntdef::SHORT;
use winapi::shared::windef::HWND;
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
use winapi::um::winuser::{GetAsyncKeyState, LPMSG, MSG};
//https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate
type FnGetAsyncKeyState = extern "system" fn(INT) -> SHORT;

pub static GetAsyncKeyStateDetour: OnceLock<Arc<RwLock<GenericDetour<FnGetAsyncKeyState>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_GetAsyncKeyState,
    FnGetAsyncKeyState,
    GetAsyncKeyStateDetour,
    "USER32",
    name_of!(GetAsyncKeyState),
    __hook__GetAsyncKeyState
);

extern "system" fn __hook__GetAsyncKeyState(vKey: INT) -> SHORT {
    let f = get_detour!(GetAsyncKeyStateDetour);

    let ret = unsafe { f.call(vKey) };
    let b = if (ret & 1i16) == 0x1 { true } else { false };
    let stat = if b { "Pressed" } else { "Not Pressed" };
    event!(
        Level::INFO,
        "[{}] {} {:x} {}",
        name_of!(GetAsyncKeyState),
        name_of!(vKey),
        vKey,
        stat
    );
    ret
}
