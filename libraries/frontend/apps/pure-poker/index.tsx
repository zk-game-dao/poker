import React, { lazy, memo } from 'react';

import { markdown as changelog } from '../../../../CHANGELOG.PP.md';
import { markdown as roadmap } from '../../../../ROADMAP.PP.md';
import { markdown as becomeAHost } from './pages/become-host.page.md';
import { App as MainApp } from '../../src/App';

export const BTCAboutUsPage = lazy(() => import("./pages/about-us/about-us.page"));

export const App = memo(() => (
  <MainApp
    homePage={<BTCAboutUsPage />}
    roadmapMarkdown={roadmap}
    becomeAHostMarkdown={becomeAHost}
    changelogMarkdown={changelog}
    isBTC
    shownCurrencyType={{ Real: { BTC: null } }}
  />
));
App.displayName = 'App';

