import classNames from 'classnames';
import { memo, useMemo, useState } from 'react';

import { CardColor, EnvironmentColor, TableColor } from '@/src/models/table-color.model';
import { TableConfig } from '@declarations/table_index/table_index.did';
import { Interactable, StepComponentProps, SteppedModalStep, TabsComponent } from '@zk-game-dao/ui';

import { ProvideTableVisuals } from '../../../context/table-visuals.context';
import { CardComponent } from '../../card/card.component';
import {
  EnvironmentBackgroundComponent
} from '../../environment-background/environment-background.component';
import { TableBackgroundComponent } from '../../table/table-background/table-background.component';

type AppeareanceStepValues = Pick<TableConfig, "color" | "card_color" | 'environment_color'>;

const AppeareanceStepComponent = memo<StepComponentProps<AppeareanceStepValues>>(({ data, patch }) => {
  const [modifying, setModifying] = useState<
    "color" | "card_color" | 'environment_color'
  >("color");

  const items = useMemo(() => {
    switch (modifying) {
      case "color":
        return Object.entries(TableColor)
          .filter((v): v is [string, TableColor] => typeof v[1] === "number")
          .map(([title, value]) => ({
            item: (
              <TableBackgroundComponent
                className="w-[130px]"
                visuals={{ color: value }}
              />
            ),
            value: BigInt(value),
            title,
          }));
      case "card_color":
        return Object.entries(CardColor)
          .filter((v): v is [string, CardColor] => typeof v[1] === "number")
          .map(([title, value]) => ({
            item: (
              <ProvideTableVisuals cardColor={value}>
                <CardComponent />
              </ProvideTableVisuals>
            ),
            value: BigInt(value),
            title,
          }));
      case "environment_color":
        return Object.entries(EnvironmentColor)
          .filter(
            (v): v is [string, EnvironmentColor] => typeof v[1] === "number",
          )
          .map(([title, value]) => ({
            item: (
              <EnvironmentBackgroundComponent
                className={classNames("absolute inset-0 z-0 rounded-[16px]", {
                  "outline outline-[3px] outline-white outline-offset-[-3px]":
                    BigInt(value) === data.environment_color,
                })}
                color={value}
              />
            ),
            value: BigInt(value),
            title,
          }));
    }
  }, [modifying, data]);

  return (
    <>
      <TabsComponent
        value={modifying}
        onChange={(v) => setModifying(v)}
        tabs={[
          { value: "color", label: "Table" },
          { value: "card_color", label: "Cards" },
          { value: "environment_color", label: "Environment" },
        ]}
      />

      <div
        className={classNames(
          "grid gap-2",
          modifying === "card_color" ? "grid-cols-3" : "grid-cols-2",
        )}
      >
        {items.map(({ item, title, value }) => (
          <div
            key={modifying + value}
            className="flex flex-col gap-3 justify-center items-center type-callout"
          >
            <Interactable
              className={classNames(
                "w-full min-h-[120px] relative p-5 bg-material-main-1 rounded-[16px] overflow-hidden flex flex-col gap-2 items-center justify-center",
                data[modifying] !== value
                  ? ""
                  : "outline outline-[3px] outline-white outline-offset-[-3px]",
              )}
              onClick={() => patch({ [modifying]: value })}
            >
              {item}
              {modifying !== "environment_color" && (
                <p className="mt-4">{title}</p>
              )}
            </Interactable>
            {modifying === "environment_color" && (
              <p className="mb-2">{title}</p>
            )}
          </div>
        ))}
      </div>
    </>
  );
});
AppeareanceStepComponent.displayName = "AppeareanceStepComponent";

export const Config: SteppedModalStep<AppeareanceStepValues> = {
  title: "Set the appearance of your table",
  defaultValues: {
    color: BigInt(TableColor.Green),
    card_color: BigInt(CardColor.Red),
    environment_color: BigInt(EnvironmentColor.Black),
  },
  Component: AppeareanceStepComponent,
  isValid: ({ color, card_color, environment_color }) =>
    color !== undefined &&
      card_color !== undefined &&
      environment_color !== undefined
      ? true
      : ["Colors are required"],
};
