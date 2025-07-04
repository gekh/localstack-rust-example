# README

This project provides a minimal example of an AWS Lambda function written in Rust, designed to run locally using Localstack on macOS.

## INSTALL

- Install docker

- Install [localstack](https://www.localstack.cloud) (needs docker)

  ```sh
  brew install localstack/tap/localstack-cli
  localstack start -d
  localstack status services
  ```

- Install awslocal
  https://github.com/localstack/awscli-local

  ```
  brew install awscli-local
  ```

- Install compiler for x86_64-unknown-linux-musl
  https://rustup.rs/
  ```sh
  rustup target add x86_64-unknown-linux-musl
  brew install FiloSottile/musl-cross/musl-cross
  ```

## USAGE

> For CLI commands use
> `awslocal`
> or
> `aws --endpoint-url=http://localhost:4566`

### CREATE

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/localstack-rust-example bootstrap
chmod +x bootstrap
zip lambda.zip bootstrap
aws --endpoint-url=http://localhost:4566 lambda create-function \
  --function-name localstack-rust-example \
  --runtime provided.al2 \
  --role arn:aws:iam::000000000000:role/lambda-role \
  --handler bootstrap \
  --zip-file fileb://lambda.zip
rm lambda.zip bootstrap response.json
```

_(press "q" to exit status message when lambda is created)_

## CHECK

```sh
awslocal lambda get-function \
  --function-name localstack-rust-example
```

## UPDATE

```
rm lambda.zip bootstrap response.json
cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/localstack-rust-example bootstrap
chmod +x bootstrap
zip lambda.zip bootstrap
awslocal lambda update-function-code \
  --function-name localstack-rust-example \
  --zip-file fileb://lambda.zip
rm lambda.zip bootstrap response.json
```

_(press "q" to exit status message after lambda is updated)_

## TEST

```sh
aws --endpoint-url=http://localhost:4566 lambda invoke \
  --function-name localstack-rust-example \
  --payload '{}' \
  --cli-binary-format raw-in-base64-out \
  response.json
```

_(press "q" to exit status message after lambda is invoked)_

Look at result

```
cat response.json
```

or with jq:

```sh
cat response.json | jq
```

(Optional) Remove response.json

```sh
rm response.json
```

## DESTROY

```
awslocal lambda delete-function \
  --function-name localstack-rust-example
```
