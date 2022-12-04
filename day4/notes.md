# Blockers

None. The puzzle was quite easy today!

# Irritants

1. Look at all the `unwrap`s below. What is the replacement for the Maybe monad of Rust? -
   ```
    let (s1, s2) = s.split_once(",").unwrap();
    let (sx1, sy1) = s1.split_once("-").unwrap();
    let (sx2, sy2) = s2.split_once("-").unwrap();
    (
        (sx1.parse().unwrap(), sy1.parse().unwrap()),
        (sx2.parse().unwrap(), sy2.parse().unwrap()),
    )
   HashSet::from(sa.chars())
   HashSet::from(sa.chars.collect::<Vec<char>>())
   ```

# Other new Things I learnt

None
