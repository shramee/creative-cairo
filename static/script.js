function prepareAssetsArray() {
  const assets = {
    0: "https://cdn.jsdelivr.net/npm/font-awesome-svg-icons@0.1.0/svg/user.svg",
    1: "https://cdn.jsdelivr.net/npm/font-awesome-svg-icons@0.1.0/svg/asterisk.svg",
    2: "https://cdn.jsdelivr.net/npm/font-awesome-svg-icons@0.1.0/svg/times.svg",
    3: "https://cdn.jsdelivr.net/npm/font-awesome-svg-icons@0.1.0/svg/circle-o.svg",
    dot: "https://cdn.jsdelivr.net/npm/font-awesome-svg-icons@0.1.0/svg/dot-circle-o.svg",
  };

  const assetsArr = [];
  for (const asset in assets) {
    assetsArr.push({
      id: asset,
      src: assets[asset],
      w: 70,
      h: 70,
    });
  }

  return assetsArr;
}

async function setupDumbRenderer() {
  const renderer = new DumbRenderer();

  renderer.setupCanvas({
    w: 512,
    h: 512,
    bg: "#fff",
  });

  // Wait for assets to load
  await renderer.loadAssets(prepareAssetsArray());

  return renderer;
}

window.submitCairo = async () => {
  let cairo = document.getElementById("cairo").value;
  let cairoResp = await fetch("//localhost:8080", {
    method: "POST",
    headers: {},
    body: cairo,
  });

  console.log(cairoResp);
};

function ready(renderer) {}

setupDumbRenderer().then(ready);
