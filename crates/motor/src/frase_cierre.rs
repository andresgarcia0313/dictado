//! Cierre de la frase: emite el final (o la descarta) y reinicia el reconocedor.
use super::Frase;
use crate::evento::Evento;
use crate::reconocedor::Reconocedor;
use std::time::Instant;

impl Frase {
    /// Cierra la frase en curso (si la hay) y emite el final.
    pub fn cerrar(&mut self, rec: &mut dyn Reconocedor) {
        if self.hist.dentro() {
            self.hist.cerrar();
            self.finalizar(rec);
        }
    }

    pub(super) fn finalizar(&mut self, rec: &mut dyn Reconocedor) {
        let t0 = Instant::now();
        let t = crate::vocabulario::aplicar(&limpiar(&rec.cerrar()));
        let (voz_ms, p_max) = (self.hist.voz_ms(), self.hist.p_max());
        registro::info!("voz fin duracion_ms={voz_ms} p_max={p_max:.2}");
        if t.is_empty() || !self.hist.es_valido() {
            let motivo = if t.is_empty() {
                "sin_texto"
            } else {
                "voz_corta"
            };
            registro::aviso!("frase descartada motivo={motivo} duracion_ms={voz_ms}");
            if !self.ultimo.is_empty() {
                Evento::Parcial("").emitir();
            }
        } else {
            Evento::Final(&super::capitalizar(&t)).emitir();
            let palabras = t.split_whitespace().count();
            registro::info!(
                "final palabras={palabras} decodificacion_ms={}",
                t0.elapsed().as_millis()
            );
        }
        self.ultimo.clear();
    }
}

/// Quita puntuacion suelta al inicio (el modelo a veces abre con ". " o ", ").
pub fn limpiar(t: &str) -> String {
    t.trim_start_matches(|c: char| c.is_whitespace() || ".,;:".contains(c))
        .to_string()
}
