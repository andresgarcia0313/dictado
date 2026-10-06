import QtQuick
import QtQuick.Layouts
import QtCore
import "componentes"

// Ventana principal. `ctl` es el Controlador (o Simulado): los componentes no lo conocen.
Window {
    id: raiz
    property var ctl: ({ estado: "iniciando", parcial: "", historial: "", nivel: 0, ramApp: 0, ramMotor: 0,
        modelo: "", mensaje: "", espectro: "", ganancia: 0, mics: "", mic: "auto", entorno: "silencio", snr: 0,
        rutaRegistros: "", saltarFiltros: true, rutaVocabulario: "", fijarSaltarFiltros(v) {}, abrirVocabulario() {}, alternar() {}, limpiar() {}, copiar() {}, fijarMejora(v) {}, fijarRuido(v) {},
        elegirMic(n) {}, abrirRegistros() {} })
    readonly property bool ancha: width >= 960
    readonly property bool angosta: width < 640
    readonly property bool escuchando: ctl && ctl.estado === "escuchando"

    title: "Dictado"; visible: true
    width: 960; height: 640; minimumWidth: 360; minimumHeight: 420
    color: Tema.fondo

    Geometria { ventana: raiz }
    Settings {
        id: audio
        category: "audio"
        property bool mejora: true; property bool ruido: false; property bool saltar: true; property string mic: "auto"
    }
    // Reenvía lo guardado al motor al iniciar (el Controlador lo repite en cada `listo`).
    Component.onCompleted: { ctl.fijarMejora(audio.mejora); ctl.fijarRuido(audio.ruido); ctl.fijarSaltarFiltros(audio.saltar); ctl.elegirMic(audio.mic) }

    Shortcut { sequence: "Space"; onActivated: ctl.alternar() }
    Shortcut { sequence: "Ctrl+L"; onActivated: ctl.limpiar() }
    Shortcut { sequence: "Ctrl+C"; enabled: !historial.haySeleccion; onActivated: ctl.copiar() }
    Shortcut { sequence: "F11"; onActivated: raiz.visibility = visibility === Window.FullScreen ? Window.Windowed : Window.FullScreen }

    GridLayout {
        anchors { fill: parent; margins: raiz.angosta ? Tema.u * 2 : Tema.u * 4 }
        columns: raiz.ancha ? 2 : 1
        columnSpacing: Tema.u * 4
        rowSpacing: Tema.u * 3

        ColumnLayout {
            Layout.fillWidth: true; Layout.fillHeight: true; spacing: Tema.u * 3
            Subtitulo {
                Layout.fillWidth: true
                Layout.preferredHeight: raiz.height * (raiz.ancha ? 0.34 : 0.2)
                tamano: raiz.ancha ? 46 : (raiz.angosta ? 26 : 34)
                parcial: ctl.parcial; activo: raiz.escuchando
            }
            Historial {
                id: historial
                Layout.fillWidth: true; Layout.fillHeight: true
                tamano: raiz.angosta ? 16 : 18; texto: ctl.historial
            }
            BarraEstado {
                Layout.fillWidth: true
                estado: ctl.estado; modelo: ctl.modelo; mensaje: ctl.mensaje
            }
        }

        Flickable {
            id: lado
            Layout.fillWidth: !raiz.ancha; Layout.alignment: Qt.AlignTop
            Layout.preferredWidth: raiz.ancha ? 280 : -1
            Layout.fillHeight: raiz.ancha
            Layout.preferredHeight: raiz.ancha ? -1 : Math.min(cuerpo.implicitHeight, raiz.height * 0.55)
            contentHeight: cuerpo.implicitHeight; clip: contentHeight > height; boundsBehavior: Flickable.StopAtBounds
            GridLayout {
                id: cuerpo; width: lado.width
                columns: raiz.ancha ? 1 : (raiz.angosta ? 2 : 4)
                columnSpacing: Tema.u * 3; rowSpacing: raiz.ancha ? Tema.u * 2 : Tema.u * 3
                Orbe {
                    Layout.alignment: Qt.AlignHCenter
                    Layout.preferredWidth: raiz.ancha ? Tema.u * 11 : Tema.u * 9
                    Layout.preferredHeight: Layout.preferredWidth
                    Layout.rowSpan: raiz.angosta ? 2 : 1
                    nivel: ctl.nivel; activo: raiz.escuchando; acento: Tema.acentoDe(ctl.estado)
                }
                MedidorRam { Layout.fillWidth: true; etiqueta: "Motor"; valor: ctl.ramMotor; maximo: 320 }
                MedidorRam { Layout.fillWidth: true; etiqueta: "App"; valor: ctl.ramApp; maximo: 120; acento: Tema.magenta }
                RowLayout {
                    Layout.fillWidth: true; Layout.columnSpan: raiz.angosta ? 2 : 1; spacing: Tema.u
                    BotonNeon { texto: raiz.escuchando ? "Pausar" : "Reanudar"; primario: true; onPulsado: ctl.alternar() }
                    BotonNeon { texto: "Limpiar"; acento: Tema.magenta; onPulsado: ctl.limpiar() }
                    BotonNeon { texto: "Copiar"; acento: Tema.verde; onPulsado: ctl.copiar() }
                }
                PanelAudio {
                    Layout.fillWidth: true; Layout.columnSpan: parent.columns
                    mejora: audio.mejora; ruido: audio.ruido; mic: ctl.mic; mics: ctl.mics
                    ganancia: ctl.ganancia; espectro: ctl.espectro; activo: raiz.escuchando
                    entorno: ctl.entorno; snr: ctl.snr; rutaRegistros: ctl.rutaRegistros; saltarFiltros: audio.saltar; rutaVocabulario: ctl.rutaVocabulario
                    onRegistrosPedidos: ctl.abrirRegistros(); onVocabularioPedido: ctl.abrirVocabulario()
                    onSaltarCambiado: v => { audio.saltar = v; ctl.fijarSaltarFiltros(v) }
                    onMejoraCambiada: v => { audio.mejora = v; ctl.fijarMejora(v) }
                    onRuidoCambiado: v => { audio.ruido = v; ctl.fijarRuido(v) }
                    onMicElegido: n => { audio.mic = n; ctl.elegirMic(n) }
                }
            }
        }
    }
}
