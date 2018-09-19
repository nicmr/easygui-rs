# easygui-rs

An extremely easy-to-use conrod interface for Rust. GUI interactions are invoked via simple function calls instead of an event-driven approach. 


## Introduction
[Conrod](https://github.com/PistonDevelopers/conrod) is one of the most popular pure Rust GUI libraries.
Conrod claims to be easy to use. Unfortunately it still lacks an up to date tutorial, which scares a lot of users away.

If you're just getting started with Rust or you just want to prototype some basic GUI functionality on top of your existing rust codebase, things can't get easy enough, so this crate aims to be even more accessible.

We achieve this by building an abstract interface on top of conrod, that calls basic GUI interaction via simple function calls instead of an event-driven approach.
```Rust
extern crate easygui;
use easygui::{ynbox, msgbox};

let choice: bool = ynbox("Can the maker repair what he makes?", "Title", ("Yes", "No"));
```
Python's [easygui](https://github.coma/robertlugg/easygui) successfully provides similar functionality for Tk, so we will try to model our interface after easygui for the early stages of development.
Later on, we might optionally expose more of conrod's complexity to the user.

Finished parts of the interface:
- [ ] msgbox
- [ ] ccbox
- [ ] ynbox
- [ ] buttonbox
- [ ] indexbox
- [ ] boolbox
- [ ] choicebox
- [ ] multchoicebox
- [ ] enterbox
- [ ] integerbox
- [ ] multenterbox
- [ ] passwordbox
- [ ] multpasswordbox
- [ ] textbox
- [ ] codebox
- [ ] diropenbox
- [ ] fileopenbox
- [ ] filesavebox
- [ ] EgStore    ?
- [ ] exceptionbox    ?
