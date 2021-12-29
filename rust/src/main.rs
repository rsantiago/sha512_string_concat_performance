
use std::slice::from_raw_parts;
use sha2::{Sha512, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::slice;

fn main() {
    string_pusher_x20();
}

fn string_pusher_x20() {
    (1..20).for_each(|_| string_pusher());
}

fn string_pusher() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mysamplestring = "myBaseString".to_string();
    let mut string_buffer = String::with_capacity(25);
    let iterations = 1000000;

    let mut i = 0;
    while i < iterations {
        string_buffer.clear();
        string_buffer.push_str(&mysamplestring);
        string_buffer.append(i);
        Sha512::digest(string_buffer.as_bytes());
        //println!("Result: {:x}", _hash);
        i += 1; // go to next iteration
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("The process took {} milliseconds", end - start);
}

pub trait Appendable {
    fn append(&mut self, to_append: i32);
}

/// there are a minor bug in this implementation
/// The first 2 strings are not correctly built.
/// They are missing the numbers.
/// But it mostly works for this use case and comparison.
impl Appendable for String {
    fn append(&mut self, to_append: i32){
        let mut actual_number = to_append;
        let mut divisor =1;
        while actual_number > divisor {
            divisor *= 10;
        }
        divisor/=10;

        while divisor > 0 {
            self.push((((actual_number/divisor) + 48) as u8) as char);
            actual_number = actual_number%divisor;
            divisor/=10;
        }
    }
}