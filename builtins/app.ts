import {
  Atom,
  Atomic,
  Config,
  Lazy,
  Node,
  Stringlike,
  SubNode,
  OutputNode,
} from "./types.ts";
import { flow } from "./flow.ts";
import { map } from "./lazy.ts";

const executeAtom = <T extends Stringlike>(atom: Atom<T>): Atomic<T> => {
  if (typeof atom !== "string") {
    return atom().toString();
  }

  return atom;
};

const runSubNode = async <T extends Stringlike>(
  subNode: SubNode<T>,
  config: Config<T>
): Promise<Lazy<Node<T>> | null> => {
  const text = map(executeAtom<T>, flow(subNode.text));

  const rawChoicesList = [...flow(subNode.choices)];

  const choices = rawChoicesList.map(({ text }) =>
    map(executeAtom<T>, flow(text))
  );

  const outputNode = {
    text,
    choices,
  } satisfies OutputNode<T>;

  const choice = await config.io.process(outputNode);

  if (choice === null) {
    return null;
  }

  return rawChoicesList[choice].then ?? null;
};

const runNode = async <T extends Stringlike>(
  node: Lazy<Node<T>>,
  config: Config<T>
): Promise<Lazy<Node<T>> | void> => {
  const { subNodes, fallback } = node();

  for (const subNode of flow(subNodes)) {
    const result = await runSubNode(subNode, config);

    if (result) {
      return result;
    }
  }

  return fallback;
};

const loop = async <T extends Stringlike>(
  initial: Lazy<Node<T>>,
  config: Config<T>
) => {
  let node = initial;

  while (true) {
    const result = await runNode(node, config);

    if (!result) {
      break;
    }

    node = result;
  }
};

export const dia = <T extends Stringlike>(config: Config<T>) => ({
  run: async (node: Lazy<Node<T>>): Promise<void> => {
    await loop(node, config);
  },
});
