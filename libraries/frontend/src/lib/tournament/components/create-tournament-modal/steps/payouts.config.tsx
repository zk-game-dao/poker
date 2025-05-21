import { TournamentData } from '@declarations/tournament_index/tournament_index.did';
import { ButtonComponent, List, NumberInputComponent, StepComponentProps, SteppedModalStep } from '@zk-game-dao/ui';
import { memo } from 'react';

type BlindStructureValues = Pick<TournamentData, "payout_structure">;

const PayoutComponent = memo<{ position: number; percentage: number; remove(): void; onChange(percentage: number): void; }>(({
  percentage, onChange, position, remove
}) => {
  const visualPosition = position + 1;
  return (
    <List
      label={`${visualPosition}${visualPosition === 1 ? "st" : visualPosition === 2 ? "nd" : visualPosition === 3 ? "rd" : "th"} place`}
      ctas={[
        { label: "Remove", onClick: remove },
      ]}
    >
      <NumberInputComponent
        label="Percentage"
        min={0}
        max={100}
        value={percentage}
        onChange={(_percentage) => onChange(_percentage)}
        symbol="%"
        hideClear
      />
    </List>
  )
});
PayoutComponent.displayName = "PayoutComponent";

const BasicsStepComponent = memo<StepComponentProps<BlindStructureValues>>(({ data, patch }) => (
  <>
    {data.payout_structure?.map(({ position, percentage }) => (
      <PayoutComponent
        key={position}
        position={position}
        percentage={percentage}
        onChange={(newPercentage) => {
          patch({
            payout_structure: data.payout_structure?.map((p) => {
              if (p.position === position) return { position, percentage: newPercentage };
              return p;
            }) ?? [],
          });
        }}
        remove={() => patch({
          payout_structure: data.payout_structure?.filter((p) => p.position !== position) ?? [],
        })}
      />
    ))}

    <ButtonComponent
      variant='outline'
      onClick={() => {
        const payout_structure = [
          ...(data.payout_structure ?? []),
          {
            position: (data.payout_structure ?? []).length,
            percentage: (data.payout_structure ?? []).reduce((acc, p) => acc - p.percentage, 100),
          },
        ];
        patch({ payout_structure });
      }}
    >
      Add position
    </ButtonComponent>
  </>
));
BasicsStepComponent.displayName = "BasicsStepComponent";

export const Config: SteppedModalStep<BlindStructureValues> = {
  title: "Payout structure",
  defaultValues: {
    payout_structure: [
      { position: 0, percentage: 100 },
    ],
  },
  Component: BasicsStepComponent,
  isValid: ({ payout_structure }) => {
    if (payout_structure?.length === 0) return ["Payout structure is required"];
    if (payout_structure?.reduce((acc, p) => acc + p.percentage, 0) !== 100) return ["Payout structure must sum to 100"];
    return true;
  },
};
