//! Similitud Jaro-Winkler entre dos cadenas (1.0 = idénticas).

pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if a.is_empty() || b.is_empty() {
        return if a == b { 1.0 } else { 0.0 };
    }
    let ventana = (a.len().max(b.len()) / 2).saturating_sub(1);
    let (mut ua, mut ub) = (vec![false; a.len()], vec![false; b.len()]);
    let mut coinc = 0.0;
    for (i, ca) in a.iter().enumerate() {
        let (desde, hasta) = (i.saturating_sub(ventana), (i + ventana + 1).min(b.len()));
        if let Some(j) = (desde..hasta).find(|&j| !ub[j] && b[j] == *ca) {
            (ua[i], ub[j]) = (true, true);
            coinc += 1.0;
        }
    }
    if coinc == 0.0 {
        return 0.0;
    }
    let ra = a.iter().zip(&ua).filter(|(_, u)| **u).map(|(c, _)| c);
    let rb = b.iter().zip(&ub).filter(|(_, u)| **u).map(|(c, _)| c);
    let trans = ra.zip(rb).filter(|(x, y)| x != y).count() as f64 / 2.0;
    let jaro = (coinc / a.len() as f64 + coinc / b.len() as f64 + (coinc - trans) / coinc) / 3.0;
    let prefijo = a.iter().zip(&b).take(4).take_while(|(x, y)| x == y).count() as f64;
    jaro + prefijo * 0.1 * (1.0 - jaro)
}
