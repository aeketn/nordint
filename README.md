# Goal of this Project
To understand the Schonhage-Strassen Algorithm of multiplication and implement the algorithm for a simple Big Integer struct in Rust.

## Breakdown of Directories
### rings-and-groups -- A practice exercise to understand Discrete Fourier Transforms
### dft-matrices -- A practice exercise to understand Discrete Fourier Transforms
### nordint -- The Big Integer Implementation

# About: Exercise 01 Rings and Groups

This is a practice exercise geared toward understanding how the `Discrete Fourier Transform (DFT)` works. Building a table under some modulus `N` for the operators addition and multiplication.  

It is imporant to notice that there are no zeros outside of the first row and first column for a multiplication table where `N` is a prime number.

# About: Exercise 02 DFT-Tables

This is a practice exercise that expands on the `Rings and Groups` exercise. This exercise focuses solely on the multiplication table as it applies to the `nth roots of unity` for a given `N`.  

In this exercise, notice patterns where `N` is a `power of 2` and how that aligns with the numbers `1`, `i`, `-1`, and `-i`.

# About: Exercise 03 Number-Theoretic Transform

This is a practice exercise that expands on the `DFT Tables` exercise. This exercise applies a `Number-Theoretic Transform (NTT)`, a special case of the `DFT`) that is defined specifically for modular arithmetic on integers. No complex numbers are used in this transform. Rather, the unit circle in the complex plane `(regular DFT)` is replaced in theory with cycling around the modulus of a given range of integers `(NTT)`.  

*Helpful Reading on NTT:* https://www.nayuki.io/page/number-theoretic-transform-integer-dft

# About: Exercise 04 Convolution Theorem

This is a practice exercise that expands on the `Number-Theoretic Transform` exercise. This exercise applies the `NTT` to two arrays of numbers and convolves them using `point-wise multiplication`. Once the convolution has been computed, the point-wise product is then transformed through the `INTT`. The result is the equivalent of multiplying the two numbers that the arrays represent. Details about how to interpret the convolution are specified in the `comments` within the `main()`.

*Helpful Reading on NTT:* https://www.nayuki.io/page/number-theoretic-transform-integer-dft

# About: Exercise 05 Fast Fourier Transform

This is a practice exercise that expands on the `Number-Theoretic Transform` exercise. The `Fast Fourier Transform (FFT)` computes the same result as the `Number-Theoretic Transform (NTT)` in `O(n log(n))` time, down from `O(n^2)` time. This particular `FFT` ipmlementation is recursive and follows the `Cooley-Tukey` algorithm.

More reading on `Cooley-Tukey FFT`: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm 

# About: nordint
A Simple (Unoptimized) Big Integer in Rust  

I couldn't resist naming this "nordint" because my last name is "Nordin." I don't expect anyone to actually use this in any production code, so I feel that a silly name is not unwarranted.  

Traditionally, a BigInteger would represent numbers in a number base that is a power of 2. For example, the most-used BigInteger on crates.io represents numbers in base 2^32. This internal representation yields many optimizations that the computer can utilize with bit shifting etc.  

My Big Integer is represented in base 1,000,000,000. This, in practice, is much slower for the computer to work with, but is much easier for me to think about.  

Since the goal of this is about an algorithm, and not about an optimally efficient implementation of a Big Integer, a base that is a power of 10 rather than a power of 2 makes more sense to me.