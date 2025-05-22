import { CurrencyType } from '@declarations/table_index/table_index.did';
import { TokenAmountToString } from '@lib/utils/token-amount-conversion';
import {
  CurrencyComponent,
  CurrencyTypeSerializer,
  CurrencyTypeSymbolComponent,
  useCurrencyManagerMeta,
  IsSameCurrencyType
} from '@zk-game-dao/currency';
import { Interactable, List, ListItem, matchRustEnum, Modal, useIsInsideModal } from '@zk-game-dao/ui';
import classNames from 'classnames';
import { AnimatePresence, motion } from 'framer-motion';
import { CSSProperties, memo, PropsWithChildren, useEffect, useMemo, useState } from 'react';

import { useSound } from '../../../../context/sound.context';

const ChipSizes = [
  1, 5, 25, 100, 500, 1000, 5000, 10000, 25000,

  250000, 1000000, 5000000,

  20000000, 100000000, 500000000, 1000000000,
] as const;

const useValuePerChip = (currencyType: CurrencyType) => useMemo(() => matchRustEnum(currencyType)({
  Fake: () => 10000n,
  Real: (c) => matchRustEnum(c)({
    ICP: () => 10000n,
    GenericICRC1: ({ decimals }) => 10n ** BigInt(decimals),
    CKETHToken: (tok) => matchRustEnum(tok)({
      ETH: () => 10n ** 9n,
      USDC: () => 10n ** 3n,
      USDT: () => 10n ** 3n,
    }),
    BTC: () => 10n ** 2n,
  })
}), [CurrencyTypeSerializer.serialize(currencyType)]);

type ChipSize = (typeof ChipSizes)[number];
type ChipRenderType = "small" | "big" | "card";

export const ChipNames: Record<
  ChipSize,
  {
    name: string;
    key: string;
    className: string;
    towerSize: number;
    type: ChipRenderType;
    box: {
      width: number;
      height: number;
    };
  }
> = {
  1: {
    type: "small",
    name: "White",
    key: "white",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  5: {
    type: "small",
    name: "Red",
    key: "red",
    className: "text-white",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  25: {
    type: "small",
    name: "Green",
    key: "green",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  100: {
    type: "small",
    name: "Black",
    key: "black",
    className: "text-white",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  500: {
    type: "small",
    name: "Purple",
    key: "purple",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  1000: {
    type: "small",
    name: "Yellow",
    key: "yellow",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  5000: {
    type: "small",
    name: "Orange",
    key: "orange",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  10000: {
    type: "small",
    name: "Gray",
    key: "gray",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },
  25000: {
    type: "small",
    name: "Light Blue",
    key: "light-blue",
    className: "text-black",
    towerSize: 10,
    box: { width: 32, height: 32 },
  },

  250000: {
    type: "big",
    name: "Big gray",
    key: "big-gray",
    className: "text-black",
    towerSize: 5,
    box: { width: 48, height: 48 },
  },
  1000000: {
    type: "big",
    name: "Big purple",
    key: "big-purple",
    className: "text-white",
    towerSize: 5,
    box: { width: 48, height: 48 },
  },
  5000000: {
    type: "big",
    name: "Big yellow",
    key: "big-yellow",
    className: "text-black",
    towerSize: 5,
    box: { width: 48, height: 48 },
  },

  20000000: {
    type: "card",
    name: "Card gray",
    key: "card-gray",
    className: "text-white",
    towerSize: 4,
    box: { width: 44, height: 60 },
  },
  100000000: {
    type: "card",
    name: "Card purple",
    key: "card-purple",
    className: "text-white",
    towerSize: 4,
    box: { width: 44, height: 60 },
  },
  500000000: {
    type: "card",
    name: "Card yellow",
    key: "card-yellow",
    className: "text-white",
    towerSize: 4,
    box: { width: 44, height: 60 },
  },
  1000000000: {
    type: "card",
    name: "Card black",
    key: "card-black",
    className: "text-white",
    towerSize: 4,
    box: { width: 44, height: 60 },
  },
};

const useChipSizes = (value: bigint): [ChipRenderType, [ChipSize, number][]][] =>
  useMemo(() => {
    // if (value >= 10n ** 15n) return [];
    const chipSizes = [...ChipSizes].reverse(); // Start with the largest chips first
    const result: [ChipSize, number][] = [];
    let remaining = value;

    // Step 1: Greedy approach to use the largest chips first
    for (const chip of chipSizes) {
      const chipValue = BigInt(chip);
      if (remaining >= chipValue) {
        const count = remaining / chipValue;
        remaining -= count * chipValue;
        result.push([chip, Number(count)]);
      }
    }

    // Step 2: Optimization with backtracking
    const backtrackOptimize = (
      chips: [ChipSize, number][],
    ): [ChipSize, number][] => {
      const optimized: [ChipSize, number][] = [];
      const chipMap = new Map<ChipSize, number>();

      // Convert result into a map for easier management
      for (const [chip, count] of chips) {
        chipMap.set(chip, (chipMap.get(chip) || 0) + count);
      }

      // Backtracking to find the most optimal solution
      for (const chip of chipSizes) {
        const chipValue = BigInt(chip);
        const count = chipMap.get(chip) || 0;

        // Try to replace this chip with smaller chips if possible
        for (const smallerChip of chipSizes) {
          if (smallerChip >= chip) continue;

          const smallerChipValue = BigInt(smallerChip * 1e8);
          const maxReplacement = chipValue / smallerChipValue;

          if (
            count > 0 &&
            chipMap.get(smallerChip) &&
            maxReplacement * smallerChipValue === chipValue
          ) {
            const smallerChipCount = chipMap.get(smallerChip)!;
            const neededSmallerChips = Number(maxReplacement) * count;

            if (
              smallerChipCount + neededSmallerChips <=
              ChipNames[smallerChip].towerSize
            ) {
              chipMap.set(smallerChip, smallerChipCount + neededSmallerChips);
              chipMap.set(chip, 0);
            }
          }
        }
      }

      // Convert the map back into an array
      for (const [chip, count] of chipMap) {
        if (count > 0) {
          optimized.push([chip, count]);
        }
      }

      return optimized;
    };

    const optimizedResult = backtrackOptimize(result);

    // Step 3: Sort the final result by chip size
    optimizedResult.sort((a, b) => a[0] - b[0]);

    // Step 3: Group by chip type
    const groupedResult: [ChipRenderType, [ChipSize, number][]][] = [];

    for (const [size, amount] of optimizedResult) {
      const type = ChipNames[size].type;
      const lastGroup = groupedResult[groupedResult.length - 1];

      if (lastGroup && lastGroup[0] === type) {
        lastGroup[1].push([size, amount]);
      } else {
        groupedResult.push([type, [[size, amount]]]);
      }
    }

    return groupedResult;
  }, [value]);

const TowerSize = 10;
export const TowerHeightInRem = (TowerSize / 16) * 5 + 1;
export const TowerHeightInRemForStyle = `${(TowerSize / 16) * 5 + 1}rem`;

const ChipStack = memo<{
  size: ChipSize;
  amount: number;
  className?: string;
  style?: CSSProperties;
}>(({ size, amount, className, style }) => {
  const isInsideModal = useIsInsideModal();

  const { key, box } = useMemo(
    () => ChipNames[size] || { key: "unknown", name: `((${size}))` },
    [size],
  );

  return (
    <motion.div
      className={classNames("flex flex-col justify-end relative", className)}
      style={{
        ...style,
        width: box.width,
        height: 80,
      }}
      title={`${size} x ${amount}`}
    >
      <AnimatePresence>
        {Array.from({ length: amount }, (_, i) => {
          const y = 0;
          return (
            <motion.div
              key={i}
              className="absolute left-0 "
              style={{
                "--chip-offset": `${i * 2}px`,
                "--chip-scale": 1 + i / TowerSize / 10,
                "--chip-rotation": `${i % 3}deg`,
                bottom: i * 5,
                zIndex: i,
                ...box,
              } as CSSProperties}
              initial={
                isInsideModal
                  ? false
                  : { opacity: 0, y: i === 0 ? "0" : y - 16 }
              }
              animate={{ opacity: 1, y }}
              exit={{ opacity: 0, y: 0, transition: { duration: 0.3 } }}
              transition={{ delay: i * 0.05 }}
            >
              <div className="absolute transition-transform inset-0 translate-y-0 rotate-(--chip-rotation) group-hover:rotate-0 group-hover:-translate-y-(--chip-offset)">
                {i === amount - 1 && (
                  <span
                    className={classNames(
                      "absolute flex left-0 top-0 w-full bottom-[2px] type-button-3 justify-center items-center text-center z-1",
                      // towerSize === 100 || towerSize === 500 ? 'text-white' : 'text-black',
                      ChipNames[size].className,
                    )}
                  >
                    {amount}
                  </span>
                )}
                <div className="inline-flex relative z-0" style={box}>
                  <div className="-inset-x-[16px] -top-[8px] absolute">
                    <img
                      src={`/icons/chip-${key}.svg`}
                      className="w-full block"
                    />
                  </div>
                </div>
              </div>
            </motion.div>
          );
        })}
      </AnimatePresence>
    </motion.div>
  );
});
ChipStack.displayName = "ChipStack";

export const ChipsStackModalComponent = memo<PropsWithChildren<{
  currencyType: CurrencyType;
  value: bigint;
  name: string;
  open: boolean;
  onClose(): void;
}>>(({ currencyType, value, name, open, onClose, children }) => {
  const meta = useCurrencyManagerMeta(currencyType);
  const valuePerChip = useValuePerChip(currencyType);

  return (
    <Modal title={name} open={open} onClose={onClose}>
      <ChipsStackComponent value={value} currencyType={currencyType} openable={false} />
      <p className="type-top text-center mt-5">
        <span className="text-orange-300">{TokenAmountToString(value, meta)}</span>{" "}
        <CurrencyTypeSymbolComponent currencyType={currencyType} />
      </p>
      {children}
      <List variant={{ type: "default", readonly: true }} className="w-full">
        {ChipSizes.map((size) => (
          <ListItem
            key={size}
            rightLabel={
              <CurrencyComponent
                currencyType={currencyType}
                variant="inline"
                currencyValue={BigInt(size) * valuePerChip}
              />
            }
            icon={
              <div className="flex w-12 flex-row">
                <img
                  src={`/icons/chip-${ChipNames[size].key}.svg`}
                  className="w-12 -mb-3"
                />
              </div>
            }
          >
            {ChipNames[size].name}
          </ListItem>
        ))}
      </List>
    </Modal>
  );
},
  (prevProps, nextProps) =>
    prevProps.value === nextProps.value &&
    prevProps.name === nextProps.name &&
    prevProps.open === nextProps.open &&
    prevProps.onClose === nextProps.onClose &&
    prevProps.children === nextProps.children && 
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType)
);
ChipsStackModalComponent.displayName = "ChipsStackModalComponent";

export const ChipsStackComponent = memo<PropsWithChildren<{
  value: bigint;
  currencyType: CurrencyType;
  name?: string;
  className?: string;
  openable?: boolean;
  hideChips?: boolean;
}>>(
  ({
    value: _value,
    className,
    openable = true,
    hideChips = false,
    name = "Stack",
    currencyType,
    children
  }) => {
    const valuePerChip = useValuePerChip(currencyType);
    const chips = useMemo(() => _value / valuePerChip, [_value, valuePerChip]);
    const chipTypes = useChipSizes(chips);
    const [modalShown, setModalShown] = useState(false);
    const isInsideModal = useIsInsideModal();
    const { play } = useSound();

    useEffect(() => {
      if (isInsideModal || chips <= 0n) return;
      play("chips-increase");
    }, [chips]);

    if (chips <= 0n) return null;

    return (
      <>
        <div className="flex flex-col relative z-0">
          <Interactable
            className={classNames(
              className,
              "w-full flex flex-row justify-center items-center group",
            )}
            onClick={openable ? () => setModalShown(true) : undefined}
          >
            {!hideChips && (
              <>
                {chipTypes.map(([type, chipSizes]) => (
                  <div key={type} className="flex justify-center flex-row">
                    {chipSizes.map(([size, amount], i, arr) => {
                      const { box } = ChipNames[size];
                      return (
                        <ChipStack
                          key={size}
                          size={size}
                          amount={amount}
                          style={
                            type === "small"
                              ? {
                                zIndex:
                                  arr.length * 2 - i - (i % 2) * arr.length,
                                marginLeft: `${-(box.width / 2)}px`,
                                marginTop: `${(i % 2) * -box.height}px`,
                              }
                              : {
                                marginTop:
                                  type === "card"
                                    ? `${-(box.height / 2)}px`
                                    : undefined,
                              }
                          }
                        />
                      );
                    })}
                  </div>
                ))}
              </>
            )}
          </Interactable>

          <Interactable
            className="flex flex-col items-center justify-end h-full"
            onClick={openable ? () => setModalShown(true) : undefined}
          >
            <CurrencyComponent
              className="mt-auto"
              variant="inline"
              size="medium"
              currencyType={currencyType}
              currencyValue={_value}
            />
          </Interactable>
        </div>
        {modalShown && (
          <ChipsStackModalComponent
            currencyType={currencyType}
            value={_value}
            name={name}
            open={modalShown}
            onClose={() => setModalShown(false)}
          >
            {children}
          </ChipsStackModalComponent>
        )}
      </>
    );
  },
  (prevProps, nextProps) =>
    prevProps.value === nextProps.value &&
    prevProps.className === nextProps.className &&
    prevProps.openable === nextProps.openable &&
    prevProps.hideChips === nextProps.hideChips &&
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType) &&
    prevProps.children === nextProps.children
);
ChipsStackComponent.displayName = "ChipsStackComponent";
