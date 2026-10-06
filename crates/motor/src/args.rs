//! Argumentos de la linea de comandos y busqueda del modelo.
use std::path::PathBuf;

const NOMBRE_MODELO: &str = "es-kroko";

fn valido(d: &std::path::Path) -> bool {
    d.join("encoder.onnx").is_file()
}

pub fn dir_modelo(arg: Option<PathBuf>) -> Option<PathBuf> {
    // una ruta pedida de forma explicita no cae en silencio a otro modelo
    if let Some(d) = arg.or_else(|| std::env::var_os("DICTADO_MODELO").map(PathBuf::from)) {
        return valido(&d).then_some(d);
    }
    [
        dirs::data_dir().map(|d| d.join("dictado/modelos").join(NOMBRE_MODELO)),
        dirs::home_dir().map(|d| d.join(".cache/sherpa").join(NOMBRE_MODELO)),
    ]
    .into_iter()
    .flatten()
    .find(|d| valido(d))
}

pub fn argumento(args: &[String], clave: &str) -> Option<PathBuf> {
    args.iter()
        .position(|a| a == clave)
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
}

/// Valores sueltos que siguen a `clave` hasta el siguiente `--flag` (modo lote: `--wav a.wav b.wav`).
pub fn lista(args: &[String], clave: &str) -> Vec<PathBuf> {
    args.iter()
        .skip_while(|a| *a != clave)
        .skip(1)
        .take_while(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .collect()
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn lista_para_en_el_siguiente_flag() {
        let a: Vec<String> = ["m", "--wav", "a.wav", "b.wav", "--ruido", "c.wav"]
            .map(String::from)
            .into();
        assert_eq!(
            lista(&a, "--wav"),
            [PathBuf::from("a.wav"), PathBuf::from("b.wav")]
        );
        assert!(lista(&a, "--nada").is_empty());
    }
}
