import { createContext, memo, PropsWithChildren, useContext } from "react";

export type ThemeContextValue = {
  theme: "light" | "dark";
};

const ThemeContext = createContext<ThemeContextValue>({ theme: "dark" });

export const ProvideTheme = memo<PropsWithChildren<Partial<ThemeContextValue>>>(
  ({ children, ...theme }) => {
    const currentTheme = useContext(ThemeContext);
    return (
      <ThemeContext.Provider value={{ ...currentTheme, ...theme }}>
        {children}
      </ThemeContext.Provider>
    );
  },
);
ProvideTheme.displayName = "ProvideTheme";

export const useTheme = () => useContext(ThemeContext);
