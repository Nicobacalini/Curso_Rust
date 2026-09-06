# Pool de Conexiones en Rust — Modulo 2

Este proyecto corresponde al desarrollo practico del **Modulo 2** del curso de Rust. Su proposito es emular un administrador de recursos (*Pool de Conexiones*) registrando auditorias de acceso y garantizando una destruccion limpia mediante el trait **`Drop`**.

---

## Objetivo del Proyecto

Demostrar la gestion de recursos mediante lifetimes implicitos en metodos que devuelven referencias mutables (`&mut T`), vectores dinamicos y la interceptacion de la destruccion de memoria con `Drop` (RAII).

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`RecursoConexion`** | `struct { id: u32, en_uso: bool }` | **RAII / Drop** | Representa una conexion individual. Advierte al destruirse si continua en uso. |
| **`Pool`** | `struct { recursos: Vec<RecursoConexion>, log_eventos: Vec<String> }` | **Coleccion Dinamica** | Administra el conjunto de conexiones y mantiene el registro de auditoria. |
| **`adquirir_y_auditar`** | `fn(&mut self, usize) -> &mut RecursoConexion` | **Prestamos Re-prestados (*Reborrowing*)** | Inserta un evento de auditoria y retorna una referencia mutable al recurso. |

---

## Glosario de Conceptos Tecnicos

* **RAII (Resource Acquisition Is Initialization):** Patron de disenio donde los recursos se liberan automaticamente al salir del ambito (*scope*).
* **Trait `Drop`:** Destructor personalizado de Rust que se ejecuta de forma determinista al destruir una variable.
* **Reborrowing (`&mut self` a `&mut T`):** Re-prestamo de una referencia mutable sobre un elemento interno mientras dura el ambito activo.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
ADVERTENCIA: Conexion 1 eliminada mientras estaba en uso
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
