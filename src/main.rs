use std::io;

fn main() {
    println!("Calculadora en Rust");

    // Pedir y leer el primer número
    println!("Introduce el primer número:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Error al leer la línea");

    // Convertir el texto de num1 a un número real
    let num1: f64 = num1
        .trim()
        .parse()
        .expect("Por favor, introduce un número válido");

    // Pedir y leer la operación
    println!("Introduce la operación (+, -, *, /):");
    let mut operacion = String::new();
    io::stdin()
        .read_line(&mut operacion)
        .expect("Error al leer la línea");
    
    // Limpiar los espacios y saltos de línea de la operación
    let operacion = operacion.trim();

    // Pedir y leer el segundo número
    println!("Introduce el segundo número:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Error al leer la línea");

    // Convertir el texto de num2 a un número real
    let num2: f64 = num2
        .trim()
        .parse()
        .expect("Por favor, introduce un número válido");

    // 4. Hacer la operación
    let resultado = match operacion {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Operación no válida");
            return; // Sale del programa si la operación no existe
        }
    };

    //  Mostrar el resultado final
    println!("El resultado es: {} {} {} = {}", num1, operacion, num2, resultado);
}