import { memo, useCallback, useEffect, useMemo, useState } from 'react';

import { FilterOptions, GameType } from '@declarations/table_index/table_index.did';
import {
  ActionTimerDurations
} from '@lib/table/components/create-table-modal/steps/time-limit-step.config';
import {
  CurrencyIconComponent, CurrencyMeta, CurrencyTypeInputComponent
} from '@zk-game-dao/currency';
import {
  ButtonComponent, DropdownInputComponent, FormComponent, FormProps, Modal, ModalFooterPortal,
  TabsInputComponent, UnwrapOptional, WrapOptional
} from '@zk-game-dao/ui';

import { Tooltips } from '../../lib/tournament/tooltips';
import { FloatToTokenAmount, TokenAmountToFloat } from '../../lib/utils/token-amount-conversion';
import { GameTypeSerializer } from '../../lib/utils/serializers';

const SelectGameTypeComponent = memo<{
  type?: GameType;
  onChange(type?: GameType): void;
}>(({ type, onChange }) => {
  const [selectedGameType, setSelectedGameType] = useState(
    type ? Object.keys(type || { NoLimit: null })[0] : "All",
  );
  const meta = useMemo((): Pick<CurrencyMeta, 'decimals' | 'thousands' | 'transactionFee'> => ({ decimals: 8, thousands: 10 ** 8, transactionFee: 10_000n }), []);

  const [limits, setLimits] = useState<{
    NoLimit: bigint | undefined;
    FixedLimit: [bigint | undefined, bigint | undefined];
    SpreadLimit: [bigint | undefined, bigint | undefined];
    PotLimit: bigint | undefined;
  }>({
    NoLimit: type && "NoLimit" in type ? type.NoLimit : undefined,
    SpreadLimit:
      type && "SpreadLimit" in type ? type.SpreadLimit : [undefined, undefined],
    FixedLimit:
      type && "FixedLimit" in type ? type.FixedLimit : [undefined, undefined],
    PotLimit: type && "PotLimit" in type ? type.PotLimit : undefined,
  });

  useEffect(() => {
    setSelectedGameType(
      type ? Object.keys(type || { NoLimit: null })[0] : "All",
    );
    setLimits({
      NoLimit: type && "NoLimit" in type ? type.NoLimit : undefined,
      SpreadLimit:
        type && "SpreadLimit" in type
          ? type.SpreadLimit
          : [undefined, undefined],
      FixedLimit:
        type && "FixedLimit" in type ? type.FixedLimit : [undefined, undefined],
      PotLimit: type && "PotLimit" in type ? type.PotLimit : undefined,
    });
  }, [type && GameTypeSerializer.serialize(type)]);

  const propagate = useCallback(
    (gameType: string, _limits: Partial<typeof limits>) => {
      const nLimits = { ...limits, ..._limits };
      setLimits(nLimits);
      let game_type: GameType | undefined;
      switch (gameType) {
        case "NoLimit":
          game_type = { NoLimit: nLimits.NoLimit as bigint };
          break;
        case "SpreadLimit":
          if (
            nLimits.SpreadLimit?.[0] === undefined ||
            nLimits.SpreadLimit?.[1] === undefined
          )
            return;
          game_type = { SpreadLimit: nLimits.SpreadLimit as [bigint, bigint] };
          break;
        case "FixedLimit":
          if (
            nLimits.FixedLimit?.[0] === undefined ||
            nLimits.FixedLimit?.[1] === undefined
          )
            return;
          game_type = { FixedLimit: nLimits.FixedLimit as [bigint, bigint] };
          break;
        case "PotLimit":
          game_type = { PotLimit: nLimits.PotLimit as bigint };
          break;
      }
      onChange(game_type);
    },
    [limits, onChange],
  );

  const formData = useMemo((): FormProps | undefined => {
    switch (selectedGameType) {
      case "PotLimit":
        return {
          fields: [
            {
              label: "Small Blind",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
            {
              label: "Big Blind",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
              disabled: true,
            },
          ],
          values: [
            TokenAmountToFloat(limits.NoLimit, meta),
            TokenAmountToFloat(
              limits.NoLimit ? limits.NoLimit * 2n : undefined,
              meta,
            ),
          ],
          onChange: (values) =>
            propagate("NoLimit", {
              NoLimit: FloatToTokenAmount(values[0] as number, meta),
            }),
        };
      case "NoLimit":
        return {
          fields: [
            {
              label: "Small Blind",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
            {
              label: "Big Blind",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
              disabled: true,
            },
          ],
          values: [
            TokenAmountToFloat(limits.NoLimit, meta),
            TokenAmountToFloat(
              limits.NoLimit ? limits.NoLimit * 2n : undefined,
              meta,
            ),
          ],
          onChange: (values) =>
            propagate("NoLimit", {
              NoLimit: FloatToTokenAmount(values[0] as number, meta),
            }),
        };
      case "SpreadLimit":
        return {
          fields: [
            {
              label: "Min Bet",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
            {
              label: "Max Bet",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
          ],
          values: limits.SpreadLimit.map((v) =>
            v === undefined ? undefined : TokenAmountToFloat(v, meta),
          ),
          onChange: (values) =>
            propagate("SpreadLimit", {
              SpreadLimit: [
                values[0]
                  ? FloatToTokenAmount(values[0] as number, meta) ?? undefined
                  : undefined,
                values[1]
                  ? FloatToTokenAmount(values[1] as number, meta) ?? undefined
                  : undefined,
              ],
            }),
        };
      case "FixedLimit":
        return {
          fields: [
            {
              label: "Small Bet",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
            {
              label: "Big Bet",
              type: "number",
              symbol: <CurrencyIconComponent currency={{ ICP: null }} />,
            },
          ],
          values: limits.FixedLimit.map((v) =>
            v === undefined ? undefined : TokenAmountToFloat(v, meta),
          ),
          onChange: (values) =>
            propagate("FixedLimit", {
              FixedLimit: [
                values[0]
                  ? FloatToTokenAmount(values[0] as number, meta) ?? undefined
                  : undefined,
                values[1]
                  ? FloatToTokenAmount(values[1] as number, meta) ?? undefined
                  : undefined,
              ],
            }),
        };
    }
  }, [limits, propagate, selectedGameType]);

  return (
    <>
      <TabsInputComponent
        label="Game type (limit)"
        value={selectedGameType}
        onChange={(newGameType) => {
          setSelectedGameType(newGameType);
          if (newGameType !== "All") return;
          onChange(undefined);
        }}
        tabs={[
          { value: "All", label: "All" },
          { value: "NoLimit", label: "No" },
          { value: "SpreadLimit", label: "Spread" },
          { value: "FixedLimit", label: "Fixed" },
          { value: "PotLimit", label: "Pot" },
        ]}
      />
      {formData && (
        <div className="h-[102px]">
          <FormComponent key={selectedGameType} {...formData} />
        </div>
      )}
    </>
  );
});
SelectGameTypeComponent.displayName = "SelectGameTypeComponent";

export const LobbyFilterModalComponent = memo<{
  options: FilterOptions;
  hideCurrencyType: boolean;
  setOptions: (options: FilterOptions) => void;
}>(({ hideCurrencyType, ...props }) => {
  const [options, setOptions] = useState<FilterOptions>(props.options);

  return (
    <Modal title="Filters" onClose={() => props.setOptions(props.options)}>
      <SelectGameTypeComponent
        type={options.game_type[0]}
        onChange={(gameType) =>
          setOptions((v) => ({ ...v, game_type: gameType ? [gameType] : [] }))
        }
      />

      {!hideCurrencyType && (
        <CurrencyTypeInputComponent
          label={<>Token <Tooltips.token /></>}
          value={UnwrapOptional(options.currency_type) ?? { Real: { ICP: null } }}
          onChange={(value) =>
            setOptions((v) => ({
              ...v,
              currency_type: WrapOptional(value),
            }))
          }
        />
      )}

      <DropdownInputComponent
        label="Seats"
        value={options.seats[0] ?? -1}
        onChange={(value) =>
          setOptions((v) => ({
            ...v,
            seats: (value as number) < 0 ? [] : [value as number],
          }))
        }
        options={[
          { label: "Any", value: -1 },
          { label: "2", value: 2 },
          { label: "3", value: 3 },
          { label: "4", value: 4 },
          { label: "5", value: 5 },
          { label: "6", value: 6 },
          { label: "7", value: 7 },
          { label: "8", value: 8 },
          { label: "9", value: 9 },
        ]}
      />

      <DropdownInputComponent
        label="Timer duration"
        value={options.timer_duration[0] ?? -1}
        onChange={(value) =>
          setOptions((v) => ({
            ...v,
            timer_duration: (value as number) < 0 ? [] : [value as number],
          }))
        }
        options={[{ label: "Any", value: -1 }, ...ActionTimerDurations]}
      />

      <ModalFooterPortal>
        <ButtonComponent
          onClick={() => props.setOptions(props.options)}
          variant="naked"
        >
          Cancel
        </ButtonComponent>

        <ButtonComponent onClick={() => props.setOptions(options)}>
          Apply
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
LobbyFilterModalComponent.displayName = "LobbyFilterModalComponent";
