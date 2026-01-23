jl is a terminal based journaling app meant for taking notes, writing descriptions, rating and answering to promts about your day. Everything is stored locally so your data is as safe as your computer. 

# Instalation guide:
To install jl run:
$ sudo make install

And to uninstall it, run: 
$ sudo make uninstall

# Flags:

Usage: jl [OPTIONS]

Options:
  -d, --description [<DESCRIPTION>]  Talk about how your day was
  -n, --note [<NOTE>]                Add a short note during the day
  -r, --rating [<RATING>]            Rate your day out of 10 (can be any number)
  -s, --sometimes [<SOMETIMES>]      Lower chances of a question being asked [possible values: true, false]
  -u, --update <UPDATE>              Update journal from x days ago
  -h, --help                         Print help

