
import { Client } from "rcubes";

let client = null;

function renderLoop() {

    client.main_loop();
    
    if (client.is_running()) {
        window.requestAnimationFrame(renderLoop);
    }
    
}

setTimeout(() => {

    client = Client.new();

    renderLoop();

}, 500);