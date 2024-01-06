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
    BOOL, DWORD, FALSE, HINSTANCE, INT, LPARAM, LPDWORD, LPVOID, LRESULT, PDWORD, TRUE, UINT,
    WPARAM,
};
use winapi::shared::ntdef::SHORT;
use winapi::shared::windef::HWND;
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
use winapi::um::winuser::{SendMessageA, LPMSG, MSG};

//todo SendMessageW

//https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagea
//https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew
type FnSendMessageA = extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;
type FnSendMessageW = extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

pub static SendMessageADetour: OnceLock<Arc<RwLock<GenericDetour<FnSendMessageA>>>> =
    OnceLock::new();
pub static SendMessageWDetour: OnceLock<Arc<RwLock<GenericDetour<FnSendMessageA>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_SendMessageA,
    FnSendMessageA,
    SendMessageADetour,
    "USER32",
    name_of!(SendMessageA),
    __hook__SendMessageA
);

extern "system" fn __hook__SendMessageA(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    let f = get_detour!(SendMessageADetour);

    let ret = unsafe { f.call(hWnd, Msg, wParam, lParam) };
    event!(
        Level::INFO,
        "[{}] {} {:x}, {} {:x}, {} {:x}, {} {:x}, returns: {}",
        name_of!(SendMessageA),
        name_of!(hWnd),
        hWnd as usize,
        name_of!(Msg),
        Msg,
        name_of!(wParam),
        wParam,
        name_of!(lParam),
        lParam,
        ret
    );
    ret
}
