import { memo, useMemo } from 'react';

import { CardColor, EnvironmentColor, TableColor } from '@/src/models/table-color.model';
import { LobbyTableCardComponent } from '@/src/pages/lobby/lobby-table-card.component';
import { TableConfig } from '@declarations/table_index/table_index.did';
import { CurrencyComponent, useIsBTC } from '@zk-game-dao/currency';
import { StepComponentProps, SteppedModalStep } from '@zk-game-dao/ui';

import { RevenueShareCostPP, RevenueShareCostZKP } from '../constants';

const PreviewStepComponent = memo<StepComponentProps<TableConfig>>(({ data }) => {
  const [small_blind, big_blind] = useMemo(() => {
    if (data.game_type) {
      if ("NoLimit" in data.game_type)
        return [data.game_type.NoLimit, data.game_type.NoLimit * 2n];
      if ("SpreadLimit" in data.game_type)
        return [
          data.game_type.SpreadLimit[0],
          data.game_type.SpreadLimit[0] / 2n,
        ];
      if ("FixedLimit" in data.game_type)
        return [
          data.game_type.FixedLimit[0],
          data.game_type.FixedLimit[0] / 2n,
        ];
    }
    return [0n, 0n];
  }, [data.game_type]);
  const isBTC = useIsBTC();

  return (
    <>
      <LobbyTableCardComponent
        variant="large"
        index={0}
        big_blind={big_blind}
        small_blind={small_blind}
        config={{
          seats: data.seats ?? 6,
          name: data.name ?? "---",
          color: BigInt(data.color ?? TableColor.Green),
          environment_color: BigInt(
            data.environment_color ?? EnvironmentColor.Green,
          ),
          card_color: BigInt(data.card_color ?? CardColor.Red),
          game_type: data.game_type ?? { NoLimit: 0n },
          timer_duration: data.timer_duration ?? 60,
          currency_type: data.currency_type ?? { Real: { ICP: null } },
          table_type: [],
        }}
        seats={[]}
      />

      {data.is_shared_rake?.[0] && data.is_shared_rake[0].length > 0 && (
        <div className="type-subheadline text-material-medium-1 px-4">
          Creating this table costs <div className='inline text-white'>
            <CurrencyComponent currencyType={{ Real: isBTC ? { BTC: null } : { ICP: null } }} className='text-white opacity-40 inline -mt-1' size="small" currencyValue={isBTC ? RevenueShareCostPP : RevenueShareCostZKP} />
          </div> because revenue sharing is enabled.
        </div>
      )}
    </>
  );
});
PreviewStepComponent.displayName = "PreviewStepComponent";

export const Config: SteppedModalStep<TableConfig> = {
  title: "Preview",
  Component: PreviewStepComponent,
  isValid: () => true,
  defaultValues: {}
};
