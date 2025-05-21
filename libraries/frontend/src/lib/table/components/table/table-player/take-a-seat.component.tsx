import classNames from 'classnames';
import { addMinutes } from 'date-fns';
import { memo, useMemo, useState } from 'react';

import { Queries } from '@lib/data';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { WrapOptional } from '@lib/utils/optional';
import { useMutation } from '@tanstack/react-query';
import {
  TokenAmountToString, useAllowance, useAuth, useCurrencyManagerMeta
} from '@zk-game-dao/currency';
import { Image, Interactable, UserError, useToast } from '@zk-game-dao/ui';

import { useTournament } from '../../../../tournament/context/tournament.context';
import { useTableSeat } from '../../../context/table-seat.context';
import { useTable } from '../../../context/table.context';
import { JoinTableModalComponent } from './join-table-modal.component';

const BB_MULT_MIN_BUY_IN = 40;
const BB_MULT_MAX_BUY_IN = 100;

export const TakeASeatComponent = memo(() => {
  const tournament = useTournament();
  const { seatIndex } = useTableSeat();
  const { table, actor, isJoined, currencyType: currency, receiver } = useTable();
  const { user: zkpUser } = useUser();
  const { authData } = useAuth();
  const { addToast } = useToast();

  const [showJoinModal, setShowJoinModal] = useState(false);

  const tableAllowance = useAllowance({ currencyType: currency, receiver, name: 'Table' });

  // const transactionFee = useTransactionFee(currency);
  const meta = useCurrencyManagerMeta(currency);

  const { maxBuyIn, minBuyIn } = useMemo(() => {
    if (!table) return { maxBuyIn: 0n, minBuyIn: 0n };
    return {
      maxBuyIn: table.big_blind * BigInt(BB_MULT_MAX_BUY_IN),
      minBuyIn: table.big_blind * BigInt(BB_MULT_MIN_BUY_IN),
    };
  }, [table, meta]);

  const [buyIn, setBuyIn] = useState(minBuyIn);
  const joinMutation = useMutation({
    mutationFn: async () => {
      if (!table || !zkpUser || !authData) throw "Table or user not found";
      if (tournament) throw new UserError("You can't join a tournament table this way");

      if (buyIn < minBuyIn)
        throw new UserError(`Minimum buy-in is ${TokenAmountToString(minBuyIn, meta)} ${currency}`);

      if (buyIn > maxBuyIn)
        throw new UserError(`Maximum buy-in is ${TokenAmountToString(maxBuyIn, meta)} ${currency}`);

      if ("Real" in table.config.currency_type)
        await tableAllowance.require({ amount: buyIn, reason: 'Join table' }, addMinutes(new Date(), 2));

      return await callActorMutation(
        actor,
        "join_table",
        zkpUser.users_canister_id,
        zkpUser.principal_id,
        WrapOptional(BigInt(seatIndex)),
        buyIn,
        false,
      );
    },
    onSuccess: () => {
      Queries.table.invalidate(table);
      Queries.walletBalance.invalidate(currency, authData);
      addToast({ children: "You have joined the table" });
      setShowJoinModal(false);
    },
  });

  // Only show the button for tournament if the tournament is joinable or rebuyable
  if (tournament)
    return null;

  return (
    <>
      {!isJoined && (
        <Interactable
          className={classNames(
            "material rounded-full p-2 z-2 active:scale-95 transition-transform",
            { "scale-110": showJoinModal },
          )}
          onClick={() => setShowJoinModal(true)}
        >
          <div className="w-6 h-6">
            <Image src="/icons/plus.svg" type="svg" alt="Empty seat" />
          </div>
        </Interactable>
      )}
      <JoinTableModalComponent
        show={showJoinModal}
        onClose={() => setShowJoinModal(false)}
        minBuyIn={minBuyIn}
        maxBuyIn={maxBuyIn}
        buyIn={buyIn}
        setBuyIn={setBuyIn}
        transactionFee={meta.transactionFee}
        currencyType={currency}
        hasTable={!!table}
        isPending={joinMutation.isPending}
        mutate={joinMutation.mutate}
        error={joinMutation.error}
      />
    </>
  );
});
TakeASeatComponent.displayName = 'TakeASeatComponent';
