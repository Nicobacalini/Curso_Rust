use Semaforo::Semaforo;

fn main() {
    println!("=== CONTROLADOR DE SEMAFORO ===\n");

    let mut estado_actual = Semaforo::Rojo;

    for paso in 1..=4 {
        println!(
            "Paso {}: Luz actual: {:?}, Duracion: {}s",
            paso,
            estado_actual,
            estado_actual.duracion_segundos()
        );
        estado_actual = estado_actual.siguiente();
    }
}