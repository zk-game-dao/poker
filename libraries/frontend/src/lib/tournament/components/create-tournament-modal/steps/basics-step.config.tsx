import { addMinutes } from 'date-fns';
import { memo, useMemo } from 'react';

import {
  BuyInOptions, NewTournament, TournamentSizeType, TournamentType
} from '@declarations/tournament_index/tournament_index.did';
import { DateToBigIntTimestamp } from '@lib/utils/time';
import { RealCurrencyInputComponent } from '@zk-game-dao/currency';
import {
  DateInputComponent, DropdownInputComponent, IsDev, List, StepComponentProps, SteppedModalStep,
  SwitchInputComponent, TextInputComponent, TimeInputComponent
} from '@zk-game-dao/ui';

import {
  ImageUploadComponent
} from '../../../../../components/common/input/image-upload.component';
import { Max } from '../../../../utils/bigint';
import { matchRustEnum } from '../../../../utils/rust';
import { Tooltips } from '../../../tooltips';
import { default_tournament_type, defaultBuyInOptions } from './type-step.config';

type BasicsStepValues = Pick<NewTournament, 'currency' | 'buy_in' | "tournament_type" | "hero_picture" | "name" | "start_time" | 'description' | 'late_registration_duration_ns' | 'require_proof_of_humanity'>;

const BasicsStepComponent = memo<StepComponentProps<BasicsStepValues>>(({ data, patch }) => {

  const size = useMemo((): TournamentSizeType => {
    if (!data.tournament_type) return { SingleTable: defaultBuyInOptions };
    return matchRustEnum(data.tournament_type)({
      SpinAndGo: ([t]) => t,
      BuyIn: t => t,
      SitAndGo: t => t,
      Freeroll: t => t
    });
  }, [data.tournament_type]);

  const currentBuyInOptions = useMemo(() => matchRustEnum(size)({
    SingleTable: (t) => t,
    MultiTable: ([t]) => t,
  }), [size]);

  const type = useMemo(() => {
    if (!data.tournament_type) return 'BuyIn';
    return matchRustEnum(data.tournament_type)({
      BuyIn: () => 'BuyIn' as const,
      SitAndGo: () => 'SitAndGo' as const,
      Freeroll: () => 'Freeroll' as const,
      SpinAndGo: () => 'SpinAndGo' as const,
    });
  }, [data.tournament_type]);

  const patchType = (type: 'BuyIn' | 'SitAndGo' | 'Freeroll' | 'SpinAndGo') => {

    let tournament_type: TournamentType;

    switch (type) {
      case 'BuyIn':
      case 'Freeroll':
        tournament_type = { Freeroll: { SingleTable: currentBuyInOptions } };
        break;
      case 'SitAndGo':
        tournament_type = { SitAndGo: { SingleTable: { ...defaultBuyInOptions, freezout: true } } };
        break;
      case 'SpinAndGo':
        tournament_type = {
          SpinAndGo: [
            { SingleTable: currentBuyInOptions },
            { multiplier: 0n, payout_structure: [] }
          ]
        };
        break;
    }

    patch({
      buy_in: type === 'Freeroll' ? 0n : data.buy_in,
      tournament_type
    });
  };

  const patchDate = (start_time: bigint) => {
    patch({
      start_time,
      tournament_type: {
        [type]: {
          SingleTable: {
            ...currentBuyInOptions,
            addon: {
              ...currentBuyInOptions.addon,
              addon_start_time: Max(start_time, currentBuyInOptions.addon?.addon_start_time ?? start_time),
              addon_end_time: Max(start_time, currentBuyInOptions.addon?.addon_end_time ?? start_time),
            },
            rebuy: {
              ...currentBuyInOptions.rebuy,
              rebuy_end_timestamp: Max(start_time, currentBuyInOptions.rebuy?.rebuy_end_timestamp ?? start_time),
            }
          } as BuyInOptions,
        }
      } as TournamentType
    });
  };

  const showDate = useMemo(() => {
    if (!data.tournament_type) return true;
    return matchRustEnum(data.tournament_type)({
      BuyIn: () => true,
      SitAndGo: () => false,
      Freeroll: () => true,
      SpinAndGo: () => true,
    });
  }, [data.tournament_type]);

  return (
    <>
      <List>
        <TextInputComponent
          label="Name"
          value={data.name}
          onChange={(name) => patch({ name })}
        />
        <TextInputComponent
          label="Description"
          value={data.description}
          onChange={(description) => patch({ description })}
        />
        <RealCurrencyInputComponent
          label={<>Token <Tooltips.token /></>}
          value={!data.currency || 'Fake' in data.currency ? undefined : data.currency.Real}
          onChange={(currency) => patch({ currency: !currency ? undefined : { Real: currency } })}
        />

      </List>

      <ImageUploadComponent
        label="Image"
        imageUrl={data.hero_picture}
        setImageUrl={(hero_picture) => patch({ hero_picture })}
      />

      <List label="Settings">
        <SwitchInputComponent
          checked={data.require_proof_of_humanity}
          onChange={(require_proof_of_humanity) => patch({ require_proof_of_humanity })}
          label="Require proof of humanity"
        />
        <DropdownInputComponent
          label={
            <>
              Type{" "}
              <Tooltips.tournament_type />
            </>
          }
          value={type}
          options={[
            { label: "Buy in", value: "BuyIn" },
            { label: "Sit and go", value: "SitAndGo" },
            { label: "Freeroll", value: "Freeroll" },
          ]}
          onChange={(type) => patchType(type as any)}
        />
        {showDate && (
          <>
            <DateInputComponent
              label="Date"
              datetime_ns={data.start_time}
              onChange={patchDate}
            />
            <TimeInputComponent
              label="Late registration duration"
              nanoseconds={data.late_registration_duration_ns}
              onChangeNanoseconds={(late_registration_duration_ns) => patch({ late_registration_duration_ns })}
            />
          </>
        )}
      </List>

    </>
  );
});
BasicsStepComponent.displayName = "BasicsStepComponent";

export const Config: SteppedModalStep<BasicsStepValues> = {
  title: "The basics",
  defaultValues: {
    start_time: DateToBigIntTimestamp(new Date()),
    late_registration_duration_ns: BigInt(0),
    tournament_type: default_tournament_type,
    hero_picture: IsDev ? 'https://pink-accessible-partridge-21.mypinata.cloud/ipfs/bafybeicebwhz6btreey77irgtel2h6wjw6ygp3xsofbdgjc3vyof7kr4fq' : undefined,
    require_proof_of_humanity: false,
    currency: { Real: { ICP: null } },
  },
  Component: BasicsStepComponent,
  isValid: ({ hero_picture, tournament_type, name, start_time, description, late_registration_duration_ns }) => {
    if (!name) return ["Name is required"];
    if (name.length < 3) return ["Name must be at least 3 characters long"];
    if (name.length > 180) return ["Name must be at most 180 characters long"];

    if (tournament_type && !('SitAndGo' in tournament_type)) {
      if (!start_time) return ["Start time is required"];
      if (start_time < DateToBigIntTimestamp(addMinutes(new Date(), 2))) return ["Start time must be at least 2 minutes in the future"];
      if (late_registration_duration_ns === undefined) return ["Late registration duration is required"];
    }

    if (!description) return ["Description is required"];

    if (!hero_picture) return ["Picture is required"];

    return true;
  },
};
