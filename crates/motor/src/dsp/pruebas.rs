//! Pruebas del AGC y de la cadena completa.
use super::agc::Agc;
use crate::audio::rms;
use std::f32::consts::PI;

pub(super) fn db(x: f32) -> f32 {
    20.0 * x.max(1e-9).log10()
}

/// Seno de 300 Hz con el RMS dado en dBFS, por bloques de 100 ms.
pub(super) fn bloque(nivel_db: f32, n0: usize) -> Vec<f32> {
    let a = 10f32.powf(nivel_db / 20.0) * 2f32.sqrt();
    (n0..n0 + 1600)
        .map(|n| a * (2.0 * PI * 300.0 * n as f32 / 16_000.0).sin())
        .collect()
}

fn correr(agc: &mut Agc, nivel_db: f32, segundos: usize) -> Vec<f32> {
    let mut ultimo = vec![];
    for i in 0..segundos * 10 {
        ultimo = bloque(nivel_db, i * 1600);
        agc.procesar(&mut ultimo, true);
    }
    ultimo
}

#[test]
fn voz_baja_sube_hacia_menos_20() {
    let mut a = Agc::nuevo();
    let sal = correr(&mut a, -40.0, 12);
    assert!((db(rms(&sal)) + 20.0).abs() < 1.5, "{}", db(rms(&sal)));
    assert!((a.ganancia_db() - 20.0).abs() < 1.5);
}

#[test]
fn voz_fuerte_baja_y_no_pasa_de_menos_12() {
    let mut a = Agc::nuevo();
    let sal = correr(&mut a, -3.0, 12);
    assert!(a.ganancia_db() >= -12.01 && sal.iter().all(|x| x.abs() <= 0.891));
}

#[test]
fn ganancia_maxima_30_db() {
    let mut a = Agc::nuevo();
    correr(&mut a, -49.0, 20);
    assert!(a.ganancia_db() <= 30.01 && a.ganancia_db() > 25.0);
}

#[test]
fn silencio_congela_y_no_amplifica_bajo_menos_50() {
    let mut a = Agc::nuevo();
    correr(&mut a, -40.0, 8);
    let g = a.ganancia_db();
    let sal = correr(&mut a, -60.0, 20);
    assert!((a.ganancia_db() - g).abs() < 0.5, "se congela");
    assert!(db(rms(&sal)) < -59.0, "sin amplificar: {}", db(rms(&sal)));
}
