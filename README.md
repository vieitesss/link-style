# Link-Style

Link-Style is a text formatter designed for creating posts in a simplified manner. This tool saves you the hassle of using external formatting applications that require constant copying and pasting. With Link-Style, you can write your post in a reduced Markdown-like format, pass the file to the program, and receive the formatted text ready to use.

## Prerequisites

- **Cargo**: Ensure that Cargo, the Rust package manager, is installed on your system. Cargo is the only requirement for building and running this project.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/vieitesss/link-style ~/link-style
   ```
2. Navigate into the project directory:
   ```bash
   cd ~/link-style
   ```
3. Compile and install using `make`:
   ```bash
   make && make install
   ```
   Alternatively, if you are using `just`, you can compile and install with:
   ```bash
   just
   ```

## Usage

To format text, call `link-style` with a text file as an argument. 

- Text surrounded by single asterisks (`*`) will be formatted as **italic**.
- Text surrounded by double asterisks (`**`) will be formatted as **bold**.
- You can also nest formatting: for example, `**bold *italic***` will format text as both **bold and italic**.

The program outputs the formatted text, which can then be directly copied to the system clipboard for easy use in your posts.

### Copy to Clipboard

For ease of use, here are the commands to copy the formatted text to the clipboard on Linux and macOS:

- **Linux**: Use `xclip` to copy the output directly to the clipboard:
  ```bash
  link-style yourfile.txt | xclip -selection clipboard
  ```

- **macOS**: Use `pbcopy` to copy the output directly to the clipboard:
  ```bash
  link-style yourfile.txt | pbcopy
  ```

Now you can paste the text directly into almost any social media!
