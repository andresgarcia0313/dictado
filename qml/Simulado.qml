import QtQuick

// Doble de `Controlador` (mismas propiedades e invokables) para trabajar sin Rust.
QtObject {
    id: raiz
    property string estado: "escuchando"
    property string parcial: ""
    property string historial: ""
    property real nivel: 0
    property real ramApp: 96
    property real ramMotor: 287
    property string modelo: "es-kroko"
    property string mensaje: ""
    property bool animar: true
    property bool mejora: true
    property bool ruido: false
    property string espectro: ""
    property real ganancia: 0
    property string mics: "Micrófono integrado|Auricular USB|Webcam HD"
    property string mic: "Micrófono integrado"
    property string entorno: "voz"
    property real snr: 12.3
    property string rutaRegistros: "~/.local/state/dictado/registros"
    property bool saltarFiltros: true
    property string rutaVocabulario: "~/.config/dictado/vocabulario.txt"
    property real _fase: 0

    readonly property var _frases: [
        "Hola, como estas.", "Quiero revisar el informe del mes.",
        "Por favor agenda la reunion del martes a las diez.",
        "Recuerda enviar las actas antes del viernes."]
    property int _i: 0
    property int _p: 0

    function alternar() { estado = estado === "escuchando" ? "pausado" : "escuchando" }
    function limpiar() { historial = ""; parcial = "" }
    function fijarMejora(v) { mejora = v }
    function fijarRuido(v) { ruido = v }
    function elegirMic(n) { mic = n === "auto" ? mics.split("|")[0] : n }
    function abrirRegistros() { console.log("abrir registros:", rutaRegistros) }
    function fijarSaltarFiltros(v) { saltarFiltros = v }
    function abrirVocabulario() { console.log("abrir vocabulario:", rutaVocabulario) }
    function copiar() { console.log("copiado:", historial.length, "caracteres") }

    property Timer _reloj: Timer {
        interval: 120; repeat: true; running: raiz.animar
        onTriggered: {
            raiz.nivel = raiz.estado === "escuchando" ? 0.5 + 0.5 * Math.sin(Date.now() / 170) * Math.random() : 0
            raiz.ramApp = 96 + 4 * Math.sin(Date.now() / 2000)
            if (raiz.estado !== "escuchando") return
            const f = raiz._frases[raiz._i % raiz._frases.length].split(" ")
            raiz._p++
            if (raiz._p <= f.length) {
                raiz.parcial = f.slice(0, raiz._p).join(" ").toLowerCase().replace(/[.,]/g, "")
            } else {
                raiz.historial += (raiz.historial ? "\n" : "") + raiz._frases[raiz._i % raiz._frases.length]
                raiz.parcial = ""; raiz._p = 0; raiz._i++
            }
        }
    }

    // Espectro simulado: 32 bandas con formantes que se mueven; en pausa, silencio.
    property Timer _espectro: Timer {
        interval: 40; repeat: true; running: raiz.animar
        onTriggered: {
            raiz._fase += 0.09
            const v = raiz.estado === "escuchando"
            raiz.espectro = Array.from({ length: 32 }, (_, i) => v ? Math.max(0, Math.min(255, Math.round(
                230 * Math.exp(-Math.pow((i - 6 - 4 * Math.sin(raiz._fase)) / 4, 2))
                + 150 * Math.exp(-Math.pow((i - 18 - 5 * Math.sin(raiz._fase * 0.7)) / 5, 2))
                + 40 * Math.random() - 15))) : 0).join(",")
            raiz.ganancia = v ? 12 + 6 * Math.sin(raiz._fase / 6) : raiz.ganancia
        }
    }

    // Entorno simulado: alterna voz, ruido y silencio cada 3,5 s.
    property int _e: 0
    property Timer _entorno: Timer {
        interval: 3500; repeat: true; running: raiz.animar
        onTriggered: {
            raiz._e = (raiz._e + 1) % 3
            raiz.entorno = ["voz", "ruido", "silencio"][raiz._e]
            raiz.snr = [14.2, 3.8, 0][raiz._e]
        }
    }
}
