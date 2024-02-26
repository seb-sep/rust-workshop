
Reference the slides [here](https://www.canva.com/design/DAF90PWKKUA/9Iq7PBZXjXR3RVi26JVysQ/view?utm_content=DAF90PWKKUA&utm_campaign=designshare&utm_medium=link&utm_source=editor)

[Godbolt, an online explorer for compiler outputs](https://godbolt.org/)

# Setup

Go to https://www.rust-lang.org/tools/install and follow the instructions, or:

#### Mac/Linux
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Windows
install [rustup](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) 


Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension for VS Code

In directory, run `cargo init [--bin/--lib] <project name>`

Use the VSCode extension or `cargo run` to build and run the project


# Exercise

You'll be implementing a simple version of `grep`: a command-line tool which takes in two arguments, the path of the file to search and the word to look for in the file. Create a new binary project 
and fill it in with the following steps:

- Extract the two command line arguments into a file path and a word
- Open the file and read it into a string
- Print each line of the file which contains the given word


