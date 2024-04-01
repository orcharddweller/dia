import { For, While, If, Lazy, Flow } from "./types.ts";
import { lazyToIterable, map } from "./lazy.ts";

export const isFlowFor = <Item, Result>(
  flow: For<Item, Result> | unknown
): flow is For<Item, Result> => {
  return (flow as For<Item, Result>).flow === "for";
};

export const isFlowIf = <Result>(
  flow: If<Result> | unknown
): flow is If<Result> => {
  return (flow as If<Result>).flow === "if";
};

export const isFlowWhile = <Result>(
  flow: While<Result> | unknown
): flow is While<Result> => {
  return (flow as While<Result>).flow === "while";
};

export const flowFor = <Item, Result>({
  callback,
  iterable,
}: For<Item, Result>): Iterable<Result> => map(callback, iterable());

export const flowIf = <Result>({
  ifs,
  else: otherwise,
}: If<Result>): Lazy<Result> => {
  for (const { condition, then } of ifs) {
    if (condition()) {
      return then;
    }
  }

  return otherwise;
};

export const flowWhile = <Result>({
  condition,
  then,
}: While<Result>): Iterable<Result> => {
  return {
    *[Symbol.iterator]() {
      while (condition()) {
        yield then();
      }
    },
  };
};

export const flowOrPassthrough = <Result, Item>(
  flow: Flow<Result, Item> | Result
): Iterable<Result> => {
  if (isFlowFor<Item, Result>(flow)) {
    return flowFor(flow);
  }

  if (isFlowIf<Result>(flow)) {
    return lazyToIterable(flowIf(flow));
  }

  if (isFlowWhile<Result>(flow)) {
    return flowWhile(flow);
  }

  return [flow];
};

export const flow = <Result, Item>(
  flows: ReadonlyArray<Flow<Result, Item> | Result>
): Iterable<Result> => {
  return {
    *[Symbol.iterator]() {
      for (const f of flows) {
        yield* flowOrPassthrough(f);
      }
    },
  };
};
