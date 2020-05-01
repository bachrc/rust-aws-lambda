# AWS Lambda Rust Application Example

This repository contains an architecture of an AWS Lambda Rust Application. 
This couldn't have been possible without the great article from [SilentByte](https://silentbyte.com/writing-aws-lambda-functions-in-rust).

*To-do soon : Putting a good structure of tests* 

## Architecture

This application is composed of four crates (subprojects) :

- `lambda_dispatcher` : contains the necessary interface for working with AWS Lambda
- `spotipal_api` : contains the DTO objects used for requests and responses
- `spotipal_business` : this contains the main business logic of the application
- `webserver` : a webserver wrapper permitting you to launch your app locally, mainly for dev purposes but you could use it to deploy your app in production without the using of the lambdas

## Launching a local webserver

*You will need Rust Nightly toolchain installed on your computer*

You can launch your webserver localy thanks to the webserver included, with the following command :

```shell script
cargo run --bin webserver
```

This should launch your webserver. You should make a POST request at `http://localhost:3000/hello-world` with the following content :

```json
{
  "name": "Dimitri"
}
```

And you will have a nice hello world message with your name on it !

## Testing locally your lambda

*You will need Serverless installed on your computer, and Docker installed, and running*

You will probably want to test your lambda before deploying it. Serverless allows local invocation of the lambda, using the following request :

```shell script
serverless invoke local -f hello-world -d '{"body": "{\"name\":\"Dimitri\"}"}'
```

And the emulated lambda will answer you ! Victory ! 

## Deploying your lambda

For this, you will need to configure your serverless account, login with it, and connect it to your aws account.

Once it's done, you will just have to do this :

```shell script
serverless deploy
```

And your lambda should be uploaded and running in seconds !