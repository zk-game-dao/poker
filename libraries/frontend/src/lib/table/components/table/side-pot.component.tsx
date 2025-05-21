import { memo, useMemo, useState } from 'react';

import { Principal } from '@dfinity/principal';
import { AvatarComponent } from '@lib/ui/avatar/avatar.component';
import { DropdownComponent, Interactable } from '@zk-game-dao/ui';

import { ProfileModalComponent } from '../../../../components/profile/profile-modal.component';
import { useTable, useTableUserFromCanisterId } from '../../context/table.context';
import { ChipsStackComponent } from '../chips-stack/chips-stack.component';

const SidePotAvatar = memo(({ canister_id }: { canister_id: Principal }) => {
  const [isShowingProfile, setIsShowingProfile] = useState(false);
  const [user] = useTableUserFromCanisterId(canister_id);

  return (
    <>
      {user && (
        <ProfileModalComponent
          user={user}
          onClose={() => setIsShowingProfile(false)}
          isOpen={isShowingProfile}
        />
      )}
      <Interactable
        onClick={() => setIsShowingProfile(true)}
        className="flex -ml-2 -mt-2 hover:relative hover:z-1 hover:scale-110 transition-transform active:scale-95"
      >
        <AvatarComponent {...user} />
      </Interactable>
    </>
  );
});
SidePotAvatar.displayName = "SidePotAvatar";

export const SidePotComponent = memo(() => {
  const { table, currencyType: currency } = useTable();

  const sidePots = useMemo(
    () => table.side_pots.filter((v) => v.confirmed_pot > 0),
    [table.side_pots],
  );

  const [selectedSidePotIndex, setShownSidePot] = useState(0);
  const selectedSidePot = useMemo(
    () => sidePots[selectedSidePotIndex],
    [sidePots, selectedSidePotIndex],
  );

  if (!sidePots || sidePots.length === 0) return null;

  return (
    <div className="flex flex-col justify-center items-start ml-auto">
      {sidePots.length > 1 ? (
        <DropdownComponent
          options={sidePots.map((_, i) => ({
            value: i,
            label: (
              <p className="flex justify-row whitespace-nowrap">
                Side pot {i + 1}
              </p>
            ),
          }))}
          value={selectedSidePotIndex}
          onChange={(v) =>
            setShownSidePot(typeof v === "string" ? Number(v) : v ?? 0)
          }
          className="mr-4 opacity-70"
        />
      ) : (
        <p className="whitespace-nowrap type-medior text-neutral-200/70">
          Side pot {selectedSidePotIndex + 1}
        </p>
      )}
      {selectedSidePot && (
        <>
          <div className="flex justify-start items-start">
            <ChipsStackComponent
              currencyType={currency}
              hideChips
              name={`Side pot ${selectedSidePotIndex + 1}`}
              value={sidePots[selectedSidePotIndex]?.confirmed_pot ?? 0n}
            />
          </div>
          <div className="grid grid-cols-5 pl-2 pt-2">
            {selectedSidePot?.user_principals.map((principal) => (
              <SidePotAvatar key={principal.toText()} canister_id={principal} />
            ))}
          </div>
        </>
      )}
    </div>
  );
});
SidePotComponent.displayName = "SidePotComponent";
