const { readFileSync } = require("fs");

const numbers = readFileSync("./numbers.txt");
  
let prev = Number.MAX_VALUE;
let timesHigher = 0;

const numbersAsInts = numbers.toString().split("\n").map((curr) => parseInt(curr));


for (let i=2; i < numbersAsInts.length; i++) {
  let curr = numbersAsInts[i-2] + numbersAsInts[i-1] + numbersAsInts[i];
  if (curr > prev) {
    timesHigher++;
  }
  prev = curr;
}

console.log(
  `List showed higher values ${timesHigher} times`
);
