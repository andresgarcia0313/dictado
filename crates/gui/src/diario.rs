//! Lo que la GUI deja en su registro sobre el motor. Nunca el texto dictado: solo conteos.
use crate::evento::Evento;
use std::io;
use std::process::ExitStatus;
use std::sync::Mutex;
use std::time::Duration;

static ENTORNO: Mutex<String> = Mutex::new(String::new());

/// Eventos del motor que merecen una línea (errores, arranque, micrófonos, cambios de entorno).
pub fn evento(ev: &Evento) {
    match ev {
        Evento::Error(m) => registro::error!("evento error del motor msg=\"{m}\""),
        Evento::Listo { modelo } => registro::info!("motor listo modelo={modelo}"),
        Evento::Mics { lista, actual } => registro::info!(
            "microfonos n={} actual=\"{actual}\"",
            lista.split('|').filter(|x| !x.is_empty()).count()
        ),
        Evento::Entorno { estado, snr } => {
            let mut previo = ENTORNO.lock().unwrap();
            if *previo != *estado {
                registro::depura!("entorno de={} a={estado} snr_db={snr:.1}", *previo);
                previo.clone_from(estado);
            }
        }
        Evento::Final(t) => registro::depura!("final recibido caracteres={}", t.chars().count()),
        _ => {}
    }
}

/// Un comando enviado al motor y si llegó a su entrada estándar.
pub fn comando(texto: &str, entregado: bool) {
    if entregado {
        registro::info!("comando enviado=\"{texto}\"");
    } else {
        registro::aviso!("comando descartado (motor sin entrada) comando=\"{texto}\"");
    }
}

/// Cómo terminó una vida del motor: código, motivo y cuánto vivió.
pub fn salida_motor(estado: &Option<io::Result<ExitStatus>>, vivio: Duration, pedida: bool) {
    let (codigo, limpio) = match estado {
        Some(Ok(s)) => (
            s.code()
                .map_or_else(|| format!("sin_codigo({s})"), |c| c.to_string()),
            s.success(),
        ),
        Some(Err(e)) => (format!("desconocido({e})"), false),
        None => ("ninguno".into(), true),
    };
    let motivo = match (limpio, pedida) {
        (true, _) => "salida limpia",
        (false, true) => "cierre pedido por la GUI",
        (false, false) => "fallo del motor",
    };
    let vivio = vivio.as_secs_f32();
    if limpio || pedida {
        registro::info!("motor termino codigo={codigo} motivo=\"{motivo}\" vivio_s={vivio:.1}");
    } else {
        registro::error!("motor termino codigo={codigo} motivo=\"{motivo}\" vivio_s={vivio:.1}");
    }
}
