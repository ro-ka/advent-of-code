const TARGET = 290431;

let recipes = '37';

const elfIndex = [0, 1];

while (recipes.length <= TARGET + 10) {
  const recipeElf1 = Number(recipes[elfIndex[0]]);
  const recipeElf2 = Number(recipes[elfIndex[1]]);
  recipes += String(recipeElf1 + recipeElf2);

  elfIndex[0] = elfIndex[0] + (recipeElf1 + 1) % recipes.length;
  elfIndex[1] = elfIndex[1] + (recipeElf2 + 1) % recipes.length;
  if (elfIndex[0] > recipes.length - 1) {
    elfIndex[0] = elfIndex[0] - recipes.length;
  }
  if (elfIndex[1] > recipes.length - 1) {
    elfIndex[1] = elfIndex[1] - recipes.length;
  }
}

console.log(recipes.substr(TARGET, 10));
