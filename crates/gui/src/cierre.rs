//! Ordenes al motor y cierre ordenado del proceso hijo.
use crate::motor::{ESTADO, HIJO};
use std::io::Write;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

/// Espera máxima al motor tras `salir` (décimas de segundo): termina el bloque en curso.
const ESPERA_DECIMAS: u32 = 50;

/// Envía un comando de una palabra (`pausa`, `reanuda`, `salir`) al motor.
pub fn enviar(comando: &str) {
    let Some(e) = ESTADO.get() else { return };
    let mut stdin = e.stdin.lock().unwrap();
    let entregado = stdin
        .as_mut()
        .is_some_and(|s| writeln!(s, "{comando}").and_then(|_| s.flush()).is_ok());
    crate::diario::comando(comando, entregado);
}

/// Cierre ordenado: pide `salir`, espera hasta 5 s y mata al hijo si sigue vivo.
pub fn detener() {
    let Some(e) = ESTADO.get() else { return };
    e.cerrando.store(true, SeqCst);
    registro::info!("cierre: se pide salir al motor pid={}", e.pid.load(SeqCst));
    enviar("salir");
    for _ in 0..ESPERA_DECIMAS {
        let vivo = HIJO
            .lock()
            .unwrap()
            .as_mut()
            .is_some_and(|h| matches!(h.try_wait(), Ok(None)));
        if !vivo {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
    if let Some(h) = HIJO.lock().unwrap().as_mut() {
        registro::aviso!("el motor no salió en 5 s; se fuerza el cierre");
        let _ = h.kill();
        let _ = h.wait();
    }
}
