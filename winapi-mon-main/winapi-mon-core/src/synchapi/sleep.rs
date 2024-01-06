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
use winapi::um::synchapi::{Sleep, SleepEx};
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR, VOID};

//only Sleep is implemented, not Ex

type FnSleep = extern "system" fn(DWORD);
pub static SleepDetour: OnceLock<Arc<RwLock<GenericDetour<FnSleep>>>> = OnceLock::new();

declare_init_hook!(
    hook_Sleep,
    FnSleep,
    SleepDetour,
    "kernel32",
    name_of!(Sleep),
    __hook__Sleep
);

extern "system" fn __hook__Sleep(dwMilliseconds: DWORD) {
    event!(
        Level::INFO,
        "[{}] {:?} msecs.",
        name_of!(Sleep),
        dwMilliseconds
    );
    // call trampoline
    let f = get_detour!(SleepDetour);

    unsafe { f.call(dwMilliseconds) }
}
