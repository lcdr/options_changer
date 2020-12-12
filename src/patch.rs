use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

use crate::BASE;

pub fn patch<T>(dst: usize, new: T) {
	unsafe {
		let dst = (dst + BASE) as LPVOID;
		let mut old_protect: DWORD = PAGE_EXECUTE_READWRITE;
		VirtualProtect(dst, std::mem::size_of::<T>(), PAGE_EXECUTE_READWRITE, &mut old_protect);
		*(dst as *mut T) = new;
		VirtualProtect(dst, std::mem::size_of::<T>(), old_protect, &mut old_protect);
	}
}
