use std::mem; 

const SHELLCODE_BYTES: &[u8] = include_bytes!("../../shellcode.bin"); 
const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len(); 

#[no_mangle]

fn main() {
    println!("Hello, world!");
}
