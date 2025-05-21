import { memo } from 'react';

import { RakeInfoComponent } from '@lib/table/components/rake-info/rake-info.component';
import { LayoutComponent } from '@zk-game-dao/ui';

export const ChangelogPage = memo(() => (
  <LayoutComponent footer>
    <RakeInfoComponent />
  </LayoutComponent>
));
ChangelogPage.displayName = 'ChangelogPage';

export default ChangelogPage;
