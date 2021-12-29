# Java vs Rust Performance
## An exercise on string building and hashing

This project has the objective of comparing the performance of two programs,
one written in Java, another in Rust.

The case goes as follows:

Suppose you had to build a string and then take a sha512 hash of it.

You do that 1 million times.

This is exactly what the program does.

### There is a Java version

The Java version lies in the /java directory.
It uses a StringBuilder object to create the strings,
and it relies on an implementation of sha512 from the
apache commons codec.

You run the Java version with the following command:

./gradlew run

### Then there is a Rust version

The Rust version is stored in the /rust directory.
It uses Strings to concatenate the strings.
Also, there is a new trait implemented, which mimics 
the behavior of the StringBuilder.append(int i) from
the Java platform. Sort of. It is much simpler.

The rust version also uses the sha2 crate.

You can run the rust version with the following command:

run --package sha512 --bin sha512  --release

## Results

Bear in mind that both implementations are far from equal. 
They are, at the very best case, somewhat similar.

The challenge was to 'just implement' the code and run in both platforms,
trying to make the most similar decisions using the available apis.

This is how the Java version performed on my laptop. 

Each line corresponds to a round of 1 million strings built + hashing of each string:

task completed in: 7994ms
task completed in: 6235ms
task completed in: 6210ms
task completed in: 6286ms
task completed in: 6270ms
task completed in: 6468ms
task completed in: 6603ms
task completed in: 6478ms
task completed in: 6844ms
task completed in: 6152ms
task completed in: 6309ms
task completed in: 6195ms
task completed in: 6266ms
task completed in: 6360ms
task completed in: 6371ms
task completed in: 6402ms
task completed in: 6434ms
task completed in: 6292ms
task completed in: 6244ms
task completed in: 6225ms

And this is how the Rust version performed in my laptop:

The process took 347 milliseconds
The process took 333 milliseconds
The process took 374 milliseconds
The process took 334 milliseconds
The process took 338 milliseconds
The process took 332 milliseconds
The process took 333 milliseconds
The process took 334 milliseconds
The process took 337 milliseconds
The process took 336 milliseconds
The process took 334 milliseconds
The process took 334 milliseconds
The process took 336 milliseconds
The process took 337 milliseconds
The process took 335 milliseconds
The process took 337 milliseconds
The process took 340 milliseconds
The process took 379 milliseconds
The process took 342 milliseconds

If I haven't done any major mistake, from similar setups, rust is regularly hitting 20x faster performance than Java. Which blows me away, as it is much simpler to program than C due to the borrow checkers and the great compiler.

