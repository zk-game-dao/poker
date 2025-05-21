import classNames from 'classnames';
import { memo } from 'react';

import { CurrencyType, GameType } from '@declarations/table_index/table_index.did';
import { useIsBTC } from '@zk-game-dao/currency';
import { MarkdownPageComponent, useIsInsideModal } from '@zk-game-dao/ui';

import { RakeInfoCalculatorComponent } from './rake-info-calculator.component';
import { markdown as mdPP } from './rake-info.pp.md';
import { markdown as mdZKP } from './rake-info.zkp.md';

export const RakeInfoComponent = memo<{
  initial_currency_type?: CurrencyType;
  initial_game_type?: GameType;
}>(() => {
  const isInModal = useIsInsideModal();
  const isBTC = useIsBTC();
  return (
    <>
      <MarkdownPageComponent className={classNames({ "container mx-auto": !isInModal })}>
        {isBTC ? mdPP : mdZKP}
      </MarkdownPageComponent>
      <RakeInfoCalculatorComponent />
    </>
  )
});
RakeInfoComponent.displayName = 'RakeInfoComponent';
