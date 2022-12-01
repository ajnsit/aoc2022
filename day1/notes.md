# Blockers

These are things that took a web search and some experimentation to figure out.

1. How do you read the input file? Found a neat trick that allows me to punt on it by reading in the input as a string at compile time using `include_str!`!
2. How do you sort in-place in Rust? Answer: you don't!
3. How do you sort in descending order? Answer: `sort_by_key` and the `Reverse` wrapper. `Reverse` is analogous to `Down` in Haskell.

# Irritants

These are things that gave me a pause but I could power through them.

1. I understand that `collect` needs a turbofish to specify the type. But why does `sum` need the type specified?!
2. The closure passed to `split` takes in refs. So if you do `.split(|x| x == 0)`, that will give you a strange sounding type error, something like "&u32 is not an integer". It makes sense in hindsight, but it's way too easy to ignore the `&` in the error message.
3. `Option<u32>` does not implement `Display`. Really? I had to use `{:?}` debug print.

# Other new Things I learnt

None
