//! Apertura de la fuente de audio (con vuelta al microfono por defecto) y marcador de pausa.
use crate::evento::Evento;
use crate::fuente::{Fuente, Lectura};

pub type Abrir<'a> = dyn FnMut(Option<&str>) -> Result<Box<dyn Fuente>, String> + 'a;

#[derive(Clone, Copy)]
pub struct Ajustes<'a> {
    pub modelo: &'a str,
    pub mejora: bool,
    pub ruido: bool,
    /// Anunciar los microfonos tras cada `listo` (no aplica con --wav).
    pub mics: bool,
}

pub fn conectar(abrir: &mut Abrir, mic: Option<&str>, aj: &Ajustes) -> Option<Box<dyn Fuente>> {
    match abrir(mic) {
        Ok(f) => {
            let dispositivo = f.dispositivo();
            registro::info!("fuente abierta pedido={mic:?} dispositivo=\"{dispositivo}\"");
            Evento::Listo {
                modelo: aj.modelo,
                dispositivo: &dispositivo,
            }
            .emitir();
            if aj.mics {
                let lista = crate::dispositivos::listar();
                registro::info!("microfonos n={} lista={lista:?}", lista.len());
                Evento::Mics {
                    lista: &lista,
                    actual: &dispositivo,
                }
                .emitir();
            }
            Some(f)
        }
        Err(e) => {
            registro::error!("no se pudo abrir la fuente pedido={mic:?} motivo={e}");
            Evento::Error(&e).emitir();
            None
        }
    }
}

/// Abre el microfono elegido; si ese falla vuelve al de por defecto.
pub fn reabrir(
    abrir: &mut Abrir,
    elegido: &mut Option<String>,
    aj: &Ajustes,
) -> Option<Box<dyn Fuente>> {
    conectar(abrir, elegido.as_deref(), aj).or_else(|| {
        let era_auto = elegido.take().is_none();
        if era_auto {
            None
        } else {
            registro::aviso!("el microfono elegido fallo; se vuelve al de por defecto");
            conectar(abrir, None, aj)
        }
    })
}

/// Marcador inerte que ocupa el lugar de la fuente mientras esta en pausa.
pub struct Pausada;

impl Fuente for Pausada {
    fn leer(&mut self) -> Lectura {
        Lectura::Vacio
    }
    fn dispositivo(&self) -> String {
        String::new()
    }
}
