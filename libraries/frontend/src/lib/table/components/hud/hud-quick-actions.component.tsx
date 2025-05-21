import classNames from 'classnames';
import { motion } from 'framer-motion';
import { memo, ReactNode } from 'react';

import { useMutation } from '@tanstack/react-query';
import { Interactable, LoadingSpinnerComponent } from '@zk-game-dao/ui';

export const HUDQuickActionComponent = memo<{ label: ReactNode; mutate(): Promise<void> }>(({ label, mutate }) => {
  const m = useMutation({ mutationFn: mutate });

  return (
    <Interactable
      onClick={m.mutateAsync}
      className={classNames(
        "flex material bg-neutral-400 bg-opacity-70 leading-none transition-all duration-150 relative",
        "text-white type-button-2 p-4 rounded-[14.4px]",
        "active:scale-95",
      )}
    >
      <p
        className={classNames(
          m.isPending ? "opacity-0" : "opacity-100",
          "transition-opacity",
        )}
      >
        {label}
      </p>
      {m.isPending && (
        <LoadingSpinnerComponent className="absolute inset-0" />
      )}
    </Interactable>
  );
},
);
HUDQuickActionComponent.displayName = "QuickAction";

type HUDQuickActionsComponentProps = {
  quickActions: [bigint, string][];
  mutate(raiseValue: bigint): Promise<void>;
};

export const HUDQuickActionsComponent = memo<HUDQuickActionsComponentProps>(({ quickActions, mutate }) => (
  <motion.div
    variants={{
      visible: {
        opacity: 1,
        y: -8,
        scale: 1,
      },
      hidden: {
        opacity: 0,
        y: 16,
        scale: 0.9,
      },
    }}
    initial="hidden"
    animate="visible"
    exit="hidden"
    className="flex flex-row justify-center items-end gap-2 whitespace-nowrap px-4 relative z-11"
  >
    <div className='absolute inset-3 bg-black blur-2xl opacity-30' />
    {quickActions.map(([amount, label]) => (
      <HUDQuickActionComponent
        key={label}
        mutate={() => mutate(amount)}
        label={label}
      />
    ))}
  </motion.div>
), (prevProps, nextProps) => {
  if (prevProps.quickActions.length !== nextProps.quickActions.length || prevProps.mutate !== nextProps.mutate)
    return false;

  const sortedPrevActions = [...prevProps.quickActions].sort((a, b) => Number(a[0] - b[0]));
  const sortedNextActions = [...nextProps.quickActions].sort((a, b) => Number(a[0] - b[0]));
  return sortedPrevActions.every((action, index) => sortedNextActions[index][0] === action[0] && sortedNextActions[index][1] === action[1]);
}
);
HUDQuickActionsComponent.displayName = "QuickActions";
