import {
  createContext, memo, ReactNode, useCallback, useContext, useEffect, useMemo, useState
} from 'react';

import { User } from '@declarations/table_canister/table_canister.did';
import { Queries, queryClient } from '@lib/data';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import {
  FloatToTokenAmount, TokenAmountToFloat, TokenAmountToString
} from '@lib/utils/token-amount-conversion';
import { useMutation } from '@tanstack/react-query';
import { CurrencyType, useCurrencyManagerMeta } from '@zk-game-dao/currency';
import { useErrorModal } from '@zk-game-dao/ui';

import { useSound } from '../../../../context/sound.context';
import { useMyTableUser, useTable } from '../../context/table.context';

export type HUDContextType = {
  // Raising
  raise?: {
    quickActions: [bigint, string][];
    value: bigint;
    change(raiseValue: bigint): void;
    min: bigint;
    max: bigint;
    showInlineInput: boolean;
    setShowInlineInput(show: boolean): void;
    // step: bigint;

    cta: {
      mutateExplicit(raiseValue: bigint): Promise<void>;
      mutate(): Promise<void>;
      isPending: boolean;
    };
  };

  // Check
  check?: {
    mutate(): void;
    isPending: boolean;
  };

  // Call
  call?: {
    mutate(): void;
    isPending: boolean;
    hoverLabel: string;
  };

  // Fold
  fold?: {
    mutate(): void;
    isPending: boolean;
  };

  allIn?: {
    mutate(): void;
    isPending: boolean;
  };

  autoCheckFold?: {
    mutate(isEnabled: boolean): void;
    isPending: boolean;
    data?: boolean;
  };

  tableUser?: User;
  currencyType: CurrencyType;
};

const HUDContext = createContext<HUDContextType>({
  currencyType: { Fake: null },
});

export const ProvideHUDBettingContext = memo<{ children: ReactNode }>(
  ({ children }) => {
    const { currentBet, table, actor: service, user, userIndex } = useTable();
    const [tableUser] = useMyTableUser();
    const [raiseTo, setRaiseTo] = useState<bigint>(0n);
    const { play } = useSound();
    const { user: zkpUser } = useUser();
    const { currencyType } = useTable();
    const meta = useCurrencyManagerMeta(currencyType);
    const callValue = useMemo(() => table.highest_bet, [table, user]);
    const showErrorModal = useErrorModal();
    const [showInlineInput, setShowInlineInput] = useState(false);

    const isUserTurn = useMemo(() => table.current_player_index === userIndex, [
      table,
      userIndex,
    ]);

    const getRaiseToFromDelta = useCallback(
      (delta: bigint) => callValue + delta,
      [table, user],
    );
    const getPrice = useCallback(
      (value: bigint) => (!user?.data ? value : value - user.data.current_total_bet),
      [table, user],
    );

    const quickActions = useMemo((): [bigint, string][] => {
      let _quickActions: [bigint, string][] = [];
      if (!isUserTurn) return _quickActions;
      if (!table || !user || !tableUser || !user.data)
        return _quickActions.map(([amount, label]) => [
          TokenAmountToFloat(amount, meta),
          label,
        ]);
      if (table.last_raise) {
        _quickActions.push(
          [getRaiseToFromDelta(table.last_raise * 2n), "Min"],
          [getRaiseToFromDelta(table.last_raise * 3n), "3x Last raise"],
        );
      } else if (table.big_blind) {
        _quickActions.push(
          [getRaiseToFromDelta(table.big_blind * 2n), "Min"],
          [getRaiseToFromDelta(table.big_blind * 3n), "3x BB"],
        );
      }

      const potToValue = getRaiseToFromDelta(table.pot);
      // If pot is bigger than min bet
      if (
        table.pot &&
        _quickActions.length > 0 &&
        potToValue > _quickActions[0][0]
      ) {
        _quickActions.push([potToValue, "Pot"]);

        // If half pot is bigger than min bet
        const halfPotToValue = getRaiseToFromDelta(table.pot / 2n);
        if (halfPotToValue > _quickActions[0][0])
          _quickActions.push([halfPotToValue, "1/2 Pot"]);
      }

      _quickActions = _quickActions.filter(
        ([amount]) => getPrice(amount) < tableUser.balance,
      );

      if (tableUser.balance > 0n)
        _quickActions.push([
          user.data.current_total_bet + tableUser.balance,
          "All in",
        ]);

      if ("PotLimit" in table.config.game_type)
        _quickActions = _quickActions.filter(
          ([amount]) => getPrice(amount) > table.pot,
        );

      return _quickActions.sort((a, b) =>
        TokenAmountToFloat(a[0] - b[0], meta),
      );
    }, [
      table,
      tableUser,
      user,
      getRaiseToFromDelta,
      getRaiseToFromDelta,
      getPrice,
      isUserTurn,
    ]);

    const [min, max] = useMemo(() => {
      if (!quickActions.length) return [0n, 0n];
      return [quickActions[0][0], quickActions[quickActions.length - 1][0]];
    }, [quickActions]);

    useEffect(() => {
      setRaiseTo((v) =>
        FloatToTokenAmount(
          Math.min(
            Math.max(
              TokenAmountToFloat(v, meta),
              TokenAmountToFloat(min, meta),
            ),
            TokenAmountToFloat(max, meta),
          ),
          meta,
        ),
      );
    }, [min, max]);

    // Rest the value everytime your turn starts
    useEffect(() => {
      if (isUserTurn) {
        setRaiseTo(min);
      } else {
        setShowInlineInput(false);
      }
    }, [min, isUserTurn]);

    const { mutateAsync: submit, isPending } = useMutation({
      mutationFn: async (_raiseTo: bigint) => {
        if (!table || !tableUser || !user) throw "Table or user not found";
        const result = await service.place_bet(tableUser.principal_id, {
          Raised: _raiseTo,
        });
        if ("Err" in result) throw result.Err;
        await queryClient.invalidateQueries({ queryKey: ["table", table.id] });
        return result.Ok;
      },
      onError: showErrorModal,
    });

    useEffect(() => {
      play("turn-notification");
    }, []);

    const { mutate: check, isPending: checking } = useMutation({
      mutationFn: async () => {
        if (!table || !zkpUser) throw "Table or user not found";
        const result = await service.check(zkpUser.principal_id);
        if ("Err" in result) throw result.Err;
        await Queries.table.invalidate(table);
        return result.Ok;
      },
      onError: showErrorModal,
    });

    const minRequiredBet = useMemo(() => {
      if (!user?.data || !currentBet) return 0n;
      return currentBet - user.data.current_total_bet;
    }, [currentBet, user?.data?.current_total_bet]);

    const { mutate: call, isPending: isCalling } = useMutation({
      mutationFn: async () => {
        if (!table || !zkpUser) throw "Table or user not found";
        const result = await service.place_bet(zkpUser.principal_id, {
          Called: null,
        });
        if ("Err" in result) throw result.Err;
        await Queries.table.invalidate(table);
        return result.Ok;
      },
      onError: showErrorModal,
    });

    const { mutate: allIn, isPending: isGoingAllIn } = useMutation({
      mutationFn: async () => {
        if (!table || !zkpUser || !tableUser || !user?.data)
          throw "Table or user not found";
        const result = await service.place_bet(zkpUser.principal_id, {
          Raised: user.data.current_total_bet + tableUser.balance,
        });
        if ("Err" in result) throw result.Err;
        await Queries.table.invalidate(table);
        return result.Ok;
      },
      onError: showErrorModal,
    });

    const { mutate: fold, isPending: isFolding } = useMutation({
      mutationFn: async () => {
        if (!table || !zkpUser) throw "Table or user not found";
        return callActorMutation(service, 'fold', zkpUser.principal_id, table.current_player_index !== userIndex);
      },
      onSuccess: () => Queries.table.invalidate(table),
      onError: showErrorModal,
    });

    // const { mutate: setAutoCheckFold, isPending: isSettingAutoCheckFold } = useMutation({
    //   mutationFn: async (isEnabled: boolean) => {
    //     if (!table || !zkpUser) throw "Table or user not found";
    //     const result = await service.set_auto_check_fold(zkpUser.principal_id, isEnabled);
    //     if ("Err" in result) throw result.Err;
    //     await Queries.table.invalidate(table);
    //     return result.Ok;
    //   },
    //   onError: showErrorModal,
    // });

    const value = useMemo(() => {
      const v: HUDContextType = {
        currencyType,
      };

      if (user?.data && !("Folded" in (user.data.player_action ?? {})))
        v.fold = {
          isPending: isFolding,
          mutate: fold,
        };

      // if (user?.data) {
      //   v.autoCheckFold = {
      //     isPending: isSettingAutoCheckFold,
      //     mutate: setAutoCheckFold,
      //     data: user.data.auto_check_fold,
      //   };
      // }

      if (minRequiredBet === 0n) {
        v.check = {
          isPending: checking,
          mutate: check,
        };
      } else {
        v.call = {
          isPending: isCalling,
          mutate: call,
          hoverLabel: TokenAmountToString(minRequiredBet, meta),
        };
      }

      if (quickActions.length > 1) {
        v.raise = {
          min,
          max,
          quickActions,
          value: raiseTo,
          change: setRaiseTo,
          showInlineInput,
          setShowInlineInput,
          cta: {
            async mutateExplicit(raiseValue) {
              setRaiseTo(raiseValue);
              await submit(raiseValue);
            },
            mutate: async () => {
              await submit(raiseTo);
            },
            isPending,
          },
        };
      }

      v.allIn = {
        isPending: isGoingAllIn,
        mutate: allIn,
      };

      return v;
    }, [
      minRequiredBet,
      checking,
      check,
      isCalling,
      call,
      isFolding,
      fold,
      isGoingAllIn,
      allIn,
      quickActions,
      raiseTo,
      setRaiseTo,
      submit,
      isPending,
      tableUser,
      getPrice,
      min,
      max,
      showInlineInput,
      setShowInlineInput,
      meta,
      user,
      userIndex,
      zkpUser,
      // setAutoCheckFold,
      // isSettingAutoCheckFold,
      currentBet,
      table,
      currencyType,
      play,
      showErrorModal,
      isUserTurn,
    ]);

    return <HUDContext.Provider value={value}>{children}</HUDContext.Provider>;
  },
);
ProvideHUDBettingContext.displayName = "ProvideHUDBettingContext";

export const useHUDBetting = () => useContext(HUDContext);

export const HUDBettingConsumer = HUDContext.Consumer;

export const useSitOut = () => {
  const { actor, table } = useTable();
  const showErrorModal = useErrorModal();
  const [user, data] = useMyTableUser();

  const { mutate, isPending } = useMutation({
    mutationFn: async (sitOut: boolean) => {
      if (!table || !user) throw "Table or user not found";
      if (sitOut)
        return await callActorMutation(actor, "player_sitting_out", user.principal_id);
      return await callActorMutation(actor, "player_sitting_in", user.users_canister_id, user.principal_id, true);
    },
    onSuccess: () => Queries.table.invalidate(table),
    onError: showErrorModal,
  });

  const isSittingOut = useMemo(
    () => data && "SittingOut" in data.player_action,
    [data?.player_action],
  );
  const isSittingBackIn = useMemo(
    () =>
      !!(
        user?.principal_id &&
        table.queue.find(
          (v) =>
            "SittingIn" in v &&
            v.SittingIn[0].compareTo(user.principal_id) === "eq",
        )
      ),
    [user?.principal_id, table.queue],
  );

  return useMemo(
    () => ({
      sitOut: () => mutate(true),
      rejoin: () => mutate(false),
      isPending,
      isSittingOut,
      isSittingBackIn,
    }),
    [mutate, isPending, isSittingOut, isSittingBackIn],
  );
};
