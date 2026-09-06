# Modulo 3 — Enums, Pattern Matching y Manejo de Errores

Este directorio contiene los proyectos practicos desarrollados durante el **Modulo 3** del curso de Rust. Cada ejercicio aborda enumeraciones avanzadas (`enum`), coincidencia de patrones (*pattern matching*), guardas de patron, `Option<T>`, `Result<T, E>`, el operador `?` y estructuracion modular con librerias.

---

## Proyectos y Ejercicios Practicos

| Proyecto / Ejercicio | Conceptos Principales | Descripcion Corta |
| :--- | :---: | :--- |
| **[`Decodificador_IoT`](./Decodificador_IoT)** | Match Guards & `@` Binding | Decodificacion de tramas de red IoT y filtrado con `if let`. |
| **[`Empleado`](./Empleado)** | `Option<T>` & `unwrap_or_else` | Gestion de empleados con departamento opcional y fallback seguro. |
| **[`Figuras_Geometricas`](./Figuras_Geometricas)** | Enums con Tuplas & Slices | Calculo de perimetros y busqueda de la figura de mayor tamano. |
| **[`Gestor_Mem_Virutal`](./Gestor_Mem_Virutal)** | `Result<T, E>` & `Option` | Simulacion de asignacion de bloques de memoria RAM y proteccion de Kernel. |
| **[`Monitoreo_sensor_I2C`](./Monitoreo_sensor_I2C)** | Operador `?` & Pipelines | Simulacion de driver I2C y propagacion automatica de fallos de hardware. |
| **[`Router_Eventos`](./Router_Eventos)** | Newtypes & Operador `?` | Enrutamiento de transacciones financieras y propagacion de errores. |
| **[`Semaforo`](./Semaforo)** | Finite State Machine & Match | Control determinista de luces de trafico y duracion de estados. |

---

## Estructura Común de los Proyectos

Todos los ejercicios estan organizados bajo el patron estandar de Rust:
* `src/lib.rs`: Contiene la logica de negocio, pruebas unitarias y doc-tests.
* `src/main.rs`: Binario delgado de ejecucion por consola.
* `README.md`: Documentacion individual explicativa de cada proyecto.

---

## Como Ejecutar los Ejercicios

Ingresa a la carpeta de cualquiera de los proyectos y ejecuta:

```bash
cargo run        # Ejecuta el binario principal
cargo test       # Corre todas las pruebas unitarias y doc-tests
cargo doc --open # Abre la documentacion interactiva en el navegador
```
