//  Running teh learning tests here

// Adding color chalk
import chalk from "chalk";

// Writing the object

const engine = {
  working: true,
};

const mustang = {
  name: "Mustang",
  engine: engine,
};

const camaro = {
  name: "Camaro",
  engine: engine,
};

function checkEngine(car: { name: string; engine: { working: boolean } }) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}

checkEngine(mustang);
checkEngine(camaro);

console.log(chalk.green(JSON.stringify(mustang, null, 2)));
console.log(chalk.yellow(JSON.stringify(camaro, null, 2)));
