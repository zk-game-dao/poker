import { createContext, Fragment, ReactElement } from 'react';

import { CurrencyType } from '@zk-game-dao/currency';
import { UIConfig } from '@zk-game-dao/ui';

export type ThemeContextType = {
  isBTC: boolean;
  shownCurrencyType: CurrencyType;

  homePage: ReactElement;
  becomeAHostMarkdown: string;
  roadmapMarkdown: string;
  changelogMarkdown: string;

  setShownCurrencyType(type: CurrencyType): void;
} & Pick<UIConfig, 'banner'>;

export const ThemeContext = createContext<ThemeContextType>({
  isBTC: false,
  shownCurrencyType: { Fake: null },
  setShownCurrencyType: () => { },
  homePage: <Fragment />,
  becomeAHostMarkdown: "",
  changelogMarkdown: "",
  roadmapMarkdown: "",
});
