use nyxvion_shield::{
    nyxvion_shield_check_request, nyxvion_shield_create, nyxvion_shield_destroy,
    nyxvion_shield_create_from_binary, nyxvion_shield_free_binary,
    nyxvion_shield_free_string, nyxvion_shield_get_cosmetic_css, nyxvion_shield_serialize,
};
use reqwest::blocking::get;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("================================================");
    println!(" Nyxvion Shield - C++ Integration Test (FFI)    ");
    println!("================================================");
    
    let easylist_url = "https://easylist.to/easylist/easylist.txt";
    println!("[Nyxvion Shield] Downloading blocklist from {} ...", easylist_url);
    let response = get(easylist_url)?;
    let mut easylist_content = response.text()?;
    
    let whitelist_rules = "\n! --- Nyxvion Shield Custom Whitelist ---\n@@||000491b06a.com/ads.js$script\n";
    easylist_content.push_str(whitelist_rules);
    
    println!("[Nyxvion Shield] Initializing core engine via FFI...");
    let start_init = Instant::now();
    let c_rules = CString::new(easylist_content).unwrap();
    let engine_ptr = nyxvion_shield_create(c_rules.as_ptr());
    let init_duration = start_init.elapsed();
    
    if engine_ptr.is_null() {
        println!("[Nyxvion Shield] Failed to create engine instance.");
        return Ok(());
    }
    
    println!("[Nyxvion Shield] Engine initialized from text in {:?}", init_duration);
    
    // Test Serialization
    println!("\n[Nyxvion Shield] Testing Serialization (Caching)...");
    let mut bin_size: usize = 0;
    let start_ser = Instant::now();
    let bin_data = nyxvion_shield_serialize(engine_ptr, &mut bin_size);
    let ser_duration = start_ser.elapsed();
    println!("[Nyxvion Shield] Serialized to {} bytes in {:?}", bin_size, ser_duration);
    
    // Destroy old engine and recreate from binary
    nyxvion_shield_destroy(engine_ptr);
    
    println!("[Nyxvion Shield] Recreating engine from binary data...");
    let start_deser = Instant::now();
    let fast_engine_ptr = nyxvion_shield_create_from_binary(bin_data, bin_size);
    let deser_duration = start_deser.elapsed();
    println!("[Nyxvion Shield] Engine instantly recreated in {:?}", deser_duration);
    
    // Free binary buffer
    nyxvion_shield_free_binary(bin_data, bin_size);

    println!("\n[Nyxvion Shield] Engine initialized successfully. Testing URLs...\n");
    
    let test_urls = [
        ("Clean URL", "https://github.com/", "https://github.com/", "document"),
        ("Whitelisted Ad Tracker", "https://000491b06a.com/ads.js", "https://example.com/", "script"),
    ];
    
    for (description, url, source_url, request_type) in test_urls.iter() {
        let c_url = CString::new(*url).unwrap();
        let c_source_url = CString::new(*source_url).unwrap();
        let c_request_type = CString::new(*request_type).unwrap();
        let c_method = CString::new("GET").unwrap();
        
        println!("{}: {}", description, url);
        
        let is_blocked = nyxvion_shield_check_request(
            fast_engine_ptr,
            c_url.as_ptr(),
            c_source_url.as_ptr(),
            c_request_type.as_ptr(),
            c_method.as_ptr()
        );
        
        if is_blocked == 1 {
            println!("Nyxvion Shield Decision: [BLOCKED]");
        } else {
            println!("Nyxvion Shield Decision: [ALLOWED]");
            
            // Get Cosmetic CSS
            let css_ptr = nyxvion_shield_get_cosmetic_css(fast_engine_ptr, c_url.as_ptr());
            if !css_ptr.is_null() {
                let css_str = unsafe { CStr::from_ptr(css_ptr) }.to_string_lossy();
                println!("Cosmetic CSS to Inject:\n{}", css_str);
                nyxvion_shield_free_string(css_ptr);
            }
        }
        println!("------------------------------------------------");
    }
    
    nyxvion_shield_destroy(fast_engine_ptr);
    println!("[Nyxvion Shield] Engine destroyed gracefully.");
    
    Ok(())
}
