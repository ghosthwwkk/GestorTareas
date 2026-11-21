// nota: esto importa el modulo de entrada y salida y el trait write para manejar la consola!
use std::io::{self, Write};


/*  
ESTRUCTURA TAREA
*/


// Representa una tarea del gestor cada tarea tiene una descripcion y un estado
struct Tarea {descripcion: String,completada: bool,}


/* 
FUNCIONES AUXILIARES
*/

// Lee un texto ingresado por el usuario
fn leer_texto(mensaje: &str) -> String {

    // Mostramos el mensaje en pantalla
    print!("{}", mensaje);

    //Forzamos la impresión del mensaje
    io::stdout().flush().expect("Error al mostrar mensaje");

    // Creamos una variable mutable para guardar la entrada
    let mut entrada = String::new();

    //Leemos lo que escribe el usuario
    io::stdin() .read_line(&mut entrada) .expect("Error al leer entrada");

    // Eliminamos el salto de línea y devolvemos el texto
    entrada.trim().to_string()
}


/*
 AGREGAR TAREAS
*/

/// Funcion que solicita una descripcion al user, valida que no este vacia  y agrega una nueva tarea al vector 
fn agregar_tarea(tareas: &mut Vec<Tarea>) {

    println!("\n AGREGAR TAREA ");

    // Solicitamos la descripcion de la tareash
    let descripcion = leer_texto("Ingrese la descripcion: ");

    // Comprobamos que no este vacia para evitar tareas sin texto !!!1!!
    if descripcion.is_empty() {
        println!("Error: la descripcion no puede estar vacia");
        return;
    }

    // creamos una nueva tarea que se inicializa como pendiente 
    let nueva_tarea = Tarea {
        descripcion: descripcion,
        completada: false,
    };

    //  Agregamos la tarea al vector 
    tareas.push(nueva_tarea);

    println!("Tarea agregada correctamente!!");
}


/*
 LISTAR TAREAS
*/


/// Funcion que muestra por consola todas las tareas registradas con su numero y estado, si la lista está vacía, avisa al usuario y termina...
fn listar_tareas(tareas: &[Tarea]) {

    println!("\n LISTA DE TAREAS ");

    //Comprobamos si es que el vector esta vacio si no hay tareas guardadas, avisamos y salimos de la funcion
    if tareas.is_empty() {
        println!("No hay tareas registradas");
        return;
    }

    // Recorremos el slice de tareas obteniendo la posicion y cada elemento!
    for (indice, tarea) in tareas.iter().enumerate() {

        // Determinamos el estado de la tarea, el booleano ahora es un texto 
        let estado = if tarea.completada { "Completada"} else {"Pendiente"};

        // Mostramos el numero, descripcion y el estado sumamos 1 para que el usuario entienda la numeracion
        println!("{}. {} | Estado: {}", indice + 1, tarea.descripcion, estado);
    }
} 



/*
COMPLETAR TAREAS
*/


///Funcion q muestra las tareas disponibles y le pide al usuario un número valido  para marcar la tarea seleccionada como completada 
fn completar_tarea(tareas: &mut Vec<Tarea>) {

    println!("\n COMPLETAR TAREA ");

    // Verificamos si existen tareas ya que si la lista esta vacía, avisamos y salimos para no pedir datos innecesarios
    if tareas.is_empty() {
        println!("No hay tareas para completar");
        return;
    }

    // Mostramos las tareas disponibles y reutilizamos listar_tareas para que el usuario vea los numeros disponibles 
    listar_tareas(tareas);

    // Solicitamos el número de la tarea
    let entrada = leer_texto("\nIngrese el numero de la tarea: ");

    // Convertimos el texto a un número, Intentamos convertir el texto ingresado a un entero sin signo
    let numero: usize = match entrada.parse() {

        // Si la conversión funciona guardamos el número
        Ok(valor) => valor,

        // Si falla mostramos un mensaje y salimos
        Err(_) => {
            println!("Debe ingresar un numero valido !");
            return;
        }
    };

    // Verificamos que el numero sea mayor que cero con esto evitamos que numero - 1 provoque un error de desbordamiento
    if numero == 0 {
        println!("Numero de tarea no valido X");
        return;
    }

    // Buscamos una referencia mutable a la tarea (restamos 1 porque el índice empieza en 0 para legibilidad )
    match tareas.get_mut(numero - 1) {

        // Si encontramos la tarea, modificamos su estado!
        Some(tarea) => {
            tarea.completada = true;
            println!("Tarea completada correctamente");
        }

        // Si no existe, mostramos un mensaje....
        None => {
            println!("No existe una tarea con ese numero!");
        }
    }
}



/* 
PROGRAMA PRINCIPAL MAIN MAIN
*/


/// Es el punto de entrada del programa e inicializa la lista de tareas en memoria y  mantiene activo el menú interactivo hasta que el usuario decida salir
fn main() {

    //Creamos un vector mutable para almacenar las tareas durante la ejecucion 
    let mut tareas: Vec<Tarea> = Vec::new();

    // El menu se ejecuta repetidamente con bucle infinito para que quede activo 
    loop {

        println!("\n                                                   ");
        println!(" GESTOR DE TAREAS!-");
        println!("");
        println!("1. Agregar tarea");
        println!("2. Listar tareas");
        println!("3. Completar tarea");
        println!("4. Salir");
        println!("                                                        ");

        // Solicitamos una opción al usuario
        let opcion = leer_texto("Seleccione una opcion: ");

        // Evaluamos la opción seleccionada, convertimos la entrada para compararla de forma limpia
        match opcion.as_str() {

            // Opción 1: agregar una tarea.
            "1" => {
                agregar_tarea(&mut tareas);
            }

            // Opción 2: mostrar todas las tareas
            "2" => {listar_tareas(&tareas);}

            // Opción 3: completar una tarea
            "3" => {completar_tarea(&mut tareas);}

            // Opción 4: finalizar el programa
            "4" => { println!("\nGracias por utilizar es gestor de tareas!!!");
                break; // sale del loop
            }

            // Cualquier otra entrada es inválida que no sea los del match
            _ => {println!("Opcion no valida. Intente nuevamente!");}
        }
    }
}
