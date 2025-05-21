import { createContext, memo, useContext, useMemo } from "react";

import { CardColor, TableColor } from "@/src/models/table-color.model";

export type TableVisualsContextType = {
  color: TableColor;
  cardColor: CardColor;
};

export const TableVisualsContext = createContext<TableVisualsContextType>({
  color: TableColor.Green,
  cardColor: CardColor.Red,
});

export const ProvideTableVisuals = memo<
  Partial<TableVisualsContextType> & { children: React.ReactNode }
>(({ children, ...styles }) => {
  const parentStyles = useContext(TableVisualsContext);
  return (
    <TableVisualsContext.Provider
      value={{
        ...parentStyles,
        ...styles,
      }}
    >
      {children}
    </TableVisualsContext.Provider>
  );
});
ProvideTableVisuals.displayName = "ProvideTableVisuals";

export const useTableVisuals = (visuals?: Partial<TableVisualsContextType>) => {
  const globalVisuals = useContext(TableVisualsContext);
  return useMemo(() => visuals || globalVisuals, [visuals, globalVisuals]);
};
