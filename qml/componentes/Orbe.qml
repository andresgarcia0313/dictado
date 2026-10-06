import QtQuick
import ".."

// Orbe de nivel de voz: el radio sigue a `nivel` (0-1). Neón con capas translúcidas.
Item {
    id: raiz
    property real nivel: 0
    property color acento: Tema.cian
    property bool activo: true
    readonly property real _n: activo ? Math.max(0, Math.min(1, nivel)) : 0

    implicitWidth: Tema.u * 12
    implicitHeight: implicitWidth

    Repeater {
        model: 3
        Rectangle {
            required property int index
            anchors.centerIn: parent
            width: raiz.width * (0.5 + 0.18 * (index + 1) * (0.4 + raiz._n))
            height: width
            radius: width / 2
            color: Tema.translucido(raiz.acento, raiz.activo ? 0.10 : 0.04)
            Behavior on width { NumberAnimation { duration: Tema.rapido; easing.type: Easing.OutCubic } }
        }
    }
    Rectangle {
        anchors.centerIn: parent
        width: raiz.width * (0.34 + 0.2 * raiz._n)
        height: width
        radius: width / 2
        color: Tema.translucido(raiz.acento, raiz.activo ? 0.85 : 0.25)
        border.width: 2
        border.color: Tema.translucido(Qt.lighter(raiz.acento, 1.4), 0.9)
        Behavior on width { NumberAnimation { duration: Tema.rapido; easing.type: Easing.OutCubic } }
        Behavior on color { ColorAnimation { duration: Tema.suave } }
    }
    Accessible.role: Accessible.Indicator
    Accessible.name: activo ? "Nivel de voz" : "Micrófono en pausa"
}
