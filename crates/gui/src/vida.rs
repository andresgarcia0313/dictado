//! Una vida del proceso motor: lanzarlo, leer sus eventos hasta que cierre y dar cuenta de la salida.
use crate::evento::{interpretar, ruta_motor, Evento};
use crate::filtros;
use crate::motor::{ESTADO, HIJO};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering::SeqCst;
use std::time::{Duration, Instant};

/// Una vida del motor: devuelve (salida limpia, tiempo que vivió).
pub(crate) fn vivir(emitir: &(impl Fn(Evento) + ?Sized)) -> Result<(bool, Duration), String> {
    let estado = ESTADO.get().expect("estado inicializado");
    let ruta = ruta_motor();
    let mut cmd = Command::new(&ruta);
    let con_filtros = filtros::CON_FILTROS.load(SeqCst);
    if con_filtros {
        cmd.arg("--con-filtros");
    }
    let mut hijo = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            registro::error!(
                "no se pudo lanzar el motor ruta=\"{}\" motivo={e}",
                ruta.display()
            );
            format!("no se pudo lanzar el motor: {e}")
        })?;
    estado.pid.store(hijo.id(), SeqCst);
    registro::info!(
        "motor lanzado pid={} ruta=\"{}\" con_filtros={con_filtros}",
        hijo.id(),
        ruta.display()
    );
    *estado.stdin.lock().unwrap() = hijo.stdin.take();
    let salida = hijo.stdout.take().expect("stdout del motor");
    *HIJO.lock().unwrap() = Some(hijo);
    let inicio = Instant::now();
    for linea in BufReader::new(salida).lines().map_while(Result::ok) {
        if let Some(ev) = interpretar(&linea) {
            crate::diario::evento(&ev);
            emitir(ev);
        }
    }
    *estado.stdin.lock().unwrap() = None;
    let hijo = HIJO.lock().unwrap().take();
    let salida = hijo.map(|mut h| h.wait());
    let limpio = salida
        .as_ref()
        .is_none_or(|s| s.as_ref().is_ok_and(|s| s.success()));
    estado.pid.store(0, SeqCst);
    let vivido = inicio.elapsed();
    crate::diario::salida_motor(&salida, vivido, estado.cerrando.load(SeqCst));
    Ok((limpio, vivido))
}
