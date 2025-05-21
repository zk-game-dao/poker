import { memo, ReactNode } from 'react';

import { LoadingAnimationComponent } from '@zk-game-dao/ui';

import { useTableUIContext } from '../../context/table-ui.context';

export const DynamicLoadingIndicator = memo<{ children: ReactNode }>(({ children }) => {
  const { animatePots } = useTableUIContext();

  return (
    <LoadingAnimationComponent variant="shimmer">
      {!animatePots ? "Waiting" : children}
    </LoadingAnimationComponent>
  );
});
DynamicLoadingIndicator.displayName = "DynamicLoadingIndicator";
