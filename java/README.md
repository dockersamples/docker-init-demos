# A Simple Spring Boot Rest Service

Example used to demonstrate `docker init` CLI for a simple Spring Boot Rest Service

This project contains a web service that will accept HTTP GET requests at
`http://localhost:8080/greeting`.


## Run the application

First, clone the repository

```
git clone https://github.com/dockersamples/docker-init-demos
cd docker-init-demos/java/
```
Build the application by running the command `mvn clean install`
You can simply use the following `mvn spring-boot:run` to test the application.

In the browser, navigate to `` to see the greeting.

## Accessing the application

```
> curl https://http://localhost:8080/greeting

{"id":1,"content":"Hello, World!"}

```

## Using Docker init

Run the following command:

```
docker init
This utility will walk you through creating the following files with sensible defaults for your project:
  - .dockerignore
  - Dockerfile
  - docker-compose.yaml

? What application platform does your project use? Java
default: "myWebApp"
? What's the relative directory (with a leading .) for your app? ./src
? What version of Java do you want to use? 17
? What port does your server listen on? 8080
```

## Accessing the WebApp

```
 docker compose up -d
```

Note: Sometime the docker compose up command will fail with the following error:

```
Error: Could not find or load main class org.springframework.boot.loader.launch.JarLauncher
Caused by: java.lang.ClassNotFoundException: org.springframework.boot.loader.launch.JarLauncher
```

To fix this error, you need to update the Dockerfile to use the `java -jar` command to run the application.
Update the ENTRYPOINT in the Dockerfile to use the following command:

```
# ENTRYPOINT [ "java", "org.springframework.boot.loader.launch.JarLauncher" ]
ENTRYPOINT [ "java", "org.springframework.boot.loader.JarLauncher" ]
```

The auto-generated Dockerfile uses the `org.springframework.boot.loader.launch.JarLauncher` class to run the application. This class is not available in the Spring Boot 3.1.0 version which is used for this demo. You need to update the Dockerfile to use the `org.springframework.boot.loader.JarLauncher` class to run the application. If you are using higher version of Spring Boot, you may not need to update the Dockerfile.

The rerun the `docker compose up --build` command to build and run the application.

You should be able to access the application at `http://localhost:8080/greeting`

