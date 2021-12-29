# Java vs Rust Performance
## An exercise on string building and hashing

This project has the objective of comparing the performance of two programs,
one written in Java, another in Rust.

The case goes as follows:

Suppose you had to build a string and then take a sha512 hash of it.

You do that 1 million times.

This is exactly what the program does.

There is a second use case, which is to just create the same sha512 hash from the same string, but this time the algorithm does that 2 million times.

---
### There is a Java version

The Java version lies in the `/java` directory.
It uses a StringBuilder object to create the strings,
and it relies on an implementation of sha512 from the
apache commons codec.

You run the Java version with the following command:

```./gradlew run```

You can choose which use case will be called by the program, by uncommenting one of the two methods in the main method.

```buildAndHash20x()``` will call the first use case: building slightly different strings and hashing them. 1 million strings total. The process is done 20 times.

```hashIt20x()``` will hash the same string 2 million times. This process is done 20 times in order to get a feel of the performance variance.

---
### Then there is a Rust version

The Rust version is stored in the `/rust` directory.
It uses Strings to concatenate the strings.
Also, there is a new trait implemented, which mimics 
the behavior of the StringBuilder.append(int i) from
the Java platform. Sort of. It is much simpler.

The rust version also uses the sha2 crate.

You can run the rust version with the following command:

```run --package sha512 --bin sha512  --release```

You can choose which use case will be called by the program, by uncommenting one of the two methods in the main method.

```buildAndHash20x()``` will call the first use case: building slightly different strings and hashing them. 1 million strings total. Do that 20 times.

```just_hash_it_x20()``` will hash the same string 2 million times. Do that 20 times.

---
## Results

Bear in mind that both implementations are far from equal. 
They are, at the very best case, somewhat similar.

The challenge was to 'just implement' the code and run in both platforms,
trying to make the most similar decisions using the available apis.

### This is for the first case (building strings and hashing it):

Each line corresponds to a round of 1 million strings built + hashing of each string:

This is how the Java version performed on my laptop.

_task completed in: 7994ms  
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
task completed in: 6225ms_

And this is how the Rust version performed in my laptop:

_The process took 347 milliseconds  
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
The process took 342 milliseconds_ 

If I haven't done any major mistake, from similar setups, rust is regularly hitting 20x faster performance than Java. Which blows me away, as it is much simpler to program than C due to the borrow checkers and the great compiler.

### This is for the second use case hashing the same string 2 million times):
JAVA  
_task completed in: 13610ms  
task completed in: 10680ms  
task completed in: 9594ms  
task completed in: 10511ms  
task completed in: 9882ms  
task completed in: 9507ms  
task completed in: 10209ms  
task completed in: 10708ms  
task completed in: 10056ms  
task completed in: 9580ms  
task completed in: 9807ms  
task completed in: 9601ms  
task completed in: 10630ms  
task completed in: 10817ms  
task completed in: 11589ms  
task completed in: 10575ms  
task completed in: 11370ms  
task completed in: 11167ms  
task completed in: 10085ms  
task completed in: 11136ms_

RUST:  
_Just hash it took 1249 milliseconds for 2000000 cycles  
Just hash it took 1216 milliseconds for 2000000 cycles  
Just hash it took 1283 milliseconds for 2000000 cycles  
Just hash it took 1136 milliseconds for 2000000 cycles  
Just hash it took 1177 milliseconds for 2000000 cycles  
Just hash it took 1184 milliseconds for 2000000 cycles  
Just hash it took 1447 milliseconds for 2000000 cycles  
Just hash it took 1183 milliseconds for 2000000 cycles  
Just hash it took 1235 milliseconds for 2000000 cycles  
Just hash it took 1304 milliseconds for 2000000 cycles  
Just hash it took 1170 milliseconds for 2000000 cycles  
Just hash it took 1091 milliseconds for 2000000 cycles  
Just hash it took 1129 milliseconds for 2000000 cycles  
Just hash it took 1355 milliseconds for 2000000 cycles  
Just hash it took 1110 milliseconds for 2000000 cycles  
Just hash it took 1161 milliseconds for 2000000 cycles  
Just hash it took 1257 milliseconds for 2000000 cycles  
Just hash it took 1186 milliseconds for 2000000 cycles  
Just hash it took 1293 milliseconds for 2000000 cycles  
Just hash it took 1222 milliseconds for 2000000 cycles_
