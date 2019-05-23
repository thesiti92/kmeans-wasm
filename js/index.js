import { run_kmeans } from "../crate/Cargo.toml";

let start_time = Date.now();
let [clusters, assignments] = run_kmeans(4, BigInt(250000));
const rust_runtime = (Date.now() - start_time) / 1000;
console.log(rust_runtime);
document.body.appendChild(
  document.createTextNode(`Rust runtime: ${rust_runtime}`)
);
let traces = assignments.map(assignment => ({
  x: assignment.map(({ x }) => x),
  y: assignment.map(({ y }) => y),
  mode: "markers",
  type: "scattergl"
}));
traces.push({
  x: clusters.map(({ x }) => x),
  y: clusters.map(({ y }) => y),
  mode: "markers",
  type: "scattergl"
});
Plotly.newPlot("plots", traces, { hovermode: false, hoverinfo: "none" });

const graph_runtime = (Date.now() - rust_runtime - start_time) / 1000;
console.log(graph_runtime);
document.body.appendChild(document.createElement("br"));
document.body.appendChild(
  document.createTextNode(`JS runtime: ${graph_runtime}`)
);
