//! Micrófono directo en Linux: evita que EasyEffects (puerta + RNNoise) filtre lo que graba el motor.
//! Antes de abrir cpal se fija `PIPEWIRE_PROPS` con `node.dont-move` y `target.object` = fuente física.

/// Fuente física a usar: la predeterminada salvo que sea de EasyEffects; si no, la primera `alsa_input.*`.
pub fn elegir_fuente(defecto: &str, lista: &str) -> Option<String> {
    let defecto = defecto.trim();
    if !defecto.is_empty() && !defecto.starts_with("easyeffects_") && !defecto.ends_with(".monitor")
    {
        return Some(defecto.to_string());
    }
    lista
        .lines()
        .filter_map(|l| l.split_whitespace().nth(1))
        .find(|n| n.starts_with("alsa_input."))
        .map(str::to_string)
}

pub fn props(fuente: &str) -> String {
    format!("{{ node.dont-move=true target.object=\"{fuente}\" }}")
}

#[cfg(target_os = "linux")]
fn pactl(args: &[&str]) -> String {
    std::process::Command::new("timeout")
        .args(["3", "pactl"])
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

/// Fija el entorno; devuelve la fuente elegida. No hace nada con `--con-filtros` ni si el usuario ya
/// definió `PIPEWIRE_PROPS`. No registra audio ni texto: solo el nombre de la fuente.
#[cfg(target_os = "linux")]
pub fn fijar_directo(con_filtros: bool) -> Option<String> {
    if con_filtros {
        registro::info!("microfono con filtros del sistema (--con-filtros)");
        return None;
    }
    if std::env::var_os("PIPEWIRE_PROPS").is_some() {
        registro::info!("PIPEWIRE_PROPS ya definido por el entorno; no se cambia");
        return None;
    }
    let lista = pactl(&["list", "short", "sources"]);
    let fuente = elegir_fuente(&pactl(&["get-default-source"]), &lista);
    match &fuente {
        Some(f) => {
            std::env::set_var("PIPEWIRE_PROPS", props(f));
            registro::info!("microfono directo fuente=\"{f}\" (sin filtros del sistema)");
        }
        None => registro::aviso!("sin fuente fisica identificable; se usa la predeterminada"),
    }
    fuente
}

/// Windows y macOS: no hay nada que saltar.
#[cfg(not(target_os = "linux"))]
pub fn fijar_directo(_con_filtros: bool) -> Option<String> {
    None
}

#[cfg(test)]
mod pruebas {
    use super::*;
    const LISTA: &str = "60\talsa_output.x.monitor\tPipeWire\n61\talsa_input.pci.analog\tPipeWire\n85\teasyeffects_source\tPipeWire\n";

    #[test]
    fn respeta_la_fisica_predeterminada() {
        assert_eq!(
            elegir_fuente("alsa_input.usb\n", LISTA).as_deref(),
            Some("alsa_input.usb")
        );
    }

    #[test]
    fn evita_easyeffects_y_monitores() {
        for d in ["easyeffects_source", "alsa_output.x.monitor", ""] {
            assert_eq!(
                elegir_fuente(d, LISTA).as_deref(),
                Some("alsa_input.pci.analog"),
                "{d}"
            );
        }
        assert_eq!(
            elegir_fuente("easyeffects_source", "85\teasyeffects_source\n"),
            None
        );
    }

    #[test]
    fn props_con_destino() {
        assert_eq!(props("a"), "{ node.dont-move=true target.object=\"a\" }");
    }
}
