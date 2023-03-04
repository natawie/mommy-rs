# mommy-rs
[![GitHub](https://img.shields.io/github/license/natawie/mommy-rs?style=for-the-badge)](https://github.com/natawie/mommy-rs/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/mommy-rs?style=for-the-badge)](https://crates.io/crates/mommy-rs)

mommy can now support you with 🪖type-safety🪖 and 🚀blazingly-fast🚀 speed~ ❤️

mommy will compliment you if things go well and will support you if something doesn't

## installation 📲
currently mommy-rs is only available through [crates.io](https://crates.io/crates/mommy-rs)

you can install mommy-rs from crates.io with:
`cargo install mommy-rs`

or from the repo directly with: 
`cargo install --git https://github.com/natawie/mommy-rs`

## usage 📜

```
USAGE:
    mommy-rs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --stdout     Prints to stdout instead of stderr
    -V, --version    Prints version information

OPTIONS:
    -e, --exit-code <exit-code>    Determines which type of response to print based on the exit code [default: 0]
```

### shell integration
to integrate mommy-rs into your shell you just need to add the following to your shell's config:

- zsh
    ```zsh
    precmd() { mommy-rs -s -e $? }
    ```
- bash
    ```bash
    PROMPT_COMMAND="mommy-rs -s -e \$?; $PROMPT_COMMAND"
    ```
- others
    ```sh
    export PS1="\$(mommy-rs -s -e \$?)$PS1"
    ```
    

## configuration ⚙️
configuration is done through environment variables:

`RUST_MOMMY_POSITIVE_RESPONSES`: mommy's positive respones : a semicolon separated string with placeholders following [the response format](#response-format-📖)

`RUST_MOMMY_NEGATIVE_RESPONSES`: mommy's negative respones : a semicolon separated string with placeholders following [the response format](#response-format-📖)

`RUST_MOMMY_PRONOUNS` : mommy's pronouns : a semicolon separated string of 3 pronouns(nominative, accusative, genitive)

`RUST_MOMMY_NAME` : mommy's name : a string

`RUST_MOMMY_USERS_NAMES` : names that mommy will call you : a semicolon separated string

`RUST_MOMMY_RESPONSE_COLOUR` : message colour : a comma separated RGB string

`RUST_MOMMY_ONLY_NEGATIVE` : mommy will only support you when something goes wrong : `"true" || "false"`

`RUST_MOMMY_ONLY_POSITIVE` : mommy will only compliment you : `"true" || "false"`

### response format 📖
- `{0}` - mommy's name
- `{1}` - mommy's pronouns
    - `{1.0}` - mommy's nominative pronoun(he/she/it/they/etc.)
    - `{1.1}` - mommy's accusative pronoun(him/her/it/them/etc.)
    - `{1.2}` - mommy's genitive pronoun(his/her/its/their/etc.)
- `{2}` - user's name

example: `{0} is so proud of {1.2} little {2}~ ❤️` -> `mommy is so proud of her little girl~ ❤️`

## acknowledgements 💖
if anything's wrong, please [open up an issue](https://github.com/natawie/mommy-rs/issues/new) or [contact me wherever](https://natawie.gay)

mommy thanks [Brodie Robertson](https://www.youtube.com/@BrodieRobertson) for making a video about [sudofox's shell-mommy](https://github.com/sudofox/shell-mommy)... otherwise i would've never known that there's such a thing

mommy thanks [Gankra](https://github.com/Gankra) for creating [cargo-mommy](https://github.com/Gankra/cargo-mommy) which is the original mommy

mommy thanks [FWDekker](https://github.com/FWDekker) for creating [mommy](https://github.com/FWDekker/mommy) from which i've gotten a few ideas and pinched some others(like this acknowledgements section... shush)
