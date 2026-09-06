# Modulo 2 — Fundamentos Avanzados de Rust

Este directorio contiene los proyectos practicos desarrollados durante el **Modulo 2** del curso de Rust. Cada ejercicio aborda conceptos clave de la memoria, ownership, borrowing, lifetimes, estructuras de datos y gestion de recursos en Rust.

---

## Proyectos y Ejercicios Practicos

| Proyecto / Ejercicio | Conceptos Principales | Descripcion Corta |
| :--- | :---: | :--- |
| **[`Gestor_Inventario`](./Gestor_Inventario)** | Ownership & Borrowing | Gestion de productos, descuentos y stock usando referencias inmutables `&` y mutables `&mut`. |
| **[`Buffer_Circular_UART`](./Buffer_Circular_UART)** | Structs, Arrays & `Drop` | Emulacion de un buffer circular FIFO de hardware con manejo de overflow y RAII. |
| **[`Analizador_Red`](./Analizador_Red)** | Lifetimes (`'a`) & Zero-Copy | Parsing de tramas Ethernet II en lectura directa sin copias de memoria. |
| **[`Micro-Decoder-Cpu`](./Micro-Decoder-Cpu)** | Match & State Machine | Emulador de una CPU de 8 bits con ciclo de ejecucion Fetch-Decode-Execute. |
| **[`Pool_Conexiones`](./Pool_Conexiones)** | Vectors, Reborrowing & `Drop` | Administrador de recursos de red con registro de auditoria y liberacion automatica. |

---

## Estructura Común de los Proyectos

Todos los ejercicios estan organizados bajo el patron estandar de Rust:
* `src/lib.rs`: Contiene la logica de negocio, pruebas unitarias y doc-tests.
* `src/main.rs`: Binario delgado de ejecucion por consola.
* `README.md`: Documentacion individual detallada de cada proyecto.

---

## Como Ejecutar los Ejercicios

Ingresa a la carpeta de cualquiera de los proyectos y ejecuta:

```bash
cargo run        # Ejecuta el binario principal
cargo test       # Corre todas las pruebas unitarias y doc-tests
cargo doc --open # Abre la documentacion interactiva en el navegador
```
