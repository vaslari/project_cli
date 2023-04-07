# Multi-task Assistant Project

Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

### Introduction

This project aims to develop an intelligent assistant program that can integrate with all the tasks a person does on a computer and assist in all those tasks in various ways. It uses the technology of OpenAI's GPT model to provide contextual suggestions and responses to user queries.

### Features

- Interaction with OpenAI's GPT model.
- Integration with common computer tasks.
- Customizable configuration to suit user needs.
- Easy to install and use.

### Prerequisites

- OpenAI account with access to the GPT model.
- OpenAI API Key.
- Rust installed on the system.

### Installation

1. Clone the repository:

git clone https://github.com/vaslari/project_cli

2. Navigate to the project directory:

cd your-repo-name

3. Build the project:

cargo build --release

4. Copy the executable to a location in your PATH (optional):

cp target/release/path /usr/local/bin/

### Configuration

1. Set the OPENAI_API_KEY environment variable with your OpenAI API key:

export OPENAI_API_KEY="your-api-key"

2. Run the program and follow the instructions to configure your preferences:

cargo run

### Usage

1. Run the program:

cargo run

2. Enter your queries, and the program will provide contextual responses and suggestions.

3. Quit:

exit

### Contributing

Contributions are welcome. If you would like to contribute to the project, please fork the repository, make your changes, and submit a pull request.

### License

This project is licensed under the MIT License - see the LICENSE file for more details.

### Contact

If you have any questions or suggestions, feel free to contact us at [alfonzo.rivas.m@gmail.com](mailto:alfonzo.rivas.m@gmail.com).