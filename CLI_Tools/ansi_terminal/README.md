# ANSI Terminal

This program depicts the use of [`ansi_term`] crate and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals.

There are two main data structures in [`ansi_term`]: [`ANSIString`] and [`Style`]. A [`Style`] holds stylistic information: colours, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An [`ANSIString`] is a string paired with a [`Style`].

[documentation]: https://docs.rs/ansi_term/
[`ansi_term`]: https://crates.io/crates/ansi_term
[`ANSIString`]: https://docs.rs/ansi_term/*/ansi_term/type.ANSIString.html
[`Style`]: https://docs.rs/ansi_term/*/ansi_term/struct.Style.html
[`Style::new()`]: https://docs.rs/ansi_term/0.11.0/ansi_term/struct.Style.html#method.new
