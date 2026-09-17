# Conversor de Temperatura en Rust — Modulo 1

Este proyecto corresponde al desarrollo practico del **Modulo 1** del curso de Rust. Su proposito es construir un conversor de temperaturas entre escalas Celsius y Fahrenheit e implementar la clasificacion de sensaciones termicas con consola interactiva CLI.

---

## Objetivo del Proyecto

Demostrar el uso de variables, funciones matematicas simples, control de flujo con `if` / `else`, iteracion con bucles `for` sobre arreglos estaticos y lectura de entrada por teclado con validacion y *shadowing*.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`celsius_a_fahrenheit`** | `fn(f64) -> f64` | **Operaciones Numericas** | Convierte grados Celsius a Fahrenheit `(c * 1.8) + 32.0`. |
| **`fahrenheit_a_celsius`** | `fn(f64) -> f64` | **Operaciones Numericas** | Convierte grados Fahrenheit a Celsius `(f - 32.0) / 1.8`. |
| **`clasificar_temperatura`** | `fn(f64) -> &'static str` | **Control de flujo** | Retorna la etiqueta "congelante", "templada" o "calurosa". |
| **`pedir_temperatura`** | `fn(&str) -> f64` | **Entrada E/S y Validacion** | Lee entrada por consola validando tipos decimales mediante `parse::<f64>()`. |

---

## Glosario de Conceptos Tecnicos

* **Variables inmutables y mutables:** Declaracion por defecto inmutable (`let x`) y mutable (`let mut entrada`).
* **Shadowing:** Reutilizacion del nombre de una variable transformando su tipo (por ejemplo de `String` a `u32`).
* **Slices estaticos (`&'static str`):** Cadenas literales integradas en el binario compilado.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
=== Pruebas iniciales con Array y For ===
-5.0 C equivale a 23.0 F (congelante)
20.0 C equivale a 68.0 F (templada)
36.5 C equivale a 97.7 F (calurosa)
========================================

--- Menu Conversor de Temperatura ---
1. Convertir de Celsius a Fahrenheit
2. Convertir de Fahrenheit a Celsius
3. Salir del programa
Elige una opcion (1-3):
```

---

## Comandos del Proyecto

* **Ejecutar el programa principal:**
  ```bash
  cargo run
  ```

* **Ejecutar los Tests Unitarios y Doc-Tests:**
  ```bash
  cargo test
  ```

* **Generar y abrir la documentacion tecnica en HTML:**
  ```bash
  cargo doc --open
  ```
