use core::{ffi::c_void, str};
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use windows::Win32::{
    Foundation::{HANDLE, HMODULE},
    System::{
        Diagnostics::Debug::ReadProcessMemory,
        ProcessStatus::{EnumProcessModules, GetModuleBaseNameA, GetModuleInformation, MODULEINFO},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

#[derive(Debug)]
pub struct Signature {
    pattern: String,
    offset: usize,
    extra: usize,
}

impl Signature {
    pub fn new(pattern: &str, offset: usize, extra: usize) -> Self {
        Signature {
            pattern: pattern.to_owned(),
            offset,
            extra,
        }
    }

    pub unsafe fn find(&self, memory: &[u8], module_ptr: *mut c_void) -> (bool, *mut c_void) {
        unsafe {
            let pattern: Vec<u8> = self.parse_pattern();
            for i in 0..memory.len() {
                let mut pattern_match = true;
                for j in 0..pattern.len() {
                    if pattern[j] != 0 && memory[i + j] != pattern[j] {
                        pattern_match = false;
                        break;
                    }
                }
                if pattern_match {
                    let pattern_address = module_ptr.add(i);
                    let of: i32 = read_memory(pattern_address.add(self.offset));
                    let result = pattern_address
                        .add(of as usize)
                        .add(self.extra)
                        .sub(module_ptr as usize);
                    log::info!("{:?}\t({})", result, self.pattern);
                    return (true, result);
                }
            }
            log::error!("not found {:?}", self);
            (false, std::ptr::null_mut::<c_void>())
        }
    }

    pub fn parse_pattern(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte_str in self.pattern.split(' ') {
            if byte_str == "?" || byte_str == "??" {
                bytes.push(0);
            } else {
                bytes.push(u8::from_str_radix(byte_str, 16).expect("pattern format"));
            }
        }
        bytes
    }
}

pub fn initialize() {
    unsafe {
        let find_offsets = true;

        let process_handle = find_process();
        set_process_handle(process_handle);

        let client_module = find_module("client.dll");
        set_client_module(client_module);

        log::info!("Initialized");

        if find_offsets {
            let client_module = get_client_module().unwrap();

            let client_memory = read_memory_bytes(
                client_module.lpBaseOfDll,
                client_module.SizeOfImage as usize,
            );

            log::info!(
                "Base of dll: {:?}. Process handle: {:?}",
                client_module.lpBaseOfDll,
                get_process_handle().unwrap()
            );

            let entity_list_sig = Signature::new("48 8B 0D ? ? ? ? C7 45 0F C8 00 00 00", 3, 7);
            crate::external::offsets::client::dwEntityList = entity_list_sig
                .find(&client_memory, client_module.lpBaseOfDll)
                .1 as usize;

            let view_matrix_sig = Signature::new("48 8D ? ? ? ? ? 48 C1 E0 06 48 03 C1 C3", 3, 7);
            crate::external::offsets::client::dwViewMatrix = view_matrix_sig
                .find(&client_memory, client_module.lpBaseOfDll)
                .1 as usize;

            let local_player_sig = Signature::new("48 8B 1D ? ? ? ? 48 8B 6C 24", 3, 7);
            crate::external::offsets::client::dwLocalPlayerController = local_player_sig
                .find(&client_memory, client_module.lpBaseOfDll)
                .1 as usize;

            let camera_sig = Signature::new("48 8D 3D ? ? ? ? 8B D9", 3, 7);
            crate::external::offsets::client::dwCCitadelCameraManager =
                camera_sig.find(&client_memory, client_module.lpBaseOfDll).1 as usize;

            let global_vars_sig = Signature::new("48 8B 05 ? ? ? ? 44 3B 40", 3, 7);
            crate::external::offsets::client::dwGlobalVars = global_vars_sig
                .find(&client_memory, client_module.lpBaseOfDll)
                .1 as usize;
            let game_rules_sig = Signature::new("48 89 1d ? ? ? ? ff 15 ? ? ? ? 84 c0", 3, 7);
            crate::external::offsets::client::dwGameRules = game_rules_sig
                .find(&client_memory, client_module.lpBaseOfDll)
                .1 as usize;

            // let game_entity_system_sig = Signature::new("48 8B 1D ? ? ? ? 48 89 1D", 3, 7);
            // crate::external::offsets::client::dwGameEntitySystem = game_entity_system_sig.find(&client_memory, CLIENT_MODULE.lpBaseOfDll).1 as usize;

            // 2 байта
            // // 8B 81 ? ? ? ? 89 02 48 8B C2 C3 CC CC CC CC 48 89 5C 24 ? 48 89 6C 24
        }
    }
}

unsafe fn find_process() -> HANDLE {
    let mut system = sysinfo::System::new_all();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let process = system.processes_by_name("deadlock.exe".as_ref()).next();

    match process {
        Some(proc) => {
            let pid = proc.pid();

            thread::spawn(move || {
                let mut monitor_system = sysinfo::System::new_all();
                loop {
                    monitor_system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                    if monitor_system.process(pid).is_none() {
                        log::info!("Game exited. Bye!");
                        crate::exit();
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            });

            let handle = unsafe {
                OpenProcess(
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    false,
                    pid.as_u32(),
                )
                .unwrap()
            };

            log::info!("Process found: {:?}", handle);
            handle
        }
        None => {
            log::warn!("Process not found");
            HANDLE::default()
        }
    }
}

unsafe fn find_module(module_name: &str) -> MODULEINFO {
    unsafe {
        let process_handle = get_process_handle().unwrap();
        if !process_handle.is_invalid() {
            let mut h_mods: [HMODULE; 256] = [HMODULE::default(); 256];
            let mut cb_needed = 0u32;
            EnumProcessModules(
                process_handle,
                &mut h_mods[0],
                std::mem::size_of::<HMODULE>() as u32 * h_mods.len() as u32,
                &mut cb_needed,
            )
            .unwrap();

            for h_mod in h_mods
                .iter()
                .take(cb_needed as usize / std::mem::size_of::<HMODULE>())
            {
                let mut file_name: [u8; 32] = [0u8; 32];
                GetModuleBaseNameA(process_handle, Some(*h_mod), &mut file_name);
                let file_name_str = str::from_utf8(&file_name).unwrap();
                if file_name_str.starts_with(module_name) {
                    let mut module_info = MODULEINFO::default();
                    GetModuleInformation(
                        process_handle,
                        *h_mod,
                        &mut module_info,
                        std::mem::size_of::<MODULEINFO>() as u32,
                    )
                    .unwrap();
                    log::info!("Client: {:?}", module_info);
                    return module_info;
                }
            }
        }
        log::error!("Client module not found");
        MODULEINFO::default()
    }
}

pub unsafe fn read_memory<T: Copy>(address: *mut c_void) -> T {
    unsafe {
        let size = std::mem::size_of::<T>();
        let mut buffer = std::mem::zeroed::<T>();
        let bytes_of_read: Option<*mut usize> = Default::default();
        _ = ReadProcessMemory(
            get_process_handle().unwrap(),
            address,
            &mut buffer as *const T as *mut c_void,
            size,
            bytes_of_read,
        );
        buffer
    }
}

pub unsafe fn read_memory_bytes(address: *mut c_void, size: usize) -> Vec<u8> {
    let buffer = vec![0u8; size];
    let buffer_ptr = buffer.as_ptr() as *mut c_void;

    let bytes_of_read: Option<*mut usize> = Default::default();
    _ = unsafe {
        ReadProcessMemory(
            get_process_handle().unwrap(),
            address,
            buffer_ptr,
            size,
            bytes_of_read,
        )
    };
    buffer
}

#[repr(transparent)]
pub struct ThreadSafeHandle(HANDLE);
unsafe impl Send for ThreadSafeHandle {}
unsafe impl Sync for ThreadSafeHandle {}

#[repr(transparent)]
pub struct ThreadSafeModule(MODULEINFO);
unsafe impl Send for ThreadSafeModule {}
unsafe impl Sync for ThreadSafeModule {}

pub static PROCESS_HANDLE: RwLock<Option<ThreadSafeHandle>> = RwLock::new(None);
pub static CLIENT_MODULE: RwLock<Option<ThreadSafeModule>> = RwLock::new(None);

fn set_process_handle(handle: HANDLE) {
    PROCESS_HANDLE
        .write()
        .unwrap()
        .replace(ThreadSafeHandle(handle));
}
pub fn get_process_handle() -> Option<HANDLE> {
    PROCESS_HANDLE.read().unwrap().as_ref().map(|h| h.0)
}

fn set_client_module(module: MODULEINFO) {
    CLIENT_MODULE
        .write()
        .unwrap()
        .replace(ThreadSafeModule(module));
}
pub fn get_client_module() -> Option<MODULEINFO> {
    CLIENT_MODULE.read().unwrap().as_ref().map(|m| m.0)
}
