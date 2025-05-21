import { memo, useMemo, useState } from 'react';

import {
  CurrencyComponent, CurrencyToString, TokenAmountToString, useCurrencyManagerMeta
} from '@zk-game-dao/currency';
import { List, ListItem, Modal, TabsComponent } from '@zk-game-dao/ui';

import { useWording } from '../../hooks/wording';
import { useJackpot } from './data';

export const HowItWorksModal = memo<{
  isOpen: boolean;
  onClose(): void;
}>(({ isOpen, onClose }) => {
  const { currency, jackpots, multipliers } = useJackpot();
  const currencyManager = useCurrencyManagerMeta({ Real: currency });

  const [shown, setShown] = useState<'weekdays' | 'weekends'>('weekdays');

  const [meta] = useMemo(() => {
    switch (shown) {
      case 'weekdays':
        return [
          {
            title: 'Weekday Leaderboard',
            date: 'Monday–Thursday',
          },
        ];
      case 'weekends':
        return [
          {
            title: 'Weekend Leaderboard',
            date: 'Friday–Sunday',
          },
        ];
    }
  }, [shown]);

  const rankings = useMemo(() => jackpots[shown], [jackpots, shown]);
  const jackpot = useMemo(() => rankings.reduce((a, b) => a + b, 0n), [rankings]);

  const wording = useWording();

  return (
    <Modal
      open={isOpen}
      onClose={onClose}
      title="How it works"
    >
      <div className="flex flex-col gap-3">
        <p className="type-body text-material-heavy-1 text-center">
          Earn XP in {wording.product} based on the number of players and then is multiplied based on pot size.
        </p>
        <p className="type-body text-material-heavy-1 text-center">
          Base XP per hand: 12 for heads-up, 17 for 3-5 players, and 25 for 6-max or full tables.
        </p>

        <div>
          <p className="type-callout text-material-medium-2 mr-auto mb-3">Multipliers</p>
          <List>
            {multipliers.map(({ fromBet, toBet, multiplier }) => (
              <ListItem rightLabel={`${multiplier}x`} key={multiplier}>
                {fromBet && !toBet && '> '}
                {!fromBet && toBet && '< '}
                {!!fromBet && `${TokenAmountToString(fromBet, { ...currencyManager, renderedDecimalPlaces: currencyManager.decimals })}`}
                {!!(fromBet && toBet) && ' – '}
                {!!toBet && `${TokenAmountToString(toBet, { ...currencyManager, renderedDecimalPlaces: currencyManager.decimals })}`}
                {` ${CurrencyToString(currency)}`}
              </ListItem>
            ))}
          </List>
        </div>
      </div>

      <div className="flex flex-col gap-2">
        <TabsComponent
          tabs={[
            { label: 'Weekdays', value: 'weekdays' },
            { label: 'Weekends', value: 'weekends' },
          ]}
          value={shown}
          onChange={(v) => setShown(v)}
          className="mb-4"
        />

        <List>
          <ListItem rightLabel={meta.date}>Date</ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyType={{ Real: currency }} currencyValue={jackpot} />}>
            Prize Pool
          </ListItem>
        </List>
      </div>
      <div>
        <p className="type-callout text-material-medium-2 mr-auto mb-3">Distribution split</p>
        <List>
          {jackpots[shown].map((prize, _place) => {
            const place = _place + 1;
            const rankName = place === 1 ? '1st' : place === 2 ? '2nd' : place === 3 ? '3rd' : `${place}th`;
            return (
              <ListItem
                key={place}
                rightLabel={<CurrencyComponent currencyType={{ Real: currency }} currencyValue={prize} />}
              >
                {rankName} place
              </ListItem>
            );
          })}
        </List>
      </div>
    </Modal >
  );
});
HowItWorksModal.displayName = 'HowItWorksModal';
