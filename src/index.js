const wasm = require('./main.rs');

// this will be populated once the wasm module loads
const ffi = {
  render: () => {},
  buffer: null,
  bufferPtr: null,
};

const width = 600;
const height = 600;
const colorWidth = 4;
const bufsize = width * height * colorWidth;

const canvas = document.getElementById('canvas');

const scene = {
  objects: [
    {
      type: 'circle',
      center: { x: 0.5, y: 0.3 },
      radius: 0.08,
      material: {
        ambient: { r: 1, g: 0, b: 0 },
        diffuse: { r: 1, g: 0, b: 0 },
        specular: { r: 0, g: 0, b: 0 },
        shininess: 0,
        reflectivity: 0.3,
      },
    },
    {
      type: 'circle',
      center: { x: 0.17, y: 0.1 },
      radius: 0.25,
      material: {
        ambient: { r: 0, g: 1, b: 0 },
        diffuse: { r: 0, g: 1, b: 0 },
        specular: { r: 0.8, g: 0.8, b: 0.8 },
        shininess: 20,
        reflectivity: 0.5,
      },
    },
    {
      type: 'circle',
      center: { x: 0.75, y: 0.3 },
      radius: 0.09,
      material: {
        ambient: { r: 0, g: 0, b: 1 },
        diffuse: { r: 0, g: 0, b: 1 },
        specular: { r: 0, g: 0, b: 0 },
        shininess: 0,
        reflectivity: 0,
      },
    },
  ],
  cam: {
    pos: {
      x: 0.2,
      y: 0.98,
    },
  },
  rays: 32,
};

const ctx = canvas.getContext('2d');

const realPos = e => {
  const rect = canvas.getBoundingClientRect();

  return {
    x: (e.clientX - rect.left) / 600,
    y: (e.clientY - rect.top) / 600,
  };
};

const startDrag = e => {
  let obj;

  const { x, y } = realPos(e);

  if (
    Math.abs(x - scene.cam.pos.x) < 0.025 &&
    Math.abs(y - scene.cam.pos.y) < 0.025
  ) {
    obj = scene.cam.pos;
  }

  if (!obj) {
    return;
  }

  const drag = e => {
    const { x, y } = realPos(e);
    obj.x = x;
    obj.y = y;
    render();
  };

  drag(e);

  const done = () => {
    canvas.removeEventListener('mousemove', drag);
    canvas.removeEventListener('mouseup', done);
  };

  canvas.addEventListener('mousemove', drag);
  canvas.addEventListener('mouseup', done);
};

const getScene = () => {
  scene.rays = parseInt(document.getElementById('rays').value, 10) || 1;

  return scene;
};

const render = () => {
  const scene = JSON.stringify(getScene());

  console.time('render scene');
  ffi.render(scene, ffi.bufferPtr, width, height);
  console.timeEnd('render scene');

  const image = ctx.createImageData(width, height);

  console.time('push screen');
  for (let i = 0; i < bufsize / colorWidth; i++) {
    image.data[i * colorWidth] = ffi.buffer[i * colorWidth];
    image.data[i * colorWidth + 1] = ffi.buffer[i * colorWidth + 1];
    image.data[i * colorWidth + 2] = ffi.buffer[i * colorWidth + 2];
    image.data[i * colorWidth + 3] = ffi.buffer[i * colorWidth + 3];
  }

  ctx.putImageData(image, 0, 0);
  console.timeEnd('push screen');
};

wasm.initialize({ noExitRuntime: true }).then(module => {
  ffi.render = module.cwrap('render_serial_scene', null, [
    'string',
    'number',
    'number',
    'number',
  ]);

  ffi.bufferPtr = module._malloc(bufsize);
  module._memset(ffi.bufferPtr, 0, bufsize);
  ffi.buffer = new Uint8Array(module.HEAPU8.buffer, ffi.bufferPtr, bufsize);

  render();
});

canvas.addEventListener('mousedown', startDrag);
window.render = render;
