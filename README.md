# jl

**jl** is a terminal-based journaling app for taking notes, writing descriptions, rating your day, and answering prompts.

Everything is stored locally, so your data is as safe as your computer.

---

## Installation Guide

To install **jl** locally run the first command, and to also install the binaries in /usr/local/bin run the second:

```bash
$ make install-user
$ sudo make install-bin
```

To uninstall it:

```bash
sudo make uninstall
```

---

## Usage

```bash
jl [OPTIONS]
```

---

## Options

| Flag | Long Option | Argument | Description |
|------|------------|----------|-------------|
| `-d` | `--description` | `[DESCRIPTION]` | Talk about how your day was |
| `-n` | `--note` | `[NOTE]` | Add a short note during the day |
| `-r` | `--rating` | `[RATING]` | Rate your day out of 10 (can be any number) |
| `-s` | `--sometimes` | `[SOMETIMES]` | Lower chances of a question being asked (`true` / `false`) |
| `-e` | `--entries` | `[<ENTRIES>]` | Show all entries into the journal [possible values: true, false] |
| `-u` | `--update` | `<UPDATE>` | Update journal from x days ago |
| `-h` | `--help` | — | Print help |

---

## Examples

Most flags work either by providing the argument immediately after the flag, or by entering the flag alone and typing your input on the following lines.

```
$ jl -d "Today was productive and calm"

$ jl -d
Add a description about the day:
>> Loved how everything just felt right today
>> met my friends at the bar and we had so much fun 

$ jl -n "i'm at home and i feel so boared, just want to lay down and do nothing all day"

$ jl -r 8

$ jl -u 2
```

## Questions file

This file contains all the questions you want to be asked when calling jl or when doing jl -s in your bashrc. The short questions are meant to be answered with numbers or short text, while the long ones expect a multi line answer. One example of a question file I currently use is the one below, mostly tracking health habits I'm currently interested in. Place this file in the ~/.jl directory.

```
s: Number of coffees? 
s: Grams of sugar?
s: Cups of tea?
s: Hours of sleep?
s: Ate meat?
s: Number of beers?

l: What's a nice thing from today?
l: What's an upsetting thing from today?
```
