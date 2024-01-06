#![feature(once_cell)]
#![feature(naked_functions)]
#![allow(non_snake_case)]

pub mod battle;
pub mod css;
pub mod d3d9;
pub mod utility;
use anyhow::Result;
use ilhook::x86::{CallbackOption, HookFlags, HookPoint, HookType, Hooker, Registers};
use nameof::name_of;
use phf::{phf_map, Map};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use tracing::{event, Level};
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::windef::HWND;
use winapi::um::fileapi::{
    CreateFileA, GetFileSize, ReadFile, WriteFile, CREATE_ALWAYS, CREATE_NEW, INVALID_FILE_SIZE,
    OPEN_ALWAYS, OPEN_EXISTING, TRUNCATE_EXISTING,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::winnt::{HANDLE, LPCSTR};
use winapi::um::winuser::{PeekMessageA, LPMSG, MSG};
use winapi_mon_core::fileapi::CreateFileADetour;

pub fn init_main_loop_inner_hook(module_address: usize) -> Result<Hooker> {
    let main_loop_inner_address = module_address as usize + sbx_offset::MAIN_LOOP_INNER_OFFSET;

    event!(
        Level::INFO,
        "main loop inner address: {:x}",
        main_loop_inner_address
    );

    let hooker = Hooker::new(
        main_loop_inner_address,
        HookType::JmpBack(__hook__main_loop_inner),
        CallbackOption::None,
        0,
        HookFlags::empty(),
    );
    Ok(hooker)
}

/// sbx main message loop
extern "cdecl" fn __hook__main_loop_inner(regs: *mut Registers, _: usize) {
    //https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagea
    /* MSG
       hwnd: HWND,
       message: UINT,
       wParam: WPARAM,
       lParam: LPARAM,
       time: DWORD,
       pt: POINT,
    */
    // event!(Level::INFO, "from main loop inner hook");
    let mut msg: MSG = MSG::default();
    let result = unsafe { PeekMessageA(&mut msg, 0 as HWND, 0, 0, 0) };
    if result != 0 {
        //message available
        let hwnd = msg.hwnd;
        if hwnd as usize == 0 {
            event!(
                Level::INFO,
                "[MAIN LOOP] ThreadMessage {}, wParam {}, lParam {}",
                msg.message,
                msg.wParam,
                msg.lParam
            );
            return;
        }
        //non thread messages
        match msg.message {
            WM_MOUSEMOVE => {
                /*
                let x_pos = GET_X_LPARAM(msg.lParam);
                let y_pos = GET_X_LPARAM(msg.lParam);
                event!(
                    Level::DEBUG,
                    "Mouse Move Message (x,y)=({},{})",
                    x_pos,
                    y_pos
                );
                */
            }
            _ => {
                event!(
                    Level::INFO,
                    "Unknown Message {}, wParam {}, lParam {}",
                    msg.message,
                    msg.wParam,
                    msg.lParam
                );
            }
        }
    }
}

pub fn init_game_loop_inner_hook(module_address: usize) -> Result<Hooker> {
    let game_loop_inner_address = module_address as usize + sbx_offset::GAME_LOOP_INNER_OFFSET;

    event!(
        Level::INFO,
        "game loop inner address: {:x}",
        game_loop_inner_address
    );

    let hooker = Hooker::new(
        game_loop_inner_address,
        HookType::JmpBack(__hook__game_loop_inner),
        CallbackOption::None,
        0,
        HookFlags::empty(),
    );
    Ok(hooker)
}

/// sbx main message loop
extern "cdecl" fn __hook__game_loop_inner(regs: *mut Registers, _: usize) {
    let mut msg: MSG = MSG::default();
    let result = unsafe { PeekMessageA(&mut msg, 0 as HWND, 0, 0, 0) };
    if result != 0 {
        //message available
        let hwnd = msg.hwnd;
        if hwnd as usize == 0 {
            event!(
                Level::INFO,
                "[GAME LOOP] ThreadMessage {}, wParam {}, lParam {}",
                msg.message,
                msg.wParam,
                msg.lParam
            );
            return;
        }
        //non thread messages
        match msg.message {
            WM_MOUSEMOVE => {
                /*
                let x_pos = GET_X_LPARAM(msg.lParam);
                let y_pos = GET_X_LPARAM(msg.lParam);
                event!(
                    Level::DEBUG,
                    "Mouse Move Message (x,y)=({},{})",
                    x_pos,
                    y_pos
                );
                */
            }
            _ => {
                event!(
                    Level::INFO,
                    "Unknown Message {}, wParam {}, lParam {}",
                    msg.message,
                    msg.wParam,
                    msg.lParam
                );
            }
        }
    }
}

static UI_MAIN_LOOP_SWITCH_FLAG_ADDRESS: OnceLock<usize> = OnceLock::new();
static UI_MAIN_LOOP_FIRST_SWITCH_CASE_BEFORE: AtomicU32 = AtomicU32::new(77777);
pub static UI_MAIN_LOOP_FIRST_SWITCH_CASE_NAME_MAP: Map<u32, &'static str> = phf_map! {
    23u32 => "CONFIG",
    24u32 => "SAVE_LOAD",
    26u32 => "ESCAPE",
    95u32 => "BRAVE_MODE_SSS",
    96u32 => "BRAVE_MODE_CSS",
    97u32 => "VS_CPU_MODE_CSS",
    98u32 => "VS_CPU_MODE_SSS",
    99u32 => "BATTLE",
};

fn get_ui_main_loop_first_switch_case_name(case: u32) -> &'static str {
    match UI_MAIN_LOOP_FIRST_SWITCH_CASE_NAME_MAP.get(&case) {
        Some(n) => n,
        None => "Unknown",
    }
}

pub fn init_ui_loop_inner_hook(module_address: usize) -> Result<Hooker> {
    UI_MAIN_LOOP_SWITCH_FLAG_ADDRESS
        .set(module_address + sbx_offset::UI_LOOP_SWITCH_FLAG_OFFSET)
        .unwrap(); //lazy to handler the error, todo

    let ui_loop_inner_address = module_address as usize + sbx_offset::UI_LOOP_INNER_OFFSET;

    event!(
        Level::INFO,
        "ui loop inner address: {:x}",
        ui_loop_inner_address
    );

    let hooker = Hooker::new(
        ui_loop_inner_address,
        HookType::JmpBack(__hook__ui_loop_inner),
        CallbackOption::None,
        0,
        HookFlags::empty(),
    );
    Ok(hooker)
}

/// sbx main message loop
extern "cdecl" fn __hook__ui_loop_inner(regs: *mut Registers, _: usize) {
    let flag_address = *UI_MAIN_LOOP_SWITCH_FLAG_ADDRESS.get().unwrap(); //already initialized by init hook function
    let case = unsafe { *(flag_address as *const u32) };
    let prev_case = UI_MAIN_LOOP_FIRST_SWITCH_CASE_BEFORE.load(Ordering::Relaxed);
    if prev_case == case {
        //To not spam log
        return;
    }
    UI_MAIN_LOOP_FIRST_SWITCH_CASE_BEFORE.store(case, Ordering::Relaxed);

    event!(
        Level::INFO,
        "[UI Main Loop] Switch Case: {}({})",
        get_ui_main_loop_first_switch_case_name(case),
        case
    );
}

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

    let name = file_name.to_str().unwrap();

    let trampoline = winapi_mon_core::get_detour!(CreateFileADetour);
    //log BG .def files
    /*
        if name.contains("bt00_BG") && name.ends_with(".def") {
            event!(
                Level::DEBUG,
                "[{}] {} {:?}, {} {}, {} {}",
                name_of!(CreateFileA),
                name_of!(lpFileName),
                file_name,
                name_of!(dwCreationDisposition),
                creation_disposition,
                name_of!(dwFlagsAndAttributes),
                flags_and_atributes
            );

            //open def file(need to do this with the trampoline, not with std::fs functions. since we are hooking CreateFileA which is (probably) used by std::fs functions.)
            let def_file_handle = unsafe {
                trampoline.call(
                    lpFileName,
                    dwDesiredAccess,
                    dwShareMode,
                    lpSecurityAttributes,
                    dwCreationDisposition,
                    dwFlagsAndAttributes,
                    hTemplateFile,
                )
            };
            assert_ne!(def_file_handle, INVALID_HANDLE_VALUE);

            event!(Level::WARN, "A");
            //get file size
            let file_size = unsafe { GetFileSize(def_file_handle, std::ptr::null_mut()) };
            assert_ne!(file_size, INVALID_FILE_SIZE);
            event!(Level::WARN, "file size: {:x}", file_size);
            let mut buffer: Vec<u8> = Vec::with_capacity(file_size as usize);
            buffer.resize(file_size as usize, 0);

            event!(Level::WARN, "B");
            let _ = unsafe {
                ReadFile(
                    def_file_handle,
                    buffer.as_mut_ptr() as LPVOID,
                    buffer.len() as u32,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };

            event!(Level::WARN, "C");
            //parse def file
            let mut def_map = std::collections::HashMap::new();
            if let Ok(file_str) = unsafe { std::ffi::CString::from_vec_unchecked(buffer) }.to_str() {
                let lines = file_str.lines();
                lines.for_each(|line| {
                    let line = line.trim_matches('\n'); //remove white spaces
                    event!(Level::DEBUG, "line: {}", line);
                    if let Some(equal_pos) = line.find("=") {
                        let (key, val) = line.split_at(equal_pos);
                        event!(Level::WARN, "key: {} val: {}", key, val);
                        def_map.insert(key.to_owned(), val.to_owned());
                    }
                });
            }

            //close original def file
            unsafe { CloseHandle(def_file_handle) };

            //modify def

            //create custom def file and let game loads it
            let mut name = unsafe { std::ffi::CStr::from_ptr(lpFileName) }
                .to_str()
                .unwrap()
                .to_owned();
            name.push_str("_custom");

            event!(Level::WARN, "D");
            //create custom def file
            let custom_def_file_handle = unsafe {
                trampoline.call(
                    std::ffi::CString::new(name).unwrap().as_ptr(),
                    dwDesiredAccess,
                    dwShareMode,
                    lpSecurityAttributes,
                    CREATE_ALWAYS,
                    dwFlagsAndAttributes,
                    hTemplateFile,
                )
            };

            //prepare buffer
            let def_svec: Vec<String> = def_map
                .into_iter()
                .map(|(key, val)| format!("{}={}\n", key, val))
                .collect();
            let def_string = std::ffi::CString::new(def_svec.concat()).unwrap();

            //write def
            let _ = unsafe {
                WriteFile(
                    custom_def_file_handle,
                    def_string.as_ptr() as LPVOID,
                    def_string.as_bytes().len() as DWORD,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };

            event!(Level::WARN, "ASAAADSfds");

            //return custom def file's handle to game
            return custom_def_file_handle;
        }
    */
    /*
    //log BG epa files
    if (name.ends_with(".epa") || name.ends_with(".EPA"))
        && (name.contains("BG") || name.contains("bg"))
    {
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
    }
    */

    /*
    //log epa file
    if name.ends_with(".epa") || name.ends_with(".EPA") {
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
    }
    */

    // call trampoline
    unsafe {
        trampoline.call(
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
