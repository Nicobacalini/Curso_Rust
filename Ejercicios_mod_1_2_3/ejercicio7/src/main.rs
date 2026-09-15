// ==============================================================================
// EJERCICIO 7: "Interprete de Maquina Virtual Stack-Based" (Nivel Intermedio)
//
// Implementa una maquina virtual de pila que ejecuta una lista de instrucciones.
// La funcion debe mantener un vector interno `Vec<i64>` como pila de datos.
//
// Variantes a manejar:
// - Push(valor): Agrega el valor a la pila.
// - Pop: Quita el elemento de la cima. Si la pila esta vacia -> Err("Stack underflow")
// - Dup: Duplica el valor de la cima. Si la pila esta vacia -> Err("Stack underflow")
// - Add, Sub, Mul:
//     * Quitan los dos ultimos operandos de la pila.
//     * Si hay menos de 2 elementos -> Err("Stack underflow en operacion binaria")
//     * El orden de la resta es (primer_elemento_en_entrar - segundo_elemento).
//     * Guardan el resultado de nuevo en la pila.
//
// Retorno final:
// - Si el programa termina con exito y quedan elementos en la pila -> Ok(Some(tope))
// - Si el programa termina con exito y la pila quedo vacia -> Ok(None)
// ==============================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum Instruccion {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Dup,
}

pub fn ejecutar_programa(instrucciones: &[Instruccion]) -> Result<Option<i64>, &'static str> {
    let mut pila: Vec<i64> = Vec::new();
    for inst in instrucciones{
        match inst{
            Instruccion::Push(valor) => {
                pila.push(*valor);
            }
            Instruccion::Pop => {
                pila.pop().ok_or("Stack underflow")?;
            }
            Instruccion::Dup => {
                let cima_ref: &i64 = pila.last().ok_or("Stack underflow")?;
                pila.push(*cima_ref);
            }

            Instruccion::Add | Instruccion::Sub | Instruccion::Mul => {
                let b = pila.pop().ok_or("Stack underflow en operacion binaria")?;
                let a = pila.pop().ok_or("Stack underflow en operacion binaria")?;

                // Evaluamos cuál de las tres operaciones toca aplicar
                let resultado = match inst {
                    Instruccion::Add => a + b,
                    Instruccion::Sub => a - b,
                    Instruccion::Mul => a * b,
                    _ => unreachable!(),
                };
                pila.push(resultado);
            }
        }
    }
    Ok(pila.pop())
}

fn main() {
    use Instruccion::*;
    // Programa: (5 + 3) * 2
    let prog = [Push(5), Push(3), Add, Push(2), Mul];
    println!("Resultado: {:?}", ejecutar_programa(&prog));
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruccion::*;

    #[test]
    fn test_vm() {
        // (5 + 3) * 2 = 16
        let prog = [Push(5), Push(3), Add, Push(2), Mul];
        assert_eq!(ejecutar_programa(&prog), Ok(Some(16)));

        // Error por falta de operandos
        let fallo = [Push(10), Sub];
        assert_eq!(ejecutar_programa(&fallo), Err("Stack underflow en operacion binaria"));

        // Duplicar y sumar
        let dup_prog = [Push(7), Dup, Add];
        assert_eq!(ejecutar_programa(&dup_prog), Ok(Some(14)));

        // Pila vacia al final
        let vacia = [Push(10), Pop];
        assert_eq!(ejecutar_programa(&vacia), Ok(None));
    }
}