# List of Docker Init Sample Apps

This repository provides a collection of sample applications in different programming languages, along with Docker init support for each language. It's a great starting point for developers who want to quickly get familiar with Docker init on various programming languages.

## Table of Contents

- [Languages](#languages)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Docker Initialization](#docker-initialization)
- [Sample Apps](#sample-apps)

## Languages

This repository includes sample applications in the following programming languages:

- [Node.js](./node)
- [Python](./python)
- Rust
- [Go](./go)

## Prerequisites

Before getting started, make sure you have the following tools installed on your system:

- [Download and Install Docker](https://www.docker.com/products/docker-desktop/)
- Your preferred code editor or IDE for the specific language you are working with.

## Getting Started

To get started with the sample apps, follow these steps:

1. Clone this repository to your local machine:

```
 git clone https://github.com/your-username/docker-init-demos.git
```

2. Navigate to the folder for the specific programming language you want to explore (e.g., `nodejs`, `python`, `rust`, or `go`).

```
 cd docker-init-demos.git
```

## Docker Initialization

Each language folder contains a sample project. To build and run the Docker container, use the following commands:

```bash
docker init
```

# Run the container
```
docker compose up -d --build
```

After running the container, you can access the sample app in your web browser by visiting http://localhost:3000/.

## Sample Apps

Here's a list of the sample applications available in each language folder:

- [Node.js: A simple "Hello, World!" web application using Express.js](./node)
- [Python: A basic "Hello, World!" Web application](./python)
- Rust: A command-line application that prints "Hello, World!" to the console.
- [Go: A minimal web server that responds with "Hello, World!" to HTTP requests](./go)


Feel free to explore each folder and modify the sample apps to experiment with Docker and your preferred language.

## Contribution

If you would like to contribute to this repository by adding more sample applications, improving documentation, or fixing issues, feel free to open a pull request. We welcome contributions from the community!

## License

This repository is licensed under the [MIT License](./LICENSE)


