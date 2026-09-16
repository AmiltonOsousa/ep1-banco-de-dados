# Banco de Dados em Memória — Rust + Lua

## Integrantes

- Amilton Oliveira Sousa

## Descrição

Aplicação de banco de dados em memória desenvolvida em Rust, com suporte
a extensões de validação e transformação implementadas em Lua.

A aplicação aceita os comandos:

- `ADD chave valor`
- `GET chave`
- `EXIT`

Os dados permanecem somente em memória durante a execução.

## Compilação

É necessário possuir Rust instalado.

Para compilar:

```bash
cargo build