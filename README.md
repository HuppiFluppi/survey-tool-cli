# Survey Tool CLI

A command-line utility for managing and validating [Survey Tool](https://github.com/HuppiFluppi/survey-tool) configuration files. This Rust-based CLI tool helps you check survey configurations for correctness and verify system prerequisites for running the Survey Tool application.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Tech Stack](#tech-stack)
- [Disclaimer](#disclaimer)
- [Contributing](#contributing)
- [License](#license)

## Overview

This CLI tool is a companion utility for the [Survey Tool](https://github.com/HuppiFluppi/survey-tool) application. It provides command-line operations for:

- **Validating survey configuration files** - Check YAML survey configurations for syntax and schema correctness
- **System prerequisite checks** - Verify that your system can run the Survey Tool application
- **[In Future] Configuration management** - Support for editing survey configurations from the command line

The tool helps ensure your survey configurations are valid before loading them into the main Survey Tool application.

> Note:
> This application mainly exists to test and extend my knowledge of the Rust language and ecosystem. It serves as a test and experiment environment but ideally also the actual need of handling Survey Tool config files.  
>
> If it helps you work with Survey Tool config files or learn something from the code, thats great. If not, it still helped me put my Rust skills into action and learn a lot.

## Features

- **Configuration Validation** - Validate survey config YAML files
- **System Checks** - Verify prerequisites for running Survey Tool
- **Error Reporting** - Detailed error messages with colored output
- **Verbose Mode** - Optional detailed output for debugging
- **Cross-platform** - Works on Windows, macOS, and Linux (only compiled for Windows+Linux)

## Installation

### From Source
A working Rust development environment is needed for building this.

```bash
git clone https://github.com/HuppiFluppi/survey-tool-cli
cd survey-tool-cli
cargo build --release
```

The binary will be available at `target/release/survey-tool-cli`.

### Via download

Go to the [github release page](https://github.com/HuppiFluppi/survey-tool-cli/releases/latest) and download the latest version for your operating system.

## Usage

### Check Survey Configuration
Validate a survey configuration file:
```bash
survey-tool-cli check path/to/survey.yaml
```

With verbose output:
```bash
survey-tool-cli check path/to/survey.yaml --verbose
```

### System Prerequisites Check
Verify your system can run Survey Tool:
```bash
survey-tool-cli setup-check
```

### Help
View all available commands:
```bash
survey-tool-cli --help
```

## Roadmap
- [x] Add Github build pipeline for release
- [ ] Add edit/create of configuration files
- [ ] Refactor project with new insights and learnings in Rust

## Tech Stack

- **Language**: Rust (Edition 2024)
- **CLI Framework**: clap 4.5
- **JSON Schema Validation**: jsonschema 0.38
- **YAML Parsing**: serde-saphyr 0.0.12
- **Output Formatting**: colored 3.0

## Disclaimer
This software is provided "as is", without warranty of any kind. The author is certain, this software could be done more concise, prettier and overall better.
The todos are plenty and bugs are likely hiding. Use at your own risk and have fun. This is a learning and experiment project.

## Contributing

Contributions are welcome! Please:
- Open issues with clear steps to reproduce and expected behavior
- Submit Pull Requests with concise descriptions and clean commit history
- Follow Rust coding conventions and run `cargo fmt` before submitting
- Follow the project’s coding style and patterns (but please feel free to suggest improvements)
- Add tests for new functionality where applicable
- Add documentation for your changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.