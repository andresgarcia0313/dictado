import QtQuick
import ".."

// Barra de memoria: `valor` y `maximo` en MB. Ámbar al 85 %, rojo al pasar el presupuesto.
Item {
    id: raiz
    property string etiqueta: "Motor"
    property real valor: 0
    property real maximo: 320
    property color acento: Tema.cian
    readonly property real _f: maximo > 0 ? Math.min(1, valor / maximo) : 0
    readonly property color _color: valor > maximo ? Tema.rojo : (_f > 0.85 ? Tema.ambar : acento)

    implicitWidth: Tema.u * 28
    implicitHeight: Tema.u * 5

    Text {
        id: rotulo
        text: raiz.etiqueta
        color: Tema.tenue
        font { family: Tema.fuente; pixelSize: Tema.tPequeno; capitalization: Font.AllUppercase; letterSpacing: 1 }
    }
    Text {
        anchors.right: parent.right
        text: Math.round(raiz.valor) + " MB"
        color: Tema.texto
        font { family: Tema.fuente; pixelSize: Tema.tCuerpo; weight: Font.DemiBold }
    }
    Rectangle {
        id: pista
        anchors { left: parent.left; right: parent.right; bottom: parent.bottom }
        height: Tema.u
        radius: height / 2
        color: Tema.elevada
        Rectangle {
            width: Math.max(height, pista.width * raiz._f)
            height: parent.height
            radius: height / 2
            color: raiz._color
            Behavior on width { NumberAnimation { duration: Tema.suave; easing.type: Easing.OutCubic } }
            Behavior on color { ColorAnimation { duration: Tema.suave } }
        }
    }
    Accessible.role: Accessible.ProgressBar
    Accessible.name: etiqueta + " " + Math.round(valor) + " MB"
}
