#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::{c_char, c_int};
include!("./bindings.rs");

pub fn compress_simple(input: &str) -> Vec<u8> {
    let mut output = vec![0; input.len() * 2];
    let len = unsafe {
        unishox2_compress_simple(
            input.as_ptr() as *const c_char,
            input.len() as c_int,
            output.as_mut_ptr() as *mut c_char,
        )
    };
    unsafe { output.set_len(len as usize) };
    println!("{:?}", output);
    output
}

pub fn decompress_simple(input: &Vec<u8>) -> String {
    let mut output = vec![0; input.len() * 2];
    let len = unsafe {
        unishox2_decompress_simple(
            input.as_ptr() as *const c_char,
            input.len() as c_int,
            output.as_mut_ptr() as *mut c_char,
        )
    };
    unsafe { output.set_len(len as usize) };
    println!("{:?}", output);
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_simple_test() {
        let input = "Hello World!";
        let output = compress_simple(input);
        print!("{:?}", output);
        assert_eq!(output, [135, 103, 199, 20, 131, 222, 183, 199, 67, 238]);
    }

    #[test]
    fn decompress_simple_test() {
        let input = vec![135, 103, 199, 20, 131, 222, 183, 199, 67, 238];
        let output = decompress_simple(&input);
        print!("{}", output);
        assert_eq!(output, "Hello World!");
    }
}
