original problem
NSTimeInterval and count
count is unsigned integer
how much space is count of integers in an array?

try changing the order
does uint * -1 == -1 * uint

are uints faster?

other languages have similar relations
c,c++,objc
 c doesn't have array.length
 c++ vector::size is of size_type which is unsigned itegral type

d
 same results as c
 array.length is a size_t (which seems to be the same)
 d recommends using size_t for array size and indexing
 (so it encourages the behavior)

rust
 returns a uint for the length of std::vec
 did the same behavior as c, was a little tougher on types

java?
  no unsigned types

c#?
 gets it right by either converting to long
 or refusing to multiply if its a ulong
 System.Array.Length is int32
 ArrayList.count is int32

go? go gets it "right" in that it doesn't get flip
  maybe break it down to the specific rule
 does it have an array count
 (does it break on trying to multiply a ulong and integer?)
 nope just gets it correct


might be nice to have a real abstraction
ada
  has a "natural" subtype bounds checking which can be turned off for performance
  what happens when I turn it off?
  and also has a mod type which does wrap

haskell


f#


are unsigned types faster than signed types?

what does array.count return in each language?


why would it be nice?


do others have a scheme for creating suptypes like ada and haskell?


problem is with dynamic typing of int and uint and floats



how could I write something in ada
