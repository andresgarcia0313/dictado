//! Ordenes que la GUI envia al motor por stdin.
#[derive(Debug, PartialEq)]
pub enum Orden {
    Pausa,
    Reanuda,
    Salir,
    Mejora(bool),
    Ruido(bool),
    /// `None` = `mic auto` (el dispositivo por defecto).
    Mic(Option<String>),
}

fn interruptor(arg: &str) -> Option<bool> {
    match arg.trim() {
        "on" => Some(true),
        "off" => Some(false),
        _ => None,
    }
}

pub fn parsear(linea: &str) -> Option<Orden> {
    let l = linea.trim();
    match l {
        "pausa" => return Some(Orden::Pausa),
        "reanuda" => return Some(Orden::Reanuda),
        "salir" => return Some(Orden::Salir),
        _ => {}
    }
    let (clave, resto) = l.split_once(' ')?;
    match clave {
        "mejora" => interruptor(resto).map(Orden::Mejora),
        "ruido" => interruptor(resto).map(Orden::Ruido),
        "mic" => match resto.trim() {
            "" => None,
            "auto" => Some(Orden::Mic(None)),
            nombre => Some(Orden::Mic(Some(nombre.to_string()))),
        },
        _ => None,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn ordenes_del_protocolo() {
        assert_eq!(parsear("pausa\n"), Some(Orden::Pausa));
        assert_eq!(parsear("reanuda"), Some(Orden::Reanuda));
        assert_eq!(parsear(" salir "), Some(Orden::Salir));
        assert_eq!(parsear("otra"), None);
    }

    #[test]
    fn ordenes_de_audio() {
        assert_eq!(parsear("mejora on"), Some(Orden::Mejora(true)));
        assert_eq!(parsear("ruido off\n"), Some(Orden::Ruido(false)));
        assert_eq!(parsear("ruido quiza"), None);
        assert_eq!(parsear("mic auto"), Some(Orden::Mic(None)));
        assert_eq!(
            parsear("mic Micrófono USB (hw:1,0)"),
            Some(Orden::Mic(Some("Micrófono USB (hw:1,0)".into())))
        );
        assert_eq!(parsear("mic "), None);
    }
}
