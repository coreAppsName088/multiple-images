## explicacion

este code abre imagenes en windows
pero con la particularidad de que puede abrir con el visor antiguo o moderno de windows 10/11

con la ayuda de quitar 

```
 let _ = Command::new("powershell")
            .args(["-Command", &format!("Unblock-File -Path '{}'", imagen)])
            .output();
```