const wasm = require('./main.rs');

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const getValues = () => {
  return {
    rays: parseInt(document.getElementById('rays').value, 10) || 1,
  };
};

wasm.initialize({ noExitRuntime: true }).then(module => {
  const render = module.cwrap('render_serial_scene', null, [
    'string',
    'number',
    'number',
    'number',
  ]);

  const width = 600;
  const height = 600;
  const bufsize = width * height * 4;

  const targetPtr = module._malloc(bufsize);
  module._memset(targetPtr, 0, bufsize);

  let buf = new Uint8Array(module.HEAPU8.buffer, targetPtr, bufsize);

  window.render = () => {
    //const scene = JSON.stringify({
    //objects: {
    //type: 'circle',
    //center: {
    //x: 0.5,
    //y: 0.5,
    //},
    //radius: 0.1,
    //material: {
    //ambient: {},
    //},
    //},
    //});

    const scene = '';

    console.log(getValues());

    console.time('render scene');
    render(scene, targetPtr, width, height);
    console.timeEnd('render scene');

    const image = ctx.createImageData(width, height);

    //console.log(got.split('').map(c => c.charCodeAt(0)));

    console.time('push screen');
    for (let i = 0; i < bufsize / 4; i++) {
      image.data[i * 4] = buf[i * 4];
      image.data[i * 4 + 1] = buf[i * 4 + 1];
      image.data[i * 4 + 2] = buf[i * 4 + 2];
      image.data[i * 4 + 3] = buf[i * 4 + 3];
    }

    ctx.putImageData(image, 0, 0);
    console.timeEnd('push screen');
  };

  window.render();
});
