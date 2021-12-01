const { readFileSync } = require("fs");

const numbers = readFileSync("./numbers.txt");
  
let prev = Number.MAX_VALUE;


const timesHigher = numbers.toString().split("\n").reduce((acc, curr) => {
  let int = parseInt(curr);
  if (int > prev) {
    acc++; 
  }
  prev = curr;
  return acc;
}, 0);

console.log(
  `List showed higher values ${timesHigher} times`
);
