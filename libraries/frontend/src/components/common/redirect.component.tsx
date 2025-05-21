import { memo, useEffect } from 'react';

import { LoadingAnimationComponent } from '@zk-game-dao/ui';

import { useRouting } from '../../hooks/routing';

export const Redirect = memo<{ to: string }>(({ to }) => {
  const { push } = useRouting()
  useEffect(() => { push(to) }, [to]);
  return <LoadingAnimationComponent variant="shimmer" className='flex flex-1'>Redirecting to {to}...</LoadingAnimationComponent>;
});
Redirect.displayName = 'Redirect';