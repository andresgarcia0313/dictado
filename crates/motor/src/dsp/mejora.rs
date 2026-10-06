//! Mejora de audio: pasa-altos 80 Hz -> analisis de entorno (VAD) -> compuerta (opcional) -> AGC.
use super::{agc::Agc, entorno::Entorno, histeresis::ENTRA, pasaaltos::Pasaaltos, ruido::Ruido};

pub struct Mejora {
    activa: bool,
    pasaaltos: Pasaaltos,
    agc: Agc,
    entorno: Entorno,
    ruido: Ruido,
    registrada_db: f32,
    ps: Vec<f32>,
}

impl Mejora {
    pub fn nueva(activa: bool, reducir_ruido: bool) -> Self {
        Self::armar(activa, reducir_ruido, Entorno::nuevo())
    }

    /// Con un VAD dado (pruebas del bucle).
    #[cfg(test)]
    pub fn con_vad(activa: bool, reducir_ruido: bool, vad: Box<dyn super::Vad>) -> Self {
        Self::armar(activa, reducir_ruido, Entorno::con_vad(vad))
    }

    fn armar(activa: bool, reducir_ruido: bool, entorno: Entorno) -> Self {
        Self {
            activa,
            pasaaltos: Pasaaltos::nuevo(80.0, crate::audio::FS as f32),
            agc: Agc::nuevo(),
            entorno,
            ruido: Ruido::nuevo(reducir_ruido),
            registrada_db: 0.0,
            ps: vec![],
        }
    }

    pub fn fijar_mejora(&mut self, on: bool) {
        if on && !self.activa {
            self.reiniciar();
        }
        self.activa = on;
    }

    pub fn fijar_ruido(&mut self, on: bool) {
        self.ruido.fijar(on);
    }

    /// Olvida lo aprendido (al cambiar de microfono el nivel anterior ya no vale).
    pub fn reiniciar(&mut self) {
        self.pasaaltos.reiniciar();
        self.agc = Agc::nuevo();
        self.entorno.reiniciar();
        self.ruido.reiniciar();
    }

    pub fn ganancia_db(&self) -> f32 {
        if self.activa {
            self.agc.ganancia_db()
        } else {
            0.0
        }
    }

    /// Probabilidad de voz (0-1) de cada trama de 16 ms completada en el ultimo bloque: la unica
    /// decision de voz del motor; `Frase` la usa con su histeresis.
    pub fn ps(&self) -> &[f32] {
        &self.ps
    }

    /// pasa-altos -> VAD/entorno -> compuerta (opcional) -> AGC: el AGC solo sube la voz que el VAD confirma.
    pub fn procesar(&mut self, entrada: &[f32]) -> Vec<f32> {
        let mut m = entrada.to_vec();
        if self.activa {
            self.pasaaltos.procesar(&mut m);
        }
        self.ps.clear();
        let (ruido, ps) = (&mut self.ruido, &mut self.ps);
        self.entorno.analizar(&m, |t, p, snr| {
            ps.push(p);
            ruido.trama(t, p, snr);
        });
        self.ruido.aplicar(&mut m);
        if self.activa {
            let voz = self.ps.iter().sum::<f32>() >= ENTRA * self.ps.len().max(1) as f32; // p media
            self.agc.procesar(&mut m, voz);
            self.registrar_ganancia();
        }
        m
    }

    fn registrar_ganancia(&mut self) {
        let g = self.agc.ganancia_db();
        if (g - self.registrada_db).abs() >= 3.0 {
            registro::info!("agc ganancia_db={g:.1} antes_db={:.1}", self.registrada_db);
            self.registrada_db = g;
        }
    }
}
