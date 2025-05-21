import classNames from 'classnames';
import { AnimatePresence, motion } from 'framer-motion';
import { memo, useEffect, useMemo, useRef, useState } from 'react';
import { createPortal } from 'react-dom';

import { CurrencyType, PublicTable } from '@declarations/table_canister/table_canister.did';
import { useUser } from '@lib/user';
import { CurrencyTypeSerializer } from '@zk-game-dao/currency';
import { Interactable } from '@zk-game-dao/ui';

import { useTournament } from '../../../../tournament/context/tournament.context';
import {
  getPositionOnTable, useSeatPosition, useTableUIContext
} from '../../../context/table-ui.context';
import { useTable, useTableUserFromCanisterId } from '../../../context/table.context';
import { ChipsStackComponent } from '../../chips-stack/chips-stack.component';
import { RakeInfoModalComponent } from '../../rake-info/rake-info-modal.component';
import {
  BetsPerStage, computeBetsPerStage, DealStageKey, DealStageKeys, DealStageState, useSeatMap,
  useHasWinners
} from './bets-per-stage';

type MovingAnimationType = "to-pot" | "to-player" | "at-player";

type MovingData = {
  value: bigint;
  type: MovingAnimationType;
  currencyType: CurrencyType;
  isInitial?: boolean;
  stepAnimationDuration: number;
  seatIndex: number;
  potPosition: { x: number; y: number };
};

type AnimationPosition = "atPlayer" | "atPot" | "atPotHidden";

// Just to optimize the data for the animation
const useMemoizedData = (
  potPosition: { x: number; y: number },
  seatIndex: number,
  type: MovingAnimationType,
): {
  variants?: { [position in AnimationPosition]?: { x: number; y: number, opacity?: number } };
  name?: string;
} => {
  const { user: zkpUser } = useUser();
  const { getSeat } = useTable();
  const data = useMemo(() => getSeat(seatIndex), [seatIndex, getSeat]);
  const [user] = useTableUserFromCanisterId(data?.canister_id);

  const name = useMemo(() => {
    if (!user) return;
    if (zkpUser?.principal_id === user.principal_id)
      return `Your ${type === "to-player" ? "winnings" : "bet"}`;
    return `${user.user_name}s ${type === "to-player" ? "winnings" : "bet"}`;
  }, [zkpUser, user, type]);

  const playerPosition = useSeatPosition(seatIndex);

  return useMemo(
    () => ({
      variants: playerPosition
        ? {
          atPlayer: playerPosition,
          atPot: potPosition,
          atPotHidden: { ...potPosition, opacity: 0 },
        }
        : undefined,
      name,
    }),
    [seatIndex, playerPosition, potPosition.x, potPosition.y, name],
  );
};

const Moving = memo<MovingData>(
  ({
    isInitial,
    seatIndex,
    value,
    currencyType: currency,
    type,
    stepAnimationDuration,
    potPosition,
  }) => {
    const { variants, name } = useMemoizedData(potPosition, seatIndex, type);

    const [initial, animate, exit] = useMemo((): [
      false | AnimationPosition,
      AnimationPosition,
      AnimationPosition,
    ] => {
      switch (type) {
        case "to-pot":
          return [isInitial ? false : "atPlayer", "atPlayer", "atPotHidden"];
        case "at-player":
          return [false, "atPlayer", "atPotHidden"];
        case "to-player":
          return [isInitial ? false : "atPot", "atPlayer", "atPlayer"];
      }
    }, [type, isInitial]);

    if (!variants) return null;

    return (
      <motion.div
        variants={variants}
        initial={initial}
        animate={animate}
        exit={exit}
        transition={{ duration: stepAnimationDuration / 1000 }}
        className={classNames(
          "absolute z-0 top-0 left-0",
          `seat-${seatIndex}`,
        )}
      >
        <div className="-translate-y-full">
          <ChipsStackComponent name={name} currencyType={currency} value={value} />
        </div>
      </motion.div>
    );
  },
  (prev, next) =>
    prev.value === next.value &&
    prev.type === next.type &&
    prev.seatIndex === next.seatIndex &&
    prev.potPosition.x === next.potPosition.x &&
    prev.potPosition.y === next.potPosition.y &&
    prev.stepAnimationDuration === next.stepAnimationDuration &&
    prev.isInitial === next.isInitial &&
    CurrencyTypeSerializer.serialize(prev.currencyType) ===
    CurrencyTypeSerializer.serialize(next.currencyType)
);
Moving.displayName = "Pot Moving";

const Wrapper = memo<{ children: React.ReactNode }>(({ children }) => {
  // return children;
  const { tableRef } = useTableUIContext();
  if (!tableRef.current) return null;
  return createPortal(children, tableRef.current);
});
Wrapper.displayName = "Pot Wrapper";

const BetAnimation = memo<{
  state: BetsPerStage;
  stepAnimationDuration?: number;
  currencyType: CurrencyType;
}>(({ state, stepAnimationDuration = 1000, currencyType }) => {
  const potRef = useRef<HTMLDivElement>(null);
  const potPosition = getPositionOnTable(potRef);
  const tournament = useTournament();

  const previousStageMeta = useMemo((): [DealStageKey, DealStageState] => {
    const sorted = Object.entries(state).sort(
      ([a], [b]) =>
        DealStageKeys.indexOf(a as DealStageKey) -
        DealStageKeys.indexOf(b as DealStageKey),
    ) as [DealStageKey, DealStageState][];
    return (
      sorted[sorted.length - 2] ?? [
        "Initial",
        { players: {}, pot: 0n, startingPot: 0n, winnings: {} },
      ]
    );
  }, [state]);

  const dealStageMeta = useMemo((): [DealStageKey, DealStageState] => {
    const sorted = Object.entries(state).sort(
      ([a], [b]) =>
        DealStageKeys.indexOf(a as DealStageKey) -
        DealStageKeys.indexOf(b as DealStageKey),
    ) as [DealStageKey, DealStageState][];
    return (
      sorted[sorted.length - 1] ?? [
        "Initial",
        { players: {}, pot: 0n, startingPot: 0n, winnings: {} },
      ]
    );
  }, [state]);

  const [dsMetaState, setDSMetaState] = useState(dealStageMeta);
  const [animateToState, setAnimateToState] = useState<[DealStageKey, DealStageState]>();

  useEffect(() => {
    if (!animateToState) return;
    const timer = setTimeout(() => {
      setDSMetaState(animateToState);
      setAnimateToState(undefined);
    }, stepAnimationDuration);
    return () => clearTimeout(timer);
  }, [animateToState?.[0], animateToState?.[1], stepAnimationDuration]);

  useEffect(() => {
    if (dsMetaState[0] !== dealStageMeta[0]) {
      setDSMetaState(previousStageMeta);
      setAnimateToState(dealStageMeta);
    } else {
      setDSMetaState(dealStageMeta);
      setAnimateToState(undefined);
    }
  }, [
    dealStageMeta[0],
    dealStageMeta[1],
    previousStageMeta[0],
    previousStageMeta[1],
    dsMetaState[0],
    dsMetaState[1]
  ]);

  const movings = useMemo((): (MovingData & {
    dealStage: DealStageKey;
    seatIndex: number;
  })[] => {
    if (!potPosition) return [];
    const playerMovements = Object.entries(dsMetaState[1].players).reduce(
      (
        all,
        [playerIndexSTR, e8],
      ): (MovingData & { seatIndex: number })[] => {
        const playerPosition = parseInt(playerIndexSTR);
        return [
          ...all,
          {
            value: e8,
            type: "to-pot",
            stepAnimationDuration,
            // playerPosition: offset,
            potPosition,
            seatIndex: playerPosition,
            currencyType: currencyType,
          },
        ];
      },
      [] as (MovingData & { seatIndex: number })[],
    );

    const winningMovements = Object.entries(dsMetaState[1].winnings).reduce(
      (
        all,
        [playerIndexSTR, e8],
      ): (MovingData & { seatIndex: number })[] => {
        const playerPosition = parseInt(playerIndexSTR);
        return [
          ...all,
          {
            value: e8,
            type: "to-player",
            stepAnimationDuration,
            // playerPosition: offset,
            potPosition,
            seatIndex: playerPosition,
            currencyType: currencyType,
          },
        ];
      },
      [] as (MovingData & { seatIndex: number })[],
    );

    return [...playerMovements, ...winningMovements]
      .map((moving) => ({ ...moving, dealStage: dsMetaState[0] }))
      .filter(({ value: e8 }) => e8 > 0n);
  }, [
    dsMetaState[0],
    dsMetaState[1],
    potPosition?.x,
    potPosition?.y,
    stepAnimationDuration,
    CurrencyTypeSerializer.serialize(currencyType)
  ]);

  const [showRakeInfo, setShowRakeInfo] = useState(false);

  return (
    <div className="relative flex justify-center items-center z-50">
      <div className="relative z-1" ref={potRef} id="pot">
        <ChipsStackComponent currencyType={currencyType} value={dsMetaState[1].pot} />
      </div>
      {!!dsMetaState[1].rake && !tournament && (
        <div className="relative z-1 flex flex-col items-end justify-end" id="Rake">
          <p className='mr-1 type-callout text-material-heavy-1 text-right'>Rake</p>
          <ChipsStackComponent name="Rake" currencyType={currencyType} hideChips value={dsMetaState[1].rake}>
            <Interactable
              className="text-material-heavy-1 underline hover:no-underline"
              onClick={() => setShowRakeInfo(true)}
            >
              What is Rake?
            </Interactable>
            <RakeInfoModalComponent
              isOpen={showRakeInfo}
              onClose={() => setShowRakeInfo(false)}
            />
          </ChipsStackComponent>
        </div>
      )}

      <Wrapper>
        {/* Animate the bets moving to the pot */}
        <AnimatePresence >
          {movings.map(({ dealStage, seatIndex, ...moving }) => (
            <Moving
              key={`bet-${dealStage}-${seatIndex}-${moving.type}`}
              {...moving}
              seatIndex={seatIndex}
              // isInitial={!isDirty}
              stepAnimationDuration={stepAnimationDuration}
            />
          ))}
        </AnimatePresence>
      </Wrapper>
    </div>
  );
});
BetAnimation.displayName = "Pot Bet Animation";

export const PotComponent = memo<{ table: PublicTable }>(({ table }) => {
  const seatMap = useSeatMap(table.seats);
  const hasWinners = useHasWinners(table.sorted_users);

  const betsPerStage = useMemo((): BetsPerStage =>
    computeBetsPerStage(
      table.pot,
      table.big_blind,
      table.small_blind,
      table.action_logs,
      table.sorted_users,
      hasWinners,
      seatMap
    ),
    [table.action_logs, table.big_blind, table.pot, seatMap, table.seats, table.sorted_users, table.small_blind]
  );

  return <BetAnimation currencyType={table.config.currency_type} state={betsPerStage} />;
});
PotComponent.displayName = "Pot Component";
