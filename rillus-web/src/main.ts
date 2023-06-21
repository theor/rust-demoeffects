import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'

import init, {compute} from '../pkg/rillus.js'
init().then(() => {
  console.log(compute(2));
});

type Type = "number";
interface Param {
  name: string;
  type: Type;
};

type Function = [Type, [Param]];

const Compute: Function = ["number", [ {name:"i", type: "number"}]];

type Editor<T> = (p:Param, v: any) => T;
type Display<T> = (t:Type, v: any) => T;

class Context<T> {
  editors: Map<Type, Editor<T>>;
  displays: Map<Type, Display<T>>;

  constructor(){
    this.editors = new Map();
    this.displays = new Map();
  }

  getEditor(t:Type): Editor<T> | undefined  { return this.editors.get(t); }
  getDisplay(t:Type): Display<T> | undefined  { return this.displays.get(t); }
  registerEditor(t:Type, e:Editor<T>)  { this.editors.set(t, e); }
  registerDisplay(t:Type, d:Display<T>)  { this.displays.set(t, d); }
};

const c = new Context<HTMLElement>();

c.registerEditor("number", (p, v) => {
 let i = document.createElement("input");
 i.type = "number";
  i.valueAsNumber = v as number;
  i.addEventListener("change", (e:Event):any => console.log(e));
 return i;
});

c.registerDisplay("number", (t, v) => {
  const p=  document.createElement("p");
  p.innerHTML = `${v}`;
  return p;
})

const app = document.querySelector<HTMLDivElement>('#app')!;

for (const p of Compute[1]) {
  app.appendChild(c.getEditor(p.type)!(p, 0));
  
}
app.appendChild(c.getDisplay(Compute[0])!(Compute[0], 0));
//.innerHTML = `
//   <div>
//     <a href="https://vitejs.dev" target="_blank">
//       <img src="${viteLogo}" class="logo" alt="Vite logo" />
//     </a>
//     <a href="https://www.typescriptlang.org/" target="_blank">
//       <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
//     </a>
//     <h1>Vite + TypeScript</h1>
//     <div class="card">
//       <button id="counter" type="button"></button>
//     </div>
//     <p class="read-the-docs">
//       Click on the Vite and TypeScript logos to learn more
//     </p>
//   </div>
// `

// setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
