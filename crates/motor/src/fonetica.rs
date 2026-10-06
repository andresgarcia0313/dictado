//! Clave fonética del español (Metaphone simplificado) y similitud Jaro-Winkler para comparar cómo suenan.

fn sin_tilde(c: char) -> char {
    match c {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        otro => otro,
    }
}

/// Minúsculas, sin tildes y solo letras: forma comparable de una palabra o frase.
pub fn normalizar(texto: &str) -> String {
    texto
        .to_lowercase()
        .chars()
        .map(sin_tilde)
        .filter(|c| c.is_alphabetic())
        .collect()
}

/// Clave fonética de una palabra o frase corta (los espacios no cuentan: "cu a" suena como "cua").
pub fn clave(texto: &str) -> String {
    let l: Vec<char> = normalizar(texto).chars().collect();
    let sig = |i: usize| l.get(i + 1).copied().unwrap_or(' ');
    let mut sal = String::new();
    let mut i = 0;
    while i < l.len() {
        let suave = matches!(sig(i), 'e' | 'i');
        let (cod, salto) = match l[i] {
            'h' => (Some(""), 0),
            'v' | 'w' => (Some("b"), 0),
            'z' => (Some("s"), 0),
            'c' if sig(i) == 'h' => (Some("x"), 1),
            'c' if suave => (Some("s"), 0),
            'c' | 'k' => (Some("k"), 0),
            'q' => (Some("k"), usize::from(sig(i) == 'u')),
            'g' if suave => (Some("j"), 0),
            'g' if sig(i) == 'u' && matches!(l.get(i + 2), Some('e' | 'i')) => (Some("g"), 1),
            'l' if sig(i) == 'l' => (Some("y"), 1),
            'x' => (Some("ks"), 0),
            'ñ' => (Some("ny"), 0),
            _ => (None, 0),
        };
        match cod {
            Some(c) => sal.push_str(c),
            None => sal.push(l[i]),
        }
        i += 1 + salto;
    }
    colapsar(&sal)
}

fn colapsar(s: &str) -> String {
    let mut sal = String::new();
    for c in s.chars() {
        if !sal.ends_with(c) {
            sal.push(c);
        }
    }
    sal
}

#[path = "fonetica_similitud.rs"]
mod similitud;
pub use similitud::jaro_winkler;

#[cfg(test)]
#[path = "fonetica_pruebas.rs"]
mod pruebas;
