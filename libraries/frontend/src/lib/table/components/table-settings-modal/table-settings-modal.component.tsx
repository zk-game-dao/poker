import { useCopyToClipboard } from '@/src/hooks/clipboard';
import { secondsToLabel } from '@lib/utils/time';
import { TokenAmountToString } from '@lib/utils/token-amount-conversion';
import { CurrencyComponent, useCurrencyManagerMeta } from '@zk-game-dao/currency';
import { List, ListItem, Modal } from '@zk-game-dao/ui';
import { memo, useState } from 'react';

import { useFeedbackContext } from '../../../../context/feedback/feedback.context';
import { useTable } from '../../context/table.context';
import { TableLogModalComponent } from '../table-log-modal/table-log-modal.component';

export const TableSettingsModalComponent = memo<{
  show: boolean;
  onClose(): void;
}>(({ show, onClose }) => {
  const { table, url, currencyType: currency } = useTable();

  const [showLog, setShowLog] = useState(false);

  const copyUrlToClipboard = useCopyToClipboard(url);
  const { openFeedback } = useFeedbackContext();
  const meta = useCurrencyManagerMeta(currency);

  if (!table) return null;

  return (
    <>
      <Modal open={show} title={table.config.name} onClose={onClose}>
        <div>
          <ListItem
            className="w-full"
            rightLabel={
              <button
                className="text-green-500 cursor-pointer"
                onClick={copyUrlToClipboard}
              >
                copy
              </button>
            }
          >
            {/* Making the max width limited like that is stupid but i have to */}
            <p
              className="text-ellipsis truncate w-full max-w-[325px]"
              onClick={copyUrlToClipboard}
            >
              {url}
            </p>
          </ListItem>
          <p className="pl-4 type-subheadline opacity-70">
            Share this link with anyone for them to join your table.
          </p>
        </div>

        <List variant={{ type: "default", readonly: true }}>
          <ListItem rightLabel={TokenAmountToString(table.small_blind, meta)}>
            Small blind
          </ListItem>
          <ListItem rightLabel={TokenAmountToString(table.big_blind, meta)}>
            Big blind
          </ListItem>
          <ListItem rightLabel={table.config.seats}>Seats</ListItem>
          <ListItem rightLabel={Object.keys(table.status)[0]}>Status</ListItem>
          <ListItem rightLabel={secondsToLabel(table.config.timer_duration)}>
            Action time limit
          </ListItem>
          <ListItem rightLabel={table.config.max_inactive_turns}>
            Max inactive turns
          </ListItem>
          <ListItem rightLabel={table.config.max_seated_out_turns}>
            Auto kick after inactive turns
          </ListItem>
          {(() => {
            if ("NoLimit" in table.config.game_type)
              return <ListItem rightLabel="No limit">Game type</ListItem>;
            if ("SpreadLimit" in table.config.game_type)
              return (
                <>
                  <ListItem rightLabel="Spread Limit">Game type</ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={currency}
                        variant="inline"
                        currencyValue={table.config.game_type.SpreadLimit[0]}
                      />
                    }
                  >
                    Min Bet
                  </ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={currency}
                        variant="inline"
                        currencyValue={table.config.game_type.SpreadLimit[1]}
                      />
                    }
                  >
                    Max Bet
                  </ListItem>
                </>
              );
            if ("FixedLimit" in table.config.game_type)
              return (
                <>
                  <ListItem rightLabel="Fixed Limit">Game type</ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={currency}
                        variant="inline"
                        currencyValue={table.config.game_type.FixedLimit[0]}
                      />
                    }
                  >
                    Small Bet
                  </ListItem>
                  <ListItem
                    rightLabel={
                      <CurrencyComponent
                        currencyType={currency}
                        variant="inline"
                        currencyValue={table.config.game_type.FixedLimit[1]}
                      />
                    }
                  >
                    Big Bet
                  </ListItem>
                </>
              );
            return <ListItem rightLabel="Unknown">Game type</ListItem>;
          })()}
        </List>

        <List>
          <ListItem onClick={() => setShowLog(true)}>Game log</ListItem>
          <ListItem onClick={openFeedback}>Submit feedback</ListItem>
          <ListItem href="/" className="text-red-500">
            Leave table
          </ListItem>
        </List>
      </Modal>
      <TableLogModalComponent
        isOpen={showLog}
        onClose={() => setShowLog(false)}
      />
    </>
  );
});
TableSettingsModalComponent.displayName = "TableSettingsModalComponent";
