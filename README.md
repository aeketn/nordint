# Goal of this Project
To understand the Schönhage-Strassen Algorithm of multiplication and implement the algorithm for a simple Big Integer struct in Rust.  
I wrote this project for an 8-week summer course on Rust programming. The project topic was entirely up to the student. This is an algorithm I had been wanting to explore.

## Breakdown of Directories
### nordint -- The Big Integer Implementation
### rings-and-groups -- A practice exercise to understand Discrete Fourier Transforms (DFT)
### dft-matrices -- A practice exercise to understand Discrete Fourier Transforms (DFT)
### number-theoretic-transform -- A practice exercise to understand Number Theoretic Transforms (NTT)
### convolution-theorem -- A practice exercise to understand how NTTs can be applied to multiplication of polynomials
### fast-fourier-transform -- A practice exercise to understand the Fast Fourier Transform (FFT) version of the NTT. 

### Thank you!
Thank you to Professor Bart Massey of Portland State University for helping me get started and providing me with excellent reference material about the algorith.  

Thank you to Professor Bryant York of Portland State University for giving me an overview of the theory behind a DFT, and for suggesting that I implement exercises 01 and 02 to get started.

Thank you to my friend Steven Libby (Graduate Student at Portland State University) for your conceptual insights about the algorithm.

# About: nordint
A Simple (Unoptimized) Big Integer in Rust, implemented only to explore the Schönhage-Strassen Algorithm.

I couldn't resist naming this "nordint" because my last name is Nordin. I don't expect anyone to actually use this in any production code, so I feel that a silly name is not unwarranted.  

Traditionally, a `BigInteger` would internally represent numbers in a number base that is a power of `2`. For example, the `num-bigint` on crates.io represents numbers in base `2^32`. This internal representation yields many optimizations that the computer can utilize by bit shifting.

My `BigUint` is represented in base `100`. This, in practice, is much, much less efficient for the computer to work with; however, working in a base that is a power of `10` (rather than a power of `2`) is much easier for a human to think about. Initially, I had chosen an internal representation of base `1,000,000,000` which is still slower than base `2^32`, but faster than base `100`. When I first started this project, I had the idea that I would implement this algorithm and benchmark it against the `BigUint` on crates.io for multiplcation of very large numbers. After all, that is what this algorithm was designed to do. Unfortunately, implementing the algorithm in such a way that it would be optimized enough to benchmark would require more time than what I had in this eight-week course.   

In short, the algorithm requires that many calculations and setups be made beforehand in order to compute the number-theoretic fast fourier transform. Many of the ways that I decided to compute these setup requirements are naive and quite slow. Without a quick setup, there is no way that this algorithm would be comprable to even naive `O(n^2)` multiplication.  

So this project turned into more of a proof of concept, and a learning exercise in how to preform a number-theoretic fast fourier transform. The `BigUint` does implement the full algorithm correctly (albeit extremely inefficiently).  Below are the descriptions of 5 exercises that I designed to help build up to performing the necessary transforms in the context of multiplcation.

# About: Exercise 01 Rings and Groups

This is a practice exercise geared toward understanding how the `Discrete Fourier Transform (DFT)` works. This is a simple exercise that explores the ring of integers contained within a number field. In this case, we are building a table under some modulus `N` for the operators addition and multiplication.  

It is imporant to notice that there are no zeros outside of the first row and first column for a multiplication table where `N` is a prime number.

# About: Exercise 02 DFT-Tables

This is a practice exercise that expands on the `Rings and Groups` exercise. This exercise focuses solely on the multiplication table as it applies to th `nth complex roots of unity` for a given `N`.  

More reading on the `Roots of Unity`: https://en.wikipedia.org/wiki/Root_of_unity

In this exercise, notice patterns where `N` is a `power of 2` and how that aligns with the numbers `1`, `i`, `-1`, and `-i`.

# About: Exercise 03 Number-Theoretic Transform

This is a practice exercise that expands on the `DFT Tables` exercise. This exercise applies a `Number-Theoretic Transform (NTT)`, a special case of the `DFT`) that is defined specifically for modular arithmetic on integers. No complex numbers are used in this transform. Rather, the unit circle in the complex plane `(as it was in the last exercise for a regular DFT)` is replaced in theory with cycling around the modulus of a given range of integers.

*Helpful Reading on NTT:* https://www.nayuki.io/page/number-theoretic-transform-integer-dft

# About: Exercise 04 Convolution Theorem

This is a practice exercise that expands on the `Number-Theoretic Transform` exercise. This exercise applies the `NTT` to two arrays of numbers and convolves them using `point-wise multiplication`. Once the convolution has been computed, the point-wise product is then transformed through the `INTT`. The result is the equivalent of multiplying the two numbers that the arrays represent. Details about how to interpret the convolution are specified in the `comments` within the `main()`.

*Helpful Reading on NTT Convolution Theorem:* https://www.nayuki.io/page/number-theoretic-transform-integer-dft

# About: Exercise 05 Fast Fourier Transform

This is a practice exercise that expands on the `Number-Theoretic Transform` exercise. The `Fast Fourier Transform (FFT)` computes the same result as the `Number-Theoretic Transform (NTT)` in `O(n log(n))` time, down from `O(n^2)` time. This particular `FFT` ipmlementation is recursive and follows the `Cooley-Tukey` algorithm.

More reading on `Cooley-Tukey FFT`: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm 
