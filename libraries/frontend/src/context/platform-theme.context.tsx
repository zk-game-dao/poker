import { createContext, Fragment, ReactElement } from 'react';

import { CurrencyType } from '@zk-game-dao/currency';

export type ThemeContextType = {
  hideTournaments: boolean;
  isBTC: boolean;
  shownCurrencyType: CurrencyType;

  homePage: ReactElement;
  becomeAHostMarkdown: string;
  roadmapMarkdown: string;
  changelogMarkdown: string;

  setShownCurrencyType(type: CurrencyType): void;
};

export const ThemeContext = createContext<ThemeContextType>({
  hideTournaments: false,
  isBTC: false,
  shownCurrencyType: { Fake: null },
  setShownCurrencyType: () => { },
  homePage: <Fragment />,
  becomeAHostMarkdown: "",
  changelogMarkdown: "",
  roadmapMarkdown: "",
});
