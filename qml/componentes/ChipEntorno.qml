import QtQuick
import ".."

// Chip de entorno: voz (cian), ruido (ámbar) o silencio (tenue) con el SNR en dB.
// `snr` en 0 significa sin dato y no se muestra.
Rectangle {
    id: raiz
    property string entorno: "silencio"
    property real snr: 0
    readonly property color acento: entorno === "voz" ? Tema.cian : (entorno === "ruido" ? Tema.ambar : Tema.tenue)

    implicitWidth: fila.implicitWidth + Tema.u * 3
    implicitHeight: Tema.u * 3.5
    radius: height / 2
    color: Tema.translucido(acento, 0.12)
    border { width: 1; color: Tema.translucido(acento, 0.6) }
    Behavior on color { ColorAnimation { duration: Tema.suave } }
    Behavior on border.color { ColorAnimation { duration: Tema.suave } }

    Row {
        id: fila
        anchors.centerIn: parent; spacing: Tema.u
        Rectangle { width: Tema.u; height: width; radius: width / 2; color: raiz.acento; anchors.verticalCenter: parent.verticalCenter }
        Text {
            text: "Entorno: " + raiz.entorno; color: raiz.acento
            font { family: Tema.fuente; pixelSize: Tema.tPequeno + 1; weight: Font.DemiBold }
        }
        Text {
            visible: raiz.snr !== 0; text: raiz.snr.toFixed(0) + " dB"; color: Tema.tenue
            font { family: Tema.fuente; pixelSize: Tema.tPequeno + 1 }
        }
    }
    Accessible.role: Accessible.StaticText
    Accessible.name: "Entorno: " + entorno + (snr !== 0 ? ", " + snr.toFixed(0) + " decibelios" : "")
}
