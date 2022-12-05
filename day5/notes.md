# Blockers

1. How do I skip over the last line of an iterator? Looks like it can't be done (makes sense). You have to `collect().pop()`.

2. Is it possible to iterate over some elements of a vec, generate a structure, and then continue processig the remaining elements of the vec? I didn't spend enough time exploring this because I decided to split the problem into into two parts with `split_once("\n\n")` and process the parts separately. So this is still TODO.

# Irritants

1. Transposing a matrix in Rust caused a lot of headache! The immutable/functional solutions I came up with would not compile, giving me lifetime or borrowing errors. Eventually I settled on a very imperative solution.

2. I decided to use the regex crate for parsing some things. This wasn't really needed, but I wanted to get started with it for later problems. It was pretty easy to use, except for all the unwrapping involved! Look at this code -

    ```
    let captures = Regex::new(r"move (\d+) from (\d+) to (\d+)")
        .unwrap()
        .captures(s)
        .unwrap();
    (
        captures[1].parse::<usize>().unwrap(),
        captures[2].parse::<usize>().unwrap(),
        captures[3].parse::<usize>().unwrap(),
    )
    ```

3. You can't `append` a mutable vector into another mutable vector! I had to instead use `extend` an iterator.

4. Adding elements at the beginning of a vector was harder than it should have been! Until I realised that I can get owned elements out of a vector with `drain(..)`, then `chain` it to the other vector. i.e. -

      ```
      blocks[to - 1] = moving.into_iter().chain(blocks[to - 1].drain(..)).collect();
      ```

# Other new Things I learnt

1. When using regex, `Captures[0]` is not the first capture group, but is the whole string!

2. You can `collect` an iterator of `char`s into a `String`! Use `.collect::<String>()`.
