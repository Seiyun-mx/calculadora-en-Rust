use std::io;

fn main() {
    println!("Calculadora en Rust");

    // bucle infinito
    loop {
        println!("\n----------------------------------------");
        
        // Pedir y leer el primer número
        println!("Introduce el primer número:");
        let mut input_num1 = String::new();
        io::stdin()
            .read_line(&mut input_num1)
            .expect("Error al leer la línea");

        let num1: f64 = match input_num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        //  menu de operaciones
        println!("Selecciona una operación:");
        println!("1 - Suma");
        println!("2 - Resta");
        println!("3 - Multiplicación");
        println!("4 - División");
        println!("5 - Salir del programa");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la línea");

        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opción no válida. Debe ser un número del 1 al 5.");
                continue;
            }
        };

        // romper ciclo
        if opcion == 5 {
            println!("¡Gracias por usar la calculadora! ¡Hasta pronto!");
            break;
        }

        // Pedir y leer el segundo número
        println!("Introduce el segundo número:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Error al leer la línea");

        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        // pedir y hacer la operación
        let (resultado, simbolo) = match opcion {
            1 => (num1 + num2, "+"),
            2 => (num1 - num2, "-"),
            3 => (num1 * num2, "*"),
            4 => {
                if num2 == 0.0 {
                    println!("Error: No se puede dividir entre cero.");
                    continue;
                }
                (num1 / num2, "/")
            },
            _ => {
                println!("Opción no válida. Debe ser un número del 1 al 5.");
                continue;
            }
        };

        // Mostrar el resultado final
        println!(">>> Resultado: {} {} {} = {}", num1, simbolo, num2, resultado);
    }
}