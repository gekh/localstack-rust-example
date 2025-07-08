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

- Install compiler for aarch64-unknown-linux-gnu
  https://rustup.rs/
  ```sh
  brew tap messense/macos-cross-toolchains
  brew install aarch64-unknown-linux-gnu
  brew install FiloSottile/musl-cross/musl-cross
  ```

## USAGE

> **Tip:** If you don't have `awslocal` installed, simply replace `awslocal` at the start of each command with:
>
> ```
> aws --endpoint-url=http://localhost:4566
> ```

### CREATE

```sh
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/localstack-rust-example bootstrap
chmod +x bootstrap
zip -9 lambda.zip bootstrap
awslocal lambda create-function \
  --function-name localstack-rust-example \
  --runtime provided.al2023 \
  --architectures arm64 \
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

You should see this when lambda is created/updated successfully:

```json
{
  // ...
  "State": "Active",
  "LastUpdateStatus": "Successful"
  // ...
}
```

## UPDATE

```
rm lambda.zip bootstrap response.json
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/localstack-rust-example bootstrap
chmod +x bootstrap
zip lambda.zip bootstrap
awslocal lambda update-function-code \
  --function-name localstack-rust-example \
  --zip-file fileb://lambda.zip
rm lambda.zip bootstrap
```

_(press "q" to exit status message after lambda is updated)_

## INVOKE

```sh
awslocal lambda invoke \
  --function-name localstack-rust-example \
  --payload '{}' \
  --cli-binary-format raw-in-base64-out \
  response.json
```

Print result:

```sh
cat response.json | jq
```

_(install jq or remove "| jq" in the end to see raw response)_

(Optional) Remove response.json

```sh
rm response.json
```

## DESTROY

```
awslocal lambda delete-function \
  --function-name localstack-rust-example
```

# NOTES

To install jq on macOS:

```sh
brew install jq
```
