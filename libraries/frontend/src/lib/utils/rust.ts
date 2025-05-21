/**
 * If you have a union type like:
 *
 *   type Foo =
 *     | { A: number }
 *     | { B: string }
 *     | { C: { x: boolean } };
 *
 * then `TagOf<Foo>` becomes "A" | "B" | "C".
 */
type TagOf<T> = T extends any ? keyof T : never;

/**
 * For a given union `T` and a specific tag `K`, `PayloadOf<T, K>`
 * is the payload type of that tag. For instance, if T is:
 *   { A: number } | { B: string },
 * then:
 *   PayloadOf<T, "A"> = number
 *   PayloadOf<T, "B"> = string
 */
type PayloadOf<T, K extends PropertyKey> = T extends { [P in K]: infer V }
  ? V
  : never;

/**
 * matchRustEnum takes a single-key union (e.g. {A: number}|{B: string})
 * and returns another function that forces you to provide a "cases" object:
 *
 *   { A: (payload: number) => R, B: (payload: string) => R }
 *
 * The result is whichever handler is called at runtime.
 */
export function matchRustEnum<T extends { [key: string]: unknown }>(value: T) {
  // First, verify there's exactly one key at runtime:
  const keys = Object.keys(value);
  if (keys.length !== 1) {
    throw new Error(
      `Expected exactly one key in variant object, but got [${keys.join(", ")}].`
    );
  }
  const [tag] = keys as [TagOf<T>];
  const payload = value[tag] as PayloadOf<T, typeof tag>;

  // Return a function that *requires* a handler for each possible tag
  return <R>(cases: {
    [K in TagOf<T>]: (payload: PayloadOf<T, K>) => R;
  }): R => {
    const handler = cases[tag];
    return handler(payload);
  };
}
