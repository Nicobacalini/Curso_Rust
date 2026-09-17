// ==============================================================================
// EJERCICIO 3: "Validador de Transacciones Seguras"
//
// Un sistema financiero procesa transacciones representadas por un Enum.
// Debes escribir la función `ejecutar_operacion` que tome una referencia mutable
// al saldo actual (&mut f64) y una Operacion, actualizando el saldo in-place si
// la operación es válida, o retornando un error descriptivo con Result.
//
// Reglas de negocio:
// 1. Deposito(monto):
//    - Si monto <= 0.0 -> Err("Monto de depósito inválido")
//    - Si es válido -> Aumentar el saldo y retornar Ok(*saldo)
//
// 2. Extraccion(monto):
//    - Si monto <= 0.0 -> Err("Monto de extracción inválido")
//    - Si monto > saldo actual -> Err("Fondos insuficientes") (¡el saldo no debe cambiar!)
//    - Si es válido -> Disminuir el saldo y retornar Ok(*saldo)
//
// 3. AjusteTasa { factor }:
//    - Si factor < 0.0 -> Err("Factor negativo no permitido")
//    - Si es válido -> Multiplicar el saldo por el factor y retornar Ok(*saldo)
// ==============================================================================



#[derive (Debug, PartialEq)]
pub enum Operacion{
    Deposito(f64),
    Extraccion(f64),
    AjusteTasa{ factor : f64},
}

pub fn ejecutar_operacion(saldo : &mut f64, op: Operacion) -> Result<f64, &'static str>{
    match op {
        Operacion::Deposito(monto) => {
            if monto <= 0.0 {
                Err("Monto de depósito inválido")
            }
            else{
                *saldo += monto;
                Ok(*saldo)
            }
        }
        Operacion::Extraccion(monto) => {
            if monto <= 0.0 {
                Err("Monto de extracción inválido")
            } else if monto > *saldo {
                Err("Fondos insuficientes")
            } else {
                *saldo -= monto;
                Ok(*saldo)
            }
        }
        Operacion::AjusteTasa {factor} => {
            if factor < 0.0{
                Err("Factor negativo no permitido")
            }else{
                *saldo *= factor;
                Ok(*saldo)
            }
        }
    }
}

fn main(){
    let mut saldo = 100.0;
    println! ("Saldo inicial: {}", saldo);

    let _ = ejecutar_operacion(&mut saldo, Operacion::Deposito(50.0));

    println! ("Saldo inicial: {}", saldo);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operaciones() {
        let mut saldo = 100.0;

        // Deposito exitoso
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::Deposito(50.0)), Ok(150.0));
        assert_eq!(saldo, 150.0);

        // Deposito invalido
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::Deposito(-5.0)), Err("Monto de depósito inválido"));
        assert_eq!(saldo, 150.0);

        // Extraccion con fondos insuficientes (no debe mutar el saldo)
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::Extraccion(200.0)), Err("Fondos insuficientes"));
        assert_eq!(saldo, 150.0);

        // Extraccion exitosa
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::Extraccion(50.0)), Ok(100.0));
        assert_eq!(saldo, 100.0);

        // Ajuste de tasa exitoso
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::AjusteTasa { factor: 1.1 }), Ok(110.0));
        assert_eq!(saldo, 110.0);

        // Ajuste negativo rechazado
        assert_eq!(ejecutar_operacion(&mut saldo, Operacion::AjusteTasa { factor: -0.5 }), Err("Factor negativo no permitido"));
        assert_eq!(saldo, 110.0);
    }
}