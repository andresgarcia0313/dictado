use super::*;

fn tono(hz: f32, amp: f32, n: usize) -> Vec<f32> {
    (0..n)
        .map(|i| amp * (std::f32::consts::TAU * hz * i as f32 / FS as f32).sin())
        .collect()
}

fn ultima(m: &[f32]) -> [u8; BANDAS] {
    let mut e = Espectro::nuevo();
    let mut t = None;
    for bloque in m.chunks(1000) {
        t = e.procesar(bloque).or(t);
    }
    t.expect("sin trama")
}

#[test]
fn silencio_da_ceros() {
    assert_eq!(ultima(&vec![0.0; 16000]), [0u8; BANDAS]);
}

#[test]
fn tono_de_1khz_enciende_su_banda() {
    let t = ultima(&tono(1000.0, 0.3, 16000));
    let pico = (0..BANDAS).max_by_key(|&b| t[b]).unwrap();
    let esperada = (BANDAS as f32 * (1000.0f32 / F_MIN).ln() / (F_MAX / F_MIN).ln()) as usize;
    assert!(
        pico.abs_diff(esperada) <= 1,
        "pico {pico} esperada {esperada}"
    );
    assert!(t[pico] > 150, "{t:?}");
    assert!(t[0] < 20 && t[BANDAS - 1] < 20, "{t:?}");
}

#[test]
fn cadencia_de_tramas() {
    let mut e = Espectro::nuevo();
    let n = (0..25)
        .filter(|_| e.procesar(&vec![0.0; 640]).is_some())
        .count();
    assert!(n >= 24, "{n}");
    assert!(Espectro::nuevo().procesar(&[0.0; 100]).is_none());
}

#[test]
fn consumo_de_cpu_de_10_segundos() {
    let m = tono(1000.0, 0.2, 160_000);
    let t0 = std::time::Instant::now();
    let mut e = Espectro::nuevo();
    let n = m.chunks(1024).filter_map(|b| e.procesar(b)).count();
    let s = t0.elapsed().as_secs_f64();
    eprintln!(
        "10 s de audio -> {n} tramas en {:.2} ms = {:.4} % de un nucleo",
        s * 1e3,
        s / 10.0 * 100.0
    );
    assert!(s / 10.0 < 0.01);
}
