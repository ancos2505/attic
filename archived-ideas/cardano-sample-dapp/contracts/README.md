# Aiken Contracts

## Instalation


### Install aiken compiler
```sh
curl -sSfL https://install.aiken-lang.org | bash
aikup
```


### Install Auto-completion
```sh
aiken completion bash | sudo tee /usr/share/bash-completion/completions/aiken
```

### Activate Auto-completion
```sh
echo 'source /usr/local/share/bash-completion/completions/aiken' | tee -a $HOME/.bashrc
```


### Vscode integrations
Launch VS Code Quick Open (Ctrl+P), paste the following command, and press enter.

```sh
ext install TxPipe.aiken
```

