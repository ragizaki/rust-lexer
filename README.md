# Rust Lexer
A lexer/scanner for the LOX programming language, written in Rust. This scanner uses a traditional approach of keeping track of the current position and token,
and peeks at next characters until we reach the end of file, in which case we are done.

## TODO
[x] Replace `String` with `&str` where it makes sense to optimize performance and clarify ownership of variables
[x] Build out lexer to scan `.lox` files and output corresponding tags (e.g. `LEFT_PAREN ( null`).
[x] Handle reserved keywords and identifiers
[ ] Build abstract syntax tree (AST) from the scanned tokens
