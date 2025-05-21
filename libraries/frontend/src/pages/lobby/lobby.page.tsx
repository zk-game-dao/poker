import classNames from 'classnames';
import { memo, useEffect, useMemo, useState } from 'react';

import { createActor as createTableActor } from '@declarations/table_canister';
import { table_index } from '@declarations/table_index';
import {
  CurrencyType, FilterOptions, PublicTable, TableConfig
} from '@declarations/table_index/table_index.did';
import { Principal } from '@dfinity/principal';
import { Queries } from '@lib/data';
import {
  CreateTableModalComponent
} from '@lib/table/components/create-table-modal/create-table-modal.component';
import { useUser } from '@lib/user';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { useQueries, useQuery } from '@tanstack/react-query';
import { useIsBTC } from '@zk-game-dao/currency';
import {
  ErrorComponent, Image, LayoutComponent, LoadingAnimationComponent, PillComponent,
  SmallHeroComponent
} from '@zk-game-dao/ui';

import { LobbyFilterModalComponent } from './lobby-filter-modal.component';
import { LobbyTableCardComponent } from './lobby-table-card.component';
import { NoPlayersActiveModal } from './no-players-active-modal.component';

function FilterTagComponent<Key extends keyof FilterOptions>({
  option,
  value,
}: {
  option: Key;
  value: FilterOptions[Key];
}) {
  if (value.length <= 0) return <p>{option}: All</p>;
  switch (option) {
    case "currency_type":
      if ("Real" in (value[0] as CurrencyType)) return "Real currency";
      return "Fake currency";
    case "seats":
      return `${value[0]} seats`;
    case "game_type":
      if (typeof value[0] !== "object") return "All limits";
      if ("FixedLimit" in value[0]) return "Fixed limit";
      if ("NoLimit" in value[0]) return "No limit";
      if ("PotLimit" in value[0]) return "Pot limit";
      if ("SpreadLimit" in value[0]) return "Spread limit";
      return "All limits";
    case "timer_duration":
      return `${value[0]}s timer`;
  }
}

export const LobbyPage = memo(() => {
  const { user } = useUser();
  const isBTC = useIsBTC();

  const [options, setOptions] = useState<FilterOptions>({
    currency_type: [],
    seats: [],
    game_type: [],
    timer_duration: [],
    exclude_currency_type: [],
    exclude_seats: [],
    exclude_game_type: [],
    exclude_timer_duration: [],
  });

  const normalOptions = useMemo((): FilterOptions => {
    if (isBTC)
      return {
        ...options,
        exclude_currency_type: [],
        currency_type: [{ Real: { BTC: null } }],
      };
    return {
      ...options,
      exclude_currency_type: [{ Real: { BTC: null } }],
    };
  }, [options, isBTC]);

  const pageSize = useMemo(() => 12, []);

  const [page, setPage] = useState<number>(0);

  const [isFilterModalOpen, setIsFilterModalOpen] = useState(false);

  useEffect(() => setPage(0), [pageSize]);

  const {
    data: publicTableIDs,
    error,
    isPending,
  } = useQuery({
    queryKey: Queries.lobby.key(normalOptions, [pageSize, page]),
    queryFn: async (): Promise<[Principal, TableConfig][]> => callActorMutation(
      table_index,
      "get_tables",
      [normalOptions],
      page,
      pageSize,
    ),
    refetchInterval: 10000,
  });

  const userTables = useQueries({
    queries: user?.active_tables?.map((id) => ({
      queryKey: Queries.table.key({ id }),
      queryFn: async (): Promise<PublicTable> => await callActorMutation(table_index, "get_table", id),
      refetchInterval: 10000,
    })) || [],
    combine: (result) =>
      result.map((r) => r.data).filter((r): r is PublicTable => !!r),
  });

  const publicTables = useQueries({
    queries: publicTableIDs?.map(([id]) => ({
      queryKey: Queries.table.key({ id }),
      queryFn: async (): Promise<PublicTable> => await callActorMutation(table_index, "get_table", id),
      refetchInterval: 10000,
    })) || [],
    combine: (result) =>
      result.map((r) => r.data).filter((r): r is PublicTable => !!r),
  });

  const [isCreateTableModalOpen, setIsCreateTableModalOpen] = useState(false);

  const resultMeta = useMemo(() => {
    if (error)
      return (
        <ErrorComponent
          error={error}
          className="m-auto"
          title="Failed to fetch tables"
        />
      );
    if (isPending)
      return (
        <LoadingAnimationComponent className="m-auto">
          Fetching tables
        </LoadingAnimationComponent>
      );
    if (!publicTables || publicTables.length === 0)
      return (
        <SmallHeroComponent
          icon={{
            type: "png",
            src: "/icons/table.png",
            width: 64,
            height: 64,
            alt: "Table",
          }}
          title="There are no active tables."
          subtitle="Create a table or wait for a table to spawn."
        />
      );
  }, [publicTables, isPending, error]);

  const activeFilterAmount = useMemo(
    () =>
      Object.entries(options).filter(([, value]) => value.length > 0).length,
    [options],
  );

  const { data: areUsersActive } = useQuery({
    queryKey: ['table_count'],
    queryFn: async (): Promise<boolean> => {
      const table = await callActorMutation(table_index, "get_tables", [], 0, 1);
      if (table.length === 0) return false;
      const actor = createTableActor(table[0][0]);
      return (await callActorMutation(actor, "get_table")).seats.some((seat) => !('Empty' in seat));
    },
    initialData: true,
    refetchInterval: 10000,
    retryDelay: 4000,
    retry: 3,
  });

  const [areUsersActiveMessage, setShowUsersActiveMessage] = useState(true);

  return (
    <LayoutComponent footer>
      {userTables.length > 0 && (
        <>
          <h1 className="container type-medior lg:mt-6 mx-auto mb-4">
            Joined tables
          </h1>
          <div className="mx-auto w-full container gap-2 grid grid-cols-1 mb-4 lg:mb-6 lg:grid-cols-2 xl:grid-cols-3">
            {userTables?.map((table, i) => (
              <LobbyTableCardComponent
                variant="small"
                {...table}
                index={i}
                key={table.id.toText()}
              />
            ))}
          </div>
        </>
      )}
      <NoPlayersActiveModal
        isOpen={!areUsersActive && areUsersActiveMessage}
        onClose={() => setShowUsersActiveMessage(false)}
        onCreate={() => {
          setShowUsersActiveMessage(false);
          setIsCreateTableModalOpen(true);
        }}
      />
      <div className="container mx-auto flex flex-row justify-start items-center relative mb-4 lg:mb-6">
        <PillComponent
          size="large"
          className={classNames("relative z-1", {
            "border-2 border-white": activeFilterAmount,
          })}
          onClick={() => setIsFilterModalOpen(true)}
        >
          {activeFilterAmount ? `${activeFilterAmount} ` : ""}Filter
          {`${activeFilterAmount === 1 ? "" : "s"}`}
        </PillComponent>

        {!!activeFilterAmount && <div className="w-4" />}

        {Object.entries(options)
          .filter(([, value]) => value.length > 0)
          .map(([key, value]) => (
            <PillComponent
              key={key}
              size="small"
              className="relative z-1 ml-2  opacity-30 hover:opacity-100 transition-opacity"
              onClick={() => setOptions((v) => ({ ...v, [key]: [] }))}
            >
              <FilterTagComponent option={key as any} value={value as any} />
              <Image
                type="svg"
                alt="Close"
                src="/icons/xmark.svg"
                className="flex pointer-events-none ml-1 h-3"
              />
            </PillComponent>
          ))}

        <div className="flex flex-1" />
        <PillComponent
          size="large"
          className="relative z-1"
          onClick={() => setIsCreateTableModalOpen(true)}
        >
          Create table
        </PillComponent>
      </div>

      <CreateTableModalComponent
        open={isCreateTableModalOpen}
        onCancel={() => setIsCreateTableModalOpen(false)}
      />

      {isFilterModalOpen && (
        <LobbyFilterModalComponent
          hideCurrencyType={isBTC}
          options={options}
          setOptions={(_options) => {
            setIsFilterModalOpen(false);
            setOptions(_options);
          }}
        />
      )}

      {resultMeta && (
        <div
          className={classNames(
            "container basis-1 h-full flex flex-col grow mb-4 mx-auto",
            {
              "absolute inset-x-0 z-10":
                publicTables && publicTables.length > 0,
            },
          )}
        >
          {resultMeta}
        </div>
      )}

      <div className="mx-auto w-full container gap-2 grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3">
        {publicTables?.map((table, i) => (
          <LobbyTableCardComponent
            variant="large"
            {...table}
            index={i}
            key={table.id.toText()}
          />
        ))}
      </div>

      <div className="container flex flex-row justify-between items-center mt-6 mx-auto">
        {page === 0 ? (
          <div className="flex flex-1" />
        ) : (
          <PillComponent
            size="large"
            className="relative z-1"
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
          >
            Previous
          </PillComponent>
        )}
        {publicTableIDs && publicTableIDs.length > pageSize && (
          <PillComponent
            size="large"
            className="relative z-1"
            onClick={() => setPage(page + 1)}
            disabled={publicTables.length < pageSize}
          >
            Next
          </PillComponent>
        )}
      </div>
    </LayoutComponent>
  );
});
LobbyPage.displayName = "LobbyPage";

export default LobbyPage;
