import { writeAll } from "../deps.ts";
import { Atomic, Stringlike, Io, OutputNode } from "../types.ts";
import { map } from "../lazy.ts";
import {
  UnsignedInt,
  UnsignedIntSchema,
  createSelectedChoiceSchema,
} from "../models.ts";

const textEncoder = new TextEncoder();

const print = async (values: Iterable<string>) => {
  for (const v of values) {
    await writeAll(Deno.stdout, textEncoder.encode(v));
  }
};

const formatAtomic = <T extends Stringlike>(atomic: Atomic<T>) => {
  if (typeof atomic === "string") {
    return atomic;
  }

  return atomic.toString();
};

export const terminalIo = {
  process: async <T extends Stringlike>(
    node: OutputNode<T>,
  ): Promise<UnsignedInt | null> => {
    await print(map(formatAtomic, node.text));

    const choices = [...node.choices].map((choice, index) => [
      `\n${index + 1}. `,
      ...map(formatAtomic, choice),
    ]);

    if (choices.length === 0) {
      return null;
    }
    print(["\n"]);

    await print(choices.flat());

    console.log("");

    const schema = createSelectedChoiceSchema(
      UnsignedIntSchema.parse(choices.length),
    );

    let selected;

    while (true) {
      selected = schema.safeParse(prompt(""));

      if (selected.success) {
        break;
      }

      await print(["Invalid choice, please try again.\n"]);
    }

    return selected.data;
  },
} satisfies Io<never>;
