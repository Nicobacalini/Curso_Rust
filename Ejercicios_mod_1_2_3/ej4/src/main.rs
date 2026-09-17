// ==============================================================================
// EJERCICIO 4: "Two Pointers: Inversión In-Place por Pares"
//
// Dada una referencia mutable a un slice de números enteros (&mut [i32]), debes
// invertir ÚNICAMENTE las posiciones de los elementos que sean pares.
// Los elementos impares deben quedarse en sus posiciones originales intactos.
//
// Reglas y técnica algorítmica:
// 1. Usar dos índices (izq al inicio, der al final arr.len() - 1).
// 2. Mientras izq < der:
//    - Si arr[izq] no es par, avanzar izq (izq += 1).
//    - Si arr[der] no es par, retroceder der (der -= 1).
//    - Si ambos son pares:
//         * Intercambiarlos usando arr.swap(izq, der).
//         * Avanzar izq (izq += 1) y retroceder der (der -= 1).
// 3. No crear vectores nuevos ni clonar. Todo debe mutarse in-place.
// ==============================================================================


pub fn invertir_pares_in_place(arr: &mut [i32]){
    // si el arr es vacio nada que hacer
    if arr. is_empty(){
        return;
    }

    let mut izq = 0;
    let mut der = arr.len() - 1; 

    while izq<der{
        let val_izq = arr[izq];
        let val_der = arr[der];
        if arr[izq] % 2 != 0{
            izq += 1;
        }
        else if arr[der] % 2 != 0 {
            der -= 1;
        }
        else{
            arr.swap(izq, der);
            izq += 1;
            der -= 1;
        }

    }
}


fn main() {
    let mut data = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Antes: {:?}", data);

    invertir_pares_in_place(&mut data);

    println!("Después: {:?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dos_punteros() {
        let mut data = [1, 2, 3, 4, 5, 6, 7, 8];
        invertir_pares_in_place(&mut data);
        assert_eq!(data, [1, 8, 3, 6, 5, 4, 7, 2]);

        let mut impares = [1, 3, 5, 7];
        invertir_pares_in_place(&mut impares);
        assert_eq!(impares, [1, 3, 5, 7]);

        let mut vacio: [i32; 0] = [];
        invertir_pares_in_place(&mut vacio);
        assert_eq!(vacio, []);
    }
}