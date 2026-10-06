//! Tabla del glosario: lectura de líneas `QA = cu a, cuba` y reemplazo por variantes o por fonética.
use crate::fonetica::{clave, jaro_winkler, normalizar};

const UMBRAL: f64 = 0.94;
const MIN_FUZZY: usize = 5; // claves más cortas solo coinciden exactas
const MAX_PALABRAS: usize = 3;

pub struct Termino {
    canon: String,
    variantes: Vec<String>, // normalizadas, sin espacios
    claves: Vec<String>,
}

pub fn parsear(contenido: &str) -> Vec<Termino> {
    let mut sal = vec![];
    for linea in contenido
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let (canon, resto) = linea.split_once('=').unwrap_or((linea, ""));
        let canon = canon.trim();
        let mut variantes: Vec<String> = resto
            .split(',')
            .map(normalizar)
            .filter(|v| !v.is_empty())
            .collect();
        variantes.push(normalizar(canon));
        variantes.retain(|v| !v.is_empty());
        let claves = variantes.iter().map(|v| clave(v)).collect();
        if !canon.is_empty() {
            sal.push(Termino {
                canon: canon.to_string(),
                variantes,
                claves,
            });
        }
    }
    sal
}

fn coincide(t: &Termino, grupo: &str) -> bool {
    if t.variantes.iter().any(|v| v == grupo) {
        return true;
    }
    let k = clave(grupo);
    t.claves.iter().any(|c| {
        k == *c || (k.len() >= MIN_FUZZY && c.len() >= MIN_FUZZY && jaro_winkler(&k, c) >= UMBRAL)
    })
}

/// Separa puntuación pegada: ("¿", "cuba", "?").
fn partir(token: &str) -> (&str, &str, &str) {
    let ini = token
        .find(|c: char| c.is_alphanumeric())
        .unwrap_or(token.len());
    let fin = token.rfind(|c: char| c.is_alphanumeric()).map_or(ini, |i| {
        i + token[i..].chars().next().map_or(0, char::len_utf8)
    });
    (&token[..ini], &token[ini..fin], &token[fin..])
}

/// Devuelve el texto corregido y cuántas sustituciones hizo.
pub fn reemplazar(texto: &str, terminos: &[Termino]) -> (String, usize) {
    let toks: Vec<(&str, &str, &str)> = texto.split_whitespace().map(partir).collect();
    let (mut sal, mut n, mut i) = (Vec::<String>::new(), 0, 0);
    while i < toks.len() {
        let hallado = (1..=MAX_PALABRAS.min(toks.len() - i)).rev().find_map(|k| {
            let cores: Vec<&str> = toks[i..i + k].iter().map(|t| t.1).collect();
            if cores.iter().any(|c| c.is_empty()) {
                return None;
            }
            let grupo = normalizar(&cores.join(" "));
            terminos
                .iter()
                .find(|t| coincide(t, &grupo))
                .map(|t| (k, t))
        });
        match hallado {
            Some((k, t)) => {
                let original = toks[i..i + k]
                    .iter()
                    .map(|t| t.1)
                    .collect::<Vec<_>>()
                    .join(" ");
                n += usize::from(original != t.canon);
                sal.push(format!("{}{}{}", toks[i].0, t.canon, toks[i + k - 1].2));
                i += k;
            }
            None => {
                sal.push(format!("{}{}{}", toks[i].0, toks[i].1, toks[i].2));
                i += 1;
            }
        }
    }
    (sal.join(" "), n)
}
