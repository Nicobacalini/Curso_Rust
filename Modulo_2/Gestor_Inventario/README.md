# Gestor de Inventario en Rust — Modulo 2

Este proyecto corresponde al desarrollo practico del **Modulo 2** del curso de Rust. Su proposito es implementar funciones de gestion de inventario para entender los conceptos de **Ownership**, **Borrowing** (prestamos inmutables y mutables) y retorno de referencias opcionales (`Option<&T>`).

---

## Objetivo del Proyecto

Demostrar la diferencia entre transferir la propiedad (*move*), prestar por referencia inmutable (`&T`), modificar datos por referencia mutable (`&mut T`) y filtrar datos sobre arreglos usando rebanadas (*slices* `&[T]`).

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`crear_producto`** | `fn(String, f64) -> String` | **Move / Ownership** | Toma posesion del parametro `nombre` consumiendo la variable original. |
| **`aplicar_descuento`** | `fn(&f64, f64) -> f64` | **Prestamo Inmutable (`&T`)** | Lee el precio original sin tomar su propiedad. |
| **`actualizar_stock`** | `fn(&mut i32, i32)` | **Prestamo Mutable (`&mut T`)** | Modifica directamente el valor del stock en memoria. |
| **`primer_producto_caro`** | `fn(&[f64], f64) -> Option<&f64>` | **Slices y `Option`** | Retorna una referencia al primer valor que supera un umbral o `None`. |

---

## Glosario de Conceptos Tecnicos

* **Ownership (Propiedad):** Regla fundamental de Rust donde cada valor tiene un solo propietario. Al pasar `String` por valor, el propietario cambia.
* **Borrowing Inmutable (`&T`):** Permite multiples lecturas simultaneas sin modificar el valor original.
* **Borrowing Mutable (`&mut T`):** Permite modificar el valor referenciado garantizando exclusividad de acceso.
* **Regla Aliasing + Mutabilidad:** No se permite tener un prestamo mutable `&mut` mientras exista un prestamo inmutable `&` activo en el mismo ambito.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
--- Inicio del programa ---
Producto: Teclado Mecanico, Precio: 79.99
[APLICAR] Precio con descuento: 71.991
[APLICAR] Precio original: 79.99, Precio final: 71.991
Stock actualizado internamente: 5
Stock verificado en main: 5
Primer producto mas caro que 50: 80
--- Fin del programa ---
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
