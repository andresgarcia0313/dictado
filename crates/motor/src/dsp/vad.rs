//! Detectores de voz por trama de 16 ms (256 muestras a 16 kHz): neuronal (earshot) y por energia.
use crate::audio::rms;
use earshot::Detector;
use std::sync::Once;

pub const TRAMA: usize = 256;
const UMBRAL_MIN: f32 = 0.02;
const FACTOR_RUIDO: f32 = 2.5;

pub trait Vad {
    /// Probabilidad de voz 0-1 de una trama de `TRAMA` muestras.
    fn p(&mut self, trama: &[f32]) -> f32;
}

struct Neuronal(Box<Detector>);

impl Vad for Neuronal {
    fn p(&mut self, trama: &[f32]) -> f32 {
        let mut t = [0.0f32; TRAMA];
        for (d, s) in t.iter_mut().zip(trama) {
            *d = s.clamp(-1.0, 1.0); // earshot exige [-1, 1]
        }
        self.0.predict_f32(&t)
    }
}

/// Respaldo: puerta por energia con piso de ruido adaptativo (p vale 0 o 1).
pub struct Energia {
    ruido: f32,
    activa: bool,
}

impl Energia {
    pub fn nuevo() -> Self {
        Self {
            ruido: UMBRAL_MIN,
            activa: false,
        }
    }
}

impl Vad for Energia {
    fn p(&mut self, trama: &[f32]) -> f32 {
        let r = rms(trama);
        let voz = r >= (self.ruido * FACTOR_RUIDO).max(UMBRAL_MIN);
        let alfa = if voz || self.activa { 0.002 } else { 0.05 };
        self.ruido += alfa * (r - self.ruido);
        self.activa = voz;
        voz as u8 as f32
    }
}

fn neuronal() -> Result<Neuronal, String> {
    let mut d = std::panic::catch_unwind(Detector::default_boxed)
        .map_err(|_| "earshot entro en panico al crearse".to_string())?;
    let p = d.predict_f32(&[0.0; TRAMA]);
    if !(0.0..=1.0).contains(&p) {
        return Err(format!("earshot devolvio p={p} sobre silencio"));
    }
    Ok(Neuronal(d))
}

fn pidio_energia() -> bool {
    std::env::args().any(|a| a == "--vad-energia")
        || std::env::var("DICTADO_VAD").is_ok_and(|v| v == "energia")
}

/// Crea el VAD: earshot salvo `--vad-energia` o si falla (se registra el motivo).
pub fn crear() -> Box<dyn Vad> {
    static AVISO: Once = Once::new();
    let (vad, nombre, motivo): (Box<dyn Vad>, _, _) = if pidio_energia() {
        (Box::new(Energia::nuevo()), "energia", "pedido".to_string())
    } else {
        match neuronal() {
            Ok(n) => (Box::new(n), "earshot", "ok".to_string()),
            Err(e) => (Box::new(Energia::nuevo()), "energia", e),
        }
    };
    AVISO.call_once(|| match motivo.as_str() {
        "ok" | "pedido" => registro::info!("vad={nombre} motivo={motivo}"),
        _ => registro::aviso!("vad respaldo={nombre} motivo={motivo}"),
    });
    vad
}

#[cfg(test)]
#[path = "vad_pruebas.rs"]
mod pruebas;
