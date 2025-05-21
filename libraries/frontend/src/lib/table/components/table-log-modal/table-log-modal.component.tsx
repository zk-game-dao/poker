import { log_store } from '@declarations/log_store';
import { ActionLog } from '@declarations/table_canister/table_canister.did';
import { Queries } from '@lib/data';
import { callActorMutation } from '@lib/utils/call-actor-mutation';
import { WrapOptional } from '@lib/utils/optional';
import { DateToBigIntTimestamp, DateToLocalDateTimeString } from '@lib/utils/time';
import { useQuery } from '@tanstack/react-query';
import { Interactable, List, ListItem, Modal, TabsComponent } from '@zk-game-dao/ui';
import { addHours } from 'date-fns';
import { memo, useMemo, useState } from 'react';

import { useTable } from '../../context/table.context';
import { TableModalLogEntryComponent } from './table-log-modal-entry.component';

const PageSize = 50;

const consolidateLogs = (logs: ActionLog[]): [string, ActionLog[]][][] => {
  const logsGroupedByHad: ActionLog[][] = logs
    .reduce(
      (acc, log) => {
        if (
          ("Stage" in log.action_type &&
            ("Opening" in log.action_type.Stage.stage ||
              "Fresh" in log.action_type.Stage.stage)) ||
          acc.length === 0
        ) {
          const group = {
            timestamp: Number(log.timestamp),
            logs: [] as ActionLog[],
          };
          acc.push(group);
        }
        acc[acc.length - 1].logs.push(log);
        return acc;
      },
      [] as { timestamp: number; logs: ActionLog[] }[],
    )
    .sort((a, b) => a.timestamp - b.timestamp)
    .map(({ logs }) => logs);

  return logsGroupedByHad.map((logs): [string, ActionLog[]][] => {
    const groupedLogs = logs.reduce(
      (acc, log) => {
        const timestamp = Number(log.timestamp);
        if (!(timestamp in acc)) acc[timestamp] = [];
        acc[timestamp] = [...acc[timestamp], log].sort(
          (a, b) => Number(b.timestamp) - Number(a.timestamp),
        );
        return acc;
      },
      {} as Record<number, ActionLog[]>,
    );

    return Object.entries(groupedLogs).sort(
      ([a], [b]) => Number(b) - Number(a),
    );
  });
};

export const TableLogModalComponent = memo<{
  isOpen?: boolean;
  onClose(): void;
}>(({ isOpen, onClose }) => {
  const [seeOngoing, setSeeOngoing] = useState(true);

  const { table } = useTable();

  const [params, setParams] = useState({
    start_timestamp: DateToBigIntTimestamp(addHours(new Date(), -1)),
    end_timestamp: DateToBigIntTimestamp(new Date()),
    offset: 0,
  });

  const page = useMemo(
    () => Math.floor(params.offset / PageSize),
    [params.offset],
  );
  const setPage = (page: number) =>
    setParams((_params) => ({
      ..._params,
      offset: Math.max(0, page * PageSize),
    }));

  const logsFromStore = useQuery({
    queryKey: Queries.tableLogStore.key(
      table.id,
      params.start_timestamp,
      params.end_timestamp,
      params.offset,
      PageSize,
    ),
    queryFn: async () => {
      if (!table.id) return [];
      return await callActorMutation(
        log_store,
        "get_action_logs",
        table.id,
        params.start_timestamp,
        params.end_timestamp,
        WrapOptional(params.offset),
        WrapOptional(PageSize),
      );
    },
    select: consolidateLogs,
    enabled: !!table.id,
    refetchInterval: 5000,
  });

  const consolidatedOngoingLogs = useMemo(
    (): [bigint, ActionLog][] =>
      table.action_logs
        .sort((a, b) => Number(b.timestamp) - Number(a.timestamp))
        .map((v) => [v.timestamp, v] as const),
    [logsFromStore.data],
  );

  const setStartTimestamp = (date: Date) =>
    setParams((_params) => ({
      ..._params,
      start_timestamp: DateToBigIntTimestamp(date),
      offset: 0,
    }));
  const setEndTimestamp = (date: Date) =>
    setParams((_params) => ({
      ..._params,
      end_timestamp: DateToBigIntTimestamp(date),
      offset: 0,
    }));

  return (
    <Modal open={isOpen} title="Log" onClose={onClose}>
      <TabsComponent
        tabs={[
          { label: "Ongoing", value: "ongoing" },
          { label: "History", value: "history" },
        ]}
        onChange={(v) => setSeeOngoing(v === "ongoing")}
        value={seeOngoing ? "ongoing" : "history"}
      />

      {seeOngoing && (
        <div className="flex flex-col gap-3">
          <p className="type-callout text-material-medium-2 mr-auto mb-2 mt-3">
            Ongoing game logs
          </p>
          <List>
            {consolidatedOngoingLogs.map(([timestamp, log]) => (
              <TableModalLogEntryComponent
                key={timestamp}
                timestamp={timestamp}
                logs={[log]}
              />
            ))}
          </List>
        </div>
      )}

      {!seeOngoing && (
        <>
          <div className="flex flex-col gap-3">
            <p className="type-callout text-material-medium-2 mr-auto mb-2 mt-3">
              Show logs between
            </p>
            <List>
              <ListItem
                rightLabel={
                  <input
                    type="datetime-local"
                    value={DateToLocalDateTimeString(
                      params.start_timestamp,
                      "yyyy-MM-dd'T'HH:mm:ss",
                    )}
                    className="bg-transparent"
                    onChange={(v) =>
                      setStartTimestamp(new Date(v.target.value))
                    }
                  />
                }
              >
                Start Date
              </ListItem>
              <ListItem
                rightLabel={
                  <input
                    type="datetime-local"
                    value={DateToLocalDateTimeString(
                      params.end_timestamp,
                      "yyyy-MM-dd'T'HH:mm:ss",
                    )}
                    className="bg-transparent"
                    onChange={(v) => setEndTimestamp(new Date(v.target.value))}
                  />
                }
              >
                End Date
              </ListItem>
            </List>
          </div>

          {!logsFromStore.data || logsFromStore.data.length === 0 ? (
            <p className="mx-auto">No logs in selected range</p>
          ) : (
            <>
              {logsFromStore.data.map((groupedLogs, i) => (
                <List variant={{ type: "default", readonly: true }} key={i}>
                  {groupedLogs.map(([time_stamp, logs]) => (
                    <TableModalLogEntryComponent
                      key={time_stamp}
                      timestamp={BigInt(time_stamp)}
                      logs={logs}
                    />
                  ))}
                </List>
              ))}
            </>
          )}

          <div className="flex justify-center gap-3 material rounded-[12px] type-button-2 px-4">
            <Interactable
              className="h-[44px] w-1/3 text-left"
              onClick={() => setPage(page - 1)}
            >
              Previous
            </Interactable>
            <input
              className="h-[44px] w-1/3 text-center bg-transparent"
              type="number"
              value={page + 1}
              onChange={(v) => setPage(Number(v.target.value) - 1)}
            />
            <Interactable
              className="h-[44px] w-1/3 text-right"
              onClick={() => setPage(page + 1)}
            >
              Next
            </Interactable>
          </div>
        </>
      )}
    </Modal>
  );
});
TableLogModalComponent.displayName = "TableLogModalComponent";
