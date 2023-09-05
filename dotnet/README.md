# A Simple Web App with ASP.NET Core

Example used to demonstrate `docker init` CLI for a simple Web App with ASP.NET Core


## Run the application

First, clone the repository

```
git clone https://github.com/dockersamples/docker-init-demos
cd docker-init-demos/dotnet/src/
```

You can simply use the following `dotnet run` to test the application.

```
dotnet run --urls http://localhost:5000
```

## Accessing the application

```
> curl https://localhost:5000
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Home page - myWebApp</title>
    <link rel="stylesheet" href="/lib/bootstrap/dist/css/bootstrap.min.css" />
    <link rel="stylesheet" href="/css/site.css?v=r22M7xOahcMheWCkoVof2Nt7pQmj7mMSFKTei-SfmQ0" />
    <link rel="stylesheet" href="/myWebApp.styles.css?v=yIgRZibdRC1c6_UR8PFv-k8GJl8ZhR1R_MMuXEzJSo8" />
</head>
<body>
...
```

## Using Docker init

Change directory to /src and run the following command:

```
docker init
This utility will walk you through creating the following files with sensible defaults for your project:
  - .dockerignore
  - Dockerfile
  - docker-compose.yaml

? What application platform does your project use? ASP.NET
default: "myWebApp"
? What's the name of your solution's main project? myWebApp
? What version of .NET do you want to use? 7.0
? What local port do you want to use to access your server? 5000
```

## Accessing the WebApp

```
 docker compose up -d
```

