//! Utilidades de audio puras: nivel RMS y remuestreo sinc a 16 kHz.
pub const FS: u32 = 16_000;

pub fn rms(m: &[f32]) -> f32 {
    if m.is_empty() {
        return 0.0;
    }
    (m.iter().map(|x| x * x).sum::<f32>() / m.len() as f32).sqrt()
}

/// Nivel 0-1 para el orbe (la voz normal ronda 0.05-0.15 de RMS).
pub fn nivel(m: &[f32]) -> f32 {
    (rms(m) * 6.0).min(1.0)
}

/// Remuestreador sinc con estado (filtro de Kaldi, corte 0,99 x Nyquist): sin pliegue de 8-24 kHz.
pub struct Remuestreador {
    sinc: Option<sherpa_onnx::LinearResampler>,
}

impl Remuestreador {
    pub fn nuevo(origen: u32) -> Self {
        let sinc = (origen != FS)
            .then(|| sherpa_onnx::LinearResampler::create(origen as i32, FS as i32))
            .flatten();
        Self { sinc }
    }

    pub fn procesar(&mut self, entrada: &[f32]) -> Vec<f32> {
        match &self.sinc {
            Some(r) => r.resample(entrada, false),
            None => entrada.to_vec(),
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn mismo_ritmo_no_cambia() {
        let mut r = Remuestreador::nuevo(16_000);
        assert_eq!(r.procesar(&[0.1, 0.2]), vec![0.1, 0.2]);
    }

    #[test]
    fn de_48k_a_16k_tercio_de_muestras() {
        let mut r = Remuestreador::nuevo(48_000);
        let n: usize = (0..10).map(|_| r.procesar(&[0.5; 480]).len()).sum();
        assert!((1590..=1602).contains(&n), "{n}");
    }

    #[test]
    fn rms_de_onda_cuadrada() {
        assert!((rms(&[0.5, -0.5]) - 0.5).abs() < 1e-6);
    }

    fn energia(x: &[f32]) -> f32 {
        rms(&x[x.len() / 4..])
    }

    #[test]
    fn el_tono_de_20_khz_no_se_pliega() {
        let tono = |hz: f32| -> Vec<f32> {
            (0..48_000)
                .map(|i| (2.0 * std::f32::consts::PI * hz * i as f32 / 48_000.0).sin() * 0.5)
                .collect()
        };
        let (mut a, mut b) = (Remuestreador::nuevo(48_000), Remuestreador::nuevo(48_000));
        let (voz, alto) = (
            energia(&a.procesar(&tono(1000.0))),
            energia(&b.procesar(&tono(20_000.0))),
        );
        assert!(voz > 0.3, "{voz}");
        assert!(alto < voz * 0.03, "se pliega: {alto} frente a {voz}");
    }
}
