# Blockers

Surprisingly none! There was nothing that took too long to figure out myself.

# Irritants

1. I wanted to return a vec of strings from the `input` function. However, I can't just return a `Vec<&str>`, as I need to specify a lifetime! Thankfully the lifetime in this case could simply be `'static` as the string was pulled in at compile time using `include_str!`.

# Other new Things I learnt

1. `split_once` splits a string into two parts and returns a tuple of strings.
