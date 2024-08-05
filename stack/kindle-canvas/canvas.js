const START_WIDTH = 40;
const START_HEIGHT = -5;
const FONT_SIZE = 50;
const FONT_WEIGHT = 33;

const MAX_STR_SIZE = 37;
document.addEventListener("DOMContentLoaded", function () {
    const canvas = document.getElementById('myCanvas');
    const ctx = canvas.getContext('2d');

    const canvasFontStyle = '' + FONT_SIZE + 'px Monospace';
    // Clear the canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.rotate(3 * Math.PI / 2);

    ctx.textAlign = "right";
    ctx.fillStyle = '#000';
    ctx.font = canvasFontStyle;

    {
        var outputA = 'ABCD';
        var col = 2;
        ctx.fillText(formatOuput(outputA, col), column(col), line(1));
    }
    {
        var outputB = 'AB';
        var col = 2;
        ctx.fillText(formatOuput(outputB, col), column(col), line(2));
    }
});
function formatOuput(str, col) {
    var output = '';
    for (var i = 0; i < col; i++) {
        output += ' ';
    }
    output += str;
    for (var i = 0; i < MAX_STR_SIZE - str.length; i++) {
        output += ' ';
    }
    return output
}
function column(col) {
    return START_HEIGHT + (FONT_WEIGHT * (col - 1))
}
function line(line) {
    return START_WIDTH + (FONT_SIZE * (line - 1))
}