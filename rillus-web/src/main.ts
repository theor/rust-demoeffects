// export type Type = string;//"number";

// type Function = [Type, [Param]];

// const Compute: Function = ["number", [ {name:"i", type: "number"}]];

// export interface Param {
//   value_type: string;
//   name: string;
// }
// export interface Function {
//   return_type: Param;
//   params: Param[];
// }

// export type Editor<T> = (t: T | undefined, p: Param, v: any, update: Update) => T;
// export type Update = () => void;

export abstract class Context<TF> {
  // editors: Map<Type, Editor<T>>;

  desc: Function;
  f: TF;

  constructor(desc: Function, f: TF) {
    // this.editors = new Map();
    this.f = f;
    this.desc = desc;
  }

  // getEditor(p: Param): Editor<T> | undefined { return this.editors.get(p.value_type) ?? this.defaultEditor(p); }
  // registerEditor(t: Type, e: Editor<T>) { this.editors.set(t, e); }
  // abstract defaultEditor(p:Param): Editor<T>;
  abstract create(): void;
  
};


