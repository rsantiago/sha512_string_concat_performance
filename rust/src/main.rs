use sha2::{Sha512, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    just_hash_it_x20()
    //string_pusher_x20()
}
fn just_hash_it_x20() {
    (0..20).for_each(|_| justHashIt());
}

fn justHashIt() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let my_sample_string = "myBaseString";
    let how_many = 2000000;
    let mut i=0;
    while i< how_many {
        let _hash = Sha512::digest(my_sample_string.as_bytes());
        //println!("Result: {:x}", _hash);
        i+=1;
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("Just hash it took {} milliseconds for {} cycles", end - start, how_many);
}

fn string_pusher_x20() {
    (0..20).for_each(|_| string_pusher());
}

fn string_pusher() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let my_sample_string = "myBaseString";
    let mut string_buffer = String::with_capacity(25);
    let iterations = 1000000;

    let mut i = 0;
    while i < iterations {
        string_buffer.clear();
        string_buffer.push_str(my_sample_string);
        string_buffer.append(i);
        let _hash = Sha512::digest(string_buffer.as_bytes());
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