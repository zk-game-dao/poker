import { useState } from "react";

export const usePersistentState = <S>(
  key: string,
  defaultValue?: S
): [S, (value: S) => void] => {
  const [state, setState] = useState<S>(
    (() => {
      const item = localStorage.getItem(key);
      try {
        return item ? (JSON.parse(item) as S) : (defaultValue as S); // Use defaultValue as R
      } catch {
        return defaultValue as S;
      }
    })()
  );

  const set = (value: S) => {
    if (value === undefined) {
      localStorage.removeItem(key);
    } else {
      localStorage.setItem(key, JSON.stringify(value));
    }
    setState(value);
  };

  return [state, set];
};
