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
  image_plane: {
    rays: 32,
    x1: { x: 0.05, y: 0.6 },
    x2: { x: 0.95, y: 0.6 },
  },
  cam: {
    pos: { x: 0.5, y: 0.98 },
  },
  lights: [
    {
      position: { x: 0.3, y: 0.9 },
      diffuse: { r: 0.8, g: 0.8, b: 0.8 },
      specular: { r: 0.8, g: 0.8, b: 0.8 },
    },
  ],
  objects: [
    {
      type: 'circle',
      center: { x: 0.17, y: 0.1 },
      radius: 0.25,
      material: {
        ambient: { r: 1, g: 0, b: 0 },
        diffuse: { r: 1, g: 0, b: 0 },
        specular: { r: 0, g: 0, b: 0 },
        shininess: 0,
        reflectivity: 0,
      },
    },
    {
      type: 'circle',
      center: { x: 0.5, y: 0.3 },
      radius: 0.08,
      material: {
        ambient: { r: 0, g: 1, b: 0 },
        diffuse: { r: 0, g: 1, b: 0 },
        specular: { r: 0.8, g: 0.8, b: 0.8 },
        shininess: 0,
        reflectivity: 1,
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
};

let sceneOptionsRoot;

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

  const targets = []
    .concat(scene.objects.map(o => o.center))
    .concat(scene.lights.map(l => l.position))
    .concat([scene.cam.pos, scene.image_plane.x1, scene.image_plane.x2]);

  for (const so of targets) {
    if (Math.abs(x - so.x) < 0.025 && Math.abs(y - so.y) < 0.025) {
      obj = so;
    }
  }

  if (!obj) {
    return;
  }

  const drag = e => {
    const { x, y } = realPos(e);
    obj.x = x;
    obj.y = y;

    sceneOptionsRoot.parentNode.removeChild(sceneOptionsRoot);
    sceneOptionsRoot = sceneOptions('scene');

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

const render = () => {
  const serializedScene = JSON.stringify(scene);

  console.time('render scene');
  ffi.render(serializedScene, ffi.bufferPtr, width, height);
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

const numberOption = (value, change, target) => {
  const input = document.createElement('input');
  input.setAttribute('type', 'number');
  input.setAttribute('value', value);
  input.addEventListener('change', e => change(parseFloat(e.target.value, 10)));

  target.appendChild(input);
};

const coordOption = (value, change, target) => {
  target.appendChild(document.createTextNode('('));
  numberOption(value.x, x => change({ x, y: value.y }), target);
  target.appendChild(document.createTextNode(', '));
  numberOption(value.y, y => change({ x: value.x, y }), target);
  target.appendChild(document.createTextNode(')'));
};

const colorOption = (value, change, target) => {
  target.appendChild(document.createTextNode('('));
  numberOption(value.r, r => change({ r, g: value.g, b: value.b }), target);
  target.appendChild(document.createTextNode(', '));
  numberOption(value.g, g => change({ r: value.r, g, b: value.b }), target);
  target.appendChild(document.createTextNode(', '));
  numberOption(value.b, b => change({ r: value.r, g: value.g, b }), target);
  target.appendChild(document.createTextNode(')'));
};

const defaultObjects = {
  objects: {
    type: 'circle',
    center: { x: 0.5, y: 0.5 },
    radius: 0.1,
    material: {
      ambient: { r: 1, g: 0, b: 1 },
      diffuse: { r: 1, g: 0, b: 1 },
      specular: { r: 0, g: 0, b: 0 },
      shininess: 0,
      reflectivity: 0,
    },
  },

  lights: {
    position: { x: 0.5, y: 0.5 },
    diffuse: { r: 0.5, g: 0.5, b: 0.5 },
    specular: { r: 0.5, g: 0.5, b: 0.5 },
  },
};

const sceneOptions = (
  key,
  obj = scene,
  change,
  removable = false,
  target = document.body,
) => {
  let collapsed = false;

  const container = document.createElement('div');
  container.className = 'option-container';
  target.appendChild(container);

  const onChange = (k, v, redraw = false) => {
    if (v === undefined && Array.isArray(obj)) {
      k = parseInt(k, 10);
      return change(obj.slice(0, k).concat(obj.slice(k + 1)), true);
    }

    obj[k] = v;

    if (redraw) {
      clear();
      addChildren();
    }

    render();
  };

  const clear = () => {
    while (container.firstChild) {
      container.removeChild(container.firstChild);
    }
  };

  const addChildren = () => {
    const collapse = document.createElement('span');
    collapse.textContent = (collapsed ? ' > ' : '') + key + ': ';
    container.appendChild(collapse);

    if (Array.isArray(obj)) {
      const add = document.createElement('span');
      add.textContent = '(+)';
      add.addEventListener('click', () => {
        obj.push(JSON.parse(JSON.stringify(defaultObjects[key])));
        clear();
        addChildren();
        render();
      });
      container.appendChild(add);
    }

    if (removable) {
      const remove = document.createElement('span');
      remove.textContent = '(x)';
      remove.addEventListener('click', () => {
        clear();
        change(undefined);
      });
      container.appendChild(remove);
    }

    if (typeof obj === 'object') {
      if (obj.x !== undefined && obj.y !== undefined) {
        coordOption(obj, v => change(v), container);
      } else if (
        obj.r !== undefined &&
        obj.g !== undefined &&
        obj.b !== undefined
      ) {
        colorOption(obj, v => change(v), container);
      } else {
        collapse.addEventListener('click', e => {
          collapsed = !collapsed;
          clear();
          addChildren();
        });

        if (collapsed) {
          return;
        }

        for (const key in obj) {
          sceneOptions(
            key,
            obj[key],
            onChange.bind(null, key),
            Array.isArray(obj),
            container,
          );
        }
      }
    } else if (typeof obj === 'number') {
      numberOption(obj, v => change(v), container);
    } else if (typeof obj === 'string') {
      const str = document.createElement('span');
      str.textContent = `"${obj}"`;
      container.appendChild(str);
    }
  };

  addChildren();

  return container;
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
sceneOptionsRoot = sceneOptions('scene');
