# Scrappy

![Scrappy Scrappy Doo.](logo.jpeg)

Scrappy is a simple CLI tool that helps you collect and format code from a directory structure, making it easy to ask questions on platforms like ChatGPT by providing complete code context. With Scrappy, you can quickly generate a single Markdown file containing all the code from a module or project, so you don't have to manually copy and paste each file.

[![Test](https://github.com/WezSieTato/scrappy/actions/workflows/test.yml/badge.svg)](https://github.com/WezSieTato/scrappy/actions/workflows/test.yml)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [Testing](#testing)
- [Acknowledgements](#acknowledgements)
- [License](#license)

## Installation

To install Scrappy, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, and then run the following command:

```sh
cargo install --path .
```

## Usage

To use Scrappy, navigate to the directory containing the code you want to compile and run the following command:

```sh
scrappy <source_directory> <output_file>
```

For example:

```sh
scrappy ./src output.md
```
This will generate an output.md file with the code from all the files in the src directory.

## Development
To contribute to the development of Scrappy, clone the repository and navigate to the project directory:

```sh
git clone https://github.com/WezSieTato/scrappy.git
cd scrappy
```

Then, build the project using Cargo:
```sh
cargo build
```

## Testing

To run the test suite for Scrappy, use the following command:
```sh
cargo test
```

## Acknowledgements

The first version of Scrappy, including this README, was generated with the help of ChatGPT, an AI language model developed by OpenAI. 

The logo for Scrappy was generated using a text-to-image diffusion model called DALLE. 

The prompt for the logo creation was also provided by ChatGPT.

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more information.
