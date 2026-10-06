//! Pasa-altos Butterworth de 2.º orden (biquad propio, forma transpuesta II).
use std::f32::consts::{PI, SQRT_2};

pub struct Pasaaltos {
    b: [f32; 3],
    a: [f32; 2],
    z: [f32; 2],
}

impl Pasaaltos {
    pub fn nuevo(corte_hz: f32, fs: f32) -> Self {
        let w = 2.0 * PI * corte_hz / fs;
        let alfa = w.sin() / SQRT_2; // 2*Q con Q = 1/sqrt(2)
        let a0 = 1.0 + alfa;
        let b0 = (1.0 + w.cos()) / 2.0 / a0;
        Self {
            b: [b0, -2.0 * b0, b0],
            a: [-2.0 * w.cos() / a0, (1.0 - alfa) / a0],
            z: [0.0; 2],
        }
    }

    pub fn reiniciar(&mut self) {
        self.z = [0.0; 2];
    }

    pub fn procesar(&mut self, m: &mut [f32]) {
        for x in m {
            let y = self.b[0] * *x + self.z[0];
            self.z[0] = self.b[1] * *x - self.a[0] * y + self.z[1];
            self.z[1] = self.b[2] * *x - self.a[1] * y;
            *x = y;
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::audio::rms;

    fn ganancia_db(hz: f32) -> f32 {
        let mut f = Pasaaltos::nuevo(80.0, 16_000.0);
        let mut s: Vec<f32> = (0..32_000)
            .map(|n| (2.0 * PI * hz * n as f32 / 16_000.0).sin())
            .collect();
        let antes = rms(&s[16_000..]);
        f.procesar(&mut s);
        20.0 * (rms(&s[16_000..]) / antes).log10()
    }

    #[test]
    fn corta_graves_y_respeta_la_voz() {
        assert!(ganancia_db(10.0) < -33.0);
        assert!((ganancia_db(80.0) + 3.0).abs() < 0.3, "-3 dB en el corte");
        assert!(ganancia_db(1000.0).abs() < 0.1);
    }

    #[test]
    fn quita_la_continua() {
        let mut f = Pasaaltos::nuevo(80.0, 16_000.0);
        let mut s = vec![0.5; 16_000];
        f.procesar(&mut s);
        assert!(s[15_999].abs() < 1e-3);
    }
}
