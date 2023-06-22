import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'

import init, { compute, compute_desc } from '../pkg/rillus.js'

type Type = string;//"number";

// type Function = [Type, [Param]];

// const Compute: Function = ["number", [ {name:"i", type: "number"}]];

interface Param {
  value_type: string;
  name: string;
}
interface Function {
  return_type: string;
  params: Param[];
}

type Editor<T> = (t: T | undefined, p: Param, v: any, update: Update) => T;
type Update = () => void;

abstract class Context<T> {
  editors: Map<Type, Editor<T>>;

  constructor() {
    this.editors = new Map();
  }

  getEditor(t: Type): Editor<T> | undefined { return this.editors.get(t) ?? this.defaultEditor(); }
  registerEditor(t: Type, e: Editor<T>) { this.editors.set(t, e); }
  abstract defaultEditor(): Editor<T>;
};

class RawHtmlContext extends Context<HTMLElement> {
  elements: HTMLElement[] = [];
  desc?: Function;

  defaultEditor(): Editor<HTMLElement> {
    return (e,p,v,u) => {
      let i: HTMLParagraphElement | undefined = e as HTMLParagraphElement | undefined;
      if(!i)
      i =  document.createElement("p");
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
      const e = this.getEditor(p.value_type)!;
      const a = (elt as any).valueAsNumber;
      e(elt, p, a, () => this.update());
      args.push(a);
    }
    const res = compute.call(null, ...args);
    console.log("res", res)
    super.getEditor(this.desc!.return_type)!(this.elements[i++], { value_type: this.desc!.return_type, name: 'return' }, res,  () => this.update())
  }
  create(app: HTMLDivElement, desc: Function) {
    this.desc = desc;
    // if(!(desc instanceof Function))
    //   throw new Error('Method not implemented.')

    for (const p of desc.params) {
      const e = this.getEditor(p.value_type)!;
      // console.log(p, e);
      if (e) {
        const elt= e(undefined, p, 0,  () => this.update());
        app.appendChild(elt);
        this.elements.push(elt);
      }

    }
    const elt = super.getEditor(desc.return_type)!(undefined, { value_type: desc.return_type, name: 'return' }, 0,  () => this.update());
    
    this.elements.push(elt);
    app.appendChild(elt);
  }
}

const c = new RawHtmlContext();

function wrap(x: HTMLElement, label:string) {
  
}
c.registerEditor("i32", (e, p, v, update) => {

  let i: HTMLInputElement | undefined = e as HTMLInputElement | undefined;
  if (!i) {
    i = document.createElement("input");
    i.type = "number";
    i.addEventListener("change", (e: Event): any => update());
    e = i;
  }
  i.valueAsNumber = v as number;

  return i;
});


const app = document.querySelector<HTMLDivElement>('#app')!;
init().then(() => {
  const desc = compute_desc();
  console.log(desc);
  c.create(app, desc);
});

