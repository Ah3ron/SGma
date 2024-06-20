# SGma Programming Language

## Overview

SGma is a simple, esoteric programming language designed for fun and educational purposes. It uses a unique set of commands inspired by humorous and informal phrases in the Russian language. This project includes a lexer for parsing SGma programs and converting them into executable commands.

## Commands

SGma operates on an array of memory cells, each initially set to zero. There is a pointer, initially pointing to the first memory cell. 

The SGma programming language consists of the following commands:

- `гол` - Increment the current memory cell.
- `шахматы` - Decrement the current memory cell.
- `сигма` - Move the memory pointer to the left.
- `гойда` - Move the memory pointer to the right.
- `лол` - Output the character signified by the cell at the pointer.
- `кринж` - Input a character and store it in the cell at the pointer.
- `какать` - Jump past the matching `стэтхэм` if the cell at the pointer is 0
- `стэтхэм` - Jump back to the matching `какать` if the cell at the pointer is nonzero

## Installation

To install the SGma programming language interpreter, you can either clone the project repository and compile it yourself or download the pre-compiled binaries from the releases section on GitHub.

### Option 1: Compile from Source

1. Clone the repository:

```sh
git clone https://github.com/Ah3ron/SGma.git
cd SGma
```

2. Compile the project:

```sh
cargo build --release
```

### Option 2: Download Pre-compiled Binaries

Visit the [releases page](https://github.com/Ah3ron/SGma/releases) on GitHub to find and download the latest release suitable for your platform. Follow the instructions provided in the release notes for installation and setup.

## Contribution

Feel free to contribute to the project by opening issues, submitting pull requests, or suggesting new features. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments


Almost all of the code was taken from this [project](https://github.com/Jomy10/Brainfuck-rs/). Many thanks to its author.
Also the whole language idea was copied from [brainfuck](https://esolangs.org/wiki/Brainfuck) programming language.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

[MIT license](LICENSE.txt)
