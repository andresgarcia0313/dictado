use super::*;

fn correr(h: &mut Histeresis, p: f32, n: usize) -> Vec<Cambio> {
    (0..n)
        .map(|_| h.paso(p))
        .filter(|c| *c != Cambio::Nada)
        .collect()
}

#[test]
fn entra_en_05_y_aguanta_hasta_035() {
    let mut h = Histeresis::default();
    assert!(correr(&mut h, 0.45, 20).is_empty());
    assert_eq!(correr(&mut h, 0.6, 1), [Cambio::Entra]);
    assert!(correr(&mut h, 0.4, 100).is_empty(), "0.4 sostiene la voz");
    assert!(h.dentro());
}

#[test]
fn sale_a_los_400_ms_y_cuenta_la_voz() {
    let mut h = Histeresis::default();
    correr(&mut h, 0.9, 20);
    assert!(correr(&mut h, 0.1, 24).is_empty());
    assert_eq!(correr(&mut h, 0.1, 1), [Cambio::Sale]);
    assert!(h.es_valido() && h.voz_ms() == 320 && h.p_max() == 0.9);
}

#[test]
fn segmento_corto_no_es_valido() {
    let mut h = Histeresis::default();
    correr(&mut h, 0.9, 10);
    correr(&mut h, 0.0, 30);
    assert!(!h.es_valido());
}
