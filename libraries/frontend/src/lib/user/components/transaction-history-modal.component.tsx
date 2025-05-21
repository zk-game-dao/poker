import { differenceInCalendarYears, differenceInDays } from 'date-fns';
import { formatInTimeZone } from 'date-fns-tz';
import { memo, useCallback, useMemo, useState } from 'react';

import { Transaction } from '@declarations/users_canister/users_canister.did';
import { useCurrencyManagerMeta } from '@zk-game-dao/currency';
import { ButtonComponent, List, ListItem, Modal, ModalFooterPortal } from '@zk-game-dao/ui';

import { TokenAmountToFloat } from '../../utils/token-amount-conversion';

const ITEMS_PER_PAGE = 25;

export const TransactionHistoryModalComponent = memo<{
  transactions: Transaction[];
  onClose(): void;
  isOpen?: boolean;
}>(({ onClose, transactions, isOpen = true }) => {
  const [page, setPage] = useState(0);
  const meta = useCurrencyManagerMeta({ Real: { ICP: null } });

  // Helper to format transaction type and amount as a readable string
  const formatTransactionDescription = useCallback(
    (transaction: Transaction) => {
      const amount = `${TokenAmountToFloat(transaction.amount, meta)} ${transaction.currency[0] ?? "-"
        }`;

      if ("Withdraw" in transaction.transaction_type)
        return `Withdrew ${amount}`;
      if ("Deposit" in transaction.transaction_type)
        return `Deposited ${amount}`;
      if ("TableWithdraw" in transaction.transaction_type)
        return `Withdrew ${amount} from Table ${transaction.transaction_type.TableWithdraw.table_id.toText()}`;
      if ("Transfer" in transaction.transaction_type)
        return `Transferred ${amount} to ${transaction.transaction_type.Transfer.recipient.toText()} (${transaction.transaction_type.Transfer.transfer_type
          })`;
      if ("TableDeposit" in transaction.transaction_type)
        return `Deposited ${amount} to Table ${transaction.transaction_type.TableDeposit.table_id.toText()}`;
      if ("Receive" in transaction.transaction_type)
        return `Received ${amount} from ${transaction.transaction_type.Receive.sender.toText()} (${transaction.transaction_type.Receive.transfer_type
          })`;
      return `Unknown Transaction: ${amount}`;
    },
    [],
  );

  // Helper to format timestamp into a human-readable date string
  const getTimeString = useCallback((timestamp: bigint) => {
    const date = new Date(Number(timestamp / 1_000_000n));
    const days = differenceInDays(new Date(), date);
    if (days === 0)
      return formatInTimeZone(
        date,
        Intl.DateTimeFormat().resolvedOptions().timeZone,
        "HH:mm",
      );
    if (days === 1) return "Yesterday";
    if (days > 1 && days < 7)
      return (
        "last " +
        formatInTimeZone(
          date,
          Intl.DateTimeFormat().resolvedOptions().timeZone,
          "EEEE",
        )
      );

    const years = differenceInCalendarYears(new Date(), date);
    if (years > 0)
      return formatInTimeZone(
        date,
        Intl.DateTimeFormat().resolvedOptions().timeZone,
        "d MMM yyyy",
      );
    return formatInTimeZone(
      date,
      Intl.DateTimeFormat().resolvedOptions().timeZone,
      "d MMM HH:mm",
    );
  }, []);

  // Pagination handlers
  const handlePrevious = () => setPage(Math.max(0, page - 1));
  const handleNext = () =>
    setPage(
      Math.min(page + 1, Math.floor(transactions.length / ITEMS_PER_PAGE)),
    );

  // Slice transactions for the current page
  const paginatedTransactions = useMemo(
    () =>
      transactions.slice(page * ITEMS_PER_PAGE, (page + 1) * ITEMS_PER_PAGE),
    [transactions, page],
  );

  return (
    <Modal title="Transaction History" onClose={onClose} open={isOpen}>
      <List>
        {paginatedTransactions.map((transaction) => (
          <ListItem
            key={transaction.transaction_id}
            rightLabel={
              <span className="whitespace-nowrap">
                {getTimeString(transaction.timestamp)}
              </span>
            }
          >
            <p className="text-left">
              {formatTransactionDescription(transaction)}
            </p>
          </ListItem>
        ))}
      </List>
      {transactions.length > ITEMS_PER_PAGE && (
        <ModalFooterPortal>
          <ButtonComponent onClick={handlePrevious} variant="naked">
            Previous
          </ButtonComponent>
          <span>
            {page + 1} / {Math.ceil(transactions.length / ITEMS_PER_PAGE)}
          </span>
          <ButtonComponent onClick={handleNext} variant="naked">
            Next
          </ButtonComponent>
        </ModalFooterPortal>
      )}
    </Modal>
  );
});
TransactionHistoryModalComponent.displayName =
  "TransactionHistoryModalComponent";