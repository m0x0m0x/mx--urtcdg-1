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

function checkEngine(car) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}

console.log(chalk.green(mustang));
console.log(chalk.blue(camaro));
