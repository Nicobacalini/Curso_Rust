# Monitoreo de Sensor I2C en Rust — Modulo 3

Este proyecto corresponde al desarrollo practico del **Modulo 3** del curso de Rust. Su proposito es simular un driver de sensores de temperatura por bus fisico I2C implementando la propagacion concisa de errores de hardware mediante el operador **`?`** y el tipo `Result<f32, ErrorI2C>`.

---

## Objetivo del Proyecto

Demostrar el encadenamiento de etapas de lectura de hardware (`ping_dispositivo`, `leer_registro_crudo`, `validar_checksum`) garantizando la interrupcion anticipada y propagacion automatica de fallos como caidas de tension, dispositivo desconectado o desalineacion de checksum.

---

## Estructura de Componentes y Funcionalidades

| Componente / Metodo | Firma de Tipos | Concepto Demostrado | Descripcion |
| :--- | :--- | :---: | :--- |
| **`ErrorI2C`** | `enum { DispositivoNoResponde, RuidoEnLinea, ChecksumIncorrecto }` | **Errores de Dominio** | Representa los fallos fisicos del bus de comunicaciones. |
| **`DriverI2C`** | `struct { voltaje_linea: f32, sensor_conectado: bool }` | **Estado de Hardware** | Contiene las condiciones de operacion de la linea de datos. |
| **`ping_dispositivo`** | `fn(&self) -> Result<(), ErrorI2C>` | **Comprobacion fisica** | Valida si el sensor esta presente en el bus (`DispositivoNoResponde`). |
| **`leer_registro_crudo`** | `fn(&self, u32) -> Result<f32, ErrorI2C>` | **Simulacion de telemetria** | Retorna la lectura o falla por `RuidoEnLinea` si el voltaje es < 3.0V. |
| **`validar_checksum`** | `fn(&self, f32) -> Result<f32, ErrorI2C>` | **Verificacion matematica** | Valida la integridad del dato leido (`ChecksumIncorrecto`). |
| **`leer_temperatura_sensor`** | `fn(&self, u32) -> Result<f32, ErrorI2C>` | **Operador `?`** | Pipeline encadenado que aborta de inmediato si ocurre cualquier error. |

---

## Glosario de Conceptos Tecnicos

* **Operador de Propagacion `?`:** Desempaqueta `Ok(T)` o retorna de forma inmediata el valor `Err(E)` a la funcion llamante.
* **Manejo de Errores en Pipeline:** Estructuracion de tareas secuenciales donde cada paso depende del exito del anterior.
* **Driver Emulado:** Abstraccion de bajo nivel para comunicacion con perifericos I2C.

---

## Flujo de Ejecucion

Al ejecutar `cargo run`:

```text
=== 1. INICIANDO MONITOREO DE TEMPERATURA (ESTABLE) ===
Fallo de hardware en paso 0: ChecksumIncorrecto
[TELEMETRIA] Temperatura actual: 21.50 C
Fallo de hardware en paso 2: ChecksumIncorrecto
[TELEMETRIA] Temperatura actual: 24.50 C
Fallo de hardware en paso 4: ChecksumIncorrecto
[TELEMETRIA] Temperatura actual: 27.50 C

=== 2. SIMULANDO CAIDA DE TENSION EN BUS I2C (RUIDO EN LINEA) ===
Fallo detectado de forma segura por el driver: RuidoEnLinea

=== 3. SIMULANDO SENSOR DESCONECTADO (DISPOSITIVO NO RESPONDE) ===
Fallo detectado de forma segura por el driver: DispositivoNoResponde
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
