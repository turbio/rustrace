const wasm = require('./main.rs');

const canvas = document.getElementById('tutorial');
const ctx = canvas.getContext('2d');

wasm.initialize({ noExitRuntime: true }).then(module => {
  const render = module.cwrap('render_serial_scene', 'array', ['string']);
  const give = JSON.stringify({
    objects: {
      type: 'circle',
      center: {
        x: 0.5,
        y: 0.5,
      },
      radius: 0.1,
      material: {
        ambient: {},
      },
    },
  });
  const got = render(give);
  console.log(`-> "${give}"`);
  console.log(`<- ${typeof got} "${got}"`);

  //console.log(got.split('').map(c => c.charCodeAt(0)));

  ctx.fillStyle = 'rgb(200, 0, 0)';
  ctx.fillRect(1, 1, 1, 1);
});
