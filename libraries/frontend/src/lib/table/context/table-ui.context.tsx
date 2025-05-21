import { createContext, memo, ReactNode, RefObject, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import { useElementBox, useElementSize } from '../../../hooks/element-box';
import { useScreenSize } from '../../../hooks/screen-size';
import { LandscapeTableBackgroundRatio, PortraitTableBackgroundRatio } from '../utils/table-position';

export enum VisualTableSize {
  mini,
  small,
  medium,
  large,
}

type TableUIContextType = {
  setSeatPosition: (
    seat: number,
    position: { x: number; y: number } | undefined,
  ) => void;
  getSeatPosition: (seat: number) => { x: number; y: number } | undefined;
  tableRect?: DOMRect;
  tableRef: RefObject<HTMLDivElement | null>;
  visualSize: VisualTableSize;
  orientation: "landscape" | "portrait";
  animatePots: boolean;
};

export const TableUIContext = createContext<TableUIContextType>({
  setSeatPosition: () => { },
  getSeatPosition: () => ({ x: 0, y: 0 }),
  tableRef: { current: null as any },
  visualSize: VisualTableSize.large,
  orientation: "landscape",
  animatePots: true,
});

export const ProvideTableUIContext = memo<{
  children: ReactNode;
  tableRef: RefObject<HTMLDivElement | null>;
  orientation: "landscape" | "portrait";
  animatePots: boolean;
}>(({ children, tableRef, orientation, animatePots }) => {
  const [positions, setPositions] = useState<{
    [seat: number]: { x: number; y: number } | undefined;
  }>({});

  const setSeatPosition = useCallback(
    (seat: number, position?: { x: number; y: number }) => {
      setPositions((prev) => ({ ...prev, [seat]: position }));
    },
    [],
  );

  const getSeatPosition = useCallback(
    (seat: number) => positions[seat],
    [positions],
  );
  const size = useScreenSize();
  const tableSize = useElementSize(tableRef);

  const tableRect = useMemo(() => {
    if (!tableRef.current) return;
    return tableRef.current.getBoundingClientRect();
  }, [
    size.height,
    size.height,
    tableSize.width,
    tableSize.height,
    tableRef.current
  ]);

  const visualSize = useMemo(() => {
    if (tableSize.width === undefined || tableSize.width < 640)
      return VisualTableSize.mini;
    if (tableSize.width < 1024) return VisualTableSize.small;
    if (tableSize.width < 1280) return VisualTableSize.medium;
    return VisualTableSize.large;
  }, [tableSize.width]);

  return (
    <TableUIContext.Provider
      value={{
        setSeatPosition,
        getSeatPosition,
        tableRef,
        tableRect,
        visualSize,
        orientation,
        animatePots,
      }}
    >
      {children}
    </TableUIContext.Provider>
  );
});
ProvideTableUIContext.displayName = 'ProvideTableUIContext';

export const getPositionOnTable = (ref: RefObject<HTMLDivElement | null>) => {
  const { tableRef, tableRect } = useContext(TableUIContext);
  const size = useElementSize(tableRef);
  const sz = useElementBox(ref);

  return useMemo(() => {
    if (!ref.current || !tableRect) return;
    const rect = ref.current.getBoundingClientRect();
    return {
      x: Math.floor(rect.left - tableRect.left),
      y: Math.floor(rect.top - tableRect.top),
    };
  }, [
    ref.current,
    tableRect?.left,
    tableRect?.top,
    size?.width,
    size?.height,
    sz?.width,
    sz?.height,
  ]);
};

export const useRegisterTableSeat = (
  seat: number,
  ref: RefObject<HTMLDivElement | null>,
) => {
  const { setSeatPosition, tableRef } = useContext(TableUIContext);
  const position = getPositionOnTable(ref);
  const size = useElementSize(tableRef);

  useEffect(() => {
    if (seat < 0) return;
    setSeatPosition(seat, position);
  }, [
    position?.x,
    position?.y,
    size.width,
    size.height,
    tableRef.current
  ]);
};

export const useTableUIContext = () => useContext(TableUIContext);

export const useSeatPosition = (seat: number) => {
  const { getSeatPosition } = useTableUIContext();
  return useMemo(() => getSeatPosition(seat), [seat, getSeatPosition]);
};

export const useVisualTableSize = () => {
  const { visualSize } = useTableUIContext();
  return useMemo(() => visualSize, [visualSize]);
};

export const useTableRect = (containerRef: RefObject<HTMLDivElement | null>) => {
  const containerSize = useElementSize(containerRef);

  /**
   * The size of the table aims to be full width of the container
   * If the height of the table would extend the container, the width is adjusted to fit the height
   */
  return useMemo(() => {
    const w = containerSize.clientWidth ?? containerSize.width;
    const h = containerSize.clientHeight ?? containerSize.height;
    if (!w || !h)
      return { width: 320, height: 320 / PortraitTableBackgroundRatio };

    // Landscape
    if (w > h) {
      const width = w;
      const height = width / LandscapeTableBackgroundRatio;
      if (height > h)
        return {
          width: h * LandscapeTableBackgroundRatio,
          height: h,
        };
      return {
        width,
        height,
      };
    }

    // Portrait
    const height = h;
    const width = height / PortraitTableBackgroundRatio;

    if (width > w) {
      if (w / PortraitTableBackgroundRatio > h)
        return {
          width: h * PortraitTableBackgroundRatio,
          height: h,
        };
      return {
        width: w,
        height: w / PortraitTableBackgroundRatio,
      };
    }

    return {
      width,
      height,
    };
  }, [
    containerSize.clientHeight,
    containerSize.clientWidth,
    containerSize.height,
    containerSize.width,
  ]);
};
