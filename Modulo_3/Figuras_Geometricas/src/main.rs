use Figuras_Geometricas::Figura;

fn main() {
    // Instanciacion de figuras geometricas
    let circulo = Figura::Circulo(5.0);
    let rectangulo = Figura::Rectangulo(4.0, 6.0);
    let cuadrado = Figura::Cuadrado(3.0);

    // Calculo e impresion de perimetros individuales
    println!("Perimetro del circulo: {}", circulo.perimetro());
    println!("Perimetro del rectangulo: {}", rectangulo.perimetro());
    println!("Perimetro del cuadrado: {}", cuadrado.perimetro());

    // Arreglo de figuras para comparacion
    let figuras = [circulo, rectangulo, cuadrado];

    // Evaluacion y desempaquetado de la figura mas grande
    if let Some(mas_grande) = Figura::figura_mas_grande(&figuras) {
        println!("La figura mas grande es: {:?}", mas_grande);
    } else {
        println!("No hay figuras en la lista.");
    }
}
