use crate::{declare_init_hook, get_detour, utility::MSG_to_string};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, LRESULT, PDWORD, TRUE, UINT,
};
use winapi::shared::windef::HWND;
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
use winapi::um::winuser::{PeekMessageA, PeekMessageW, LPMSG, MSG};

type FnPeekMessageA = extern "system" fn(LPMSG, HWND, UINT, UINT, UINT) -> BOOL;
type FnPeekMessageW = extern "system" fn(LPMSG, HWND, UINT, UINT, UINT) -> BOOL;

pub static PeekMessageADetour: OnceLock<Arc<RwLock<GenericDetour<FnPeekMessageA>>>> =
    OnceLock::new();
pub static PeekMessageWDetour: OnceLock<Arc<RwLock<GenericDetour<FnPeekMessageW>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_PeekMessageA,
    FnPeekMessageA,
    PeekMessageADetour,
    "USER32",
    name_of!(PeekMessageA),
    __hook__PeekMessageA
);

declare_init_hook!(
    hook_PeekMessageW,
    FnPeekMessageW,
    PeekMessageWDetour,
    "USER32",
    name_of!(PeekMessageW),
    __hook__PeekMessageW
);

extern "system" fn __hook__PeekMessageA(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFileterMax: UINT,
    wRemoveMsg: UINT,
) -> BOOL {
    event!(
        Level::INFO,
        "[{}] {} {:?}, {} {:x}, {} {}, {} {}, {} {}",
        name_of!(PeekMessageA),
        name_of!(lpMsg),
        MSG_to_string(unsafe { *lpMsg }),
        name_of!(hWnd),
        hWnd as usize,
        name_of!(wMsgFilterMin),
        wMsgFilterMin,
        name_of!(wMsgFileterMax),
        wMsgFileterMax,
        name_of!(wRemoveMsg),
        wRemoveMsg
    );
    // call trampoline
    let f = get_detour!(PeekMessageADetour);

    unsafe { f.call(lpMsg, hWnd, wMsgFilterMin, wMsgFileterMax, wRemoveMsg) }
}

extern "system" fn __hook__PeekMessageW(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFileterMax: UINT,
    wRemoveMsg: UINT,
) -> BOOL {
    event!(
        Level::INFO,
        "[{}] {} {:?}, {} {:x}, {} {}, {} {}, {} {}",
        name_of!(PeekMessageW),
        name_of!(lpMsg),
        lpMsg,
        name_of!(hWnd),
        hWnd as usize,
        name_of!(wMsgFilterMin),
        wMsgFilterMin,
        name_of!(wMsgFileterMax),
        wMsgFileterMax,
        name_of!(wRemoveMsg),
        wRemoveMsg
    );
    // call trampoline
    let f = get_detour!(PeekMessageWDetour);

    unsafe { f.call(lpMsg, hWnd, wMsgFilterMin, wMsgFileterMax, wRemoveMsg) }
}
