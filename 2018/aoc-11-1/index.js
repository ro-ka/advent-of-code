const SERIAL = 7139;

const grid = {};

for (let x = 1; x <= 300; x++) {
  grid[x] = {};

  for (let y = 1; y <= 300; y++) {
    grid[x][y] = getPowerForCell(x, y);
  }
}

let maxPower = 0;
const maxPowerCoords = {x: 0, y: 0};

for (let x = 1; x <= 298; x++) {
  for (let y = 1; y <= 298; y++) {
    const totalPower = getTotalPowerForGrid(x, y);

    if (totalPower > maxPower) {
      maxPower = totalPower;
      maxPowerCoords.x = x;
      maxPowerCoords.y = y;
    }
  }
}

console.log(maxPower, maxPowerCoords);

function getPowerForCell(x, y) {
  const rackId = x + 10;
  const powerLevel = (rackId * y + SERIAL) * rackId;
  const powerLevelString = String(powerLevel);
  const hundred = Number(powerLevelString[powerLevelString.length - 3]) || 0;
  return hundred - 5;
}

function getTotalPowerForGrid(x, y) {
  let totalPower = 0;

  for (let i = 0; i <= 2; i++) {
    for (let k = 0; k <= 2; k++) {
      totalPower += grid[x+i][y+k];
    }
  }

  return totalPower;
}
