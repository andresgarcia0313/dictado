//! Filtros del sistema y vocabulario: relanza el motor con `--con-filtros` y abre el glosario.
use crate::controlador::qobject;
use crate::motor::HIJO;
use crate::nativo;
use core::pin::Pin;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

/// El usuario apagó "Saltar filtros del sistema": el motor se lanza con `--con-filtros`.
pub static CON_FILTROS: AtomicBool = AtomicBool::new(false);
/// El cierre en curso lo pidió la GUI para relanzar: no cuenta como fallo del motor.
pub static RELANZANDO: AtomicBool = AtomicBool::new(false);

/// Glosario del usuario (mismo sitio que usa el motor).
pub fn ruta_vocabulario() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_default()
        .join("dictado")
        .join("vocabulario.txt")
}

fn relanzar_motor() {
    RELANZANDO.store(true, SeqCst);
    let matado = HIJO
        .lock()
        .unwrap()
        .as_mut()
        .is_some_and(|h| h.kill().is_ok());
    if !matado {
        RELANZANDO.store(false, SeqCst);
    }
}

impl qobject::Controlador {
    /// ON (por defecto) = el motor se conecta directo al micrófono; OFF = relanza con `--con-filtros`.
    pub fn fijar_saltar_filtros(mut self: Pin<&mut Self>, activo: bool) {
        if *self.saltar_filtros() == activo {
            return;
        }
        self.as_mut().set_saltar_filtros(activo);
        CON_FILTROS.store(!activo, SeqCst);
        registro::info!("se relanza el motor saltar_filtros={activo}");
        relanzar_motor();
    }

    pub fn abrir_vocabulario(&self) {
        let ruta = ruta_vocabulario();
        registro::info!("se abre el vocabulario");
        // Si aún no existe (el motor lo crea al arrancar), se abre su carpeta.
        let destino = if ruta.exists() {
            ruta
        } else {
            ruta.parent().map(PathBuf::from).unwrap_or_default()
        };
        nativo::abrir_carpeta(&destino.display().to_string());
    }
}
