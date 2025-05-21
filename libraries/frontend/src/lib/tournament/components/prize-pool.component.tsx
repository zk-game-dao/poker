import { useMutation } from '@tanstack/react-query';
import { CurrencyComponent, CurrencyInputComponent, useAllowance, useAuth } from '@zk-game-dao/currency';
import {
  ButtonComponent,
  ErrorComponent,
  FauxLoadingBarAnimationComponent,
  List,
  ListItem,
  Modal,
  ModalFooterPortal,
  TitleTextComponent,
  useToast,
} from '@zk-game-dao/ui';
import classNames from 'classnames';
import { addMinutes } from 'date-fns';
import { memo, useMemo, useState } from 'react';

import { Queries } from '../../data';
import { callActorMutation } from '../../utils/call-actor-mutation';
import { useTournament } from '../context/tournament.context';

const RakePercentage = 15n;

export const PricePool = memo<{ hideOnDesktop?: boolean; hideOnMobile?: boolean; }>(({ hideOnDesktop, hideOnMobile }) => {
  const { authData } = useAuth();
  const { currencyType, actor, prizepool, receiver, data } = useTournament(true);

  const [showDonate, setShowDonate] = useState(false);
  const [donationAmount, setDonationAmount] = useState(0n);
  const allowance = useAllowance({ currencyType, receiver, name: 'Tournament' });

  const absoluteRake = useMemo(() => donationAmount / 100n * RakePercentage, [donationAmount]);
  const donationAfterRake = useMemo(() => donationAmount - absoluteRake, [donationAmount]);
  const prizePoolAfterDonation = useMemo(() => donationAfterRake + prizepool, [donationAfterRake, prizepool]);

  const { addToast } = useToast();

  const donateMutation = useMutation({
    mutationFn: async () => {
      if (!authData) throw new Error('You are not logged in');
      await allowance.require({ amount: donationAmount, reason: 'Donate' }, addMinutes(new Date(), 2));
      return await callActorMutation(actor, 'deposit_prize_pool',
        donationAmount,
        authData.principal,
      );
    },
    onSuccess: () => {
      setShowDonate(false)
      addToast({
        children: 'Donation successful',
      });
      Queries.tournament.invalidate(data.id);
    },
  });

  return (
    <div className={classNames("flex-col md:w-[300px] mt-4", hideOnMobile || hideOnDesktop ? { 'hidden md:flex': hideOnMobile, 'flex md:hidden': hideOnDesktop } : 'flex')}>
      <Modal title="Donate" open={showDonate} onClose={() => setShowDonate(false)}>
        <TitleTextComponent
          title="Donate"
          text="Contribute to the prize pool and support the tournament! Your donation will help increase the excitement and rewards for all participants. Thank you for your generosity!"
        />
        {donateMutation.isPending && (
          <FauxLoadingBarAnimationComponent>
            Donating
          </FauxLoadingBarAnimationComponent>
        )}

        <ErrorComponent
          title="donateMutation"
          error={donateMutation.error}
        />

        <List>
          <CurrencyInputComponent
            currencyType={currencyType}
            value={donationAmount}
            onChange={setDonationAmount}
            label="Amount"
          />
          <ListItem rightLabel={`${RakePercentage.toString()}%`}>
            Rake
          </ListItem>
          <ListItem rightLabel={<CurrencyComponent currencyValue={prizePoolAfterDonation} currencyType={currencyType} />}>
            Prizepool after donation
          </ListItem>
        </List>

        <ModalFooterPortal>
          <ButtonComponent variant="naked" onClick={() => setShowDonate(false)}>Cancel</ButtonComponent>
          <ButtonComponent onClick={donateMutation.mutate} isLoading={donateMutation.isPending}>Donate</ButtonComponent>
        </ModalFooterPortal>
      </Modal>
      <List>
        <ListItem rightLabel={<CurrencyComponent currencyValue={prizepool} currencyType={currencyType} />}>
          Prize Pool
        </ListItem>
        {!('Cancelled' in data.state || 'Completed' in data.state) && (
          <ListItem onClick={() => setShowDonate(true)}>
            Donate
          </ListItem>
        )}
      </List>
    </div>
  )
});
PricePool.displayName = "PricePool";
