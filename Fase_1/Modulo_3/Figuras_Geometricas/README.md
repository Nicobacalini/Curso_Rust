# Calculador de Figuras Geometricas en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es modelar figuras geometricas utilizando **Enumeraciones con datos asociados (`enum`)**, coincidencia de patrones (**`match`**), calculo de perimetros y busqueda de la figura con mayor perimetro sobre referencias a arreglos o rebanadas (**`&[T]`**).

---

## Objetivo del Proyecto

Demostrar el uso de enumeraciones compuestas (*Algebraic Data Types*) que contienen datos asociados, el manejo de referencias prestadas en colecciones y el uso de `Option<&T>` para manejar listas vacias o busquedas de forma segura sin excepciones en tiempo de ejecucion.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`Figura::Circulo`** | `Circulo(f64)` | **Enum con tupla** | Variante que contiene el radio de la circunferencia. |
| **`Figura::Rectangulo`** | `Rectangulo(f64, f64)` | **Enum con tupla** | Variante que contiene la base y la altura. |
| **`Figura::Cuadrado`** | `Cuadrado(f64)` | **Enum con tupla** | Variante que contiene la longitud del lado. |
| **`perimetro`** | `fn(&self) -> f64` | **Pattern Matching (`match`)** | Calcula el perimetro exacto segun la variante geométrica. |
| **`figura_mas_grande`** | `fn(&[Figura]) -> Option<&Figura>` | **Borrowing y `Option`** | Devuelve una referencia opcional a la figura con mayor perimetro de un slice. Retorna `None` si el slice esta vacio. |

---

## Glosario de Conceptos Tecnicos

* **Enumeraciones con datos (`enum`):** En Rust, las variantes de un `enum` pueden almacenar valores de diferentes tipos (como tuplas `(f64)` o `(f64, f64)`).
* **Pattern Matching (`match`):** Desestructura las variantes del `enum` extrayendo los valores internos de cada figura para operar con ellos.
* **Borrowing y Slices (`&[T]`):** Permite iterar y analizar colecciones de datos sin tomar posesion (*ownership*) de los elementos.
* **Manejo de `Option<&T>`:** Representa la presencia (`Some(&Figura)`) o ausencia (`None`) de un resultado sin riesgo de errores por punteros nulos.
* **Traits Derivados (`Debug`, `Clone`, `Copy`, `PartialEq`):** Permiten duplicacion por valor, comparacion de igualdad e impresion formateada.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
Perimetro del circulo: 31.41592653589793
Perimetro del rectangulo: 20
Perimetro del cuadrado: 12
La figura mas grande es: Circulo(5.0)
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
