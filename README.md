# Kata: Count the "the"s

## Challenge

Count the "the" from stdin without an "if" or a loop.

## Test Data

 * "hello to the world" => 1
 * "the quick brown fox jumps over the lazy dog" => 2
 * "The quick brown fox jumps over the lazy dog" => 2
 * "Their name is The." => 2
 * "Their name is The\0" => 1
 
# Running

Format code

```
rustup run nightly cargo fmt --all --
```

Run tests

```
$ rustup run stable cargo test
   Compiling kata-count-number-of-the v0.1.0 (file:///tmp/kata-count-number-of-the)
    Finished dev [unoptimized + debuginfo] target(s) in 2.49 secs
     Running target/debug/deps/countthe-fe3999f5b5c72633

running 5 tests
test tests::it_counts_ignores_capitalisation ... ok
test tests::it_counts_the_multiple_the ... ok
test tests::it_ignores_punctuation ... ok
test tests::it_null_characters_at_the_end_of_string_are_stripped ... ok
test tests::it_the_single_the ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/countthe-e8ab169716f0301f

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests countthe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

Run the app

```
echo "Their name is The" | rustup run stable cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/countthe`
1
```