static mut CURRENT_ID: usize = 0;

pub fn generate_id() -> usize {
    unsafe {
        CURRENT_ID += 1;
        return CURRENT_ID;
    }
}
