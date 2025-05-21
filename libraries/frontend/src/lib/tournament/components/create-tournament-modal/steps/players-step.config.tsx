import { TableConfig } from '@declarations/table_index/table_index.did';
import {
  NewTournament,
  SpinGoMultiplier,
  TableBalancer,
  TournamentSizeType,
  TournamentType,
} from '@declarations/tournament_index/tournament_index.did';
import {
  List,
  NumberInputComponent,
  StepComponentProps,
  SteppedModalStep,
  SwitchInputComponent,
  TimeInputComponent,
} from '@zk-game-dao/ui';
import { memo, useMemo } from 'react';

import { matchRustEnum } from '../../../../utils/rust';
import { defaultBuyInOptions } from './type-step.config';

type BasicsStepValues = Pick<NewTournament, "min_players" | "max_players" | 'tournament_type'> & Pick<TableConfig, 'seats'>;

const maxPerTable = 8;

const PlayerSettingsStepComponent = memo<StepComponentProps<BasicsStepValues>>(({ data, patch }) => {

  const size = useMemo((): TournamentSizeType => {
    if (!data.tournament_type) return { SingleTable: defaultBuyInOptions };
    return matchRustEnum(data.tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });
  }, [data.tournament_type]);

  const isMultiTable = useMemo(() => matchRustEnum(size)({
    SingleTable: () => false,
    MultiTable: () => true,
  }), [size]);

  const tableBalancerOptions = useMemo((): TableBalancer => matchRustEnum(size)({
    SingleTable: () => ({
      'max_players_per_table': 8,
      'min_players_per_table': 2,
      'balance_interval_ns': 300_000_000_000n, // 5 minutes in nanoseconds
    }),
    MultiTable: ([, t]) => t,
  }), [size]);

  const patchTableBalancer = (patchData: Partial<TableBalancer>) => {
    if (!data.tournament_type) return;
    const size = matchRustEnum(data.tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });

    if (!('MultiTable' in size)) return;

    const balancer: TableBalancer = { ...size.MultiTable[1], ...patchData };

    const nSize: TournamentSizeType = {
      MultiTable: [size.MultiTable[0], balancer]
    };

    const nType: TournamentType = matchRustEnum(data.tournament_type)({
      SpinAndGo: ([, muptiplier]): TournamentType => ({
        SpinAndGo: [nSize, muptiplier] as [TournamentSizeType, SpinGoMultiplier]
      }),
      BuyIn: (): TournamentType => ({ BuyIn: nSize }),
      SitAndGo: (): TournamentType => ({ SitAndGo: nSize }),
      Freeroll: (): TournamentType => ({ Freeroll: nSize }),
    });

    patch({ tournament_type: nType });
  };

  return (
    <>
      <List label="Multi table">
        <SwitchInputComponent
          checked={isMultiTable}
          label="Enabled"
          onChange={(isEnabled) => {
            if (!data.tournament_type) return;
            let size: TournamentSizeType = matchRustEnum(data.tournament_type)({
              SpinAndGo: ([t]) => t,
              BuyIn: t => t,
              SitAndGo: t => t,
              Freeroll: t => t
            });

            if (isEnabled && 'SingleTable' in size)
              size = { MultiTable: [size.SingleTable, tableBalancerOptions] };
            else if (!isEnabled && 'MultiTable' in size)
              size = { SingleTable: size.MultiTable[0] };

            patch({
              tournament_type: matchRustEnum(data.tournament_type)({
                SpinAndGo: ([, muptiplier]): TournamentType => ({
                  SpinAndGo: [size, muptiplier]
                }),
                BuyIn: (): TournamentType => ({ BuyIn: size }),
                SitAndGo: (): TournamentType => ({ SitAndGo: size }),
                Freeroll: (): TournamentType => ({ Freeroll: size }),
              })
            })
          }}
        />
        {isMultiTable && (
          <>
            <NumberInputComponent
              label="Min players per table"
              min={2}
              max={8}
              value={tableBalancerOptions.min_players_per_table}
              onChange={(min_players_per_table) => patchTableBalancer({ min_players_per_table })}
            />
            <NumberInputComponent
              label="Max players per table"
              min={tableBalancerOptions.min_players_per_table}
              max={8}
              value={tableBalancerOptions.max_players_per_table}
              onChange={(max_players_per_table) => {
                patch({ seats: max_players_per_table });
                patchTableBalancer({ max_players_per_table });
              }}
            />
            <TimeInputComponent
              label="Balance interval"
              nanoseconds={tableBalancerOptions.balance_interval_ns}
              onChangeNanoseconds={(balance_interval_ns) => patchTableBalancer({ balance_interval_ns })}
            />
          </>
        )}
      </List>

      <List label="Settings">
        <NumberInputComponent
          label="Minimum players"
          min={2}
          value={data.min_players}
          onChange={(min_players) => patch({ min_players })}
        />
        <NumberInputComponent
          label="Maximum players"
          min={data.min_players}
          max={isMultiTable ? 8_000_000_000 : 8}
          value={data.max_players}
          onChange={(max_players) => patch({ max_players })}
        />
      </List>
    </>
  );
});
PlayerSettingsStepComponent.displayName = "PlayerSettingsStepComponent";

export const Config: SteppedModalStep<BasicsStepValues> = {
  title: "Player settings",
  defaultValues: {
    min_players: 2,
    seats: 8,
    max_players: maxPerTable,
  },
  enabled: ({ tournament_type }) => {
    if (!tournament_type) return false;
    return matchRustEnum(tournament_type)({
      BuyIn: () => true,
      SitAndGo: () => false,
      Freeroll: () => true,
      SpinAndGo: () => false,
    });
  },
  Component: PlayerSettingsStepComponent,
  isValid: ({ min_players, max_players, tournament_type }) => {

    const size = tournament_type && matchRustEnum(tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });

    const isMultiTable = size && 'MultiTable' in size;

    const maxMaxPlayers = isMultiTable ? 8_000_000_000 : maxPerTable;

    if (min_players === undefined) return ["Minimum players is required"];
    if (min_players < 2) return ["Minimum players must be at least 2"];
    if (min_players > maxMaxPlayers) return [`Minimum players must be at most ${maxMaxPlayers}`];

    if (max_players === undefined) return ["Maximum players is required"];
    if (max_players < min_players) return ["Maximum players must be greater than or equal to minimum players"];
    if (max_players > maxMaxPlayers) return [`Maximum players must be at most ${maxMaxPlayers}`];

    if (size && 'MultiTable' in size) {
      const balancer = size.MultiTable[1];
      if (balancer.min_players_per_table > balancer.max_players_per_table) return ["Minimum players per table must be less than or equal to maximum players per table"];
      if (balancer.min_players_per_table < 2) return ["Minimum players per table must be at least 2"];
      if (balancer.max_players_per_table > maxPerTable) return ["Maximum players per table must be at most 8"];
    }

    if (max_players === undefined) return ["Maximum players is required"];
    if (max_players < min_players) return ["Maximum players must be greater than or equal to minimum players"];

    return true;
  },
};
