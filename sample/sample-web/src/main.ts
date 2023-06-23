import './style.css'

import init, { Kind, TestStruct, compute, compute_desc,  svg_func } from '../pkg/sample_rust.js'
import { Context, Editor, Function, Param } from '../../../rillus-web/src/main.js';
import { Pane, InputParams, InputBindingApi } from 'tweakpane';

class RawHtmlContext extends Context<HTMLElement> {
  elements: HTMLElement[] = [];
  container: HTMLDivElement;

  constructor(app: HTMLDivElement, desc: Function, f: any) {
    super(desc, f);
    this.container = app;

  }
  defaultEditor(p:Param): Editor<HTMLElement> {
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


type Binding = InputBindingApi<unknown,unknown>;

class TweakpaneContext extends Context<Binding> {
  pane?: Pane;
  defaultEditor(p:Param): Editor<Binding> {
    this.pane!.addBlade({
      view: 'text',
      disabled: true,
      label: p.name,
    })
    throw new Error('Method not implemented.');
  }
  create() {
    const obj = this.createObject();
    this.pane = new Pane();
    // for (const p of this.desc.params) {
    //   this.pane.addInput(obj, p.name);
    // }
    console.log(Kind)
    this.pane.addInput(obj, 'k', {options: {a:"A", b:"B"}})
    this.pane.on('change', (ev) => {
      console.log('changed: ' + JSON.stringify(ev.value), obj);
    });
  }
}

init().then(() => {
  // const desc = compute_desc();
  // console.log(desc);
  // c.create(document.querySelector<HTMLDivElement>('#compute')!, desc, compute);

  console.log(svg_func);
const c = new TweakpaneContext(svg_func.desc(), svg_func.func);

  c.create();
  console.log(new TestStruct())
});
