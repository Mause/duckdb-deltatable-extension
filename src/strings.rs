use std::ffi::{c_void, CString};
use std::intrinsics::transmute;
use std::mem::size_of;
use std::os::raw::c_char;
use std::slice;
use std::sync::Once;

pub unsafe fn convert_string(val: *const c_void, idx: usize) -> CString {
    assert!(idx >= 1);

    let base_ptr = val.add((idx - 1) * size_of::<DuckDBStringT>());
    let length_ptr = base_ptr.cast::<i32>();
    let length = *length_ptr;
    if length <= STRING_INLINE_LENGTH {
        let prefix_ptr = base_ptr.add(size_of::<i32>());
        unsafe_string(prefix_ptr.cast::<u8>(), length)
    } else {
        let ptr_ptr = base_ptr.add(size_of::<i32>() * 2).cast::<*const u8>();
        let data_ptr = *ptr_ptr;
        unsafe_string(data_ptr, length)
    }
}

#[repr(C)]
struct DuckDBStringT {
    length: u32,
    data: *const c_char,
}

const STRING_INLINE_LENGTH: i32 = 12;

unsafe fn unsafe_string(ptr: *const u8, len: i32) -> CString {
    let slice = slice::from_raw_parts(ptr, len as usize);

    CString::from_vec_unchecked(slice.to_vec())
}

static START: Once = Once::new();
static mut VERSION_DATA: *const CString = 0 as *const CString;

pub unsafe fn static_version_string(res: CString) -> *const c_char {
    START.call_once(|| {
        VERSION_DATA = transmute(Box::new(res));
    });

    (*VERSION_DATA).as_ptr()
}
