# Controlador de Semáforo en Rust — Módulo 3

Este proyecto corresponde al desarrollo práctico del **Módulo 3** del curso de Rust. Su propósito es modelar un controlador de tráfico emulado mediante **Enumeraciones (`enum`)**, coincidencia exhaustiva de patrones (**`match`**) y métodos asociados.

---

## Objetivo del Proyecto

Demostrar el uso de tipos de datos algebraicos (*Algebraic Data Types*) para modelar máquinas de estados finitos deterministas de forma segura en tiempo de compilación.

---

## Estructura de Componentes y Funcionalidades

| Componente / Método | Firma de Tipos | Concepto Demostrado | Descripción |
| :--- | :--- | :---: | :--- |
| **`Semaforo`** | `enum { Rojo, Amarillo, Verde }` | **Enumeración (`enum`)** | Representa los estados físicos del semáforo. |
| **`siguiente`** | `fn(&self) -> Semaforo` | **Pattern Matching (`match`)** | Transición de estados determinista: **Rojo ➔ Verde ➔ Amarillo ➔ Rojo**. |
| **`duracion_segundos`** | `fn(&self) -> u32` | **Métodos Asociados (`impl`)** | Devuelve la duración en segundos del estado activo (Rojo: 30s, Amarillo: 5s, Verde: 25s). |

---

## Glosario de Conceptos Técnicos

* **Enumeraciones (`enum`):** Tipo de dato que define un conjunto cerrado de variantes posibles. En Rust, las variantes son tipos de datos algebraicos seguros.
* **Pattern Matching Exhaustivo (`match`):** Mecanismo de control de flujo que obliga a cubrir el 100% de los casos posibles de un `enum`. Si falta un caso, el compilador rechaza compilar.
* **Métodos en Enums (`impl Enum`):** Permite asociar funciones miembro directamente a un `enum` operando sobre `&self`.
* **Traits `Copy` y `Clone`:** Permiten la duplicación transparente por valor del estado del semáforo al transicionar.

---

## Flujo de Ejecución

Al ejecutar `cargo run`:

```text
=== CONTROLADOR DE SEMAFORO ===

Paso 1: Luz actual: Rojo, Duracion: 30s
Paso 2: Luz actual: Verde, Duracion: 25s
Paso 3: Luz actual: Amarillo, Duracion: 5s
Paso 4: Luz actual: Rojo, Duracion: 30s
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

* **Generar y abrir la documentación técnica en HTML:**
  ```bash
  cargo doc --open
  ```
