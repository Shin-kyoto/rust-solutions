use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void){
    println!("{:p}", ptr);
}
