// general

import { UnsignedInt } from "./models.ts";

export type Lazy<T> = () => T;

// flow control

export type For<Item, Result> = {
  flow: "for";
  iterable: Lazy<Iterable<Item>>;
  callback: (value: Item) => Result;
};

export type If<Result> = {
  flow: "if";
  ifs: ReadonlyArray<{
    condition: Lazy<boolean>;
    then: Lazy<Result>;
  }>;
  else: Lazy<Result>;
};

export type While<Result> = {
  flow: "while";
  condition: Lazy<boolean>;
  then: Lazy<Result>;
};

// deno-lint-ignore no-explicit-any
export type Flow<Result, Item = any> =
  | For<Item, Result>
  | If<Result>
  | While<Result>;

// dia

export type Stringlike = { toString: () => string };

export type Atomic<T extends Stringlike> = string | T;

export type Atom<T extends Stringlike> =
  | string
  | (() => Stringlike)
  | (() => T);

export type Text<T extends Stringlike> = ReadonlyArray<Atom<T> | Flow<Atom<T>>>;

export type Choice<T extends Stringlike> = Readonly<{
  text: Text<T>;
  then?: Lazy<Node<T>>;
}>;

export type Choices<T extends Stringlike> = ReadonlyArray<
  Choice<T> | Flow<Choice<T>>
>;

export type SubNode<T extends Stringlike> = Readonly<{
  text: Text<T>;
  choices: Choices<T>;
}>;

export type SubNodes<T extends Stringlike> = ReadonlyArray<
  SubNode<T> | Flow<SubNode<T>>
>;

export type Node<T extends Stringlike> = Readonly<{
  subNodes: SubNodes<T>;
  fallback?: Lazy<Node<T>>;
}>;

// interfaces

export type OutputText<T extends Stringlike> = Iterable<Atomic<T>>;
export type OutputChoice<T extends Stringlike> = OutputText<T>;
export type OutputChoices<T extends Stringlike> = Iterable<OutputChoice<T>>;

export type OutputNode<T extends Stringlike> = {
  text: OutputText<T>;
  choices: OutputChoices<T>;
};

export type Io<T extends Stringlike> = {
  process: (node: OutputNode<T>) => Promise<UnsignedInt | null>;
};

export type Config<T extends Stringlike> = {
  io: Io<T>;
};
