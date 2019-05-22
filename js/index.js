import module from "../crate/Cargo.toml";

let points = [
  { x: 1, y: 2 },
  { x: 2, y: 2 },
  { x: 2, y: 1 },
  { x: 10, y: 20 },
  { x: 10, y: 21 }
];
console.log(module.run_kmeans(points, 2));
