onmessage = (ev) => {
  compute(ev.data);
};

async function compute(data) {
  const mod = await import("../crate/pkg/wasm");
  postMessage(mod.compute_win_rate(data));
}
