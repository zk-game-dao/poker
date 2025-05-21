import classNames from 'classnames';
import { AnimatePresence, motion } from 'framer-motion';
import { memo } from 'react';

import { TableConfig } from '@declarations/table_index/table_index.did';
import { Image, Interactable, StepComponentProps, SteppedModalStep } from '@zk-game-dao/ui';

type Value = Pick<TableConfig, "seats">;

const PlayerCountStepComponent = memo<StepComponentProps<Value>>(({ data, patch }) => (
  <div className="flex flex-row justify-center items-center gap-4">
    <Interactable
      onClick={() => patch({ seats: Math.max(2, (data.seats ?? 2) - 1) })}
      className={classNames("active:scale-95 transition-transform", {
        "opacity-50 pointer-events-none": data.seats && data.seats <= 2,
      })}
    >
      <Image type="png" src="/icons/minus.png" width={48} height={48} alt="+" />
    </Interactable>
    <div className="w-[65px] h-[116px] relative">
      <AnimatePresence>
        <motion.div
          key={data.seats}
          variants={{
            hidden: { opacity: 0, scale: 0.5, rotate: 180 },
            visible: { opacity: 1, scale: 1, rotate: 1 },
          }}
          initial="hidden"
          animate="visible"
          exit="hidden"
          className="absolute inset-0"
        >
          <Image
            type="png"
            src={`/icons/${data.seats}.png`}
            width={65}
            height={116}
            alt={"" + data.seats}
          />
        </motion.div>
      </AnimatePresence>
    </div>
    <Interactable
      onClick={() => patch({ seats: Math.min(8, (data.seats ?? 2) + 1) })}
      className={classNames("active:scale-95 transition-transform", {
        "opacity-50 pointer-events-none": data.seats && data.seats >= 8,
      })}
    >
      <Image type="png" src="/icons/plus.png" width={48} height={48} alt="+" />
    </Interactable>
  </div>
));
PlayerCountStepComponent.displayName = "PlayerCountStepComponent";

export const Config: SteppedModalStep<Value> = {
  title: "How many people can join your table?",
  Component: PlayerCountStepComponent,
  isValid: ({ seats }) =>
    seats !== undefined ? true : ["Seats are required"],
  defaultValues: { seats: 6 },
};
