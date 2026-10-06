//! Eventos del protocolo motor -> GUI: una linea JSON por evento en stdout.
use serde_json::{json, Value};
use std::io::Write;

pub enum Evento<'a> {
    Listo {
        modelo: &'a str,
        dispositivo: &'a str,
    },
    Parcial(&'a str),
    Final(&'a str),
    Nivel(f32),
    /// Trama del espectrograma: BANDAS valores 0-255.
    Espectro(&'a [u8]),
    /// Ganancia que aplica la nivelacion, en dB.
    Ganancia(f32),
    /// Microfonos disponibles y el que se usa.
    Mics {
        lista: &'a [String],
        actual: &'a str,
    },
    /// Entorno acustico: `voz`, `ruido` o `silencio`, con p del VAD y SNR en dB.
    Entorno {
        estado: &'a str,
        p: f32,
        snr: f32,
    },
    Error(&'a str),
    /// Modo lote (`--wav a b c`): empieza un archivo.
    Archivo(&'a str),
}

impl Evento<'_> {
    pub fn json(&self) -> Value {
        match self {
            Evento::Listo {
                modelo,
                dispositivo,
            } => {
                json!({"t":"listo","modelo":modelo,"dispositivo":dispositivo})
            }
            Evento::Parcial(t) => json!({"t":"parcial","texto":t}),
            Evento::Final(t) => json!({"t":"final","texto":t}),
            Evento::Nivel(r) => json!({"t":"nivel","rms":((*r as f64) * 100.0).round() / 100.0}),
            Evento::Espectro(b) => json!({"t":"espectro","b":b}),
            Evento::Ganancia(g) => json!({"t":"ganancia","db":((*g as f64) * 10.0).round() / 10.0}),
            Evento::Mics { lista, actual } => json!({"t":"mics","lista":lista,"actual":actual}),
            Evento::Entorno { estado, p, snr } => json!({"t":"entorno","estado":estado,
                "p":((*p as f64) * 100.0).round() / 100.0,"snr":((*snr as f64) * 10.0).round() / 10.0}),
            Evento::Error(m) => json!({"t":"error","msg":m}),
            Evento::Archivo(r) => json!({"t":"archivo","ruta":r}),
        }
    }

    /// Escribe y vacia el buffer; si la GUI cerro la tuberia, el motor termina.
    pub fn emitir(&self) {
        let mut out = std::io::stdout().lock();
        if writeln!(out, "{}", self.json())
            .and_then(|_| out.flush())
            .is_err()
        {
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn formatos_del_protocolo() {
        assert_eq!(
            Evento::Parcial("hola").json(),
            json!({"t":"parcial","texto":"hola"})
        );
        assert_eq!(
            Evento::Nivel(0.4213).json(),
            json!({"t":"nivel","rms":0.42})
        );
        assert_eq!(
            Evento::Entorno {
                estado: "voz",
                p: 0.834,
                snr: 12.34
            }
            .json(),
            json!({"t":"entorno","estado":"voz","p":0.83,"snr":12.3})
        );
        let l = Evento::Listo {
            modelo: "es-kroko",
            dispositivo: "mic",
        }
        .json();
        assert_eq!(l["dispositivo"], "mic");
    }
}
