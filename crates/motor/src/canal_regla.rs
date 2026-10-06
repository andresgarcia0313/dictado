//! Regla de decisión del canal (pura, con histéresis).

/// Ventana de decisión en segundos (la primera es de medio segundo para no arrastrar un canal muerto).
pub const VENTANA_S: f32 = 2.0;
pub const PRIMERA_S: f32 = 0.5;
/// Un canal más de 10 dB por debajo del fuerte se descarta; vuelve al promedio solo con menos de 6 dB.
const ENTRA_DB: f32 = 10.0;
const SALE_DB: f32 = 6.0;
/// Otro canal debe superar al actual en 3 dB para tomar su lugar.
const CAMBIA_DB: f32 = 3.0;
const CORR_MIN: f32 = 0.7;
/// Por debajo de este RMS (silencio) no hay datos para decidir.
const RMS_MIN: f32 = 1e-4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Modo {
    Promedio,
    Solo(usize),
}

fn db(a: f32, b: f32) -> f32 {
    20.0 * (a.max(1e-9) / b.max(1e-9)).log10()
}

/// Decisión pura: `rms` por canal y correlación de cada canal con el más fuerte.
pub fn decidir(actual: Modo, rms: &[f32], corr: &[f32]) -> Modo {
    let (fuerte, max) =
        rms.iter()
            .copied()
            .enumerate()
            .fold((0, 0.0), |m, (i, r)| if r > m.1 { (i, r) } else { m });
    if max < RMS_MIN {
        return actual;
    }
    let brecha = rms.iter().map(|&r| db(max, r)).fold(0.0, f32::max);
    let parecidos = brecha < SALE_DB && corr.iter().all(|&c| c > CORR_MIN);
    match actual {
        Modo::Promedio
            if brecha > ENTRA_DB || brecha > CAMBIA_DB && corr.iter().any(|&c| c < CORR_MIN) =>
        {
            Modo::Solo(fuerte)
        }
        Modo::Promedio => actual,
        Modo::Solo(_) if parecidos => Modo::Promedio,
        Modo::Solo(i) if i != fuerte && db(rms[fuerte], rms[i]) > CAMBIA_DB => Modo::Solo(fuerte),
        Modo::Solo(_) => actual,
    }
}
