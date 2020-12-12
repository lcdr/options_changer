mod patch;
mod scale_ui;
use scale_ui::patch_ui_scale;

use winapi::{
	shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
	um::libloaderapi::GetModuleHandleA,
};

static mut BASE: usize = 0;

#[no_mangle]
pub extern "system" fn DllMain(
	_dll_module: HINSTANCE,
	call_reason: DWORD,
	_reserved: LPVOID)
	-> BOOL {
	const DLL_PROCESS_ATTACH: DWORD = 1;

	match call_reason {
		DLL_PROCESS_ATTACH => init(),
		_ => TRUE
	}
}

fn init() -> BOOL {
	unsafe { BASE = GetModuleHandleA(std::ptr::null()) as usize; }
	patch_ui_scale();
	TRUE
}
