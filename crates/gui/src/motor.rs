//! Proceso hijo del motor STT: lanza `dictado-motor`, lee eventos JSON y lo relanza si cae.
pub use crate::cierre::{detener, enviar};
pub use crate::evento::Evento;
use crate::filtros;
use crate::vida::vivir;
use std::process::{Child, ChildStdin};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering::SeqCst};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

const MAX_REINTENTOS: u32 = 3;
/// Un motor que vive al menos esto reinicia la cuenta de fallos seguidos.
const ESTABLE: Duration = Duration::from_secs(30);

pub(crate) struct Estado {
    pub(crate) stdin: Mutex<Option<ChildStdin>>,
    pub(crate) pid: AtomicU32,
    pub(crate) cerrando: AtomicBool,
}

pub(crate) static ESTADO: OnceLock<Estado> = OnceLock::new();
pub(crate) static HIJO: Mutex<Option<Child>> = Mutex::new(None);

/// PID del motor vivo (0 si no hay).
pub fn pid() -> u32 {
    ESTADO.get().map_or(0, |e| e.pid.load(SeqCst))
}

/// Lanza el hilo supervisor: emite eventos por `emitir` y relanza hasta 3 veces seguidas.
pub fn iniciar(emitir: impl Fn(Evento) + Send + 'static) {
    let estado = Estado {
        stdin: Mutex::new(None),
        pid: AtomicU32::new(0),
        cerrando: AtomicBool::new(false),
    };
    if ESTADO.set(estado).is_err() {
        return;
    }
    thread::spawn(move || {
        let mut fallos = 0;
        loop {
            emitir(Evento::Iniciando);
            match vivir(&emitir) {
                Ok((true, _)) => return emitir(Evento::Fin(true)),
                Ok((false, _)) if filtros::RELANZANDO.swap(false, SeqCst) => fallos = 0,
                Ok((false, vivido)) => fallos = if vivido >= ESTABLE { 1 } else { fallos + 1 },
                Err(msg) => {
                    emitir(Evento::Error(msg));
                    fallos += 1;
                }
            }
            if ESTADO.get().is_some_and(|e| e.cerrando.load(SeqCst)) {
                return;
            }
            if fallos >= MAX_REINTENTOS {
                registro::error!("el motor falló {fallos} veces seguidas; no se relanza más");
                emitir(Evento::Error(
                    "el motor se detuvo varias veces seguidas".into(),
                ));
                return emitir(Evento::Fin(false));
            }
            registro::aviso!(
                "relanzamiento del motor intento={}/{MAX_REINTENTOS}",
                fallos + 1
            );
            thread::sleep(Duration::from_millis(500));
        }
    });
}
