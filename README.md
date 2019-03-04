# prob_prime

A Rust library implementing a couple of fast, probabilistic algorithms for checking whether integer is prime or not. 
Implements:
* [Rabin-Miller](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
* [Baillie-PSW](https://en.wikipedia.org/wiki/Baillie%E2%80%93PSW_primality_test) 

Strictly speaking, these are 'compositionality' tests; that is, the algorithms check whether a number is composite. If so (output=true) then the number is certainly composite. Otherwise the number is prime with high probability.
