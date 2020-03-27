
import { Client } from "rcubes";

let client = Client.new();
let animationId = null;

const renderLoop = () => {
    fps.render();

    client.main_loop(fps.delta);

    animationId = requestAnimationFrame(renderLoop);
}
setTimeout(() => {
    renderLoop();    
}, 500);

// setTimeout(() => {

//     client = Client.new();

//     renderLoop();

// }, 500);

// FPS
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStep = performance.now();
        this.delta = 0;
        this.showStats = true;
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStep;
        this.lastFrameTimeStep = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100) {
          this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        this.delta = delta;
        
        if (this.showStats) {
            // Render the statistics.
            this.fps.textContent = 
                        `
                        Frames per Second:
                                latest = ${Math.round(fps)}
                        avg of last 100 = ${Math.round(mean)}
                        min of last 100 = ${Math.round(min)}
                        max of last 100 = ${Math.round(max)}
                        `.trim();
        }
    }
}