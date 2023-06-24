import './style.css'

import init, { Kind, compute, compute_desc, do_setup, svg_test, svg_test_desc, } from '../pkg/sample_rust.js'
import { Context, Editor, Function, Param } from '../../../rillus-web/src/main.js';
import { Pane, InputParams, InputBindingApi } from 'tweakpane';

class RawHtmlContext extends Context<HTMLElement> {
  elements: HTMLElement[] = [];
  container: HTMLDivElement;

  constructor(app: HTMLDivElement, desc: Function, f: any) {
    super(desc, f);
    this.container = app;

  }
  defaultEditor(p: Param): Editor<HTMLElement> {
    return (e, p, v, u) => {
      let i: HTMLParagraphElement | undefined = e as HTMLParagraphElement | undefined;
      if (!i)
        i = document.createElement("p");
      i.innerText = v;
      return i;
    };
  }
  update() {
    console.log("update", this.desc);
    let i = 0;
    let args = [];
    for (const p of this.desc!.params) {
      let elt = this.elements[i++];
      const e = this.getEditor(p)!;
      const a = (elt as any).valueAsNumber;
      e(elt, p, a, () => this.update());
      args.push(a);
    }
    const res = this.f.call(null, ...args);
    console.log("res", res)
    super.getEditor(this.desc.return_type)!(this.elements[i++], this.desc.return_type, res, () => this.update())
  }
  create() {
    // if(!(desc instanceof Function))
    //   throw new Error('Method not implemented.')

    for (const p of this.desc.params) {
      const e = this.getEditor(p)!;
      // console.log(p, e);
      if (e) {
        const elt = e(undefined, p, 0, () => this.update());
        this.container.appendChild(elt);
        this.elements.push(elt);
      }

    }
    const elt = super.getEditor(this.desc.return_type)!(undefined, this.desc.return_type, 0, () => this.update());

    this.elements.push(elt);
    this.container.appendChild(elt);
  }
}

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
class TweakpaneContext<T> extends Context<Binding> {
  pane?: Pane;
  div: HTMLDivElement;
  onChange: (ui: T, obj: any) => any;
  mapData: (data: any) => any;
  onCreate: (div: HTMLDivElement, pane: Pane, data: any) => T;
  constructor(div: HTMLDivElement, desc: Function, f: any,
    onCreate: (div: HTMLDivElement, pane: Pane, data: any) => T,
    mapData: (data: any) => any,
    onChange: (ui: T, obj: any) => any) {
    super(desc, f);
    this.div = div;
    this.onCreate = onCreate;
    this.onChange = onChange;
    this.mapData = mapData;
  }
  defaultEditor(p: Param): Editor<Binding> {
    this.pane!.addBlade({
      view: 'text',
      disabled: true,
      label: p.name,
    })
    throw new Error('Method not implemented.');
  }
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
    const obj = this.createObject();
    obj.result = this.f(...Object.values(obj));
    const container = document.createElement("div");
    container.className = "tp";
    this.div.appendChild(container);
    this.pane = new Pane({ container: container, title: "SVG" });
    const ui = this.onCreate(this.div, this.pane, obj);

    this.pane.on('change', (ev) => {
      console.log('changed: ' + JSON.stringify(ev.value), obj);
      obj.result = this.f(...Object.values(this.mapData(obj)));
      this.onChange(ui, obj);
    });
    this.onChange(ui, obj);
  }
}

init().then(() => {
  new TweakpaneContext(document.querySelector<HTMLDivElement>('#compute')!,
    undefined, svg_test,
    (div, pane, data) => {

      data.p = {x:data.i, y: data.j};

      pane.addInput(data, 'p', {
        picker: 'inline',
        expanded: true,
        x: {min: 0, max:100},
        y: {min: 0, max:100},
      })
      pane.addInput(data, 'k', { options: fromEnum(Kind) })
      pane.addMonitor(data, 'result', {
        multiline: true,
        lineCount: 5,
      });

      const output = document.createElement("div");
      div.appendChild(output);
      return output;
    },
    (data) => {
      data.i = data.p.x;
      data.j = data.p.y;
      return data;
    },
    (ui, obj) => {
      ui.innerHTML = obj.result;
    },
  ).create();
  // new TweakpaneContext(document.querySelector<HTMLDivElement>('#svg')!, svg_test_desc(), svg_test).create();
  // console.log(new TestStruct())
});
