import { Lazy } from "./types.ts";

export function* map<T, U>(
  mapFn: ((item: T) => U) | ((item: T, index: number) => U),
  iterable: Iterable<T>
): IterableIterator<U> {
  let index = 0;

  if (mapFn.length === 2) {
    for (const item of iterable) {
      yield mapFn(item, index++);
    }

    return;
  }

  for (const item of iterable) {
    yield (mapFn as (item: T) => U)(item);
  }
}

export function* once<T>(item: T): Iterable<T> {
  yield item;
}

export function* lazyToIterable<T>(lazy: Lazy<T>): Iterable<T> {
  yield lazy();
}
