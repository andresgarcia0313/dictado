use super::*;

fn tono(a: f32) -> Vec<f32> {
    (0..256).map(|i| a * (i as f32 * 0.3).sin()).collect()
}

fn alimentar(m: &mut Medida, intercalar_ceros: bool) {
    let ceros = vec![0.0; 256];
    for _ in 0..200 {
        m.trama(&tono(0.1), 0.95); // voz a unos -23 dBFS
        m.trama(&tono(0.0001), 0.1); // ruido a unos -83 dBFS
        if intercalar_ceros {
            m.trama(&ceros, 0.0);
            m.trama(&ceros, 0.0);
        }
    }
}

#[test]
fn ceros_exactos_no_cuentan() {
    let mut m = Medida::nueva();
    m.trama(&[0.0; 256], 0.0);
    assert_eq!(m.conteos(), (0, 0));
}

#[test]
fn ceros_intercalados_no_cambian_el_snr() {
    let (mut a, mut b) = (Medida::nueva(), Medida::nueva());
    alimentar(&mut a, false);
    alimentar(&mut b, true);
    assert_eq!(a.snr(), b.snr());
    assert_eq!(a.conteos(), b.conteos());
}

#[test]
fn snr_acotado_a_0_40_con_piso_realista() {
    let mut m = Medida::nueva();
    alimentar(&mut m, true);
    let snr = m.snr().unwrap();
    assert!((0.0..=40.0).contains(&snr), "snr={snr}");
    assert!(m.piso_db() >= -70.0, "piso {}", m.piso_db());
}
