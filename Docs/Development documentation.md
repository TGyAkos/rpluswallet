# Development documentation

---

## Table of contents

 - [Specifications](#specifications)
 - [Running environment](#running-environment)
 - [Development environment](#development-environment)
 - [Architecture](#architecture)
 - [Design patterns](#design-patterns)
 - [Code conventions](#code-conventions)
 - [Tests](#tests)
 - [Efficiency](#efficiency)
 - [Developed by](#developed-by)

---

### Specifications

#### Goal
A simple console application, for ledgering expenses. Is achieved by a CLI and saved to a simple TXT file.

#### Efficiency and performance requirements
The application must be able to run on low-end computers, with a single core and 1GB of RAM.

---

### Running environment
Operating system: Windows 11 (23H2)
CPU: Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz
RAM: 16GB
Storage: 512GB SSD

---

### Development environment
Language: Rust (rustc 1.75.0)

---

### Architecture
The application is structured in 3 main modules:
 - `console`: responsible for reading from and writing to the console
 - `entities`: where the data structures are defined
 - `file`: responsible for reading from and writing to the TXT file (can be replaced with an actual database in the future)
 - `main.rs`: the entry point of the application, where the business logic can be found

---

### Design patterns
The main design pattern used is the MVC (Model-View-Controller) pattern, where:
 - `console` is the View
 - `entities` is the Model
 - `main.rs` is the Controller
 - `file` is the Data Access Layer

---

### Code conventions
For variables and functions, the snake_case convention is used, while for types, structs and implementations the PascalCase convention is used.

---

### Tests
TO BE DONE xd

---

### Efficiency
Since the application is not very complex, it can run on low-end computers, with a single core and 1GB of RAM. I'm not too sure, since **profiling hasn't been done**.

---

### Developed by
 - [TGyAkos](https://github.com/TGyAkos)

Repo Link: [https://github.com/TGyAkos/rwallet](https://github.com/TGyAkos/rwallet)