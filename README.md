# 🦀 Curso Completo de Rust: De Cero a Producción

## 📑 Índice General

- [**Fase 1: Fundamentos del Lenguaje**](#fase-1-fundamentos-del-lenguaje)
  - [**1. Fundamentos y Tooling**](#1-fundamentos-y-tooling)
  - [**2. Ownership & Memory Management *(el corazón de Rust)***](#2-ownership--memory-management-el-corazón-de-rust)
  - [**3. Structs, Enums y Pattern Matching**](#3-structs-enums-y-pattern-matching)
  - [**4. Colecciones y Manejo de Errores Idiomático**](#4-colecciones-y-manejo-de-errores-idiomático)
  - [**5. Traits y Genéricos *(Sistema de Tipos)***](#5-traits-y-genéricos-sistema-de-tipos)
  - [**6. Lifetimes *(Tiempos de Vida)***](#6-lifetimes-tiempos-de-vida)
- [**Fase 2: Abstracciones Avanzadas**](#fase-2-abstracciones-avanzadas)
  - [**7. Punteros Inteligentes *(Box, Rc, Arc, RefCell)***](#7-punteros-inteligentes-box-rc-arc-refcell)
  - [**8. Concurrencia Nativa y Segura**](#8-concurrencia-nativa-y-segura)
  - [**9. Programación Asíncrona *(tokio)***](#9-programación-asíncrona-tokio)
- [**Fase 3: Producción Real**](#fase-3-producción-real)
  - [**10. Backend REST con Axum**](#10-backend-rest-con-axum)
  - [**11. Bases de Datos con sqlx**](#11-bases-de-datos-con-sqlx)
  - [**12. CLIs con clap + Parsing Binario con nom**](#12-clis-con-clap--parsing-binario-con-nom)
  - [**13. Unsafe Rust e Interoperabilidad de Bajo Nivel**](#13-unsafe-rust-e-interoperabilidad-de-bajo-nivel)
  - [**14. Testing, Benchmarking y Producción**](#14-testing-benchmarking-y-producción)

---

## 📋 Estructura del Curso (14 Módulos)

> **Metodología:** Cada módulo sigue el mismo formato: **teoría → diagrama mental → código compilable → errores típicos del compilador → ejercicio integrador + ejercicio integrador avanzado.**

---

### FASE 1: Fundamentos del Lenguaje

#### 1. Fundamentos y Tooling
- **1.1.** Instalación y Toolchain (`rustup`, `rustc`, `cargo`)
- **1.2.** Configuración del Entorno Profesional: `rust-analyzer`, `clippy`, `rustfmt`
- **1.3.** Meta-Habilidad: Lectura de Errores y `rustc --explain`
- **1.4.** Estructura de un Proyecto Cargo
- **1.5.** Sintaxis de Documentación Técnica (`///` y `//!`)
- **1.6.** Variables, Inmutabilidad e Inferencia de Tipos
- **1.7.** Constantes, Variables Estáticas, Type Aliases y Casting Numérico
- **1.8.** Shadowing
- **1.9.** Tipos Primitivos
- **1.10.** Control de Flujo
- **1.11.** Funciones

#### 2. Ownership & Memory Management *(el corazón de Rust)*
- **2.1.** El Problema que Ownership Resuelve (C/C++ vs Java vs Rust)
- **2.2.** Diagrama Mental: Stack vs. Heap
- **2.3.** Las Reglas de Ownership
- **2.4.** Trait Drop Explícito, RAII y Trazabilidad de Recursos
- **2.5.** Move Semantics
- **2.6.** Copy vs. Move
- **2.7.** Copy vs Clone a Nivel de Memoria
- **2.8.** Ownership y Funciones
- **2.9.** Borrowing: Referencias sin Transferir Ownership
- **2.10.** Las Reglas del Borrow Checker
- **2.11.** Borrow Splitting: Préstamos Independientes de Campos
- **2.12.** Antipatrones del Borrow Checker y Scopes Reducidos
- **2.13.** Slices: Referencias a Partes de una Colección

#### 3. Structs, Enums y Pattern Matching
- **3.1.** Structs: clásica, tupla, unitaria, métodos con `impl`
- **3.2.** Patrón Newtype: Type Safety de Dominio y Encapsulación
- **3.3.** Derivación de Default e Inicialización con `..Default::default()`
- **3.4.** Enums: Modelando Variantes (ADTs)
- **3.5.** Pattern Matching con `match` (exhaustividad)
- **3.6.** `Option<T>`: Adiós a `null`
- **3.7.** Dualidad `Option<T>` y `Result<T, E>`: Preview del Operador `?`
- **3.8.** `if let` y `while let`
- **3.9.** Pattern Matching Avanzado: Destructuring
- **3.10.** Omisión de Campos (`..`), `@` Bindings y Match Guards

#### 4. Colecciones y Manejo de Errores Idiomático
- **4.1.** `Vec<T>`: Arrays Dinámicos
- **4.2.** `HashMap<K, V>`
- **4.3.** Colecciones Especializadas: `HashSet`, `BTreeMap`, `VecDeque`
- **4.4.** Mutación Eficiente: `.drain()` y `.retain()`
- **4.5.** `String` vs `&str`: La Distinción que Confunde a Todos
- **4.6.** Closures e Iteradores Profundos: Traits `Fn`, `FnMut`, `FnOnce` y Pipelines Funcionales
- **4.7.** `Result<T, E>`: Errores como Valores
- **4.8.** El Operador `?`: Propagación Idiomática
- **4.9.** Combinadores Avanzados de Result (`map_err`, `and_then`, `context`)
- **4.10.** Errores Personalizados: El Enum como Taxonomía de Fallos
- **4.11.** `thiserror`: Errores de Librería (tipados)
- **4.12.** `anyhow`: Errores de Aplicación (opacos, con contexto)
- **4.13.** Resiliencia en Runtime: `panic!` vs `Result` y `catch_unwind`

#### 5. Traits y Genéricos *(Sistema de Tipos)*
- **5.1.** Traits: Contratos de Comportamiento
- **5.2.** Traits de Conversión: `From`/`Into`, `AsRef`/`AsMut`, Blanket Implementations
- **5.3.** La Regla del Huérfano (Orphan Rule) y Desacoplamiento con Newtypes
- **5.4.** Traits Estándar Fundamentales (`Debug`, `Clone`, `Copy`, `Default`, `PartialEq`, `Display`, `Hash`)
- **5.5.** Genéricos: Código sobre Cualquier Tipo
- **5.6.** Trait Bounds y `where` clauses
- **5.7.** `impl Trait`: Sintaxis Simplificada
- **5.8.** Dispatch Estático vs. Dinámico (Monomorfización vs Vtables)
- **5.9.** El Trait `Sized`, `?Sized` y Estructura de Fat Pointers
- **5.10.** Traits con Tipos Asociados
- **5.11.** Supertraits y Sobrecarga de Operadores con `std::ops`

#### 6. Lifetimes *(Tiempos de Vida)*
- **6.1.** El Problema que Resuelven
- **6.2.** Sintaxis de Lifetimes (`'a`)
- **6.3.** Reglas de Elisión de Lifetimes
- **6.4.** Modelado Visual de Líneas de Vida Cruzadas en Retornos
- **6.5.** Structs con Referencias
- **6.6.** El Lifetime `'static`
- **6.7.** Combinando Lifetimes con Trait Bounds
- **6.8.** Subtipado y Bounds de Lifetimes (`'a: 'b`)
- **6.9.** Higher-Ranked Trait Bounds (HRTBs) con `for<'a>`
- **6.10.** Resolución del ejercicio `buscar_por_autor` del Módulo 4

---

### FASE 2: Abstracciones Avanzadas

#### 7. Punteros Inteligentes *(Box, Rc, Arc, RefCell)*
- **7.1.** `Box<T>`: Ownership en el Heap (tipos recursivos, trait objects)
- **7.2.** Traits `Deref`/`DerefMut` y Mecánica de Deref Coercion
- **7.3.** `Rc<T>`: Reference Counting — Múltiples Dueños (Single-Thread)
- **7.4.** `Arc<T>`: Atomic Reference Counting — Multi-Thread
- **7.5.** `RefCell<T>`: Interior Mutability
- **7.6.** Mutabilidad Interior en Single-Thread: `Cell<T>` vs `RefCell<T>`
- **7.7.** Manejo Seguro de `BorrowMutError` con `try_borrow_mut`
- **7.8.** Caso Práctico: Árbol con `Weak<T>` (evitar fugas de memoria)

#### 8. Concurrencia Nativa y Segura
- **8.1.** El Problema: Data Races
- **8.2.** Threads del Sistema Operativo
- **8.3.** Concurrencia Estructurada: `std::thread::scope`
- **8.4.** `Send` y `Sync`: Los Traits que Hacen Posible Todo
- **8.5.** `Mutex<T>`: Exclusión Mutua
- **8.6.** Prevención de Deadlocks: Orden Determinista y `try_lock()`
- **8.7.** `Arc<Mutex<T>>`: El Patrón Central
- **8.8.** Message Passing: Canales (`mpsc`)
- **8.9.** `RwLock<T>`: Múltiples Lectores O Un Escritor
- **8.10.** Sincronización Avanzada: `Condvar` y `Barrier`
- **8.11.** Diferenciación Crítica: `std::sync::Mutex` vs `tokio::sync::Mutex`
- **8.12.** Atomics para el Caso Simple
- **8.13.** Paralelismo de Datos Multihilo con `rayon`

#### 9. Programación Asíncrona *(tokio)*
- **9.1.** Threads del SO vs. Async: Problemas Diferentes
- **9.2.** `Future`: La Abstracción Central
- **9.3.** Anatomía de Futures Internos: `Pin<T>`, `Unpin` y Estructuras Auto-referenciales
- **9.4.** Configurando `tokio`
- **9.5.** `async`/`await` Básico
- **9.6.** `tokio::spawn`: Tasks Independientes
- **9.7.** Canales Asíncronos: `tokio::sync::mpsc`
- **9.8.** Canales Avanzados: `oneshot`, `broadcast`, `Notify`
- **9.9.** Timers y Timeouts
- **9.10.** Flujos Competitivos y Cancelaciones con `tokio::select!`
- **9.11.** `async fn` en Traits
- **9.12.** Procesamiento Reactivo con el Trait `Stream`
- **9.13.** Error Común de Arquitectura: Bloquear el Runtime

---

### FASE 3: Producción Real

#### 10. Backend REST con Axum
- **10.1.** Setup del Proyecto
- **10.2.** Servidor Mínimo
- **10.3.** Routing y Handlers
- **10.4.** Extractors: `Path`, `Query`, `Json`, `State`
- **10.5.** Extractores Custom: `FromRequestParts` vs `FromRequest`
- **10.6.** Autenticación en Producción: Extractor Custom de JWT
- **10.7.** Estado Compartido con `State` (`Arc<Mutex<T>>`)
- **10.8.** Manejo de Errores: `IntoResponse`
- **10.9.** Middlewares (logging, `tower-http`)
- **10.10.** Resiliencia y Cero-Downtime: Graceful Shutdown con `tokio::signal`
- **10.11.** Testing de Integración sin Sockets: `tower::ServiceExt::oneshot`
- **10.12.** Serialización Avanzada con `serde`
- **10.13.** Estructura de Proyecto (Nivel Producción)

#### 11. Bases de Datos con sqlx
- **11.1.** ¿Por Qué `sqlx` y No un ORM Tradicional?
- **11.2.** Setup del Proyecto (SQLite / PostgreSQL)
- **11.3.** Conexión y Pool
- **11.4.** Migraciones
- **11.5.** Queries Básicas con `query!` (Verificado en Compile-Time)
- **11.6.** Mapeo de Enums de Base de Datos con `#[derive(sqlx::Type)]`
- **11.7.** `query_as!`: Mapeo Directo a Structs
- **11.8.** UPDATE, DELETE y Verificar Filas Afectadas
- **11.9.** Queries Dinámicas con `sqlx::QueryBuilder`
- **11.10.** Transacciones (RAII → rollback automático)
- **11.11.** Abstracción de Ejecución con `sqlx::Executor`
- **11.12.** Integrando con Axum
- **11.13.** Modo Offline (Compilación sin Base de Datos Activa)
- **11.14.** Flujo de CI/CD: Gestión de Migraciones y Validación Offline

#### 12. CLIs con clap + Parsing Binario con nom
##### PARTE A — CLIs con clap:
- **12.1.** Setup y Filosofía Derive
- **12.2.** CLI Básico (argumentos, flags, doc comments → `--help`)
- **12.3.** Subcomandos (patrón git commit)
- **12.4.** Validación y Grupos de Argumentos (`ValueEnum`, `ArgAction::Count`)
- **12.5.** Validación Avanzada: `ArgGroup`, `conflicts_with`, `requires`

##### PARTE B — Parsing con nom:
- **12.6.** ¿Por Qué `nom`? (parser combinators, zero-copy)
- **12.7.** Conceptos Básicos: Parsers como Funciones (`IResult`)
- **12.8.** Arquitectura: Flujos complete vs streaming
- **12.9.** Combinando Parsers: Un Header Binario Real (`tuple`)
- **12.10.** Diagnóstico con `VerboseError` y Conversión a `anyhow::Error`
- **12.11.** Parsers Alternativos y Repetición (`alt`, `many0`, `map_res`)

#### 13. Unsafe Rust e Interoperabilidad de Bajo Nivel
- **13.1.** ¿Qué es Realmente `unsafe`? (los 5 superpoderes)
- **13.2.** Punteros Crudos: `*const T` y `*mut T`
- **13.3.** Punteros Avanzados: `NonNull<T>` y Null Pointer Optimization
- **13.4.** Funciones `unsafe`
- **13.5.** El Patrón Central: Abstracciones Seguras Sobre Código Inseguro
- **13.6.** Inicialización Segura de Memoria con `MaybeUninit<T>`
- **13.7.** FFI: Interoperabilidad con C (llamar C, exponer Rust)
- **13.8.** Protocolo de Seguridad en FFI (punteros nulos, alineación, encapsulación)
- **13.9.** `#[repr(C)]`: Layout Compatible con C
- **13.10.** Uniones
- **13.11.** `static mut` y alternativas modernas (`AtomicI32`)
- **13.12.** Cuándo (No) Usar `unsafe`: Guía Práctica

#### 14. Testing, Benchmarking y Producción
- **14.1.** Tests Unitarios (`#[cfg(test)]`, `#[should_panic]`)
- **14.2.** Testing Asíncrono (`#[tokio::test]`) y Mocks con `mockall`
- **14.3.** Tests de Integración (`tests/`)
- **14.4.** Asserts y Macros de Testing
- **14.5.** Benchmarking con `criterion` (`black_box`)
- **14.6.** `clippy`: El Linter que Enseña Rust Idiomático
- **14.7.** `rustfmt`: Formateo Automático
- **14.8.** Metaprogramación con Macros Declarativas (`macro_rules!`)
- **14.9.** Perfiles de Compilación: debug vs release (LTO, codegen-units, strip)
- **14.10.** Workspaces: Proyectos Multi-Crate
- **14.11.** Arquitectura Modular con Feature Flags (`#[cfg(feature)]`)
- **14.12.** Documentación: `cargo doc` (doc tests ejecutables)
- **14.13.** Checklist de Nivel Producción
- **14.14.** Auditoría de Cadena de Suministro y Seguridad (`cargo-audit`, `cargo-deny`)

---
