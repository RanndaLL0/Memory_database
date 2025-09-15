# Memory_database

Este projeto é um **EP de Paradigmas de Programação** que implementa um banco de dados em memória (*in-memory database*) utilizando **Rust** e **Lua**.  
O objetivo é demonstrar conceitos de diferentes paradigmas de programação, integração entre linguagens e construção de estruturas de dados eficientes.

---

## 📚 Descrição

O **Memory_database** funciona como um mini banco de dados simples, armazenando pares **chave → valor** em memória.  

Ele permite ao usuário:  
- Adicionar entradas  
- Recuperar valores por chave  
- Validar e armazenar CPFs corretamente formatados  
- Interagir via **terminal (Lua)** com comandos que chamam a biblioteca desenvolvida em **Rust**

Esse projeto foi desenvolvido como parte de um exercício-programa da disciplina **Paradigmas de Programação**.

---

## 🎯 Objetivos

- Explorar a interação entre **Rust** (linguagem de sistemas, imperativa e segura) e **Lua** (linguagem de script, dinâmica).  
- Aplicar conceitos de **paradigmas de programação**:  
  - Imperativo (manipulação direta de estruturas)  
  - Funcional (uso de `map`, `collect`, imutabilidade)  
  - Declarativo (regras de validação, ex. CPF)  
- Implementar um banco em memória de fácil uso no terminal.  

---

## 🛠️ Estrutura do Projeto

Memory_database/
├── Cargo.toml # Configuração do projeto Rust
├── src/
│ ├── lib.rs # Implementação principal da MiniDB em Rust
│ ├── main.rs # Ponto de entrada Rust (chamadas de integração)
│ └── extensions/
│ └── constants/
│ ├── terminal.lua # Configurações de terminal em Lua
│ └── colors.lua # Definições de cores ANSI
├── README.md # Documentação


---

## ⚙️ Funcionalidades

- **MiniDB (Rust)**  
  - Estrutura baseada em `HashMap<String, String>`  
  - Funções de `add`, `get` e validação de entradas  
  - Função especial de **validação de CPF**  

- **Interface (Lua)**  
  - Exibe banner inicial com cores  
  - Oferece menu interativo no terminal:
    ```
    (1) ADD, (2) GET, (3) help
    ```
  - Encaminha as operações para a biblioteca Rust  

---

## 🚀 Como Executar

### Pré-requisitos
- [Rust](https://www.rust-lang.org/) (≥ 1.70)  
- [Lua](https://www.lua.org/) (≥ 5.4)  

### Passo a passo

Clone o repositório:
```bash
git clone https://github.com/RanndaLL0/Memory_database.git
cd Memory_database
```

Compile o projeto Rust:
```
cargo build
```
Rode o projeto:
```
cargo run
```
