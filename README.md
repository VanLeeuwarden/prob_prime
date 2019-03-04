# prob_prime

A Rust library implementing fast, probabilistic algorithms for checking whether integer is prime or not.
Strictly speaking, these are 'compositionality' tests; that is, the algorithms check whether a number is composite. If so (output=true) then the number is certainly composite. Otherwise the number is prime with high probability.

Implements:
* [Rabin-Miller](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test) correct with probability 4<sup>- # of rounds</sup> 
* [Baillie-PSW](https://en.wikipedia.org/wiki/Baillie%E2%80%93PSW_primality_test)




