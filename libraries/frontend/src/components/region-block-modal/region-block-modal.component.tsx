import { getName } from 'country-list';
import { memo, useMemo } from 'react';

import { useIsBTC } from '@zk-game-dao/currency';
import { Interactable, Modal, TitleTextComponent } from '@zk-game-dao/ui';

import { useWording } from '../../hooks/wording';
import { useRegionLockingQuery } from '../../lib/region-locking';

export const RegionBlockModalComponent = memo(() => {
  const { data } = useRegionLockingQuery();
  const wording = useWording();
  const isBTC = useIsBTC();

  const links = useMemo(() => {
    if (isBTC) {
      return [
        {
          icon: '/icons/socials/twitter.svg',
          href: 'https://x.com/purepokerapp',
        }
      ];
    }
    return [
      {
        icon: '/icons/socials/twitter.svg',
        href: 'https://x.com/zkpokerapp',
      },
      {
        icon: '/icons/socials/telegram.svg',
        href: 'https://tr.ee/J_Qy8lgvuS',
      },
      {
        icon: '/icons/socials/linktree.svg',
        href: 'https://linktr.ee/zkpoker',
      },
    ];
  }, [isBTC]);


  if (!data?.isBlocked) {
    return null;
  }

  const countryName = getName(data.country) ?? 'your region';

  return (
    <Modal open>
      <TitleTextComponent
        title="Access Restricted"
        className="mt-4"
        text={`Unfortunately, ${wording.product} is not available in ${countryName}.`}
      />

      <div className='flex flex-row justify-center items-center gap-8 mb-4'>
        {links.map((link) => (
          <Interactable
            key={link.href}
            href={link.href}
          >
            <img
              src={link.icon}
              alt="Link"
              className="size-6"
            />
          </Interactable>
        ))}
      </div>
    </Modal>
  );
});

RegionBlockModalComponent.displayName = 'RegionBlockModalComponent';
