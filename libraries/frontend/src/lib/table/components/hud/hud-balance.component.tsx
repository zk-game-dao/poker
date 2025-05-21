import { motion } from 'framer-motion';
import { memo } from 'react';

import { IsSameCurrencyType, CurrencyComponent, CurrencyType } from '@zk-game-dao/currency';

export const HudBalanceComponent = memo<{ balance: bigint; currencyType: CurrencyType }>(({ currencyType, balance }) => (
  <motion.div className='flex lg:hidden flex-row justify-center items-center type-tiny gap-2 opacity-70'>
    Balance
    <CurrencyComponent currencyValue={balance} variant='inline' size='small' currencyType={currencyType} />
  </motion.div>
),
  (prevProps, nextProps) =>
    prevProps.balance === nextProps.balance &&
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType)
);
HudBalanceComponent.displayName = "HudBalanceComponent";
