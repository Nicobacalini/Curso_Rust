# Gestion de Empleados y Departamentos en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es modelar la informacion de empleados utilizando la estructura **`Empleado`** (`nombre: String`, `salario: f64`, `departamento: Option<String>`), metodos asociados y el uso de **`match`** o **`unwrap_or_else`** para manejar valores ausentes ("Sin asignar").

---

## Objetivo del Proyecto

Demostrar el uso del tipo `Option<T>` en estructuras con multiples campos y el manejo seguro de valores mediante coincidencia de patrones (`match`) y metodos funcionales como `unwrap_or_else`.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`Empleado`** | `struct { nombre: String, salario: f64, departamento: Option<String> }` | **Estructura con `Option`** | Modelo de datos de un empleado con salario y departamento opcional. |
| **`nuevo`** | `fn(&str, f64, Option<&str>) -> Empleado` | **Constructor Asociado** | Instancia un empleado convirtiendo cadenas de texto y valores flotantes. |
| **`obtener_departamento`** | `fn(&self) -> String` | **Metodo `unwrap_or_else`** | Devuelve el nombre del departamento o "Sin asignar" si la variante es `None`. |
| **`imprimir_departamento`** | `fn(&self)` | **Pattern Matching (`match`)** | Evalua la variante de departamento e imprime la informacion del empleado. |

---

## Glosario de Conceptos Tecnicos

* **Estructura con `Option<T>`:** Representa campos opcionales dentro de un `struct` eliminando la necesidad de valores nulos.
* **Metodo `unwrap_or_else`:** Permite obtener el valor contenido en `Some` o ejecutar una clausula closure/fallback cuando es `None`.
* **Pattern Matching (`match`):** Desestructura variantes de enumeraciones para manejar cada escenario de forma exhaustiva.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
=== GESTION DE EMPLEADOS Y DEPARTAMENTOS ===

--- Impresion con match ---
Empleado: Juan, Salario: $1500.00, Departamento: Ventas
Empleado: Maria, Salario: $1800.00, Departamento: Sin asignar

--- Consulta con unwrap_or_else ---
Depto empleado 1: Ventas
Depto empleado 2: Sin asignar
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
