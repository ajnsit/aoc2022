# Blockers

1. How do I do sliding window processing in Rust? Found that it's an unstable feature - `array_window`.

2. How do I use the array_window function? The signature looks like this -
   ```
    pub fn array_windows<const N: usize>(&self) ...
   ```

   The `N` is the window size, which I needed to be `4` for part 1. However none of the examples in the documentation mention how the size should be specified. The examples all use the default size of `2`. This is something I have seen earlier as well, Rust documentation is not always complete.

   So I decided to ask Chat-GPT (chat.openai.com). It gave me a plausible sounding but completely wrong answer - `array_windows(4)`!

   However, a little more searching on the web led me to "Const Generics" in Rust! Const generics currently give you type level integers, and using them is obvious in hindsight - `array_windows::<4>()`

3. How do I check a char slice for duplicates? In Haskell I would have done `sort` followed by `group`. Here, I could not find an analogue for group. The next approach was somehow deduping the slice and checking the length. However the closest to `dedup` I could find was `partition_dedup` which looked to be slightly different and required enabling a language extension.

   With the benefit of hindsight, I can say that asking Chat-GPT again after it had been so unhelpful the previous time was a bad decision. However, I asked Chat-GPT how to dedup a char slice, and it again gave me a plausible but completely wrong answer - `slice.dedup()`. It even helpfully provided a link to the corresponding section in the rust docs! But of course there is no such function defined on slices. I told GPT that there was no such functions for slices, and it came back with "You are correct there is no such function on slices, however this function actually exists on Iterator. You should first get an iterator from the slice and then call dedup like so - `slice.iter().dedup()`". Again, it included a rust doc link. The function `dedup` does not exist on `Iterator`s either, and the link goes nowhere. Fool me once...

   Eventually using `partition_dedup` to do this was easy. I wish I had looked closely at that function in the first place.

# Irritants

None. This was a pretty easy problem.

# Other new Things I learnt

None.
