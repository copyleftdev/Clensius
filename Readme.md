# Clensius

Clensius is a command-line utility for cleaning wordlists by removing lines that match specific regular expressions. It helps you preprocess wordlists before using them for various purposes, such as password cracking or data analysis.

## Features

- **Regex-based Cleaning**: Clensius uses a set of predefined regular expressions to identify and remove lines from wordlists that match specific patterns. These patterns typically represent noise or undesired data.

- **Parallel Processing**: Clensius utilizes parallel processing to improve the performance of wordlist cleaning. It leverages the Rayon library to process lines in parallel, making it faster on multi-core systems.


## Usage

To use Clensius, follow these steps:

1. **Build the Executable**:

    ```shell
    cargo build --release
    ```

2. **Run Clensius**:

    ```shell
    ./target/release/clensius -wl <wordlist_file>
    ```

    Replace `<wordlist_file>` with the path to the wordlist you want to clean.

## Options

- `-wl <wordlist_file>`: Specify the path to the wordlist file you want to clean.

## Examples

Clean a wordlist file:

```shell
./target/release/clensius -wl wordlist.txt
```

## Performance

Clensius is designed to efficiently clean large wordlists. It leverages parallel processing to take full advantage of multi-core processors, making it suitable for high-performance wordlist cleaning tasks.


## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or create a pull request.

## Credits

Clensius is developed by Don Johnson and is based on the original Bash script by John Barber [https://github.com/BonJarber].

## Contact

For questions or feedback, you can contact the project maintainers at [dj@codetestcode.io].

