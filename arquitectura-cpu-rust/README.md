## Estructura del Proyecto

```text
arquitectura-cpu-rust/
├── Cargo.toml                       # Configuración raíz del Cargo Workspace
├── README.md                        # Documentación, diagramas y especificaciones
│
├── cpu-pipeline/                    # Crate de biblioteca: Simulación del procesador
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                   # Punto de entrada de la crate, API pública del CPU
│       ├── instruction.rs           # Definición de opcodes, tipos de datos y decodificación
│       ├── registers.rs             # Banco de registros (register file) y PC
│       ├── pipeline.rs              # Registros de desacople (IF/ID, ID/EX...) y avance de etapas
│       └── hazard.rs                # Unidad de riesgos: detección de stalls y forwarding
│
├── cache-controller/                # Crate de biblioteca: Subsistema de memoria y caché
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                   # Punto de entrada de la crate y struct CacheController
│       ├── storage.rs               # Estructura física: líneas, bloques, sets y tags
│       ├── policy.rs                # Políticas de reemplazo (LRU/FIFO) y escritura (WT/WB)
│       └── bus.rs                   # Interfaz de comunicación con memoria principal / penalizaciones
│
└── sistema-integrado/               # Crate binaria: Driver ejecutable y simulador
    ├── Cargo.toml
    └── src/
        ├── main.rs                  # Loop principal de simulación por ciclos de reloj
        └── display.rs               # Monitoreo y formateo del estado del pipeline en consola
```