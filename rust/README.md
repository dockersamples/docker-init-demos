# Docker Init for Rust Developer

# A Simple Hello Docker Rust Demo 

Example used to demonstrate ```docker init``` CLI for a simple Hello Docker Rust Program

## Clone the repository

```
git clone https://github.com/dockersamples/docker-init-demos
cd docker-init-demos/rust
```

```
cargo init
cargo build --release
```

## Running docker init

```
docker init
```

```
docker init

Welcome to the Docker Init CLI!

This utility will walk you through creating the following files with sensible defaults for your project:
  - .dockerignore
  - Dockerfile
  - compose.yaml

Let's get started!

WARNING: The following Docker files already exist in this directory:
  - .dockerignore

? Do you want to overwrite them? Yes
? What application platform does your project use? Rust
? What version of Rust do you want to use? 1.70.0
? What port does your server listen on? 3030

CREATED: .dockerignore
CREATED: Dockerfile
CREATED: compose.yaml

✔ Your Docker files are ready!

Take a moment to review them and tailor them to your application.

When you're ready, start your application by running: docker compose up --build

Your application will be available at http://localhost:3030
```


## Accessing the app on the browser

```
##         .
## ## ##        ==
## ## ## ## ##    ===
/""""""""""""""""\___/ ===
{                       /  ===-
\______ O           __/
 \    \         __/
  \____\_______/

Hello from Docker!
```
