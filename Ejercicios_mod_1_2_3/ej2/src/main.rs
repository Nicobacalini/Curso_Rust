// ==============================================================================
// EJERCICIO 2: "Máxima Racha Positiva"
//
// Dada una referencia a un slice de enteros `&[i32]`, una "racha positiva" es una
// subsecuencia contigua donde todos los números son estrictamente mayores a 0 (> 0).
//
// Escribe una función que devuelva el sub-slice exacto (&[i32]) correspondiente a
// la racha positiva de mayor longitud:
// - Si hay un empate en longitud, retorna la que ocurre primero.
// - Si no hay elementos positivos (o el slice está vacío), retorna un slice vacío &[][cite: 1].
//
// Restricción: No clonar datos, solo devolver una vista prestada (&[i32]) del slice original[cite: 2].
// ==============================================================================

pub fn maxima_racha_positiva(metricas: &[i32]) -> &[i32] {
    let mut max_inicio = 0;
    let mut max_longitud = 0;

    let mut inicio_actual = 0;
    let mut longitud_actual = 0;

    for (i, &num) in metricas.iter().enumerate() {
        if num > 0 {
            if longitud_actual == 0 {
                inicio_actual = i;
            }
            longitud_actual += 1;

            if longitud_actual > max_longitud {
                max_longitud = longitud_actual;
                max_inicio = inicio_actual;
            }
        } else {
            longitud_actual = 0;
        }
    }
    &metricas[max_inicio..max_inicio + max_longitud]
}

fn main() {
    let datos = [1, 2, -1, 4, 5, 6, -3, 7, 8];
    let resultado = maxima_racha_positiva(&datos);
    println!("Racha máxima: {:?}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_racha() {
        let datos = [1, 2, -1, 4, 5, 6, -3, 7, 8];
        assert_eq!(maxima_racha_positiva(&datos), &[4, 5, 6]);

        let todos_negativos = [-1, -5, -2];
        assert_eq!(maxima_racha_positiva(&todos_negativos), &[]);

        let empate = [1, 2, -1, 3, 4];
        assert_eq!(maxima_racha_positiva(&empate), &[1, 2]);
    }
}