let lastTime = 0;

let processes = [];

let at = 0;
let aimingAt = 0;
let sizeAll = 0;

let posX = 0;
let posY = 0;

let maxSlot = 100;
let startSlot = 3;

let countZero = 0;
let connected = false;

let selected = 0;

let webSoc;

let allZero = [];
let allNeg = [];

document.addEventListener('contextmenu', event => event.preventDefault());

function setup() {
  createCanvas(windowWidth, windowHeight);
  maxSlot = round(width / 20);

  newWebSoc();
  frameRate(30);

  processes.push(new graphProcess(1000, color(0, 255, 255), "MEMORY", " KB"));
  processes.push(new graphProcess(5, color(0, 128, 255), "CPU   ", " %"));
  processes.push(new graphProcess(10000, color(0, 255, 0), "READ  ", " B"));
  processes.push(new graphProcess(10000, color(255, 0, 0), "WRITE ", " B"));

  for (let i = 0; i < processes.length; i++) {
    allZero.push(0);
    allNeg.push(-100);
  }

  for (let i = 0; i < startSlot; i++) {
    pushToProcesses(allZero);
  }

  lastTime = millis();
}

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
  maxSlot = round(width / 20);
}

function pushToProcesses(values) {
  let isZero = true;

  for (let j = 0; j < processes.length; j++) {
    processes[j].push(values[j]);

    if (values[j] != 0) {
      isZero = false;
    }
  }

  if (isZero) {
    countZero++;
  } else {
    countZero = 0;
  }

  if (processes[selected].show.length >= maxSlot && aimingAt + 5 >= processes[selected].all.length - processes[selected].show.length) {
    aimingAt++;
    sizeAll++;
  }
}

function draw() {
  background(0);

  if (!connected && countZero < maxSlot) {
    pushToProcesses(allZero);
  }

  if (processes[selected].show.length > startSlot) {
    processes[selected].calculMinMax();
  }

  if (at != aimingAt) {
    at = lerp(at, aimingAt, 0.3);
    processes[selected].update();
  }

  processes[selected].render();

  textSize(50);
  for (let j = 0; j < processes.length; j++) {
    processes[j].showValue(j * textSize());
  }

  strokeWeight(5);
  stroke(255);

  if (connected) {
    fill(255);
  } else {
    if (sq(mouseX - (width - 30)) + sq(mouseY - 30) <= sq(25)) {
      fill(255);
    } else {
      fill(0);
    }
  }

  ellipse(width - 30, 30, 50, 50);
}

function mouseWheel(event) {
  aimingAt = constrain(aimingAt - int(event.delta / 10), 0, processes[selected].all.length - processes[selected].show.length);
}

function mouseClicked(fxn) {
  if (!connected && sq(mouseX - (width - 30)) + sq(mouseY - 30) <= sq(25)) {
    newWebSoc();
  }
}

function keyReleased() {
  let buf = selected;

  switch (keyCode) {
    case DOWN_ARROW:
    case LEFT_ARROW:
      selected = constrain(selected + 1, 0, processes.length - 1);
      break;
    case UP_ARROW:
    case RIGHT_ARROW:
      selected = constrain(selected - 1, 0, processes.length - 1);
      break;
  }

  if (buf != selected) {
    processes[selected].update();
  }
}