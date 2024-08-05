const START_WIDTH = 40;
const START_HEIGHT = -5;
const FONT_SIZE = 50;
const FONT_WEIGHT = 33;

const MAX_STR_SIZE = 37;
document.addEventListener("DOMContentLoaded", function () {
    function line(line) {
        return START_WIDTH + (FONT_SIZE * (line - 1))
    }
    function column(col) {
        return START_HEIGHT - (FONT_WEIGHT * (MAX_STR_SIZE - col))
    }
    function printChar(c, col, lin) {
        ctx.fillText(c, column(col), line(lin));
    }
    function printString(str, col, lin) {
        for (var i = 0; i < str.length; i++) {
            var c = str[i];
            printChar(c, col + i, lin)
        }

    }

    const canvas = document.getElementById('myCanvas');
    const ctx = canvas.getContext('2d');

    const canvasFontStyle = '' + FONT_SIZE + 'px Monospace';
    // Clear the canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.rotate(3 * Math.PI / 2);

    ctx.textAlign = "right";
    ctx.fillStyle = '#000';
    ctx.font = canvasFontStyle;

    /// BEGIN
    // Print rulers
    {
        printString('000000000111111111122222222223333333', 3, 1);
        printString('123456789012345678901234567890123456', 3, 2);
        for (var i = 1; i < 20; i++) {
            if (i < 10) {
                printChar('0', 1, 2 + i);
            }
            printChar(i, 2, 2 + i);
        }
    }
    printChar('A', 5 + 2, 5 + 2);
    printString('Hello', 6 + 2, 6 + 2);
});
