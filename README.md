# aster of code

aster's advent of code solutions in rust!

## usage

> [!NOTE]
> you may need to install the rust nightly toolchain to compile this project. to do so, run `rustup toolchain install nightly`.

to run both parts of a given day:

`cargo run -- <DAY> --file <INPUT_FILE>`

where `<INPUT_FILE>` is the name of a file containing your puzzle input.



you can also have your puzzle input fetched dynamically by creating a `.env` file that sets the `SESSION_ID` variable:

```
SESSION_ID=<YOUR_SESSION_ID>
```

where `<YOUR_SESSION_ID>` is the session token passed as a cookie in your requests to <https://adventofcode.com>. this can also be passed as a regular environment variable or via the `--session-id` option.

for more options, run `cargo run -- --help`.
