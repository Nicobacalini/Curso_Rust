// ==============================================================================
// EJERCICIO 9: "Algoritmo de Kadane con Reconstruccion de Subslice"
//
// Dado un slice de enteros `&[i32]`, debes encontrar la subsecuencia contigua cuya
// suma sea maxima.
//
// Retorno:
// Una tupla `(i64, &[i32])` donde:
// - El primer valor es la suma acumulada maxima (en tipo i64).
// - El segundo valor es la vista prestada del slice original (&arr[inicio..fin]).
//
// Reglas y Casos Borde:
// 1. Si el slice esta vacio -> retornar (0, &[]).
// 2. Si todos los numeros son negativos -> la suma maxima sera el numero negativo
//    mas cercano a cero (ej: [-5, -2, -8] -> (-2, &[-2])).
// 3. Complejidad: O(N) de tiempo y O(1) de memoria adicional (sin Vec, sin .clone()).
// ==============================================================================

pub fn subslice_suma_maxima(arr: &[i32]) -> (i64, &[i32]) {
    if arr.is_empty(){
        return (0, &[]);
    }
    let mut suma_actual = arr[0] as i64;
    let mut suma_maxima = arr[0] as i64;

    let mut inicio_actual = 0;
    let mut mejor_inicio = 0;
    let mut mejor_fin = 1;
    for i in 1..arr.len(){
        let val = arr[i] as i64;
        if val > suma_actual + val {
            suma_actual = val; 
            inicio_actual = i;
        }
        else {
            suma_actual += val;
        }
        if suma_actual > suma_maxima {
            suma_maxima = suma_actual;
            mejor_inicio = inicio_actual;
            mejor_fin = i + 1;
        }
    }
    

    (suma_maxima, &arr[mejor_inicio..mejor_fin])
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let (suma, slice) = subslice_suma_maxima(&arr);
    println!("Suma maxima: {}", suma);
    println!("Subslice: {:?}", slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane() {
        // Caso clasico: la subsecuencia optima es [4, -1, 2, 1] que suma 6
        let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let (suma, slice) = subslice_suma_maxima(&arr);
        assert_eq!(suma, 6);
        assert_eq!(slice, &[4, -1, 2, 1]);

        // Caso slice vacio
        let vacio: [i32; 0] = [];
        assert_eq!(subslice_suma_maxima(&vacio), (0, &[]));

        // Caso todos negativos (debe elegir el mayor individual)
        let todos_negativos = [-5, -2, -8];
        let (suma_neg, slice_neg) = subslice_suma_maxima(&todos_negativos);
        assert_eq!(suma_neg, -2);
        assert_eq!(slice_neg, &[-2]);
    }
}