# A Simple Hello Docker Python Demo 

Example used to demonstrate ```docker init``` CLI for a simple Hello World Python Program


## Run the application





You can simply use `python3 app.py` command.


This code defines a handler that responds to GET requests with the specified text and starts an HTTP server listening on port 8080. When you run the script, you can access the server at http://localhost:8080 and see the same message as the Python program.

Those commands will start a http server listening on port `8080` 
and if your request `http://localhost:8080` you'll see the following output: 
```shell
❯ curl http://localhost:8080

          ##         .
    ## ## ##        ==
 ## ## ## ## ##    ===
/"""""""""""""""""\___/ ===
{                       /  ===-
\______ O           __/
 \    \         __/
  \____\_______/


Hello from Docker!

```


## Using Docker init

### Run the following command:

```bash
 docker init
```

This utility will walk you through creating the following files with sensible defaults for your project:
  - .dockerignore
  - Dockerfile
  - docker-compose.yaml

## Modify the Dockerfile

```
FROM python:3.8-alpine
RUN mkdir /app
ADD . /app
WORKDIR /app
CMD ["python3", "app.py"]
```



## Modify the Docker Compose file


```
 version: '3'

services:
  app:
    build: .
    ports:
      - "8080:8080"
    command: python3 app.py
 ```
 
 ## Running the container service
 
 ```
  docker compose up -d --build
 ```
 
 ## Accessing the Python app
 
 ```
 curl localhost:8080

          ##         .
    ## ## ##        ==
 ## ## ## ## ##    ===
/"""""""""""""""""\___/ ===
{                       /  ===-
\______ O           __/
 \    \         __/
  \____\_______/


Hello from Docker!
```

## FAQs

## My requirements.txt shows gunicorn. What's that?

Gunicorn (Green Unicorn) is a popular Python Web Server Gateway Interface (WSGI) HTTP server. It is designed to be a lightweight and reliable server for running Python web applications. Gunicorn is commonly used in production environments to serve Python web applications, especially when combined with popular web frameworks like Django or Flask.

Here are some key features and characteristics of Gunicorn:

- WSGI Server: Gunicorn is a WSGI HTTP server, which means it adheres to the Web Server Gateway Interface specification. This allows it to interface with Python web applications that also follow the WSGI standard.

- Pre-fork Worker Model: Gunicorn follows a pre-fork worker model, where it creates multiple worker processes during startup. Each worker is a separate Python process that can handle incoming requests independently. This model allows Gunicorn to handle concurrent requests efficiently.

- Load Balancing: Gunicorn supports various worker types, including synchronous, asynchronous, and thread-based workers. This enables it to handle different types of applications and workloads efficiently.

- Graceful Restart: Gunicorn supports graceful restarts, which means it can reload workers without dropping active connections. This feature is crucial for maintaining the availability of the web application during updates or code changes.

- Configurability: Gunicorn provides various configuration options that allow you to customize its behavior according to your specific application needs.

- Integration with Popular Frameworks: Gunicorn integrates well with popular Python web frameworks like Django, Flask, Pyramid, and more. It can be used as a standalone server or in combination with other tools like Nginx or Apache to create a production-ready web application stack.

- Using Gunicorn as a WSGI server is a common choice for deploying Python web applications in production environments due to its performance, reliability, and ease of use. It helps handle concurrent requests efficiently, making it suitable for serving web applications to multiple users simultaneously.
