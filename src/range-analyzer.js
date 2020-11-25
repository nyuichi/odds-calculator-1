onmessage = (ev) => {
  compute(ev.data);
};

async function compute(data) {
  const mod = await import("../crate/pkg/wasm");
  postMessage(mod.analyze_range(data));
}
