export type Optional<T> = [T] | [];
export function UnwrapOptional<T>(t?: Optional<T>): T | undefined {
  if (!t) return undefined;
  return t[0];
}
export function WrapOptional<T>(t?: T | undefined): Optional<T> {
  return t === undefined ? [] : [t];
}
export function IsOptional<T>(t: T | Optional<T>): t is Optional<T> {
  return Array.isArray(t);
}
