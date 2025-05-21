import { memo } from 'react';

import {
  CurrencyComponent, CurrencyInputComponent, CurrencyType, IsSameCurrencyType
} from '@zk-game-dao/currency';
import {
  ButtonComponent, ErrorComponent, FauxLoadingBarAnimationComponent, List, Modal,
  ModalFooterPortal
} from '@zk-game-dao/ui';

import { BB_MULT_MAX_BUY_IN } from './constants';

export const JoinTableModalComponent = memo<{
  show: boolean;
  onClose: () => void;
  error?: unknown;
  minBuyIn: bigint;
  maxBuyIn: bigint;
  buyIn: bigint;
  setBuyIn: (buyIn: bigint) => void;
  transactionFee: bigint;
  currencyType: CurrencyType;
  hasTable: boolean;
  isPending: boolean;
  mutate(): void;
}>(({
  show,
  onClose,
  error,
  minBuyIn,
  maxBuyIn,
  buyIn,
  setBuyIn,
  transactionFee,
  currencyType: currency,
  hasTable,
  isPending,
  mutate,
}) => (
  <Modal open={show} onClose={onClose}>
    <div className="flex flex-col gap-3">
      <p className="type-top text-center w-full">
        Join the table
      </p>
      <div className="text-center w-full whitespace-pre-wrap inline">
        {'You need to deposit at least '}
        <CurrencyComponent
          currencyType={currency}
          size="small"
          className="inline ml-2 -translate-y-0.25"
          currencyValue={minBuyIn}
        />
        {' to join the table.'}
      </div>
    </div>
    <ErrorComponent error={error} />
    {hasTable && (
      <List>
        <CurrencyInputComponent
          currencyType={currency}
          label="Buy-in"
          value={buyIn}
          onChange={setBuyIn}
          min={minBuyIn}
          max={maxBuyIn}
        />
        <CurrencyInputComponent
          currencyType={currency}
          label="Fee"
          value={transactionFee}
          disabled
        />
      </List>
    )}

    {(isPending) && (
      <FauxLoadingBarAnimationComponent>
        Joining
      </FauxLoadingBarAnimationComponent>
    )}

    <div className="type-callout opacity-30 inline whitespace-pre-wrap">
      The maximum buy-in is
      <CurrencyComponent
        currencyType={currency}
        size="small"
        className="inline ml-2 -translate-y-0.25"
        currencyValue={maxBuyIn}
      />
      {` (${BB_MULT_MAX_BUY_IN}x the big blind).`}
    </div>

    <ModalFooterPortal>
      <ButtonComponent
        variant="naked"
        onClick={onClose}
      >
        Cancel
      </ButtonComponent>
      <ButtonComponent
        onClick={mutate}
        isLoading={isPending}
      >
        Join table
      </ButtonComponent>
    </ModalFooterPortal>
  </Modal>
),
  (prevProps, nextProps) => (
    prevProps.show === nextProps.show &&
    prevProps.minBuyIn === nextProps.minBuyIn &&
    prevProps.maxBuyIn === nextProps.maxBuyIn &&
    prevProps.buyIn === nextProps.buyIn &&
    prevProps.transactionFee === nextProps.transactionFee &&
    IsSameCurrencyType(prevProps.currencyType, nextProps.currencyType) &&
    prevProps.hasTable === nextProps.hasTable &&
    prevProps.isPending === nextProps.isPending &&
    prevProps.error === nextProps.error &&
    prevProps.setBuyIn === nextProps.setBuyIn &&
    prevProps.mutate === nextProps.mutate &&
    prevProps.onClose === nextProps.onClose
  )
);
JoinTableModalComponent.displayName = 'JoinTableModalComponent';
