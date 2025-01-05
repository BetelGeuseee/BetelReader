```markdown
# Betel

Betel is a command-line application written in Rust that allows you to browse and view manga chapters directly from your terminal. It fetches manga chapters from a specified website, caches your reading history, and displays images using the `feh` image viewer.

## Table of Contents


- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Listing Chapters](#listing-chapters)
  - [Viewing a Chapter](#viewing-a-chapter)
- [Caching](#caching)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Features

- **List Chapters**: Retrieve and display a list of available manga chapters. - One Piece
- **View Chapters**: Download and view specific manga chapters using the `feh` image viewer.
- **Caching**: Maintains a history of viewed manga using a JSON cache file.
- **Configuration**: Easily configurable via a `Config.toml` file.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **feh**: Install `feh`, a lightweight image viewer.

    - **Ubuntu/Debian**:

      ```bash
      sudo apt-get update
      sudo apt-get install feh
      ```

    - **Arch Linux**:

      ```bash
      sudo pacman -S feh
      ```

    - **macOS** (using Homebrew):

      ```bash
      brew install feh
      ```

### Clone the Repository

```bash
git clone https://github.com/BetelGeuseee/BetelReader.git
cd betel-manga
```

### Build the Application

```bash
cargo build --release
```

The executable will be located at `target/release/betel`.

## Configuration

Create a `Config.toml` file in the project's root directory with the following structure:

```toml
[config]
cache = "/home/.cache/manga-his.json"
url = "https://example.com/manga/{{}}"
chapter_list = "https://example.com/manga/chapters"
```

- **cache**: Path to the JSON file used for caching your reading history.
- **url**: URL template for accessing specific manga chapters. Use `{{}}` as a placeholder for the chapter number.
- **chapter_list**: URL to the page that lists all available chapters.

## Usage

Navigate to the project directory and run the executable with the desired options.

Copy the executable to the bin directory 

```bash
cp target/release/betel /usr/local/bin
```
### Listing Chapters

To list all available manga chapters:

```bash
betel --list
```

This command fetches the chapter list from the URL specified in the `chapter_list` configuration and displays them in the terminal.

### Viewing a Chapter

To view a specific chapter, use the `--num` (or `-n`) option followed by the chapter number:

```bash
betel --num 5
```

This command formats the chapter URL using the provided number, downloads all images in the chapter, and displays them using `feh`.

#### Command-Line Options

- `-n, --num <NUMBER>`: Specify the chapter number to view.
- `-l, --list`: List all available chapters.

### Examples

- **List Chapters**:

  ```bash
  betel --list
  ```

- **View Chapter 10**:

  ```bash
  betel --num 10
  ```

## Caching

Betel maintains a cache to keep track of your reading history. The cache file is specified in the `Config.toml` and is stored in JSON format. This allows the application to remember which chapters you've viewed, enhancing your browsing experience.

## Testing

To run the tests, use the following command:

```bash
cargo test
```

This will execute the tests defined in the `tests` module to ensure that the configuration is read correctly and other functionalities are working as expected.

## Contributing

Contributions are welcome! 


## Note
This README is generated by LLM.

