//! LingCode FFI - C API for cross-platform frontends
//!
//! This crate provides a C-compatible API for integrating LingCode
//! with native input method frameworks (macOS InputMethodKit,
//! Windows TSF, Linux IBus/Fcitx, etc.)

use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::ptr;

use lingcode_pinyin::PinyinEngine;
use lingcode_pinyin::simplified::SimplifiedPinyinEngine;

/// Opaque handle to the input method engine
pub struct LingCodeEngine {
    engine: SimplifiedPinyinEngine,
    input_buffer: String,
}

/// Create a new input method engine
///
/// # Safety
/// The returned pointer must be freed with `lingcode_engine_free`
#[no_mangle]
pub extern "C" fn lingcode_engine_new() -> *mut c_void {
    let engine = SimplifiedPinyinEngine::new();

    let engine = LingCodeEngine {
        engine,
        input_buffer: String::new(),
    };
    let boxed = Box::new(engine);
    Box::into_raw(boxed) as *mut c_void
}

/// Create engine with Rime dictionaries
#[no_mangle]
pub extern "C" fn lingcode_engine_with_dicts(dict_dir: *const c_char) -> *mut c_void {
    let dict_dir = if dict_dir.is_null() {
        None
    } else {
        unsafe {
            std::ffi::CStr::from_ptr(dict_dir)
                .to_str()
                .ok()
                .map(|s| s.to_string())
        }
    };

    let mut engine = SimplifiedPinyinEngine::new();

    if let Some(dir) = dict_dir {
        engine.load_rime_dicts(&dir);
    }

    let engine = LingCodeEngine {
        engine,
        input_buffer: String::new(),
    };
    let boxed = Box::new(engine);
    Box::into_raw(boxed) as *mut c_void
}

/// Free the input method engine
///
/// # Safety
/// `engine` must be a valid pointer returned by `lingcode_engine_new`
#[no_mangle]
pub extern "C" fn lingcode_engine_free(engine: *mut c_void) {
    if engine.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(engine as *mut LingCodeEngine);
    }
}

/// Process a key event
///
/// # Arguments
/// * `engine` - The engine handle
/// * `key` - The key character (ASCII lowercase letter)
///
/// # Returns
/// 1 if the key was consumed, 0 otherwise
#[no_mangle]
pub extern "C" fn lingcode_process_key(engine: *mut c_void, key: c_char) -> c_int {
    if engine.is_null() {
        return 0;
    }

    let engine = unsafe { &mut *(engine as *mut LingCodeEngine) };
    let key = key as u8 as char;

    // Handle lowercase letters
    if key.is_ascii_lowercase() {
        engine.input_buffer.push(key);
        1
    } else {
        0
    }
}

/// Get the current input buffer
///
/// # Safety
/// The returned string must be freed with `lingcode_string_free`
#[no_mangle]
pub extern "C" fn lingcode_get_buffer(engine: *mut c_void) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }

    let engine = unsafe { &*(engine as *mut LingCodeEngine) };

    match CString::new(engine.input_buffer.clone()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Get the number of candidates
#[no_mangle]
pub extern "C" fn lingcode_get_candidate_count(engine: *mut c_void) -> c_int {
    if engine.is_null() {
        return 0;
    }

    let engine = unsafe { &*(engine as *mut LingCodeEngine) };

    if engine.input_buffer.is_empty() {
        return 0;
    }

    match engine.engine.get_candidates(&engine.input_buffer) {
        Ok(candidates) => candidates.len() as c_int,
        Err(_) => 0,
    }
}

/// Get a candidate by index
///
/// # Safety
/// The returned string must be freed with `lingcode_string_free`
#[no_mangle]
pub extern "C" fn lingcode_get_candidate(engine: *mut c_void, index: c_int) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }

    let engine = unsafe { &*(engine as *mut LingCodeEngine) };

    if engine.input_buffer.is_empty() {
        return ptr::null_mut();
    }

    let candidates = match engine.engine.get_candidates(&engine.input_buffer) {
        Ok(c) => c,
        Err(_) => return ptr::null_mut(),
    };

    let index = index as usize;
    let candidate = match candidates.get(index) {
        Some(c) => c,
        None => return ptr::null_mut(),
    };

    match CString::new(candidate.text.clone()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Select a candidate by index and return the committed text
///
/// # Safety
/// The returned string must be freed with `lingcode_string_free`
#[no_mangle]
pub extern "C" fn lingcode_select_candidate(engine: *mut c_void, index: c_int) -> *mut c_char {
    if engine.is_null() {
        return ptr::null_mut();
    }

    let engine = unsafe { &mut *(engine as *mut LingCodeEngine) };

    if engine.input_buffer.is_empty() {
        return ptr::null_mut();
    }

    let candidates = match engine.engine.get_candidates(&engine.input_buffer) {
        Ok(c) => c,
        Err(_) => return ptr::null_mut(),
    };

    let index = index as usize;
    let candidate = match candidates.get(index) {
        Some(c) => c,
        None => return ptr::null_mut(),
    };

    let committed = candidate.text.clone();
    engine.input_buffer.clear();

    match CString::new(committed) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Handle backspace
///
/// # Returns
/// 1 if backspace was handled, 0 if buffer was empty
#[no_mangle]
pub extern "C" fn lingcode_backspace(engine: *mut c_void) -> c_int {
    if engine.is_null() {
        return 0;
    }

    let engine = unsafe { &mut *(engine as *mut LingCodeEngine) };

    if engine.input_buffer.is_empty() {
        0
    } else {
        engine.input_buffer.pop();
        1
    }
}

/// Clear the input state
#[no_mangle]
pub extern "C" fn lingcode_clear(engine: *mut c_void) {
    if engine.is_null() {
        return;
    }

    let engine = unsafe { &mut *(engine as *mut LingCodeEngine) };
    engine.input_buffer.clear();
}

/// Free a string returned by the API
///
/// # Safety
/// `s` must be a string returned by one of the API functions
#[no_mangle]
pub extern "C" fn lingcode_string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}