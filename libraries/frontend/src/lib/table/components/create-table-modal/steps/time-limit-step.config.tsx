import { TableConfig } from '@declarations/table_index/table_index.did';
import {
  DropdownInputComponent,
  Interactable,
  List,
  NumberInputComponent,
  StepComponentProps,
  SteppedModalStep,
  TooltipComponent,
} from '@zk-game-dao/ui';
import { memo, useState } from 'react';

import { UnwrapOptional, WrapOptional } from '../../../../utils/optional';

type Value = Pick<TableConfig,
  "timer_duration" |
  "auto_start_timer" |
  "max_inactive_turns" |
  "max_seated_out_turns"
>;
export const ActionTimerDurations = [
  { label: "10 seconds", value: 10 },
  { label: "20 seconds", value: 20 },
  { label: "30 seconds", value: 30 },
  { label: "1 minute", value: 60 },
  { label: "2 minutes", value: 120 },
  { label: "5 minutes", value: 300 },
  { label: "10 minutes", value: 600 },
];

export const TURNS_PER_HAND = 4;

const TimeLimitStepComponent = memo<StepComponentProps<Value> & { hideAutoKick?: boolean; }>(({ data, patch, hideAutoKick = false }) => {
  const [showStartTimer, setShowStartTimer] = useState(false);

  return (
    <div className="flex flex-col gap-2">
      <List>
        <DropdownInputComponent
          label={
            <div className="flex flex-row justify-center items-center">
              <div className='truncate'>
                Action time limit
              </div>
              <TooltipComponent overlayClassName="w-32 w-full" className="ml-1">
                Each player gets this amount of time to fold, check, bet, call,
                or Raise.
              </TooltipComponent>
            </div>
          }
          value={data.timer_duration}
          options={ActionTimerDurations}
          onChange={(timer) => patch({ timer_duration: timer as number })}
        />

        <NumberInputComponent
          label="Max inactive turns"
          value={data.max_inactive_turns}
          min={1}
          step={1}
          max={100}
          onChange={(max_inactive_turns) => patch({ max_inactive_turns })}
        />

        {!hideAutoKick && (
          <NumberInputComponent
            label="Auto kick after inactive turns"
            value={UnwrapOptional(data.max_seated_out_turns)}
            min={1}
            max={100}
            step={1}
            onChange={(seated_out_turns) => patch({ max_seated_out_turns: WrapOptional(seated_out_turns) })}
          />
        )}
      </List>

      {!showStartTimer ? (
        <Interactable
          className="type-subheadline text-material-medium-1 px-4 w-full text-start"
          onClick={() => setShowStartTimer(!showStartTimer)}
        >
          Set new round timer
        </Interactable>
      ) : (
        <List>
          <DropdownInputComponent
            label="Start timer duration"
            value={data.auto_start_timer}
            options={[
              { label: "5 seconds", value: 5 },
              { label: "7 seconds", value: 7 },
              { label: "10 seconds", value: 10 },
              { label: "20 seconds", value: 20 },
              { label: "30 seconds", value: 30 },
              { label: "1 minute", value: 60 },
              { label: "2 minutes", value: 120 },
            ]}
            onChange={(auto_start_timer) =>
              patch({ auto_start_timer: auto_start_timer as number })
            }
          />
        </List>
      )}
    </div>
  );
});
TimeLimitStepComponent.displayName = "TimeLimitStepComponent";

const TimeLimitStepComponentWithoutAutoKick = memo<StepComponentProps<Value>>((props) =>
  <TimeLimitStepComponent {...props} hideAutoKick />
);
TimeLimitStepComponentWithoutAutoKick.displayName = "TimeLimitStepComponentWithoutAutoKick";

export const Config: SteppedModalStep<Value> = {
  title: "How long do players have to make a move?",
  Component: TimeLimitStepComponent,
  isValid: ({ timer_duration, max_inactive_turns }) => {
    const errors: string[] = [];
    if (timer_duration === undefined) errors.push("Timer is required");
    if (max_inactive_turns === undefined)
      errors.push("Max inactive turns is required");
    return errors.length ? errors : true;
  },
  defaultValues: {
    timer_duration: 20,
    auto_start_timer: 7,
    max_inactive_turns: 4,
    // 5 Hands
    max_seated_out_turns: [TURNS_PER_HAND * 5],
  },
};

export const ConfigWithoutAutoKick: SteppedModalStep<Value> = {
  ...Config,
  Component: TimeLimitStepComponentWithoutAutoKick,
};
