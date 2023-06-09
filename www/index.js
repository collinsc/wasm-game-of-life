import { Universe,  CreationStrategy } from "collinsc-wasm-game-of-life";
import { memory } from "collinsc-wasm-game-of-life/collinsc_wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC"
const DEAD_COLOR = "#FFFFFF"
const ALIVE_COLOR = "#00A7E1"

const width = 64
const height = 64
const universe = Universe.new(width, height)
universe.init(CreationStrategy.FiftyFifty)

const canvas = document.getElementById("game-of-life-canvas")
canvas.height = (CELL_SIZE + 1) * height + 1
canvas.width = (CELL_SIZE + 1) * width + 1

const ctx = canvas.getContext('2d')
let ct = 0
const renderLoop = () => {
  ct = (ct + 1) % 1000
  if (ct === 0) {
    universe.init(CreationStrategy.FiftyFifty)
  }
  universe.tick()
  drawGrid()
  drawCells()

  requestAnimationFrame(renderLoop)
}

const drawGrid = () => {
  ctx.beginPath()
  ctx.strokeStyle = GRID_COLOR

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0)
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const drawCells = () => {
  const cellsPtr = universe.cell_ptr();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
  ctx.beginPath();
  let n
  let mask
  let isSet
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      n = row * width + col;
      mask = 1 << (n % 8);
      isSet = (cells[Math.floor(n / 8)] & mask) === mask;

      ctx.fillStyle = isSet === false
        ? DEAD_COLOR
        : ALIVE_COLOR

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }

  ctx.stroke()
}

drawGrid()
drawCells()
requestAnimationFrame(renderLoop)