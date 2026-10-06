use super::tabla::{parsear, reemplazar};

fn r(t: &str) -> (String, usize) {
    let tabla = parsear("QA = cu a, cuba, kiu ei\nREQ = rec\nKubernetes\nRedmine\n# nota\n");
    reemplazar(t, &tabla)
}

#[test]
fn variantes_explicitas_con_limites_de_palabra() {
    assert_eq!(r("el ambiente de cu a"), ("el ambiente de QA".into(), 1));
    assert_eq!(r("prueba de cuba."), ("prueba de QA.".into(), 1));
    assert_eq!(r("Kiu ei listo"), ("QA listo".into(), 1));
    assert_eq!(r("cubano cubata"), ("cubano cubata".into(), 0));
}

#[test]
fn fonetica_sin_variante_escrita() {
    assert_eq!(
        r("el equipo de kubernetes").0,
        "el equipo de Kubernetes"
    );
    assert_eq!(r("abre redmain ya").0, "abre Redmine ya");
    assert_eq!(r("kubernetis").1, 1);
}

#[test]
fn no_toca_palabras_comunes_ni_cuenta_lo_ya_correcto() {
    assert_eq!(r("la reunion es hoy"), ("la reunion es hoy".into(), 0));
    assert_eq!(r("QA y REQ"), ("QA y REQ".into(), 0));
    assert_eq!(r(""), ("".into(), 0));
}

#[test]
fn ignora_comentarios_y_lineas_vacias() {
    assert_eq!(parsear("# x\n\n  \nSAP = zap\n").len(), 1);
}
