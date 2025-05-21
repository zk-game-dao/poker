import clone from 'lodash/cloneDeep';
import { createContext, JSX, RefObject, useCallback, useContext, useEffect, useId, useMemo, useState } from 'react';

type Size = Pick<ResizeObserverEntry["contentRect"], "height" | "width">;

type TrackableElement =
  | HTMLButtonElement
  | HTMLAnchorElement
  | HTMLDivElement
  | HTMLDialogElement
  | null;

const ElementBoxContext = createContext<{
  readonly boxes: { [element: string]: Size };
  observe(el: RefObject<TrackableElement>, id: string): () => void;
}>({
  boxes: {},
  observe: () => () => { },
});

const DATASET_ID_NAME = "rid";

const useBoxMap = (): [
  { [element: string]: Size },
  (id: string, size?: Size) => void,
] => {
  const [boxes, setBoxes] = useState<{ [element: string]: Size }>({});

  const setBoxSize = useCallback((id: string, size?: Size) => {
    const localBoxes: { [element: string]: Size } = {};
    if (!size) {
      delete localBoxes[id];
    } else {
      localBoxes[id] = {
        width: Math.round(size.width),
        height: Math.round(size.height),
      };
    }
    setBoxes(clone({ ...localBoxes }));
  }, []);

  return [boxes, setBoxSize];
};

export const ElementBoxProvider = ({ children }: { children: JSX.Element }) => {
  const [boxes, setBoxSize] = useBoxMap();
  const [resizeObserver, setResizeObserver] = useState<ResizeObserver>();

  useEffect(() => {
    if (typeof window === "undefined") return undefined;
    const _resizeObserver = new ResizeObserver((entries) => {
      entries
        ?.filter((v) => v.contentRect)
        .forEach(({ target, contentRect }) => {
          const targetElement = target as HTMLElement;
          if (!targetElement?.dataset || !(DATASET_ID_NAME in targetElement.dataset)) return;
          const id = (target as HTMLElement)?.dataset[DATASET_ID_NAME];
          if (!id) return;
          setBoxSize(id, contentRect);
        });
    });
    setResizeObserver(_resizeObserver);
    return () => _resizeObserver?.disconnect();
  }, [setBoxSize]);

  const unobserve = useCallback(
    (el: RefObject<TrackableElement>, id: string) => {
      if (!resizeObserver || !el.current) return;
      resizeObserver.unobserve(el.current);
      setBoxSize(id, undefined);
    },
    [resizeObserver, setBoxSize],
  );

  const observe = useCallback(
    (el: RefObject<TrackableElement>, id: string) => {
      if (!resizeObserver || !el.current) return () => { };
      resizeObserver.observe(el.current, {
        box:
          el.current.nodeName === "div"
            ? "device-pixel-content-box"
            : "content-box",
      });
      el.current.dataset[DATASET_ID_NAME] = id;
      // Set the box size immediately to prevent jerky animatinos
      const box = el.current.getBoundingClientRect();
      setBoxSize(id, { width: box.width, height: box.height });
      return () => unobserve(el, id);
    },
    [resizeObserver, unobserve],
  );

  return (
    <ElementBoxContext.Provider
      value={{
        boxes,
        observe,
      }}
    >
      {children}
    </ElementBoxContext.Provider>
  );
};

export const useElementSize = (
  _el: RefObject<TrackableElement>,
  options?: { log?: boolean },
): Partial<
  Size &
  Pick<
    HTMLDivElement,
    | "clientWidth"
    | "clientHeight"
    | "scrollWidth"
    | "scrollHeight"
    | "offsetHeight"
    | "offsetWidth"
  > & {
    isScrollableX: boolean;
    isScrollableY: boolean;
  }
> => {
  const id = useId();
  const el = useMemo(() => _el, [_el.current]);
  const { observe, boxes } = useContext(ElementBoxContext);

  useEffect(() => observe(el, id), [el.current, id, observe]);

  return useMemo(() => {
    if (id in boxes) return boxes[id];
    if (!el.current) return {};
    const boundingClientRect = el.current.getBoundingClientRect();

    const width = Math.round(boundingClientRect.width);
    const height = Math.round(boundingClientRect.height);
    const { clientWidth, clientHeight, scrollWidth, scrollHeight } = el.current;
    const isScrollableX = scrollWidth > width;
    const isScrollableY = scrollHeight > height;
    return {
      offsetHeight: el.current.offsetHeight,
      offsetWidth: el.current.offsetWidth,
      width,
      height,
      clientWidth,
      clientHeight,
      scrollWidth,
      scrollHeight,
      isScrollableX,
      isScrollableY,
    };
  }, [boxes, el, id, options]);
};

export const useElementBox = (_el: RefObject<TrackableElement>) => {
  const id = useId();
  const el = useMemo(() => _el, [_el.current]);
  const { observe, boxes } = useContext(ElementBoxContext);

  useEffect(() => observe(el, id), [el.current, id, observe]);

  return useMemo(() => boxes[id], [boxes, id]);
};
