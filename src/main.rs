use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs;

fn main() {
    let mut imagenes: Vec<String> = Vec::new();

    for entrada in fs::read_dir("img").expect("No se pudo leer la carpeta") {
        let entrada = entrada.expect("Error al leer una entrada");
        let ruta = entrada.path();

        let Some(extension) = ruta.extension() else {
            continue;
        };

        // Convertimos a String en minúsculas para comparar bien
        let ext = extension.to_string_lossy().to_lowercase();

        if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif" || ext == "bmp" {
            println!("Imagen encontrada: {:?}", ruta);
            // Convertimos a ruta absoluta
            let mut ruta_absoluta = fs::canonicalize(&ruta)
                .expect("No se pudo obtener la ruta absoluta")
                .to_string_lossy()
                .to_string();

            // Quitar el prefijo \\?\
            if ruta_absoluta.starts_with(r"\\?\") {
                ruta_absoluta = ruta_absoluta[4..].to_string();
            }
            
            imagenes.push(ruta_absoluta);
        }
    }

    println!("Total de imágenes encontradas: {}", imagenes.len());

   let cantidad_por_grupo = 2; // cambia este número (2, 3, 4...)

    for chunk in imagenes.chunks(cantidad_por_grupo) {
        for imagen in chunk {
            println!("Abriendo: {}", imagen);

            // Quitar la advertencia
            let _ = Command::new("powershell")
                .args(["-Command", &format!("Unblock-File -Path '{}'", imagen)])
                .output();

            // Abrir la imagen
            let _ = Command::new("cmd")
                .args(["/C", "start", "", imagen])
                .spawn();
        }

        // Espera 5 segundos antes del siguiente grupo
        thread::sleep(Duration::from_secs(5));
    }

    println!("Terminado.");
}