import { CurrencyType, GameType } from '@declarations/table_index/table_index.did';
import { Modal } from '@zk-game-dao/ui';
import { memo } from 'react';

import { RakeInfoComponent } from './rake-info.component';

export const RakeInfoModalComponent = memo<{
  isOpen?: boolean;
  onClose(): void;
  initial_currency_type?: CurrencyType;
  initial_game_type?: GameType;
}>(({ isOpen, onClose, initial_game_type, initial_currency_type }) => {
  return (
    <Modal open={isOpen} onClose={onClose} title="Rake calculator">
      <RakeInfoComponent
        initial_game_type={initial_game_type}
        initial_currency_type={initial_currency_type}
      />
    </Modal>
  )
});
RakeInfoModalComponent.displayName = 'RakeInfoModalComponent';
