# Blockers

1. The following code fails to compile because `intersection` demands that I pass it references. And it returns an iterator for the references.
   I needed something like `intersection_into` but I couldn't find a method like that.

   ```
   map(|(a,b)| {
     let sa = a.chars().collect::<HashSet<char>>();
     let sb = b.chars().collect::<HashSet<char>>();
     sa.intersection(&sb).collect::<HashSet<&char>>()
   })
   ```

   However, after flailing around for a bit, on a whim I tried just dereferencing with `*` and that worked!

   ```
   sa.intersection(&sb).map(|x| *x).collect::<HashSet<char>>()
   ```

   I am still not quite certain why this works. When can I dereference and when I cannot. And why do we have an `iter()` vs a `into_iter()`, but not a corresponding `into_intersection()` to go with `intersection()`. What's the general rule here?

2. How do I convert a `char` to an integer?

   `to_digit()` only converts string representations e.g. converts '1' to 1.

   Took some searching, but apparently my old friend the cast to u32 did the trick!

   ```'a' as u32```

# Irritants

1. How can I create a hashset of the characters in a string? Neither of these worked -
   ```
   HashSet::from(sa.chars())
   HashSet::from(sa.chars.collect::<Vec<char>>())
   ```

   But this worked -
   ```
   sa.chars().collect::<HashSet<char>>()
   ```

2. This monstrosity!

   ```
      h1.intersection(&h2)
        .map(|x| *x)
        .collect::<HashSet<char>>()
        .intersection(&h3)
        .map(|x| priority(*x))
        .sum::<u32>()
   ```

   I tried a bunch of things, but this seems to be the shortest way to intersect three hashsets together! In particular I can't just do chained intersections like `foo.intersection().intersection()` because `intersection` is not defined for iterators.

# Other new Things I learnt

None

---

# Improvements made afterwards / Comments by other people

1. Use `collect::<Vec<_>>()` instead of `collect::<Vec<u32>>()`.
2. Use range pattern matching - `..=`
3. Use `b'a'` to get a `u8` value for a char `'a'`.
4. Use `copied()` instead of `.map(|x| *x*)`. (Or `cloned()`, when the elements are `Clone`). This communicates the intent better, since the dereference won't work with elements which are not `Copy`.
5. Rust won't add `intersection` for iterators since it requires extra allocation. So yes, the way I did a 3 way intersection was fine.
