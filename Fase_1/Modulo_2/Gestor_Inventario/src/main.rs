use Gestor_Inventario::{actualizar_stock, aplicar_descuento, crear_producto, primer_producto_caro};

fn main() {
    println!("--- Inicio del programa ---");
    let nombre = String::from("Teclado Mecanico");
    let precio = 79.99;

    // Crear producto moviendo la propiedad de nombre
    let producto = crear_producto(nombre, precio);
    println!("{}", producto);

    // Aplicar descuento prestando el precio por referencia inmutable
    let porcentaje = 0.10;
    let precio_final = aplicar_descuento(&precio, porcentaje);
    println!(
        "[APLICAR] Precio original: {}, Precio final: {}",
        precio, precio_final
    );

    // Actualizar stock por referencia mutable
    let mut stock = 10;
    actualizar_stock(&mut stock, -5);
    println!("Stock verificado en main: {}", stock);

    // Busqueda de producto caro en un slice
    let lista_precios = [25.50, 80.00, 120.00, 45.00];
    let limite = 50.0;
    if let Some(caro) = primer_producto_caro(&lista_precios, limite) {
        println!("Primer producto mas caro que {}: {}", limite, caro);
    } else {
        println!("No se encontro ningun producto superior al limite.");
    }

    println!("--- Fin del programa ---");
}
