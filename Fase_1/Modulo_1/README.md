# Modulo 1 — Introduccion a Rust y Sintaxis Basica

Este directorio contiene los proyectos practicos desarrollados durante el **Modulo 1** del curso de Rust. Su objetivo es establecer las bases de la sintaxis, tipos de datos primitivos, control de flujo y operaciones de entrada/salida.

---

## Proyectos y Ejercicios Practicos

| Proyecto / Ejercicio | Conceptos Principales | Descripcion Corta |
| :--- | :---: | :--- |
| **[`Conversor_Temp`](./Conversor_Temp)** | Sintaxis Basica, I/O & Loops | Conversor CLI interactivo de temperaturas (Celsius/Fahrenheit) y clasificador de sensacion termica. |

---

## Estructura Común de los Proyectos

Todos los ejercicios estan organizados bajo el patron estandar de Rust:
* `src/lib.rs`: Contiene las funciones puras de conversion, pruebas unitarias y doc-tests.
* `src/main.rs`: Binario interactivo para interaccion con el usuario por consola.
* `README.md`: Documentacion individual explicativa del proyecto.

---

## Como Ejecutar los Ejercicios

Ingresa a la carpeta del proyecto y ejecuta:

```bash
cargo run        # Ejecuta el binario principal CLI
cargo test       # Corre las pruebas unitarias y doc-tests
cargo doc --open # Abre la documentacion interactiva en el navegador
```
