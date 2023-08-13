use std::collections::HashMap;
use native_dialog::FileDialog;
use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};
use std::path::Path;
use std::fs;
use fs_extra::dir::{self, CopyOptions};


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    proyectos: HashMap<String, String>,
}

fn main() {
    println!("v0.1.0 -----------------------------");
    println!("1. Create a new project");
    println!("2. Open a project");
    println!("3. Add a project");
    println!("4. Delete a project");
    println!("5. Get Java Ant version for a project");

    let mut option = String::new();

    std::io::stdin().read_line(&mut option).unwrap();

    option = option.trim().to_string();

    match option.as_str() {
        "1" => {
            if let Err(e) = create_project() {
                println!("Error creating project: {}", e);
            }
        }
        "2" => {
            if let Err(e) = open_project() {
                println!("Error creating project: {}", e);
            }
        },
        "3" => {
            if let Err(e) = add_project() {
                println!("Error adding project: {}", e);
            }
        },
        "4" => {
            if let Err(e) = delete_project() {
                println!("Error deleting project: {}", e);
            }
        },
        "5" => {
            if let Err(e) = get_java_ant() {
                println!("Error getting Java Ant: {}", e);
            }
        },
        _   => println!("Opción no válida"),
    }
}

fn create_project() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ingrese el nombre del proyecto:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name)?;
    project_name = project_name.trim().to_string();

    println!("Select a folder to create the project");

    let result = FileDialog::new().show_open_single_dir().unwrap();
    let path = result.ok_or("No directory selected")?;

    // Ruta de la carpeta que quieres copiar
    let source_folder = "assets/VSCodeConfig";

    // Opciones para la copia
    let mut options = CopyOptions::new();
    options.overwrite = true; // Sobrescribir si la carpeta ya existe

    // Copiar la carpeta a la ubicación seleccionada
    dir::copy(source_folder, &path, &options)?;

    // Renombrar la carpeta copiada al nombre del proyecto
    let old_project_folder = path.join("VSCodeConfig");
    let new_project_folder = path.join(&project_name);
    let new_project_folder_clone = new_project_folder.clone(); // Clonar la variable
    fs::rename(old_project_folder, new_project_folder)?;

    // Leer el archivo TOML existente
    let mut file = File::open("assets/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el archivo TOML en una estructura Config
    let mut config: Config = from_str(&contents)?;

    // Agregar el nuevo proyecto a la estructura Config
    let project_path = new_project_folder_clone.to_str().ok_or("Invalid path")?.to_string(); // Utilizar la variable clonada
    config.proyectos.insert(project_name, project_path);


    // Serializar la estructura Config modificada en una cadena TOML
    let toml_str = to_string(&config)?;

    // Escribir la cadena TOML en el archivo
    let mut file = File::create("assets/config.toml")?;
    file.write_all(toml_str.as_bytes())?;

    println!("config.toml file updated successfully!");
    Ok(())
}

fn open_project() -> Result<(), Box<dyn std::error::Error>> {
    // Leer el archivo TOML existente
    let mut file = File::open("assets/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el archivo TOML en una estructura Config
    let config: Config = from_str(&contents)?;

    println!("Proyectos disponibles:");
    for (name, _) in &config.proyectos {
        println!("{}", name);
    }

    println!("Ingrese el nombre del proyecto que desea abrir:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name)?;
    project_name = project_name.trim().to_string();

    // Obtener la dirección del proyecto seleccionado
    if let Some(path) = config.proyectos.get(&project_name) {
        println!("Abriendo el proyecto {} en VS Code...", project_name);

        // Comando para abrir el proyecto con VS Code
        let open_in_vs_code = Command::new("code")
            .arg(path)
            .status()?;

        if open_in_vs_code.success() {
            println!("Proyecto abierto exitosamente en VS Code!");
        } else {
            println!("Ocurrió un error al abrir el proyecto en VS Code.");
        }
    } else {
        println!("Proyecto no encontrado.");
    }

    Ok(())
}

fn add_project() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ingrese el nombre del proyecto:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name)?;
    project_name = project_name.trim().to_string();

    println!("Seleccione un directorio para el proyecto:");

    let result = FileDialog::new().show_open_single_dir().unwrap();
    let path = result.ok_or("No directory selected")?;

    // Leer el archivo TOML existente
    let mut file = File::open("assets/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el archivo TOML en una estructura Config
    let mut config: Config = from_str(&contents)?;

    // Añadir el nuevo proyecto a la estructura Config
    let project_path = path.to_str().ok_or("Invalid path")?.to_string();
    config.proyectos.insert(project_name, project_path);

    // Serializar la estructura Config modificada en una cadena TOML
    let toml_str = to_string(&config)?;

    // Escribir la cadena TOML en el archivo
    let mut file = File::create("assets/config.toml")?;
    file.write_all(toml_str.as_bytes())?;

    println!("Proyecto añadido exitosamente!");
    Ok(())
}

fn delete_project() -> Result<(), Box<dyn std::error::Error>> {
    // Leer el archivo TOML existente
    let mut file = File::open("assets/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el archivo TOML en una estructura Config
    let mut config: Config = from_str(&contents)?;

    println!("Proyectos disponibles:");
    for (name, _) in &config.proyectos {
        println!("{}", name);
    }

    println!("Ingrese el nombre del proyecto que desea eliminar:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name)?;
    project_name = project_name.trim().to_string();

    // Eliminar el proyecto de la estructura Config
    if config.proyectos.remove(&project_name).is_some() {
        // Serializar la estructura Config modificada en una cadena TOML
        let toml_str = to_string(&config)?;

        // Escribir la cadena TOML en el archivo
        let mut file = File::create("assets/config.toml")?;
        file.write_all(toml_str.as_bytes())?;

        println!("Proyecto eliminado exitosamente!");
    } else {
        println!("Proyecto no encontrado.");
    }

    Ok(())
}


fn get_java_ant() -> Result<(), Box<dyn std::error::Error>> {
    // Leer el archivo TOML existente
    let mut file = File::open("assets/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializar el archivo TOML en una estructura Config
    let config: Config = from_str(&contents)?;

    println!("Proyectos disponibles:");
    for (name, _) in &config.proyectos {
        println!("{}", name);
    }

    println!("Ingrese el nombre del proyecto para obtener Java Ant:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name)?;
    project_name = project_name.trim().to_string();

    let project_path = config.proyectos.get(&project_name).ok_or("Proyecto no encontrado")?;

    println!("Seleccione una carpeta para guardar la versión de Java Ant:");
    let result = FileDialog::new().show_open_single_dir().unwrap();
    let target_folder = result.ok_or("No directory selected")?;

    // Copiar la carpeta assets\AntConfig al folder objetivo
    let ant_config_path = "assets/AntConfig";
    let mut options = CopyOptions::new();
    options.overwrite = true;
    dir::copy(ant_config_path, &target_folder, &options)?;

    // Reemplazar la carpeta src en la carpeta AntConfig con la del proyecto original
    let src_path = Path::new(project_path).join("src");
    let target_ant_config_path = target_folder.join("AntConfig");
    let target_src_path = target_ant_config_path.join("src");
    if src_path.exists() && target_src_path.exists() {
        dir::remove(target_src_path)?;
        dir::copy(src_path, target_ant_config_path, &options)?;
    }

    println!("Java Ant obtenido exitosamente!");
    Ok(())
}
