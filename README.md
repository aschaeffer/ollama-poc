# ollama-poc

## Install ollama

```shell
curl -fsSL https://ollama.com/install.sh | sh
```

## Install and run tinyllama model

```shell
ollama run tinyllama
```

## Run

```shell
cargo run
```

## Use a custom prompt and/or custom model

```shell
cargo run --release -- --model tinyllama --prompt "What is ollama?"
```
