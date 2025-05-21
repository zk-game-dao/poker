import { CurrencyType, GameType, Rake, TableConfig } from '@declarations/table_index/table_index.did';
import { CurrencyComponent, CurrencyTypeComponent, useCurrencyManagerMeta } from '@zk-game-dao/currency';
import { List, ListItem, LoadingAnimationComponent, NumberInputComponent, useIsInsideModal } from '@zk-game-dao/ui';
import classNames from 'classnames';
import { memo, useState } from 'react';

import { useRakeInfo } from '../../hooks/rake-info.hook';
import { GameTypeStepComponent } from '../create-table-modal/steps/game-type-step.config';

const RakeInfoComponent = memo<Rake & { currencyType: CurrencyType }>(({ currencyType: currency, cap_4_plus_players, cap_2_3_players, percentage_millipercent }) => (
  <div className="flex flex-col justify-start items-start gap-2">
    <div className="type-callout text-material-medium-2">Rake info</div>
    <List variant={{ type: 'default', 'readonly': true }}>
      <ListItem rightLabel={Number(percentage_millipercent) / 1000 + '%'}>
        Percentage
      </ListItem>
      <ListItem rightLabel={<CurrencyComponent currencyValue={cap_2_3_players} currencyType={currency} />}>
        Cap 2-3 players
      </ListItem>
      <ListItem rightLabel={<CurrencyComponent currencyValue={cap_4_plus_players} currencyType={currency} />}>
        Cap 4+ players
      </ListItem>
    </List>
  </div>
));
RakeInfoComponent.displayName = 'RakeInfoComponent';

export const RakeInfoCalculatorComponent = memo<{
  initial_currency_type?: CurrencyType;
  initial_game_type?: GameType;
}>(({ initial_game_type, initial_currency_type }) => {

  const isInModal = useIsInsideModal();

  const [small_blind, setSmallBlind] = useState<bigint>(0n);

  const [formData, setFormData] = useState<Partial<Pick<TableConfig, 'currency_type' | 'game_type'>>>({
    currency_type: initial_currency_type,
    game_type: initial_game_type
  });
  const patchFormData = (data: Partial<Pick<TableConfig, 'currency_type' | 'game_type'>>) => setFormData({ ...formData, ...data });

  const { data, isPending } = useRakeInfo(small_blind, !formData.currency_type || 'Fake' in formData.currency_type ? { ICP: null } : formData.currency_type.Real, formData.game_type);

  const { thousands } = useCurrencyManagerMeta(formData.currency_type ?? { Fake: null });

  return (
    <div
      className={classNames(" flex flex-col gap-3", {
        'material py-6 rounded-xl container mx-auto max-w-[650px] mt-6': !isInModal
      })}
      id="rake-info-calculator"
    >

      <p>Calculator</p>

      <GameTypeStepComponent
        data={formData}
        patch={patchFormData}
        hideFake
        localState
        patchLocalState={() => { }}
      />

      <List>
        <NumberInputComponent
          showLabelInList
          label="Example Small Blind"
          symbol={<CurrencyTypeComponent currencyType={formData.currency_type ?? { Fake: null }} />}
          value={Number(small_blind) / thousands}
          onChange={(value) => setSmallBlind(BigInt(value * thousands))}
        />
      </List>
      {data?.[0] && <RakeInfoComponent currencyType={formData.currency_type ?? { Real: { ICP: null } }} {...data[0]} />}
      {isPending && (
        <LoadingAnimationComponent variant="shimmer">
          Loading Rake Info
        </LoadingAnimationComponent>
      )}
    </div>
  )
})
RakeInfoCalculatorComponent.displayName = 'RakeInfoCalculatorComponent';
