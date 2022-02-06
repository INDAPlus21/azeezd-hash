# Azeez Daoud - Database Database Wow wow!


# Mathy math
## Formula for size checking
The Map's *max* is the length of the internal array (buckets). It also has a field called *size* which is the amount of occupied slots inside that internal array.

Automatic resizing is done when the *size* is 75% of the *max*. The formula to check this uses bitshifts for faster calculations.

Call *max* **N** and occupation size for **n** then

<img src="./images/size_control_formula.png" width="700">

hence the check for the size is done with:
```rust
// max = self.buckets.len()
self.size > (max >> 1) + (max >> 2)
```