import { LayoutComponent, MarkdownPageComponent } from '@zk-game-dao/ui';
import { memo } from 'react';

export const RoadmapPage = memo<{ markdown: string }>(({ markdown }) => (
  <LayoutComponent footer>
    <MarkdownPageComponent className="container mx-auto">
      {markdown}
    </MarkdownPageComponent>
  </LayoutComponent>
));
RoadmapPage.displayName = 'RoadmapPage';

export default RoadmapPage;
