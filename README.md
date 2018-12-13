# prob_prime

A Rust library fimplementing a couple of fast, probabilistic algorithms for checking whether integer is prime or not. Implements:

Rabin-Miller (https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
Baillie-PSW  (https://en.wikipedia.org/wiki/Baillie%E2%80%93PSW_primality_test) 

Strictly speaking, these are 'compositionality' tests i.e the algorithms check whether a number is composite - if deemed so (output=true) then the number is certainly composite; if the output is false, then the number is prime with high probability.
