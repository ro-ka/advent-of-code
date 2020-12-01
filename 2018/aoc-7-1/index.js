const fs = require('fs');

const input = fs.readFileSync('./input.txt', 'utf8');
const steps = {};
input.split('\n').forEach(raw => {
  if (!raw) return;

  const step = raw[36];
  const finishFirst = raw[5];

  steps[step] = steps[step]
    ? steps[step] += finishFirst
    : finishFirst;
});

const allSteps = [...getFreeSteps(), ...Object.keys(steps)];
let doneSteps = '';

findNextStep();
console.log(doneSteps);

function getFreeSteps() {
  return Object.values(steps)
    .reduce((all, step) => all += step, '')
    .split('')
    .filter(step => !steps[step]);
}

function findNextStep() {
  const possibleSteps = allSteps.filter(step => {
    return !doneSteps.includes(step) && (
      !steps[step] ||
      (steps[step] && steps[step].split('').every(finishFirst => doneSteps.includes(finishFirst)))
    )
  });
  if (!possibleSteps.length) return;
  const nextStep = possibleSteps.sort()[0];
  doneSteps += nextStep;
  findNextStep();
}
