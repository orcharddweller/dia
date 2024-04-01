import { z } from "./deps.ts";

export const UnsignedIntSchema = z.number().int().min(0).brand("UnsignedInt");

export type UnsignedInt = z.infer<typeof UnsignedIntSchema>;

export const createSelectedChoiceSchema = (nChoices: UnsignedInt) =>
  z.coerce
    .number()
    .int()
    .min(1)
    .max(nChoices)
    .transform((v) => UnsignedIntSchema.parse(v - 1));
