use super::*;

fn onda(n0: usize, f: f32, a: f32) -> Vec<f32> {
    (n0..n0 + TRAMA)
        .map(|n| a * (std::f32::consts::TAU * f * n as f32 / 16_000.0).sin())
        .collect()
}

#[test]
fn inactiva_reconstruye_la_entrada_con_una_trama_de_retraso() {
    let mut c = Compuerta::nueva();
    let (mut ent, mut sal) = (vec![], vec![]);
    for i in 0..40 {
        let x = onda(i * TRAMA, 440.0, 0.3);
        sal.extend(c.trama(&x, 0.9, Some(30.0)));
        ent.extend(x);
    }
    let err = (TRAMA..ent.len() - TRAMA)
        .map(|i| (sal[i + TRAMA] - ent[i]).abs())
        .fold(0.0f32, f32::max);
    assert!(err < 1e-4, "error {err}");
}

#[test]
fn activa_baja_el_ruido_estacionario_hasta_el_piso() {
    let mut c = Compuerta::nueva();
    let mut ultima = vec![];
    for i in 0..200 {
        ultima = c.trama(&onda(i * TRAMA, 1000.0, 0.05), 0.0, Some(0.0));
    }
    assert!(c.activa());
    let e = |v: &[f32]| v.iter().map(|x| x * x).sum::<f32>();
    let ratio = (e(&ultima) / e(&onda(200 * TRAMA, 1000.0, 0.05))).sqrt();
    assert!((0.4..0.6).contains(&ratio), "ratio {ratio}");
}
