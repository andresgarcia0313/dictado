import QtQuick
import QtQuick.Layouts
import ".."

// Panel "Audio": mejora, reducir ruido, micrófono, ganancia y espectrograma.
// `espectro` llega como CSV de 32 enteros y `mics` separado por "|"; solo emite señales.
Rectangle {
    id: raiz
    property bool mejora: true
    property bool ruido: false
    property string mic: "auto"
    property string mics: ""
    property real ganancia: 0
    property string espectro: ""
    property bool activo: true
    property string entorno: "silencio"
    property real snr: 0
    property string rutaRegistros: ""
    property bool saltarFiltros: true
    property string rutaVocabulario: ""
    signal mejoraCambiada(bool valor)
    signal ruidoCambiado(bool valor)
    signal micElegido(string nombre)
    signal registrosPedidos()
    signal saltarCambiado(bool valor)
    signal vocabularioPedido()

    implicitWidth: Tema.u * 34
    implicitHeight: col.implicitHeight + Tema.u * 4
    radius: Tema.radio; color: Tema.superficie
    border { width: 1; color: Tema.borde }

    ColumnLayout {
        id: col
        anchors { fill: parent; margins: Tema.u * 2 }
        spacing: Tema.u * 1.5
        RowLayout {
            Layout.fillWidth: true
            Text { text: "AUDIO"; color: Tema.tenue; font { family: Tema.fuente; pixelSize: Tema.tPequeno; letterSpacing: 1.5; weight: Font.DemiBold } }
            Item { Layout.fillWidth: true }
            Text {
                visible: raiz.rutaRegistros !== "" && raiz.width >= Tema.u * 44
                Layout.maximumWidth: raiz.width - Tema.u * 28; text: raiz.rutaRegistros
                elide: Text.ElideMiddle; color: Tema.tenue; font { family: Tema.fuente; pixelSize: Tema.tPequeno - 2 }
            }
            Text {
                text: "Registros"; color: zona.containsMouse ? Tema.texto : Tema.tenue
                font { family: Tema.fuente; pixelSize: Tema.tPequeno; underline: zona.containsMouse }
                MouseArea { id: zona; anchors { fill: parent; margins: -6 } hoverEnabled: true; cursorShape: Qt.PointingHandCursor; onClicked: raiz.registrosPedidos() }
                Accessible.role: Accessible.Button; Accessible.name: "Registros"; Accessible.onPressAction: raiz.registrosPedidos()
            }
        }
        RowLayout {
            Layout.fillWidth: true
            ChipEntorno { entorno: raiz.entorno; snr: raiz.snr }
            Item { Layout.fillWidth: true }
            Text {
                text: (raiz.ganancia >= 0 ? "+" : "") + raiz.ganancia.toFixed(1) + " dB"
                color: raiz.mejora ? Tema.cian : Tema.tenue
                font { family: Tema.fuente; pixelSize: Tema.tPequeno + 1; weight: Font.DemiBold }
            }
        }
        Interruptor {
            Layout.fillWidth: true; texto: "Mejora de audio"; marcado: raiz.mejora
            onCambiado: v => raiz.mejoraCambiada(v)
        }
        Interruptor {
            Layout.fillWidth: true; texto: "Reducir ruido de calle"; acento: Tema.magenta
            nota: "automático: actúa solo si hay mucho ruido"
            marcado: raiz.ruido; onCambiado: v => raiz.ruidoCambiado(v)
        }
        Interruptor {
            Layout.fillWidth: true; texto: "Saltar filtros del sistema"; acento: Tema.verde
            nota: "evita la puerta y el filtro de ruido de las videollamadas"
            marcado: raiz.saltarFiltros; onCambiado: v => raiz.saltarCambiado(v)
        }
        BotonNeon {
            texto: "Vocabulario"; acento: Tema.verde; onPulsado: raiz.vocabularioPedido()
            Accessible.description: raiz.rutaVocabulario
        }
        Selector {
            Layout.fillWidth: true
            opciones: ["auto"].concat(raiz.mics ? raiz.mics.split("|") : [])
            actual: raiz.mic
            onElegido: n => raiz.micElegido(n)
        }
        Espectrograma {
            Layout.fillWidth: true; Layout.preferredHeight: Tema.u * 8
            activo: raiz.activo
            bandas: raiz.espectro ? raiz.espectro.split(",").map(Number) : []
        }
    }
}
