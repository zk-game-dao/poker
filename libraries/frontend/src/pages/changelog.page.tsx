import { memo } from 'react';

import { LayoutComponent, MarkdownPageComponent } from '@zk-game-dao/ui';

export const ChangelogPage = memo<{ markdown: string }>(({ markdown }) => (
  <LayoutComponent footer>
    <MarkdownPageComponent className="container mx-auto">
      {markdown}
    </MarkdownPageComponent>
  </LayoutComponent>
));
ChangelogPage.displayName = 'ChangelogPage';

export default ChangelogPage;
