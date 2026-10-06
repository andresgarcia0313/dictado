pragma Singleton
import QtQuick

// Tokens de diseño: colores, espaciado de 8 pt, radios, tipografía y tiempos.
QtObject {
    readonly property color fondo: "#0B0D12"
    readonly property color superficie: "#12151C"
    readonly property color elevada: "#181C26"
    readonly property color texto: "#E8ECF4"
    readonly property color tenue: "#8A93A6"
    readonly property color cian: "#22D3EE"
    readonly property color magenta: "#E879F9"
    readonly property color verde: "#4ADE80"
    readonly property color ambar: "#FBBF24"
    readonly property color rojo: "#FB7185"
    readonly property color borde: "#232838"

    readonly property int u: 8
    readonly property int radio: 16
    readonly property int radioChico: 10

    readonly property string fuente: Qt.application.font.family
    readonly property int tCuerpo: 16
    readonly property int tPequeno: 12
    readonly property int tTitulo: 20

    readonly property int rapido: 120
    readonly property int suave: 200

    function acentoDe(estado) {
        switch (estado) {
        case "escuchando": return cian
        case "pausado": return ambar
        case "error": return rojo
        default: return tenue
        }
    }
    function translucido(c, a) { return Qt.rgba(c.r, c.g, c.b, a) }
}
