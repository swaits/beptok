# bpetok

`bpetok` is a simple command-line interface (CLI) application written in Rust for tokenizing text input using Byte Pair Encoding (BPE). The primary goal of the tool is to provide efficient and flexible tokenization for various applications that rely on text processing, natural language processing (NLP), or any pipeline where tokenized input is necessary.

Given an input text stream from stdin, `bpetok` produces tokenized sentences to stdout. It supports multiple built-in vocabulary sizes (small, medium, large), and also allows for the loading of custom vocabularies.

## Features

- **Tokenization using Byte Pair Encoding (BPE)**: Tokenizes input text using the BPE algorithm.
- **Multiple built-in vocabularies**:
  - Small (100k sized vocabulary)
  - Medium (320k sized vocabulary) [default]
  - Large (1M sized vocabulary)
- **Custom vocabulary support**: Load your own BPE vocabulary from a file.
- **CLI-friendly**: Simple and intuitive command-line arguments.
- **Stream-based**: Tokenizes text from standard input line-by-line, emitting tokens to standard output.

## Installation

You can install `bpetok` directly from [crates.io](https://crates.io/crates/bpetok) using Cargo:

```sh
cargo install bpetok
```

Once installed, the `bpetok` binary is available to use globally on your system.

### Usage

```sh
bpetok [OPTIONS]
```

#### Flags and Options

- **Default Vocabulary Size (medium)**: By default, it uses the **medium** vocabulary (320k tokens).

- `-s`, `--small`: Use the smaller vocabulary (100k tokens).

- `-l`, `--large`: Use the larger vocabulary (1M tokens).

- `-v`, `--vocab FILE`: Path to custom BPE vocabulary file. When this flag is set, the built-in vocabularies are ignored.

A BPE vocabulary file is expected to follow this format:

```text
<token>\t<score>\n
```

Each line should consist of:

- A token (a string) followed by a tab character (`\t`)
- A score (an integer) as either a positive or negative value.

Example lines from the file:

```text
<unk> 0
<s>   0
</s>  0
00    -0
an    -1
▁d    -2
en    -3
er    -4
▁s    -5
in    -6
▁p    -7
ar    -8
▁a    -9
▁00   -10
▁m    -11
▁t    -12
es    -13
on    -14
▁k    -15
or    -16
▁n    -17
la    -18
▁b    -19
is    -20
▁c    -21
```

#### Examples

##### Tokenizing with the default (medium) vocabulary

```sh
echo "Hello world" | bpetok
```

##### Tokenizing using the **small** vocabulary

```sh
echo "Hello world" | bpetok --small
```

##### Tokenizing using the **large** vocabulary

```sh
echo "Hello world" | bpetok --large
```

##### Tokenizing using a **custom vocabulary file**

```sh
echo "Hello world" | bpetok --vocab path/to/vocabulary.bpe
```

### Error Handling

- If an invalid vocabulary file is provided using the `--vocab` option, the program will gracefully fail and print an error message to `stderr`.

- Similarly, any issues with initializing the default vocabularies (small, medium, or large) will result in an error with appropriate feedback printed to the terminal.

## Development

To contribute or modify `bpetok`, you'll need a working installation of the Rust toolchain. Once set up, feel free to modify or extend the functionality of the tool and submit a PR.

## Acknowledgements

The default pre-built small, medium, and large vocabularies are 275-language multilingual vocabularies that were originally trained on Wikipedia by the [BPEmb project](https://bpemb.h-its.org/). We thank the contributors from BPEmb for making these vocabularies open and accessible to the community.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
