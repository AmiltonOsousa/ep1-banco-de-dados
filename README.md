# Banco de Dados em Memória — Rust + Lua

## Integrantes

- Amilton Oliveira Sousa

## Descrição

Aplicação de banco de dados em memória desenvolvida em Rust, com suporte a extensões de validação e transformação implementadas em Lua.

A aplicação aceita os comandos:

- `ADD chave valor`
- `GET chave`
- `EXIT`

Os dados permanecem somente em memória durante a execução do programa.

As extensões são carregadas automaticamente a partir do diretório `extensions/`, permitindo adicionar novos comportamentos sem alterar ou recompilar o código Rust.

---

## Compilação

É necessário possuir o Rust instalado.

Para compilar:

```bash
cargo build
```

Para executar:

```bash
cargo run
```

Também é possível executar utilizando entrada redirecionada de um arquivo:

```bash
cargo run < casos_teste.txt
```

A aplicação também pode ser executada diretamente após a compilação:

```bash
target/debug/banco-memoria-ep1
```

No Windows:

```bat
target\debug\banco-memoria-ep1.exe
```

---

## Comandos

### ADD

Adiciona ou substitui um valor associado a uma chave.

Formato:

```text
ADD chave valor
```

Exemplo:

```text
ADD nome Amilton
```

Resposta:

```text
OK
```

O valor corresponde ao restante da linha após a chave e pode conter espaços.

Por exemplo:

```text
ADD nome completo Amilton Oliveira Sousa
```

armazena:

```text
Amilton Oliveira Sousa
```

como valor.

### GET

Consulta o valor armazenado para uma chave.

Formato:

```text
GET chave
```

Exemplo:

```text
GET nome
```

Se a chave existir, o valor é retornado. Caso contrário:

```text
ERRO: chave inexistente
```

### EXIT

Encerra a execução:

```text
EXIT
```

Quando a entrada é fornecida por arquivo ou pipe, o fim da entrada (`EOF`) também encerra o programa.

---

## Tratamento de erros

Entradas inválidas não devem encerrar o programa.

Os erros são apresentados em uma única linha no formato:

```text
ERRO: mensagem
```

Exemplos:

```text
ERRO: linha vazia
ERRO: comando desconhecido
ERRO: chave inexistente
```

Após um erro, a aplicação retorna ao prompt e continua aceitando comandos.

---

## Estrutura do projeto

```text
.
├── Cargo.toml
├── README.md
├── casos_teste.txt
├── casos_teste_ip.txt
├── extensions/
│   ├── cpf.lua
│   ├── data.lua
│   └── ip.lua
└── src/
    ├── main.rs
    ├── command.rs
    ├── input.rs
    ├── lua.rs
    └── storage.rs
```

### Módulos Rust

#### `main.rs`

Responsável por inicializar o armazenamento, criar a ponte com Lua e iniciar o loop principal da aplicação.

#### `input.rs`

Responsável pelo loop interativo, leitura da entrada, impressão do prompt e execução dos comandos.

#### `command.rs`

Responsável pelo parsing dos comandos `ADD`, `GET` e `EXIT`.

#### `storage.rs`

Implementa o banco de dados em memória utilizando uma estrutura chave-valor.

Também disponibiliza operações de consulta utilizadas pelas extensões Lua.

#### `lua.rs`

Responsável pela integração com a máquina virtual Lua.

Suas responsabilidades incluem:

- criação da máquina Lua;
- descoberta das extensões;
- carregamento dos arquivos `.lua`;
- registro das extensões;
- identificação da extensão correspondente a uma chave;
- execução das operações `ADD` e `GET`;
- disponibilização da consulta ao banco para Lua;
- conversão dos resultados Lua para estruturas Rust;
- tratamento dos erros provenientes das extensões.

A biblioteca `mlua` é utilizada exclusivamente neste módulo.

---

## Extensões Lua

As extensões ficam no diretório:

```text
extensions/
```

Todos os arquivos com extensão `.lua` presentes nesse diretório são carregados automaticamente na inicialização.

O Rust não possui uma lista fixa de extensões. Portanto, adicionar ou remover uma extensão Lua altera o comportamento da aplicação sem necessidade de modificar ou recompilar o código Rust.

Cada extensão deve retornar uma tabela contendo:

```lua
return {
    prefix = "prefixo_",

    add = function(key, value)
        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        return {
            ok = true,
            value = value
        }
    end
}
```

A extensão deve possuir:

- `prefix`: prefixo utilizado para identificar as chaves atendidas;
- `add`: função chamada durante um `ADD`;
- `get`: função chamada durante um `GET`.

Por exemplo, uma extensão com:

```lua
prefix = "cpf_"
```

será utilizada para chaves como:

```text
cpf_cliente
cpf_usuario
cpf_teste
```

---

## Resultado das extensões

As operações Lua retornam uma tabela que é convertida pelo Rust para uma estrutura específica:

```rust
struct ExtensionResult {
    ok: bool,
    value: Option<String>,
    error: Option<String>,
}
```

Em caso de sucesso:

```lua
{
    ok = true,
    value = "valor"
}
```

Em caso de erro:

```lua
{
    ok = false,
    error = "mensagem de erro"
}
```

O Rust interpreta esse resultado e decide se a operação deve continuar ou ser rejeitada.

Assim, os detalhes do resultado produzido pelo Lua não são utilizados diretamente pelo restante da aplicação.

Erros de execução da própria extensão Lua também são capturados pelo Rust e convertidos para o formato:

```text
ERRO: mensagem
```

---

## Consulta ao banco pelo Lua

As extensões possuem acesso a uma interface genérica de consulta ao banco por meio do objeto:

```lua
db
```

A interface disponibiliza:

```lua
db.get(key)
```

e:

```lua
db.find_by_value(value, ignored_key)
```

`db.get` permite consultar o valor associado a uma chave.

`db.find_by_value` procura uma chave que possua determinado valor e permite informar uma chave que deve ser ignorada na busca.

A consulta é feita diretamente sobre o armazenamento Rust. O banco inteiro não é copiado para o Lua.

A interface foi mantida genérica para que novas extensões possam utilizá-la sem que seja necessário adicionar lógica específica no Rust para cada tipo de dado.

---

## Resolução da validação durante um ADD

Durante um `ADD`, a extensão Lua é executada antes de o novo valor ser armazenado.

Isso é necessário para permitir que a extensão consulte o estado atual do banco.

No caso do CPF, por exemplo, é necessário verificar se o mesmo CPF já está associado a outra chave.

A extensão utiliza:

```lua
db.find_by_value(value, key)
```

A própria chave que está sendo atualizada é ignorada na consulta.

Isso permite que:

```text
ADD cpf_cliente 52998224725
```

seja executado novamente para a mesma chave, enquanto:

```text
ADD cpf_cliente 52998224725
ADD cpf_outro 52998224725
```

faz com que o segundo comando seja rejeitado.

A consulta ocorre antes da inserção do novo valor no armazenamento.

---

## Extensão de CPF

O arquivo:

```text
extensions/cpf.lua
```

atende às chaves cujo prefixo é:

```text
cpf_
```

Durante o `ADD`, a extensão verifica:

1. se o valor possui exatamente 11 caracteres;
2. se todos os caracteres são dígitos;
3. se o CPF não possui todos os dígitos iguais;
4. se os dois dígitos verificadores são válidos;
5. se o CPF já está associado a outra chave.

Exemplo válido:

```text
ADD cpf_cliente 52998224725
```

Resposta:

```text
OK
```

No `GET`, o CPF é formatado:

```text
529.982.247-25
```

A validação e a transformação são realizadas pela extensão Lua.

---

## Extensão de data

O arquivo:

```text
extensions/data.lua
```

atende às chaves cujo prefixo é:

```text
data_
```

Durante o `ADD`, a extensão verifica se a data possui exatamente o formato:

```text
aaaa-mm-dd
```

Também verifica se a data realmente existe no calendário.

A regra de ano bissexto utilizada é:

- divisível por 4;
- exceto quando divisível por 100;
- a menos que também seja divisível por 400.

Por exemplo:

```text
ADD data_nascimento 2024-02-29
```

é válido.

Já:

```text
ADD data_invalida 2023-02-29
```

é rejeitado.

No `GET`, a data é transformada para:

```text
dd/mm/aaaa
```

Exemplo:

```text
29/02/2024
```

---

## Terceira extensão: IPv4

O arquivo:

```text
extensions/ip.lua
```

é a terceira extensão da aplicação e atende às chaves:

```text
ip_
```

Essa extensão foi escolhida para demonstrar uma validação diferente das extensões de CPF e data.

Durante o `ADD`, são verificadas as quatro partes de um endereço IPv4 e se cada uma está entre `0` e `255`.

Exemplo:

```text
ADD ip_servidor 192.168.1.10
```

é aceito.

Um endereço como:

```text
ADD ip_invalido 192.168.1.300
```

é rejeitado.

No `GET`, além de retornar o endereço, a extensão identifica se ele pertence às faixas privadas mais comuns:

- `10.0.0.0/8`;
- `172.16.0.0/12`;
- `192.168.0.0/16`.

Exemplo:

```text
GET ip_servidor
```

pode retornar:

```text
192.168.1.10 (privado)
```

Enquanto:

```text
GET ip_publico
```

para `8.8.8.8` retorna:

```text
8.8.8.8 (público)
```

Essa extensão utiliza validação estrutural e classificação do endereço, exercitando uma lógica diferente das validações de CPF e data.

---

## Como adicionar uma nova extensão

Para adicionar uma nova extensão, basta criar um arquivo `.lua` dentro de:

```text
extensions/
```

Não é necessário alterar nenhum arquivo Rust.

A extensão deve seguir o protocolo:

```lua
return {
    prefix = "meu_prefixo_",

    add = function(key, value)
        -- validação ou transformação do ADD

        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        -- transformação do GET

        return {
            ok = true,
            value = value
        }
    end
}
```

Depois de adicionar o arquivo, basta executar novamente a aplicação.

A nova extensão será descoberta automaticamente durante a inicialização.

---

## Dependências

O projeto utiliza:

```toml
mlua = { version = "0.10", features = ["lua54", "vendored"] }
```

A biblioteca `mlua` é utilizada para integrar Rust com Lua.

A feature `vendored` permite utilizar a implementação do Lua fornecida pela própria dependência, sem exigir uma instalação externa do Lua para compilar o projeto.

Não são utilizadas bibliotecas de terceiros para validação ou formatação de CPF e datas.

Em particular, não são utilizadas bibliotecas como `chrono`, `time` ou `regex`.

As validações de CPF, datas e IPv4 são implementadas nas próprias extensões Lua.

---

## Testes

O arquivo:

```text
casos_teste.txt
```

contém testes das operações básicas, CPF, datas, IPv4, chaves inexistentes e comandos inválidos.

Para executá-lo:

```bash
cargo run < casos_teste.txt
```

O arquivo:

```text
casos_teste_ip.txt
```

contém testes específicos da extensão de IPv4.

Para executá-lo:

```bash
cargo run < casos_teste_ip.txt
```

Os testes incluem casos válidos e inválidos para verificar tanto a aceitação quanto a rejeição dos valores.

---

## Decisões de projeto

O armazenamento foi implementado utilizando um `HashMap<String, String>`, pois o banco exigido pelo projeto é exclusivamente em memória e utiliza uma relação simples entre chave e valor.

O parsing dos comandos foi separado do armazenamento para manter cada responsabilidade isolada.

A integração com Lua também foi isolada em seu próprio módulo. Dessa forma, os demais módulos Rust não precisam conhecer os detalhes da biblioteca `mlua`.

As extensões são descobertas dinamicamente pelo diretório `extensions/`, evitando que o código Rust precise conhecer previamente quais extensões existem.

A comunicação entre Rust e Lua utiliza uma estrutura de resultado padronizada, permitindo separar a execução da extensão da interpretação do resultado pela aplicação principal.

A consulta ao banco é disponibilizada ao Lua por meio de uma interface genérica. Dessa maneira, uma extensão pode consultar o estado atual do banco sem receber uma cópia completa dos dados.

---

## Estrutura de execução

O fluxo principal da aplicação é:

```text
Entrada do usuário
       |
       v
  command.rs
       |
       v
    input.rs
       |
       +------ ADD ------> lua.rs ------> extensão Lua
       |                         |
       |                         v
       |                    valida/transforma
       |                         |
       |                         v
       |                    storage.rs
       |
       +------ GET ------> storage.rs
       |                       |
       |                       v
       |                    lua.rs
       |                       |
       |                       v
       |                  extensão Lua
       |
       +------ EXIT -----> encerra
```

O banco permanece exclusivamente em memória e é descartado quando o processo termina.