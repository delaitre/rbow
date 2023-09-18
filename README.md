# rbow

A simple program colorizing its input based on regular expressions.

This is a command line and stripped down version of [flan](https://github.com/delaitre/flan).

While `flan` has a UI allowing live editing/enabling of matching expressions and support directly
several data input channels (stdin, serial port, file, scratch buffer...), `rbow` focuses on just
loading the expressions from a file and getting it's input from stdin. This allows the tool to be
much simpler and data sources are just external tools (minicom, cat...) piping their output into
`rbow`.

This project started a small project for me to learn Rust.

It's called `rbow` (rainbow) as it colorizes its input!
