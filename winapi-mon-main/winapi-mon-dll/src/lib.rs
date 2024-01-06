#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::arch::asm;
use tracing::{event, Level};
use winapi::{
    shared::minwindef::{
        BOOL, DWORD, FALSE, HINSTANCE, LPARAM, LPVOID, LRESULT, TRUE, UINT, WPARAM,
    },
    shared::windef::HWND,
    um::consoleapi::AllocConsole,
    um::libloaderapi::DisableThreadLibraryCalls,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
    um::wincon::FreeConsole,
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

// Example custom hook
extern "system" fn __hook__Sleep(dwMilliseconds: DWORD) {
    // read the rax(eax) register for the sake of example
    // Rust inline assembly: https://rust-lang.github.io/rfcs/2873-inline-asm.html
    // https://doc.rust-lang.org/reference/inline-assembly.html

    let xax_val: usize;
    if cfg!(target_pointer_width = "64") {
        unsafe {
            asm! {
                "mov {tmp}, rax",
            tmp= out(reg) xax_val
            }
        }
    } else {
        //32bit
        unsafe {
            asm! {
                "mov {tmp}, eax",
            tmp= out(reg) xax_val
            }
        }
    }

    event!(
        Level::WARN,
        "I do not sleep({}) rax(or eax): {:x}",
        dwMilliseconds,
        xax_val
    );
}

fn attached_main() -> anyhow::Result<()> {
    unsafe { AllocConsole() };
    ansi_term::enable_ansi_support().unwrap();

    // let file_appender = tracing_appender::rolling::never("log", "winapi-mon.log"); //uncommnet this to use file log
    tracing_subscriber::fmt()
        //    .with_writer(file_appender) //uncommnet this to use file log
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    event!(Level::INFO, "Initialized the logger!");

    winapi_mon_core::fileapi::hook_ReadFile(None, true)?;

    winapi_mon_core::fileapi::hook_GetFinalPathNameByHandleA(None, true)?;

    winapi_mon_core::libloaderapi::hook_LoadLibraryA(None, true)?;

    winapi_mon_core::libloaderapi::hook_LoadLibraryW(None, true)?;

    winapi_mon_core::libloaderapi::hook_GetProcAddress(None, true)?;

    let detour = winapi_mon_core::fileapi::hook_CreateFileA(None, false)?;

    //You can enable the hook later
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    //Custom Hook
    //provide Some(your_hook) to use your own hook function
    winapi_mon_core::synchapi::hook_Sleep(Some(__hook__Sleep), true)?;

    event!(Level::INFO, "All Done");

    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(dll_module) };
            attached_main().unwrap()
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    TRUE
}
