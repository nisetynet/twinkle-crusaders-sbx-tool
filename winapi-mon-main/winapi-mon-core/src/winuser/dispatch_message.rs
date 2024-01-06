use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use nameof::name_of;
use retour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, LRESULT, PDWORD, TRUE,
};
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
use winapi::um::winuser::{DispatchMessageA, DispatchMessageW, MSG};

type FnDispatchMessageA = extern "system" fn(*const MSG) -> LRESULT;
type FnDispatchMessageW = extern "system" fn(*const MSG) -> LRESULT;

pub static DispatchMessageADetour: OnceLock<Arc<RwLock<GenericDetour<FnDispatchMessageA>>>> =
    OnceLock::new();
pub static DispatchMessageWDetour: OnceLock<Arc<RwLock<GenericDetour<FnDispatchMessageW>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_DispatchMessageA,
    FnDispatchMessageA,
    DispatchMessageADetour,
    "USER32",
    name_of!(DispatchMessageA),
    __hook__DispatchMessageA
);

extern "system" fn __hook__DispatchMessageA(lpMsg: *const MSG) -> LRESULT {
    event!(
        Level::INFO,
        "[{}] {} {:?}",
        name_of!(DispatchMessageA),
        name_of!(lpMsg),
        lpMsg
    );
    // call trampoline
    let f = get_detour!(DispatchMessageADetour);

    unsafe { f.call(lpMsg) }
}

declare_init_hook!(
    hook_DispatchMessageW,
    FnDispatchMessageW,
    DispatchMessageWDetour,
    "USER32",
    name_of!(DispatchMessageW),
    __hook__DispatchMessageW
);

extern "system" fn __hook__DispatchMessageW(lpMsg: *const MSG) -> LRESULT {
    event!(
        Level::INFO,
        "[{}] {} {:?}",
        name_of!(DispatchMessageW),
        name_of!(lpMsg),
        lpMsg
    );
    // call trampoline
    let f = get_detour!(DispatchMessageWDetour);

    unsafe { f.call(lpMsg) }
}
