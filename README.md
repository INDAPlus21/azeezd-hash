# Azeez Daoud - Database Database Wow wow!


# Mathy math
## Formula for size checking
The Map's *max* is the length of the internal vector (buckets). It also has a field called *size* which is the amount of occupied slots inside that internal vector.

Automatic resizing is done when the *size* is 75% of the *max*. The formula to check this uses bitshifts for faster calculations.

Call *max* **N** and occupation size for **n** then

<img src="./images/size_control_formula.png" width="700">

hence the check for the size is done with:
```rust
// max = self.buckets.len()
self.size > (max >> 1) + (max >> 2)
```

## Initial size and resize formula
As mentioned in [Size Checking](formula-for-size-checking), when the amount of occupied slots reach ~75% the Map resizes itself using the formula below.
Note the initial size for the Map is 31 and continues on increasing using the "* 2 - 1".

<img src="./images/map_resize.png" width="700">

The first few numbers that this sequence produces are primes or have few factors of large prime numbers. Here are some of the few terms
<img src="./images/resize_sequence.png" width="700">