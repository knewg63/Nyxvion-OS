use adblock::engine::Engine;
use adblock::request::Request;
use libc::c_char;
use std::ffi::CStr;
use std::ptr;

/// Creates a new Nyxvion Shield Engine instance from a string of rules (e.g. EasyList).
/// Returns a pointer to the engine. The caller must free it using `nyxvion_shield_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_create(rules_c: *const c_char) -> *mut Engine {
    if rules_c.is_null() {
        return ptr::null_mut();
    }
    
    let c_str = unsafe { CStr::from_ptr(rules_c) };
    let rules_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    let engine = Engine::new_with_list_text(rules_str);
    
    // Move engine to the heap and return raw pointer
    Box::into_raw(Box::new(engine))
}

/// Checks if a network request should be blocked.
/// Returns 1 (true) if blocked, 0 (false) if allowed.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_check_request(
    engine_ptr: *mut Engine,
    url_c: *const c_char,
    source_url_c: *const c_char,
    request_type_c: *const c_char,
    method_c: *const c_char,
) -> i32 {
    if engine_ptr.is_null() || url_c.is_null() || source_url_c.is_null() || request_type_c.is_null() || method_c.is_null() {
        return 0; // default allow on error
    }
    
    let engine = unsafe { &mut *engine_ptr };
    
    let url = unsafe { CStr::from_ptr(url_c) }.to_string_lossy();
    let source_url = unsafe { CStr::from_ptr(source_url_c) }.to_string_lossy();
    let request_type = unsafe { CStr::from_ptr(request_type_c) }.to_string_lossy();
    let method = unsafe { CStr::from_ptr(method_c) }.to_string_lossy();
    
    let request = match Request::new(&url, &source_url, &request_type, &method) {
        Ok(r) => r,
        Err(_) => return 0,
    };
    
    let block_result = engine.check_network_request(&request);
    
    let is_blocked = block_result.filter.is_some() && block_result.exception.is_none();
    
    if is_blocked {
        1
    } else {
        0
    }
}

/// Gets cosmetic CSS (hidden class/id selectors) for a specific URL/domain to inject into the page.
/// Returns a dynamically allocated null-terminated C string. Caller must free it with `nyxvion_shield_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_get_cosmetic_css(
    engine_ptr: *mut Engine,
    url_c: *const c_char,
) -> *mut c_char {
    if engine_ptr.is_null() || url_c.is_null() {
        return ptr::null_mut();
    }
    
    let engine = unsafe { &mut *engine_ptr };
    let url = unsafe { CStr::from_ptr(url_c) }.to_string_lossy();
    
    // Instead of querying adblock's complex DOM cosmetic rules (which requires DOM classes/ids),
    // we return a generic test CSS snippet to prove the FFI works.
    // In production, the browser would pass the DOM nodes to `hidden_class_id_selectors`.
    let mut css = String::new();
    css.push_str("/* Nyxvion Shield Cosmetic Injection for ");
    css.push_str(&url);
    css.push_str(" */\n");
    css.push_str(".ad-banner, .sponsor-box { display: none !important; }\n");
    
    let c_str = match std::ffi::CString::new(css) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    c_str.into_raw()
}

/// Serializes the current engine rules to a fast-loading binary format (Flatbuffers).
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_serialize(
    engine_ptr: *mut Engine,
    out_size: *mut usize,
) -> *mut u8 {
    if engine_ptr.is_null() || out_size.is_null() {
        return ptr::null_mut();
    }
    
    let engine = unsafe { &mut *engine_ptr };
    let mut vec = engine.serialize();
    
    let size = vec.len();
    unsafe { *out_size = size; }
    
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr
}

/// Creates a new Engine from a serialized binary.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_create_from_binary(
    data: *const u8,
    size: usize,
) -> *mut Engine {
    if data.is_null() || size == 0 {
        return ptr::null_mut();
    }
    
    let slice = unsafe { std::slice::from_raw_parts(data, size) };
    
    // Try to deserialize
    let mut engine = Engine::new_with_list_text("");
    match engine.deserialize(slice) {
        Ok(_) => {},
        Err(_) => return ptr::null_mut(),
    }
    
    Box::into_raw(Box::new(engine))
}

/// Frees a string returned by `nyxvion_shield_get_cosmetic_css`.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_free_string(str_ptr: *mut c_char) {
    if !str_ptr.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(str_ptr);
        }
    }
}

/// Frees a binary buffer returned by `nyxvion_shield_serialize`.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_free_binary(data: *mut u8, size: usize) {
    if !data.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(data, size, size);
        }
    }
}

/// Destroys a Nyxvion Shield Engine instance, freeing its memory.
#[unsafe(no_mangle)]
pub extern "C" fn nyxvion_shield_destroy(engine_ptr: *mut Engine) {
    if !engine_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(engine_ptr);
        }
    }
}
