//! Limitador suave a -1 dBFS.
pub const TECHO: f32 = 0.891;

/// Compresion suave sobre 0.7 que nunca pasa de TECHO.
pub fn limitar(y: f32) -> f32 {
    const RODILLA: f32 = 0.7;
    let a = y.abs();
    if a <= RODILLA {
        return y;
    }
    let k = TECHO - RODILLA;
    y.signum() * (RODILLA + k * ((a - RODILLA) / k).tanh())
}

/// Aplica la ganancia con rampa lineal de `de` a `a` a lo largo del bloque, con limitador.
pub fn rampa(m: &mut [f32], de: f32, a: f32) {
    let n = m.len().max(1) as f32;
    for (i, x) in m.iter_mut().enumerate() {
        let g = de + (a - de) * (i + 1) as f32 / n;
        *x = limitar(*x * g);
    }
}
