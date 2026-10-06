//! Bucle principal: fuente -> mejora -> espectro -> frase/reconocedor, atendiendo las ordenes.
use crate::conexion::{reabrir, Abrir, Ajustes, Pausada};
use crate::dsp::Mejora;
use crate::fin::{registrar, salir};
use crate::frase::Frase;
use crate::fuente::Lectura;
use crate::orden::Orden;
use crate::reconocedor::Reconocedor;
use crate::resumen::Resumen;
use crate::telemetria::Telemetria;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

/// Devuelve el codigo de salida del proceso.
pub fn correr(
    abrir: &mut Abrir,
    rec: &mut dyn Reconocedor,
    ordenes: Receiver<Orden>,
    aj: Ajustes,
) -> i32 {
    let mejora = Mejora::nueva(aj.mejora, aj.ruido);
    ciclo(abrir, rec, ordenes, aj, mejora)
}

fn ciclo(
    abrir: &mut Abrir,
    rec: &mut dyn Reconocedor,
    ordenes: Receiver<Orden>,
    aj: Ajustes,
    mut mejora: Mejora,
) -> i32 {
    let mut elegido: Option<String> = None;
    let Some(mut fuente) = reabrir(abrir, &mut elegido, &aj) else {
        return registrar(3, "no se pudo abrir la fuente al arrancar");
    };
    let mut fuente_viva = true;
    let mut frase = Frase::nueva();
    let mut tele = Telemetria::nueva();
    let mut resumen = Resumen::nuevo();
    loop {
        let orden = if fuente_viva {
            match ordenes.try_recv() {
                Ok(o) => Some(o),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => Some(Orden::Salir),
            }
        } else {
            Some(ordenes.recv().unwrap_or(Orden::Salir)) // pausado: bloquea sin gastar CPU
        };
        match orden {
            Some(Orden::Salir) => return salir(fuente),
            Some(Orden::Pausa) if fuente_viva => {
                frase.cerrar(rec);
                fuente = Box::new(Pausada); // suelta el microfono
                fuente_viva = false;
            }
            Some(Orden::Reanuda) if !fuente_viva => {
                mejora.reiniciar();
                let Some(f) = reabrir(abrir, &mut elegido, &aj) else {
                    return registrar(3, "no se pudo reabrir la fuente al reanudar");
                };
                (fuente, fuente_viva) = (f, true);
            }
            Some(Orden::Mejora(on)) => mejora.fijar_mejora(on),
            Some(Orden::Ruido(on)) => mejora.fijar_ruido(on),
            Some(Orden::Mic(nombre)) => {
                registro::info!("cambio de microfono pedido={nombre:?} vivo={fuente_viva}");
                elegido = nombre; // en pausa se aplica al reanudar
                if fuente_viva {
                    frase.cerrar(rec);
                    drop(std::mem::replace(&mut fuente, Box::new(Pausada))); // libera el actual antes de abrir el nuevo
                    mejora.reiniciar();
                    let Some(f) = reabrir(abrir, &mut elegido, &aj) else {
                        return registrar(3, "no se pudo abrir el microfono elegido");
                    };
                    fuente = f;
                }
            }
            Some(_) => {}
            None => match fuente.leer() {
                Lectura::Muestras(m) => {
                    let mejorado = mejora.procesar(&m);
                    tele.alimentar(&mejorado, mejora.ganancia_db());
                    resumen.anotar(&mejorado, mejora.ganancia_db());
                    frase.procesar(rec, &mejorado, mejora.ps());
                    std::thread::sleep(Duration::from_millis(fuente.ritmo_ms()));
                }
                Lectura::Vacio => {}
                Lectura::Fin => {
                    frase.cerrar(rec);
                    return registrar(0, "fin del audio");
                }
            },
        }
    }
}

#[cfg(test)]
#[path = "bucle_pruebas.rs"]
mod pruebas;
