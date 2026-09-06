use Empleado::Empleado;

fn main() {
    println!("=== GESTION DE EMPLEADOS Y DEPARTAMENTOS ===\n");

    // Creacion de un empleado con departamento asignado
    let empleado1 = Empleado::nuevo("Juan", 1500.0, Some("Ventas"));

    // Creacion de un empleado sin departamento asignado
    let empleado2 = Empleado::nuevo("Maria", 1800.0, None);

    // Demostración con imprimir_departamento (usando match)
    println!("--- Impresion con match ---");
    empleado1.imprimir_departamento();
    empleado2.imprimir_departamento();

    // Demostración con obtener_departamento (usando unwrap_or_else)
    println!("\n--- Consulta con unwrap_or_else ---");
    println!("Depto empleado 1: {}", empleado1.obtener_departamento());
    println!("Depto empleado 2: {}", empleado2.obtener_departamento());
}
