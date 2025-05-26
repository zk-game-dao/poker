import { lazy, memo } from 'react';

import { markdown as changelog } from '../../../../CHANGELOG.PP.md';
import { markdown as roadmap } from '../../../../ROADMAP.PP.md';
import { App as MainApp } from '../../src/App';
import { markdown as becomeAHost } from './pages/become-host.page.md';

export const AboutUs = lazy(() => import("./pages/about-us/about-us.page"));

export const App = memo(() => (
  <MainApp
    homePage={<AboutUs />}
    roadmapMarkdown={roadmap}
    becomeAHostMarkdown={becomeAHost}
    changelogMarkdown={changelog}

    isBTC={false}
    hideTournaments={false}
    shownCurrencyType={{ Real: { ICP: null } }} />
));
App.displayName = 'App';
