import { memo, useState } from 'react';

import { TableConfig } from '@declarations/table_index/table_index.did';
import { AccountIdentifier } from '@dfinity/ledger-icp';
import { Principal } from '@dfinity/principal';
import { CurrencyComponent, useAuth, useIsBTC } from '@zk-game-dao/currency';
import {
  Interactable, List, ListItem, StepComponentProps, SteppedModalStep, SwitchInputComponent,
  TextInputComponent, UserError
} from '@zk-game-dao/ui';

import { RakeInfoModalComponent } from '../../rake-info/rake-info-modal.component';
import { RevenueShareCostPP, RevenueShareCostZKP } from '../constants';

type Value = Pick<TableConfig, "name" | "is_private" | 'is_shared_rake' | 'currency_type' | 'game_type' | 'require_proof_of_humanity' | 'is_paused'>;
type State = {
  revenueSharePrincipal?: string;
  revenueShareAccountID?: string;
  agrees_to_rake: boolean;
  is_shared_rake: boolean;
};

const NameStepComponent = memo<StepComponentProps<Value, State>>(({ data, patch, localState, patchLocalState }) => {
  const [showRakeInfo, setShowRakeInfo] = useState(false);
  const { authData } = useAuth();
  const isBTC = useIsBTC();
  if (!authData?.principal || !authData?.accountIdentifier) return null;

  return (
    <>
      <TextInputComponent
        label="Table Name"
        value={data.name}
        onChange={(name) => patch({ name })}
      />
      <div className="flex flex-col gap-2">
        <SwitchInputComponent
          label="Private Table"
          checked={data.is_private && data.is_private[0]}
          onChange={(privateTable) => patch({ is_private: [privateTable] })}
        />
        <p className="type-subheadline text-material-medium-1 px-4">
          Private games won’t be listed in the Lobby. Only invited players can
          join.
        </p>
      </div>
      <div className="flex flex-col gap-2">
        <SwitchInputComponent
          label="Require proof of humanity"
          checked={data.require_proof_of_humanity?.[0]}
          onChange={(require_proof_of_humanity) => patch({ require_proof_of_humanity: [require_proof_of_humanity] })}
        />
        <p className="type-subheadline text-material-medium-1 px-4">
          Only allow players who have verified their humanity.
        </p>
      </div>

      <div className="flex flex-col gap-2">
        <List>
          <SwitchInputComponent
            label={<>Enable revenue sharing</>}
            checked={localState.is_shared_rake}
            onChange={(is_shared_rake) => patchLocalState({
              is_shared_rake,
              revenueSharePrincipal: is_shared_rake ? authData.principal.toText() : undefined,
              revenueShareAccountID: is_shared_rake ? authData?.accountIdentifier.toHex() : undefined,
            })}
          />
          {localState.is_shared_rake && (
            <>
              <ListItem>
                Your revenue share will be deposited into your {isBTC ? 'Pure Poker' : 'ZKP'} wallet.
              </ListItem>

              <SwitchInputComponent
                label={<>{'I agree to the '}<Interactable className="inline-flex underline hover:no-underline ml-1" onClick={() => setShowRakeInfo(true)}>rake calculation</Interactable></>}
                checked={localState.agrees_to_rake}
                onChange={(agrees_to_rake) => patchLocalState({ agrees_to_rake })}
              />

              <RakeInfoModalComponent
                isOpen={showRakeInfo}
                onClose={() => setShowRakeInfo(false)}
                initial_currency_type={data.currency_type}
                initial_game_type={data.game_type}
              />
            </>
          )}
        </List>
        <div className="type-subheadline text-material-medium-1 px-4">
          We will split the rake 50/50 with your zkp wallet if revenue sharing is enabled.
          Enabling revenue sharing costs
          <div className='inline text-white'>
            <CurrencyComponent
              currencyType={{ Real: isBTC ? { BTC: null } : { ICP: null } }}
              className='text-white opacity-40 inline -mt-1'
              size="small"
              currencyValue={isBTC ? RevenueShareCostPP : RevenueShareCostZKP}
            />
          </div>.
        </div>

      </div >
    </>
  );
});
NameStepComponent.displayName = "NameStepComponent";

export const Config: SteppedModalStep<Value, State> = {
  title: "Name your table",
  Component: NameStepComponent,
  isValid: ({ name }, { is_shared_rake, agrees_to_rake, revenueShareAccountID, revenueSharePrincipal }) => {
    if (!name) return ["Name is required"];
    if (is_shared_rake) {
      if (!agrees_to_rake) return ["You must agree to the rake calculation"];
      if (!revenueSharePrincipal) return ["Revenue share principal is required"];
      try {
        Principal.fromText(revenueSharePrincipal);
      } catch {
        return ["Invalid revenue share principal"];
      }
      if (!revenueShareAccountID) return ["Revenue share account identifier is required"];
      try {
        AccountIdentifier.fromHex(revenueShareAccountID);
      } catch {
        return ["Invalid revenue share account identifer"];
      }
    }
    return true;
  },
  defaultValues: { is_private: [false], is_shared_rake: [], require_proof_of_humanity: [false], is_paused: [false] },
  deriveLocalState: (v) => ({
    rake_enabled: !!v.is_shared_rake,
    agrees_to_rake: false,
    revenueSharePrincipal: v.is_shared_rake?.[0]?.[0].toText(),
    revenueShareAccountID: v.is_shared_rake?.[0]?.[1]
  }),
  applyLocalState: (v, l) => {
    console.log('lslslslss', { l });

    if (!l.is_shared_rake)
      return { ...v, is_shared_rake: [] };


    if (!l.agrees_to_rake) throw new UserError("Must agree to rake calculation");
    if (!l.revenueSharePrincipal) throw new UserError("Revenue share principal is required");
    if (!l.revenueShareAccountID) throw new UserError("Revenue share account identifier is required");

    const principal = Principal.fromText(l.revenueSharePrincipal);
    const accountID = AccountIdentifier.fromHex(l.revenueShareAccountID);

    return {
      ...v,
      is_shared_rake: [[principal, accountID.toHex()]]
    }
  }
};