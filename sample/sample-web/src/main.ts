import './style.css'

import init, { Kind, compute, compute_desc, halton_demo, svg_test, svg_test_desc, } from '../pkg/sample_rust.js'
import { Context } from '../../../rillus-web/src/main.js';
import { Pane, InputParams, InputBindingApi } from 'tweakpane';

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
  onChange: (data: any, f: T, nextFrame: () => void) => any;
  onCreate: (div: HTMLDivElement, pane: Pane) => any;

  constructor(div: HTMLDivElement, f: T,
    onCreate: (div: HTMLDivElement, pane: Pane) => TD,
    onChange: (data: TD, f: T, nextFrame: () => void) => any) {
    super(undefined!, f);
    this.div = div;
    this.onCreate = onCreate;
    this.onChange = onChange;
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
    const update = () => {
      this.onChange(data, this.f, () => update())

    };
    this.pane.on('change', (ev) => {
      // console.log('changed: ' + JSON.stringify(ev.value), data);
      // data.result = this.f(...Object.values(this.mapData(data)));
      this.onChange(data, this.f, () => {})
    });
    update()
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
  new TweakpaneContext(document.querySelector<HTMLDivElement>('#compute')!,
    svg_test,
    (div, pane) => {

      const output = document.createElement("div");
      div.appendChild(output);
      const data = { p: { x: 0, y: 0 }, k: Kind.A, output: output, result: '', t: 0, pane: pane };

      pane.addInput(data, 'p', {
        picker: 'inline',
        expanded: true,
        x: { min: 0, max: 100 },
        y: { min: 0, max: 100 },
      })
      pane.addInput(data, 'k', { options: fromEnum(Kind) })
      const mt = pane.addMonitor(data, 't');
      pane.addMonitor(data, 'result', {
        multiline: true,
        lineCount: 5,
      });


      return data;
    },
    (data, f, update) => {
      data.output.innerHTML = data.result = f(data.p.x + Math.cos(data.t) * 10, data.p.y + Math.sin(data.t) * 10, data.k);
      const frame = () => {
        data.t += 0.16;
        
        update();
      };
      requestAnimationFrame(frame);
    },
  ).create();


  new TweakpaneContext(
    document.querySelector<HTMLDivElement>('#svg')!,
    halton_demo,
    (div, pane) => {
      const data = { step: 1, count: 20, svg: document.createElement('div'), buffer: new Float64Array(40) };
      pane.addInput(data, "step", { step: 1 });
      pane.addInput(data, "count", { step: 1, min: 1, max: 100 });
      div.appendChild(data.svg);
      return data;
    },
    (data, f, update) => {
      data.buffer = resize(data.buffer, Float64Array, data.count * 2);
      f(data.step, data.buffer);
      // console.log(, arr);
      let points = "";
      for (let i = 0; i < data.buffer.length / 2; i++)
        points += `<circle r="0.01" cx="${data.buffer[i * 2]}" cy="${data.buffer[i * 2 + 1]}"/>`;
      data.svg.innerHTML = `<svg  viewBox="0 0 1 1" xmlns="http://www.w3.org/2000/svg">
      ${points}
      </svg>`;
      // console.log(points);
    }
  ).create();
});
