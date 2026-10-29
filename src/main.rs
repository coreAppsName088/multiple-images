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

    // Abre cada imagen cada 5 segundos
    for (i, imagen) in imagenes.iter().enumerate() {
        println!("Abriendo imagen #{}: {}", i + 1, imagen);


        let _ = Command::new("powershell")
            .args(["-Command", &format!("Unblock-File -Path '{}'", imagen)])
            .output();
        
        let _resultado = if i % 2 == 0 {
            // Visor antiguo
            Command::new("rundll32.exe")
                .arg(r"C:\Windows\System32\shimgvw.dll,ImageView_Fullscreen")
                .arg(imagen)
                .spawn()
        } else {
            // Visor nuevo (predeterminado)
            Command::new("cmd")
                .args(["/C", "start", "", imagen])
                .spawn()
        };



        thread::sleep(Duration::from_secs(5));
    }

    println!("Terminado.");
}