# Omit

## Description

Omit is a Rust-based project that allows you to securely share environment variables and secrets using a single key. It provides a convenient way to deploy and manage sensitive information while ensuring the safety of your data.

The project utilizes AES encryption to protect your secrets and integrates seamlessly with your git commands, ensuring that your repository stays up to date with the latest changes.

## Features

- Securely share environment variables and secrets using a single key
- AES encryption for data protection
- Integration with git commands for easy management and synchronization

## Installation

Thanks to cargo Omit can be installed with one simple command
`cargo install omit`

## Usage

To use Omit, follow these steps:

1. Create a  git repository and set it up
2. Run the `omit init` command in the repositories folder
3. Add your secrets with the `omit < filename >` command

For more commands run `omit help`

## Contributing

Contributions to Omit are welcome! If you have any ideas, bug reports, or feature requests, please open an issue or submit a pull request on the [GitHub repository](https://github.com/TesDevelopment/Omit/).

## License

Omit is licensed under the [MIT License](https://opensource.org/licenses/MIT).
