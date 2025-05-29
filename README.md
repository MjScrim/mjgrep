# mjgrep

Um buscador de texto assíncrono, rápido e extensível feito em Rust — inspirado no `ripgrep`.

## ✨ Recursos

- 🔍 Busca por padrão em arquivos e diretórios recursivamente
- 🎯 Filtro por extensão de arquivo (`--ext`)
- 🔠 Opção de ignorar maiúsculas/minúsculas (`--ignore-case`)
- 🔢 Exibição do número da linha (`--line-number`)
- 📁 Caminho de busca configurável (`--path`)
- 📦 Saída em JSON estruturado (`--json`)
- 🔁 Contador de ocorrências por padrão (`--count`)

## 🚀 Uso

```bash
cargo run -- "<PADRÃO>" [--path <CAMINHO>] [--ext <EXTENSÃO>] [--ignore-case] [--line-number] [--json] [--count]

