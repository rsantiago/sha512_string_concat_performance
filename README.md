# Performance comparison among Java and Rust

This project has the objective of comparing the performance of two programs,
one written in Java, another in Rust.

The case goes as follows:

Suppose you had to build a string and then take a sha512 hash of it.

You do that 1 million times.

This is exactly what the program does.

### There is a Java version

The Java version lies in the [code]/java[/code] directory.
It uses a StringBuilder object to create the strings,
and it relies on an implementation of sha512 from the
apache commons codec.

You run the Java version with the following command:

'''./gradlew run'''

### Then there is a Rust version

The Rust version is stored in the /rust directory.
It uses Strings to concatenate the strings.
Also, there is a new trait implemented, which mimics 
the behavior of the StringBuilder.append(int i) from
the Java platform. Sort of. It is much simpler.

The rust version also uses the sha2 crate.

You can run the rust version with the following command:

'''run --package sha512 --bin sha512  --release'''

## Results

Bear in mind that both implementations are far from equal. 
They are, at the very best case, somewhat similar.

The challenge was to 'just implement' the code and run in both technologies.



https://github.com/hoodie/concatenation_benchmarks-rs
