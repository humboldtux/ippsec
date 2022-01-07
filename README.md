# README

An interactive cli to fuzzy find through [Ippsec Videos](https://ippsec.rocks/),
with the help of [Skim](https://github.com/lotabout/skim),
and open the selected videos in your browser.

## Installation

Use `cargo install` to install it:

```
$ cargo install ippsec
```

## Usage

To use it, just launch it at the command line:

```
$ ippsec
```

A Skim menu is created, listing every Ippsec videos, loaded from
[Ippsec Data](https://github.com/IppSec/ippsec.github.io/blob/master/dataset.json).
You can narrow down the results by using
[Skim search syntax](https://github.com/lotabout/skim#search-syntax).

![menu](https://user-images.githubusercontent.com/65201/148590365-147b47b2-c1fb-46b4-800a-5d0f827417b5.png)

Once your selection is made (see *Key Bindings* section), type *Enter*,
the selected videos are opened in your default browser.

## Key Bindings

| Key               | Action                                     |
|------------------:|--------------------------------------------|
| Enter             | Open selected videos url                   |
| ESC/Ctrl-G        | Abort                                      |
| Ctrl-P/Up         | Move cursor up                             |
| Ctrl-N/Down       | Move cursor Down                           |
| TAB               | Toggle selection and move down             |
| Shift-TAB         | Toggle selection and move up               |
| Alt-Z             | De-select All
