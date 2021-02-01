class graphProcess {
    constructor(scale, col, name, text) {
        this.show = [];
        this.all = [];

        this.maxShow = 10000;
        this.minShow = 0;

        this.showLerp = 0;

        this.scale = scale;

        this.fillCol = color(red(col) / 3, green(col) / 3, blue(col) / 3, 150);
        this.strokeCol = color(red(col), green(col), blue(col), 255);
        this.lineCol = color(red(col) / 5, green(col) / 5, blue(col) / 5, 200);

        this.name = name;
        this.text = text;
    }

    push(value) {
        if (this.show.length < maxSlot) {
            this.show.push(value);
        }

        this.all.push(value);
        this.showLerp = lerp(this.showLerp, value, 0.2);
    }

    calculMinMax() {
        let newMaxShow = 0;
        let newMinShow = 1000000000;

        for (let i = 0; i < this.show.length; i++) {
            if (max(0, this.show[i]) < max(0, newMinShow)) {
                newMinShow = max(0, this.show[i]);
            } else if (max(0, this.show[i]) > max(0, newMaxShow)) {
                newMaxShow = max(0, this.show[i]);
            }
        }

        let diff = max((newMaxShow - newMinShow) / 100, this.scale / 8);

        newMaxShow += diff * 10;
        newMinShow -= diff * 5;

        if (this.maxShow == 0) {
            this.maxShow = newMaxShow;
            this.minShow = newMinShow;
        } else {
            this.maxShow = lerp(this.maxShow, newMaxShow, 0.2);
            this.minShow = lerp(this.minShow, newMinShow, 0.2);
        }
    }

    update() {
        this.show = this.all.slice(at, at + this.show.length);
    }

    render() {
        let size = width / (maxSlot - 3);
        let hSize = height / ((this.maxShow - this.minShow));

        strokeWeight(2);
        stroke(this.lineCol);

        let step = max(2, this.scale * hSize);
        for (let y = (height + (this.minShow * hSize)) % step; y < height; y += step) {
            line(0, y, width, y);
        }

        push();

        let trans = -(at % 1) * size;
        translate(trans, 0);

        for (let x = 0; x < width; x += size) {
            line(x, 0, x, height);
        }

        fill(this.fillCol);
        stroke(this.strokeCol);
        strokeWeight(5);

        beginShape();

        vertex(-size, height * 2);
        vertex(-size, height * 2);

        vertex(-size, height - ((this.show[0] - this.minShow) * hSize));

        fill(this.strokeCol);
        for (let i = 0; i < this.show.length - 1; i++) {
            curveVertex(size * (i - 1), height - ((this.show[i] - this.minShow) * hSize));
            ellipse(size * (i - 1), height - ((this.show[i] - this.minShow) * hSize), 7.5, 7.5);
        }
        fill(this.fillCol);

        curveVertex(size * (this.show.length - 1), height - ((this.show[this.show.length - 1] - this.minShow) * hSize));
        vertex(size * (this.show.length), height - ((this.show[this.show.length - 1] - this.minShow) * hSize));

        vertex(size * (this.show.length), height * 2);
        vertex(size * (this.show.length), height * 2);

        endShape(CLOSE);

        let pos = constrain(round((mouseX - trans) / size) + 1, 0, this.show.length - 1);

        let x = (pos - 1) * size;
        let y = height - ((this.show[pos] - this.minShow) * hSize);

        if (!isNaN(y) && !isNaN(x)) {
            posX = lerp(posX, x, 0.3);
            posY = lerp(posY, y, 0.3);
        }

        strokeWeight(2.5);
        ellipse(x, y, 15, 15);

        strokeWeight(5);

        textSize(50);
        textAlign(CENTER, BOTTOM);
        text(this.show[pos] + this.text, posX, posY - 10);

        pop();
    }

    showValue(decal) {
        fill(this.fillCol);
        stroke(this.strokeCol);
        strokeWeight(2);

        textAlign(LEFT, TOP);
        text(this.name, 5, 5 + decal);
        text(round(this.showLerp) + this.text, 5 + this.name.length * textSize(), 5 + decal);
    }
}