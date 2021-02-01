
function newWebSoc() {
    aimingAt = sizeAll;
    webSoc = new WebSocket('ws://localhost:8889');

    webSoc.onopen = function (event) {
        console.log("WebSocket is open now.");
        connected = true;
        countZero = 0;
    };

    webSoc.onclose = function (event) {
        console.log("WebSocket is now closed.");
        connected = false;

        pushToProcesses(allZero);
    };

    webSoc.onmessage = function (event) {
        let json = JSON.parse(event.data);
        let tab = [json.memory, json.cpu, json.read, json.write];

        pushToProcesses(tab);

        lastTime = millis();
    };
}