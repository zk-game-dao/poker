import { memo } from 'react';

import { Image } from '@zk-game-dao/ui';

export const DealerButtonComponent = memo(() => (
  <div className='flex flex-col relative w-6 h-6'>
    <Image
      type="png"
      src="/icons/dealer_indicator.png"
      width={96}
      height={96}
      className="absolute -inset-x-[44%] -top-[30%] -bottom-[70%]"
      alt="Dealer Button"
    />
  </div>
));
DealerButtonComponent.displayName = 'DealerButtonComponent';
