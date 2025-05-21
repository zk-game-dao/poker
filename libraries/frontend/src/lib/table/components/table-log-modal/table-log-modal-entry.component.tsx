import { DateToLocalDateTimeString } from '@lib/utils/time';
import { ListItem } from '@zk-game-dao/ui';
import { ComponentProps, memo, useMemo } from 'react';

import { ActionLogComponent } from '../action-log/action-log.component';

export const TableModalLogEntryComponent = memo<{
  timestamp: bigint;
  logs: ComponentProps<typeof ActionLogComponent>[];
}>(({ timestamp, logs }) => {
  const timeString = useMemo(() => DateToLocalDateTimeString(timestamp), []);

  return (
    <ListItem rightLabel={timeString}>
      <div className="flex flex-col gap-1">
        {logs.map((v, i) => (
          <div className="flex flex-row" key={i}>
            <ActionLogComponent expanded key={i} {...v} />
          </div>
        ))}
      </div>
    </ListItem>
  );
});
TableModalLogEntryComponent.displayName = 'TableModalLogEntryComponent';

