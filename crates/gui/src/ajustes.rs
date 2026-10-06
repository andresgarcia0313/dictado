//! Ajustes de audio: invokables de QML y reenvío al motor cada vez que arranca (`listo`).
use crate::controlador::qobject;
use crate::motor;
use core::pin::Pin;
use cxx_qt_lib::QString;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Mutex;

/// Ya se mandaron los ajustes a esta vida del motor (se reinicia con cada lanzamiento).
static ENVIADOS: AtomicBool = AtomicBool::new(false);

/// Micrófono pedido por el usuario; la propiedad `mic` muestra el que el motor dice usar.
static DESEO: Mutex<String> = Mutex::new(String::new());

fn onoff(activo: bool) -> &'static str {
    if activo {
        "on"
    } else {
        "off"
    }
}

/// El protocolo es de una línea por comando: un nombre no puede traer saltos de línea.
fn limpio(nombre: &str) -> String {
    let n = nombre.replace(['\n', '\r'], " ");
    let n = n.trim();
    if n.is_empty() { "auto" } else { n }.to_string()
}

pub fn nueva_vida() {
    ENVIADOS.store(false, SeqCst);
}

impl qobject::Controlador {
    pub fn fijar_mejora(mut self: Pin<&mut Self>, activo: bool) {
        self.as_mut().set_mejora(activo);
        motor::enviar(&format!("mejora {}", onoff(activo)));
    }

    pub fn fijar_ruido(mut self: Pin<&mut Self>, activo: bool) {
        self.as_mut().set_ruido(activo);
        motor::enviar(&format!("ruido {}", onoff(activo)));
    }

    pub fn elegir_mic(mut self: Pin<&mut Self>, nombre: &QString) {
        let nombre = limpio(&nombre.to_string());
        let previo = std::mem::replace(&mut *DESEO.lock().unwrap(), nombre.clone());
        self.as_mut().set_mic(QString::from(&nombre));
        // Mismo micrófono que ya se pidió: reabrir la captura no aporta nada.
        if limpio(&previo) == nombre {
            return;
        }
        motor::enviar(&format!("mic {nombre}"));
    }

    /// Primer `listo` de cada vida del motor: le reenvía lo último elegido. Cambiar de
    /// micrófono vuelve a emitir `listo`, y no se reenvía para no entrar en bucle.
    pub fn reenviar_ajustes(&self) {
        if ENVIADOS.swap(true, SeqCst) {
            return;
        }
        motor::enviar(&format!("mejora {}", onoff(*self.mejora())));
        motor::enviar(&format!("ruido {}", onoff(*self.ruido())));
        let mic = limpio(&DESEO.lock().unwrap());
        if mic != "auto" {
            motor::enviar(&format!("mic {mic}"));
        }
    }

    /// Solo pruebas: `DICTADO_AJUSTES="mejora=off,ruido=on,mic=Nombre"` simula al usuario.
    /// Se aplica en cada `listo` inicial de una vida del motor (así gana a los valores de QML).
    pub fn ajustes_de_entorno(mut self: Pin<&mut Self>) {
        let Ok(v) = std::env::var("DICTADO_AJUSTES") else {
            return;
        };
        if ENVIADOS.load(SeqCst) {
            return;
        }
        for par in v.split(',') {
            match par.split_once('=') {
                Some(("mejora", x)) => self.as_mut().fijar_mejora(x == "on"),
                Some(("ruido", x)) => self.as_mut().fijar_ruido(x == "on"),
                Some(("saltar", x)) => self.as_mut().fijar_saltar_filtros(x == "on"),
                Some(("mic", x)) => self.as_mut().elegir_mic(&QString::from(x)),
                _ => {}
            }
        }
    }
}
