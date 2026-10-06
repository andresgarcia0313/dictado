//! Medidas del entorno por trama: potencia de voz y de ruido (SNR), nivel y piso de ruido.
use super::histeresis::{ENTRA, SALE};

const VOZ_SEGURA: f32 = 0.8; // el SNR se mide solo con tramas claras de voz y de no-voz

const SOBRE_PISO_DB: f32 = 6.0;
const PISO_RUIDO_DB: f32 = -70.0; // ningun micro real baja de aqui: evita SNR inflado
const SNR_MAX_DB: f32 = 40.0;

fn db(pot: f32) -> f32 {
    10.0 * pot.max(1e-10).log10()
}

fn db_a_pot(d: f32) -> f32 {
    10f32.powf(d / 10.0)
}

pub struct Medida {
    pot_voz: f32,
    pot_ruido: f32,
    n_voz: u32,
    n_ruido: u32,
    nivel_db: f32,
    piso_db: Option<f32>,
}

impl Medida {
    pub fn nueva() -> Self {
        Self {
            pot_voz: 0.0,
            pot_ruido: 0.0,
            n_voz: 0,
            n_ruido: 0,
            nivel_db: -100.0,
            piso_db: None,
        }
    }

    /// SNR en dB acotado a 0-40 (voz frente al ruido estimado, con piso de -70 dBFS); `None` sin datos de ambos.
    pub fn snr(&self) -> Option<f32> {
        let ruido = self.pot_ruido.max(db_a_pot(PISO_RUIDO_DB));
        (self.n_voz >= 10 && self.n_ruido >= 20)
            .then(|| db(((self.pot_voz - ruido) / ruido).max(1e-3)).clamp(0.0, SNR_MAX_DB))
    }

    pub fn piso_db(&self) -> f32 {
        self.piso_db.unwrap_or(0.0).max(PISO_RUIDO_DB) // un micro real nunca baja de aqui
    }

    pub fn nivel_db(&self) -> f32 {
        self.nivel_db
    }

    /// Voz si p >= 0.5; ruido si no hay voz pero el nivel supera al piso + 6 dB; si no, silencio.
    pub fn estado(&self, p: f32) -> &'static str {
        if p >= ENTRA {
            "voz"
        } else if self.nivel_db > self.piso_db() + SOBRE_PISO_DB {
            "ruido"
        } else {
            "silencio"
        }
    }

    pub fn trama(&mut self, t: &[f32], p: f32) {
        let pot = t.iter().map(|x| x * x).sum::<f32>() / t.len() as f32;
        if pot == 0.0 {
            return; // el capturador entrega tramas de ceros exactos: no son silencio medible
        }
        if p >= VOZ_SEGURA {
            let a = if self.n_voz == 0 { 1.0 } else { 0.05 };
            self.pot_voz += a * (pot - self.pot_voz);
            self.n_voz += 1;
        } else if p < SALE {
            let a = if self.n_ruido == 0 { 1.0 } else { 0.02 };
            self.pot_ruido += a * (pot - self.pot_ruido);
            self.n_ruido += 1;
        }
        let d = db(pot).max(-100.0);
        self.nivel_db += 0.1 * (d - self.nivel_db);
        let piso = self.piso_db.unwrap_or(d);
        let paso = if d < piso { 0.2 } else { 0.0003 }; // baja rapido, sube en ~1 min
        self.piso_db = Some(piso + paso * (d - piso));
    }

    pub fn conteos(&self) -> (u32, u32) {
        (self.n_voz, self.n_ruido)
    }
}

#[cfg(test)]
#[path = "medida_pruebas.rs"]
mod pruebas;
