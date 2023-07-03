import './style.css'

import init, { Kind, compute, compute_desc, halton_demo, make_fragment, render, svg_test, svg_test_desc, } from '../pkg/sample_rust.js'
import { Context } from '../../../rillus-web/src/main.js';
import { Pane, InputParams, InputBindingApi } from 'tweakpane';

import TinySDF from 'https://cdn.skypack.dev/@mapbox/tiny-sdf';
// const c = new RawHtmlContext();


// function getOrCreate<T extends HTMLElement>(e:HTMLElement|undefined, f: () => T): [T, boolean] {
//   let i: T | undefined = e as T | undefined;
//   let created = false;
//   if (!i) {
//     i = f();
//     created = true;
//   }
//   return [i, created];
// }
// c.registerEditor("i32", (e, p, v, update) => {

//   let [i, created] = getOrCreate<HTMLInputElement>(e,  ()=> document.createElement("input"));
//   if (created) {
//     i.type = "number";
//     i.addEventListener("change", (e: Event): any => update());
//     e = i;
//   }
//   i.valueAsNumber = v as number;

//   return i;
// });


// c.registerEditor("String", (e, p, v, update) => {
//   let [i, created] = getOrCreate<HTMLDivElement>(e,  ()=> document.createElement("div"));
//   if (created) {
//   }
//   i.innerHTML = v;
//   return i;
// });


type Binding = InputBindingApi<unknown, unknown>;


function fromEnum<T extends {}>(t: T) {
  let res: any = {};
  for (const k in t) {// Object.keys(t)) {
    if (typeof (t[k]) === 'number')
      res[k] = t[k];
    // console.log(k, typeof(k), t[k],typeof(t[k]));
  }
  return res;
}
class TweakpaneContext<T, TD> extends Context<T> {
  pane?: Pane;
  div: HTMLDivElement;
  onChange: (data: any, f: T) => any;
  onCreate: (div: HTMLDivElement, pane: Pane) => any;
  onUpdate?: (data: TD) => void;

  constructor(div: HTMLDivElement, f: T,
    onCreate: (div: HTMLDivElement, pane: Pane) => TD,
    onChange: (data: TD, f: T) => any,
    onUpdate?: (data: TD) => void) {
    super(undefined!, f);
    this.div = div;
    this.onCreate = onCreate;
    this.onChange = onChange;
    this.onUpdate = onUpdate;
  }

  // defaultEditor(p: Param): Editor<Binding> {
  //   this.pane!.addBlade({
  //     view: 'text',
  //     disabled: true,
  //     label: p.name,
  //   })
  //   throw new Error('Method not implemented.');
  // }
  // createObject() {
  //   const o = super.createObject();
  //   o.p = { x: o.i, y: o.j };
  //   console.log(o);
  //   return o;
  // }
  // mapObject(o: any) {
  //   o.i = o.p.x;
  //   o.j = o.p.y;
  //   return o;
  // }

  create() {
    const container = document.createElement("div");
    container.className = "tp";
    this.div.appendChild(container);
    this.pane = new Pane({ container: container, title: "SVG" });
    const data = this.onCreate(this.div, this.pane);

    this.pane.on('change', (ev) => {
      // console.log('changed: ' + JSON.stringify(ev.value), data);
      // data.result = this.f(...Object.values(this.mapData(data)));
      this.onChange(data, this.f)
    });
    if (this.onUpdate) {
      (data as any).paused = false;
      this.pane.addInput(data, "paused");
      const update = () => {
        if (!data.paused) {
          this.onUpdate!(data);
          this.onChange(data, this.f);
        }
        requestAnimationFrame(update);
      }

      requestAnimationFrame(update);
    } else {

      this.onChange(data, this.f)
    }
  }
}
function resize<T extends ArrayBufferLike & { length: number, }>(b: T, type: { new(size: number): T }, newSize: number) {
  if (b.length === newSize) {
    return b;
  }
  console.log("resize", b.length, newSize);
  return new type(newSize);

}
init().then(() => {
  //   new TweakpaneContext(document.querySelector<HTMLDivElement>('#compute')!,
  //     svg_test,
  //     (div, pane) => {

  //       const output = document.createElement("div");
  //       div.appendChild(output);
  //       const data = { p: { x: 0, y: 0 }, k: Kind.A, output: output, result: '', t: 0, pane: pane };

  //       pane.addInput(data, 'p', {
  //         picker: 'inline',
  //         expanded: true,
  //         x: { min: 0, max: 100 },
  //         y: { min: 0, max: 100 },
  //       })
  //       pane.addInput(data, 'k', { options: fromEnum(Kind) })
  //       const mt = pane.addMonitor(data, 't');
  //       pane.addMonitor(data, 'result', {
  //         multiline: true,
  //         lineCount: 5,
  //       });


  //       return data;
  //     },
  //     (data, f) => {
  //       data.output.innerHTML = data.result = f(data.p.x + Math.cos(data.t) * 10, data.p.y + Math.sin(data.t) * 10, data.k);

  //     },
  //     data => {
  //       data.t += 0.16;

  //     }
  //   ).create();


  //   new TweakpaneContext(
  //     document.querySelector<HTMLDivElement>('#svg')!,
  //     halton_demo,
  //     (div, pane) => {
  //       const data = { step: 1, count: 20, svg: document.createElement('div'), buffer: new Float64Array(40) };
  //       pane.addInput(data, "step", { step: 1 });
  //       pane.addInput(data, "count", { step: 1, min: 1, max: 100 });
  //       div.appendChild(data.svg);
  //       return data;
  //     },
  //     (data, f) => {
  //       data.buffer = resize(data.buffer, Float64Array, data.count * 2);
  //       f(data.step, data.buffer);
  //       let points = "";
  //       for (let i = 0; i < data.buffer.length / 2; i++)
  //         points += `<circle r="0.01" cx="${data.buffer[i * 2]}" cy="${data.buffer[i * 2 + 1]}"/>`;
  //       data.svg.innerHTML = `<svg  viewBox="0 0 1 1" xmlns="http://www.w3.org/2000/svg">
  //       ${points}
  //       </svg>`;
  //     }
  //   ).create();
  // });

  // new TweakpaneContext(
  //   document.querySelector<HTMLDivElement>('#frag')!,
  //   make_fragment,
  //   (div, pane) => {

  //     const output = document.createElement("div");
  //     div.appendChild(output);
  //     const data = {div:output, x: 50, t: 0};
  //     pane.addInput(data, "x");
  //     pane.addMonitor(data, "t");
  //     return data;
  //   },
  //   (data, f) => {
  //     const fragment = f(data.x, data.t);
  //     if(data.div.hasChildNodes())
  //       data.div.replaceChild(fragment, data.div.firstChild!)
  //       else
  //       data.div.appendChild(fragment);
  //   },
  //       data => {
  //         data.t += 0.16;

  //       }
  // ).create();

  const WIDTH = 32 * 8;
  const HEIGHT = 32 * 8;
  new TweakpaneContext(
    document.querySelector<HTMLDivElement>('#render')!,
    render,
    (div, pane) => {

      const canvas = document.createElement("canvas");
      canvas.width = WIDTH;
      canvas.height = HEIGHT;
      div.appendChild(canvas);



      const arrayBuffer = new ArrayBuffer(WIDTH * HEIGHT * 4);
      const fireBuffer = new ArrayBuffer(WIDTH * HEIGHT);

      const tinySdf = new TinySDF({
        fontSize: 24,             // Font size in pixels
        fontFamily: 'sans-serif', // CSS font-family
        fontWeight: 'normal',     // CSS font-weight
        fontStyle: 'normal',      // CSS font-style
        buffer: 3,                // Whitespace buffer around a glyph in pixels
        radius: 8,                // How many pixels around the glyph shape to use for encoding distance
        cutoff: 0.25              // How much of the radius (relative) is used for the inside part of the glyph
      });

      const glyph = tinySdf.draw('z'); // draw a single character

      const data = {
        canvas: canvas,
        ctx: canvas.getContext('2d')!,
        x: 0.995,
        t: 0,
        r: 5,
        b: arrayBuffer,
        fireBuffer: fireBuffer,
        mousePos: [0, 0],
        glyph: glyph,
        cutoff: 40,
      };
      console.log(glyph)
      pane.addInput(data, "r", { min: 1, max: 50, step: 1 });
      pane.addInput(data, "x", { min: 0.9, max: 1, step: 0.005 });
      pane.addInput(data, "cutoff", { min: -200, max: 250, step: 1 });
      pane.addMonitor(data, "t");


      canvas.addEventListener('mousemove', e => {
        const bb = canvas.getBoundingClientRect();
        const x = Math.floor((e.clientX - bb.left) / bb.width * canvas.width);
        const y = Math.floor((e.clientY - bb.top) / bb.height * canvas.height);

        data.mousePos = [x, y];
      });

      return data;
    },
    (data, f) => {
      const pixels = new Uint8ClampedArray(data.b);
      const b = new ImageData(pixels, WIDTH, HEIGHT);
      const fb = new Uint8Array(data.fireBuffer);
      const r = data.r;

      // draw letter
      const s = [WIDTH / 2, HEIGHT / 2];
      for (let x = 0; x < data.glyph.height; x++) {
        for (let y = 0; y < data.glyph.width; y++) {
          const d = data.glyph.data[y * data.glyph.width + x];
          if (d > data.cutoff) 
          {
            const i = Math.min(HEIGHT - 1, Math.max(0, s[1] + y)) * WIDTH +
              (s[0] + x) % WIDTH;
            fb[i] = Math.min(254, Math.max(fb[i], Math.random() * Math.pow(d / 255.0, 5) * 255));
          }
          // fb[i] +=  Math.random() * Math.pow(d / 255.0, 2) * 255;
        }
      }

      // circle around mouse pos
      for (let x = -r; x < r; x++) {
        for (let y = -r; y < r; y++) {
          const d = x * x + y * y;
          if (d <= r * r)
            fb[Math.min(HEIGHT - 1, Math.max(0, data.mousePos[1] + y)) * WIDTH +
              ((data.mousePos[0] + x) % WIDTH)] =
              Math.random() * 255 * ((d / (r)));

        }
      }


      f(data.x, new Uint8Array(data.b), fb, WIDTH, HEIGHT);

      data.ctx.putImageData(b, 0, 0);
    },
    data => {
      data.t += 0.16;

    }
  ).create();
});
