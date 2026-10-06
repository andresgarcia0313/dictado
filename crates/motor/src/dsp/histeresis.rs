//! Histeresis del VAD: entra con p >= 0.5, sale tras 400 ms seguidos con p < 0.35.
//! Cuenta las tramas de voz del segmento para descartar los de menos de 250 ms.
pub const ENTRA: f32 = 0.5;
pub const SALE: f32 = 0.35;
pub const MS_TRAMA: usize = 16;
const HANGOVER: usize = 400 / MS_TRAMA;
pub const MIN_VOZ: usize = 250 / MS_TRAMA + 1;

#[derive(Debug, PartialEq)]
pub enum Cambio {
    Nada,
    Entra,
    Sale,
}

#[derive(Default)]
pub struct Histeresis {
    dentro: bool,
    callado: usize,
    voz: usize,
    p_max: f32,
}

impl Histeresis {
    pub fn paso(&mut self, p: f32) -> Cambio {
        if !self.dentro {
            if p < ENTRA {
                return Cambio::Nada;
            }
            *self = Self {
                dentro: true,
                voz: 1,
                p_max: p,
                ..Self::default()
            };
            return Cambio::Entra;
        }
        self.p_max = self.p_max.max(p);
        if p >= SALE {
            (self.callado, self.voz) = (0, self.voz + 1);
            return Cambio::Nada;
        }
        self.callado += 1;
        if self.callado < HANGOVER {
            return Cambio::Nada;
        }
        self.dentro = false;
        Cambio::Sale
    }

    pub fn dentro(&self) -> bool {
        self.dentro
    }

    /// Fuerza la salida (pausa, cambio de microfono, fin de archivo).
    pub fn cerrar(&mut self) {
        self.dentro = false;
    }

    pub fn voz_ms(&self) -> usize {
        self.voz * MS_TRAMA
    }

    pub fn es_valido(&self) -> bool {
        self.voz >= MIN_VOZ
    }

    pub fn p_max(&self) -> f32 {
        self.p_max
    }
}

#[cfg(test)]
#[path = "histeresis_pruebas.rs"]
mod pruebas;
