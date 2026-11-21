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

