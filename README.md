# easygui-rs

An extremely easy-to-use conrod interface for Rust. GUI interactions are invoked via simple function calls instead of an event-driven approach. 


## Introduction
[Conrod](https://github.com/PistonDevelopers/conrod) is one of the most popular pure Rust GUI libraries.
One of its goals is being easy to use. Unfortunately, it still lacks a complete tutorial and requires a fair bit of boilerplate. This results in a disproportionate time investment for those that are just looking for basic GUI functionality.

Easygui intends to provide an extremely accessible interface for these basic gui needs.

We achieve this by building an abstract interface on top of conrod, that allows invoking basic GUI interaction via simple function calls instead of an event-driven approach.
```Rust
extern crate easygui;
use easygui::ynbox;

let choice: bool = ynbox("Can the maker repair what he makes?", "Title", "Yes", "No");
```
[Python's easygui](https://github.coma/robertlugg/easygui) successfully provides similar functionality for Tk, so we will try to model our interface after easygui for the early stages of development.
Later on, we might optionally expose more of conrod's complexity to the user, or add handy features like reading the user's preferred color schemes from dotfiles.

Implemented parts of the interface:
- [x] msgbox
- [x] ynbox  
- ~~ccbox~~ *scrapped, basically just a ynbox*
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

Documentation finished for

- [ ] msgbox
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
