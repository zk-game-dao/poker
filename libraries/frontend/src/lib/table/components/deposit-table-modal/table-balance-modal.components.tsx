import { Queries } from '@lib/data';
import { useUser } from '@lib/user';
import { FloatToTokenAmount } from '@lib/utils/token-amount-conversion';
import {
  CurrencyComponent,
  CurrencyInputComponent,
  useAllowance,
  useAuth,
  useCurrencyManager,
  useTransactionFee,
} from '@zk-game-dao/currency';
import { ButtonComponent, ErrorComponent, List, ListItem, Modal, ModalFooterPortal, TabsComponent } from '@zk-game-dao/ui';
import { addMinutes } from 'date-fns';
import { memo, useMemo, useState } from 'react';

import { useActorMutation } from '../../../../hooks/actor-mutation';
import { Min } from '../../../utils/bigint';
import { useMyTableUser, useTable } from '../../context/table.context';

export const TableBalanceModalComponent = memo<{
  isOpen: boolean;
  onClose(): void;
}>(({ isOpen, onClose }) => {
  const [type, setType] = useState<"deposit" | "withdraw">("deposit");

  const { user } = useUser();
  const { authData } = useAuth();
  const { actor, table, isOngoing, currencyType: currency, receiver } = useTable();
  const [tableUser] = useMyTableUser();
  const allowance = useAllowance({ currencyType: currency, receiver, name: 'Table' });

  const [amount, setAmount] = useState(0n);

  const transactionFee = useTransactionFee(currency);
  const { meta } = useCurrencyManager(currency);

  const cumulatedQueuedDepositAmount = useMemo((): bigint => {
    if (!tableUser) return 0n;
    if (!table) return 0n;
    return table.queue.reduce((acc, item) => {
      if ("Deposit" in item && item.Deposit[0].compareTo(tableUser.users_canister_id) === "eq") {
        return acc + item.Deposit[2] + transactionFee;
      }
      return acc;
    }, 0n);
  }, [table.queue, tableUser, transactionFee]);

  const maxHandDurationInMinutes = useMemo(() => {
    if (!table) return 30;
    // Lets assume that one card is raised 5 times raised on average
    const minutesPerBettingRound = (table.config.timer_duration / 60) * table.users.users.length * 5;
    // A betting round is flop, turn, river, and showdown...
    return minutesPerBettingRound * 4 + table.config.auto_start_timer / 60;
  }, [table]);

  const depositMutation = useActorMutation(actor, "deposit_to_table", {
    normalizeParams: async () => {
      if (!tableUser || !authData) throw "User or table user not found";
      await allowance.require(
        {
          amount: cumulatedQueuedDepositAmount + amount,
          reason: 'Deposit to table'
        },
        addMinutes(new Date(), maxHandDurationInMinutes)
      );
      return [
        tableUser.users_canister_id,
        authData.principal,
        amount,
        false
      ];
    },
    invalidateQueries: [Queries.table.key(table)],
    onSuccess: onClose,
  });

  const withdrawMutation = useActorMutation(actor, "withdraw_from_table", {
    normalizeParams: async () => {
      if (!tableUser || !authData) throw "User or table user not found";
      return [
        tableUser.users_canister_id,
        authData.principal,
        amount,
      ];
    },
    invalidateQueries: [Queries.table.key(table)],
    onSuccess: onClose,
  });

  if (!user || !tableUser) return null;

  return (
    <Modal title="Your balance on the table" open={isOpen} onClose={onClose}>
      <TabsComponent
        value={type}
        onChange={(v) => setType(v)}
        tabs={[
          { label: "Deposit", value: "deposit" },
          { label: "Withdraw", value: "withdraw" },
        ]}
      />

      <List>
        <CurrencyInputComponent
          min={0n}
          max={
            type === "withdraw"
              ? tableUser.balance
              : Min(tableUser.balance, FloatToTokenAmount(100, meta))
          }
          currencyType={currency}
          label={`Amount to ${type}`}
          value={amount}
          onChange={setAmount}
        />
        <ListItem
          rightLabel={
            <CurrencyComponent
              currencyType={currency}
              variant="inline"
              currencyValue={transactionFee}
            />
          }
        >
          Fee
        </ListItem>
      </List>

      <ErrorComponent error={withdrawMutation.error || depositMutation.error} />

      {isOngoing && (
        <p className="type-body text-material-heavy-1">
          Your table balance will be updated after the round is finished.
        </p>
      )}

      <ModalFooterPortal>
        <ButtonComponent variant="naked" onClick={onClose}>
          Close
        </ButtonComponent>
        <ButtonComponent
          isLoading={withdrawMutation.isPending || depositMutation.isPending}
          onClick={
            type === "deposit"
              ? depositMutation.mutate
              : withdrawMutation.mutate
          }
        >
          {type === "deposit" ? "Deposit" : "Withdraw"}
        </ButtonComponent>
      </ModalFooterPortal>
    </Modal>
  );
});
TableBalanceModalComponent.displayName = 'TableBalanceModalComponent';
