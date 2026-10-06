//! Eventos del protocolo motor -> GUI (una línea JSON por evento) y localización del ejecutable.
use serde_json::Value;
use std::{env, path::PathBuf};

pub enum Evento {
    Iniciando,
    Listo {
        modelo: String,
    },
    Parcial(String),
    Final(String),
    Nivel(f64),
    Error(String),
    /// 32 bandas (0-255) como CSV, listo para QML.
    Espectro(String),
    Ganancia(f64),
    Mics {
        lista: String,
        actual: String,
    },
    /// Qué oye el micrófono: `voz`, `ruido` o `silencio`, con la relación señal/ruido en dB.
    Entorno {
        estado: String,
        snr: f64,
    },
    /// El motor terminó y no se relanzará; `true` si fue salida limpia.
    Fin(bool),
}

pub fn ruta_motor() -> PathBuf {
    if let Some(r) = env::var_os("DICTADO_MOTOR") {
        return r.into();
    }
    let nombre = format!("dictado-motor{}", env::consts::EXE_SUFFIX);
    env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(|p| p.join(&nombre)))
        .unwrap_or_else(|| nombre.into())
}

pub fn interpretar(linea: &str) -> Option<Evento> {
    let v: Value = serde_json::from_str(linea).ok()?;
    let texto = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    Some(match v.get("t")?.as_str()? {
        "listo" => Evento::Listo {
            modelo: texto("modelo"),
        },
        "parcial" => Evento::Parcial(texto("texto")),
        "final" => Evento::Final(texto("texto")),
        "nivel" => Evento::Nivel(v.get("rms").and_then(Value::as_f64).unwrap_or(0.0)),
        "error" => Evento::Error(texto("msg")),
        "espectro" => Evento::Espectro(
            v.get("b")?
                .as_array()?
                .iter()
                .map(|x| x.as_u64().unwrap_or(0).min(255).to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
        "entorno" => Evento::Entorno {
            estado: texto("estado"),
            snr: v.get("snr").and_then(Value::as_f64).unwrap_or(0.0),
        },
        "ganancia" => Evento::Ganancia(v.get("db").and_then(Value::as_f64).unwrap_or(0.0)),
        "mics" => Evento::Mics {
            lista: v
                .get("lista")
                .and_then(Value::as_array)
                .map(|l| {
                    l.iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("|")
                })
                .unwrap_or_default(),
            actual: texto("actual"),
        },
        _ => return None,
    })
}
