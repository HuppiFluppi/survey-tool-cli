# Survey Tool CLI

A command-line utility for managing and validating [Survey Tool](https://github.com/HuppiFluppi/survey-tool) configuration files. 
This Rust-based CLI tool helps you check survey configurations for correctness and verify system prerequisites for running the Survey Tool application.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Roadmap](#roadmap)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Usage](#usage)
- [Disclaimer](#disclaimer)
- [Contributing](#contributing)
- [License](#license)

## Overview

This CLI tool is a companion utility for the [Survey Tool](https://github.com/HuppiFluppi/survey-tool) application. It provides command-line operations for:

- **Validating survey configuration files** - Check YAML survey configurations for syntax and schema correctness
- **System prerequisite checks** - Verify that your system can run the Survey Tool application
- **Configuration management** - Support for editing survey configurations from the command line
- **[In Future] Survey tool server interaction** - Upload surveys to the server

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
- **Cross-platform** - Works on Windows and Linux (should work on macOS, but not tested)

## Roadmap
- [x] Add Github build pipeline for release
- [x] Add edit/create of configuration files
- [ ] Add interaction with survey-tool-server(s)
- [ ] Refactor project with new insights and learnings in Rust

## Tech Stack

- **Language**: Rust (Edition 2024)
- **CLI Framework**: clap 4.6
- **Interactive Prompts**: inquire 0.9
- **JSON Schema Validation**: jsonschema 0.46
- **YAML Parsing**: serde-saphyr 0.0.28 (with serde-json 1.0)
- **Output Formatting**: colored 3.1
- **Regex parsing**: regex 1.12

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
$ survey-tool-cli check path/to/survey.yaml
```

With verbose output:
```bash
$ survey-tool-cli check path/to/survey.yaml --verbose
```

### System Prerequisites Check
Verify your system can run Survey Tool:
```bash
$ survey-tool-cli setup-check
```

### List survey configuration elements
Initialize a survey and then query its contents:
```
$ survey-tool-cli config template.yml init

📝 Enter survey information

> Which type do you want? survey
> Enter survey title: My survey
> Enter survey description: A good description
? Enter optional image path (skip with ESC): <canceled>
? Enter optional background image path (skip with ESC): <canceled>

  Enter first page information

? Enter page title (skip with ESC): <canceled>
? Enter page description (skip with ESC): <canceled>
? Enter optional image path (skip with ESC): <canceled>
> Make conditional? No
 👍 successfully init file. You can now add content(questions) and more pages

$ survey-tool-cli config template.yml ls

📘 Survey title: My survey
   Survey desc:  A good description
   Survey type:  survey                                          Conditionals: false

────────────────────────────────────────────────────────────────────────────────────────────────────
  Pages(1):
   1. <not set>

```

### Help
View all available commands:
```bash
survey-tool-cli --help
```

## Disclaimer

This software is provided "as is", without warranty of any kind. The author is certain, this software could be done more concise, prettier and overall better.
The todos are plenty and bugs are likely hiding. Use at your own risk and have fun. This is a learning and experiment project.

## Contributing

Contributions are welcome! Please:
- Open issues with clear steps to reproduce and expected behavior
- Submit Pull Requests with concise descriptions and clean commit history
- Follow Rust coding conventions and run `cargo fmt` before submitting
- Run `cargo clippy` and fix any new findings
- Follow the project’s coding style and patterns (but please feel free to suggest improvements)
- Add tests for new functionality where applicable
- Add documentation for your changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
