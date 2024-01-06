use crate::{declare_init_hook, get_detour};
use nameof::name_of;
use retour::GenericDetour;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::{
    shared::{
        basetsd::SIZE_T,
        minwindef::{DWORD, LPDWORD, LPVOID},
    },
    um::{
        minwinbase::{LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE},
        processthreadsapi::CreateThread,
        winbase::CREATE_SUSPENDED,
        winnt::HANDLE,
    },
};

type FnCreateThread = extern "system" fn(
    LPSECURITY_ATTRIBUTES,
    SIZE_T,
    LPTHREAD_START_ROUTINE,
    LPVOID,
    DWORD,
    LPDWORD,
) -> HANDLE;
pub static CreateThreadDetour: OnceLock<Arc<RwLock<GenericDetour<FnCreateThread>>>> =
    OnceLock::new();

declare_init_hook!(
    hook_CreateThread,
    FnCreateThread,
    CreateThreadDetour,
    "kernel32",
    name_of!(CreateThread),
    __hook__CreateThread
);

extern "system" fn __hook__CreateThread(
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    dwStackSize: SIZE_T,
    lpStartAddress: LPTHREAD_START_ROUTINE,
    lpParameter: LPVOID,
    dwCreationFlags: DWORD,
    lpThreadId: LPDWORD,
) -> HANDLE {
    let detour = get_detour!(CreateThreadDetour);
    //call trampoline first
    let ret = unsafe {
        detour.call(
            lpThreadAttributes,
            dwStackSize,
            lpStartAddress,
            lpParameter,
            dwCreationFlags,
            lpThreadId,
        )
    };

    let creation_flag = match dwCreationFlags {
        0 => "0",
        CREATE_SUSPENDED => name_of!(CREATE_SUSPENDED),
        0x00010000 => "STACK_SIZE_PARAM_IS_A_RESERVATION",
        _ => "Unknown",
    };

    event!(
        Level::INFO,
        "[{}] {} {}, {} {}, {} {:x}, {} {:x}, {} {}, {} {:p}, returns {:x}",
        name_of!(CreateThread),
        name_of!(lpThreadAttributes),
        "TODO",
        name_of!(dwStackSize),
        dwStackSize,
        name_of!(lpStartAddress),
        match lpStartAddress {
            Some(f) => f as usize,
            None => 0usize,
        },
        name_of!(lpParameter),
        lpParameter as usize,
        name_of!(dwCreationFlags),
        creation_flag,
        name_of!(lpThreadId),
        lpThreadId,
        ret as usize
    );

    ret
}
