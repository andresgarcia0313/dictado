//! dictado-motor: captura voz y emite eventos JSON por stdout (ver escenario.md).
mod args;
mod audio;
mod bucle;
mod canal;
mod canal_regla;
mod conexion;
mod dispositivos;
mod dsp;
mod espectro;
mod evento;
mod fin;
mod fonetica;
mod frase;
mod fuente;
mod lote;
mod mic;
mod mic_cpal;
mod orden;
mod pipewire;
mod reconocedor;
mod resumen;
mod sherpa;
mod sherpa_config;
mod telemetria;
mod vocabulario;
mod wav;

use args::{argumento, dir_modelo, lista};
use evento::Evento;
use fuente::Fuente;
use std::io::BufRead;
use std::sync::mpsc::channel;

fn fallar(msg: &str, codigo: i32) -> ! {
    registro::error!("fin codigo={codigo} motivo={msg}");
    Evento::Error(msg).emitir();
    std::process::exit(codigo)
}

fn main() {
    registro::iniciar("motor");
    vocabulario::aplicar_parcial(""); // crea la semilla del vocabulario si no existe
    let args: Vec<String> = std::env::args().collect();
    let wavs = lista(&args, "--wav");
    let wav = wavs.first().cloned();
    if wav.is_none() {
        pipewire::fijar_directo(args.iter().any(|a| a == "--con-filtros"));
    }
    registro::info!(
        "arranque version={} modo={} args={}",
        env!("CARGO_PKG_VERSION"),
        if wav.is_some() { "wav" } else { "microfono" },
        args.len() - 1
    );
    let Some(dir) = dir_modelo(argumento(&args, "--modelo")) else {
        fallar(
            "no se encontro el modelo es-kroko (use --modelo o DICTADO_MODELO)",
            2,
        )
    };
    let mut rec = sherpa::Sherpa::cargar(&dir).unwrap_or_else(|e| fallar(&e, 2));
    let (tx, rx) = channel();
    let mut abrir: Box<conexion::Abrir> =
        Box::new(|nombre| Ok(Box::new(mic::Mic::abrir(nombre)?) as Box<dyn Fuente>));
    if wav.is_none() {
        std::thread::spawn(move || {
            for l in std::io::stdin().lock().lines().map_while(Result::ok) {
                let corta: String = l.trim().chars().take(80).collect();
                match orden::parsear(&l) {
                    Some(o) => {
                        registro::info!("orden recibida=\"{corta}\"");
                        if matches!(o, orden::Orden::Salir) {
                            fin::registrar(0, "orden salir recibida");
                        }
                        let _ = tx.send(o);
                    }
                    None => registro::aviso!("orden desconocida=\"{corta}\""),
                }
            }
            registro::info!("stdin cerrado: la GUI cerro la tuberia");
            fin::registrar(0, "tuberia cerrada");
            let _ = tx.send(orden::Orden::Salir);
        });
    }
    let modelo = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ajustes = conexion::Ajustes {
        modelo: &modelo,
        mejora: !args.iter().any(|a| a == "--sin-mejora"),
        ruido: args.iter().any(|a| a == "--ruido"),
        mics: wav.is_none(),
    };
    if wavs.is_empty() {
        std::process::exit(bucle::correr(&mut *abrir, &mut rec, rx, ajustes));
    }
    std::process::exit(lote::correr(&wavs, &mut rec, ajustes));
}
