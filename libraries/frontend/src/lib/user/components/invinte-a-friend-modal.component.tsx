import { memo } from 'react';

import { Principal } from '@dfinity/principal';
import { IsSamePrincipal } from '@zk-game-dao/currency';
import { CopiableTextComponent, List, ListItem, Modal, TitleTextComponent } from '@zk-game-dao/ui';

export const InviteAFriendModalComponent = memo<{
  isOpen: boolean;
  principal: Principal;
  onClose(): void;
}>(({ isOpen, onClose, principal }) => (
  <Modal open={isOpen} onClose={onClose}>
    <TitleTextComponent
      className="text-center"
      title="Invite A Friend"
      text="Invite your friends to join and earn rewards!"
    />

    <List label="Invite link">
      <ListItem className='bg-material-main-1 px-4 h-12 rounded-[12px]'>
        <CopiableTextComponent
          text={`${window.location.origin}/?referrer=${principal.toText()}`}
        />
      </ListItem>
    </List>
  </Modal>
),
  (prevProps, nextProps) =>
    prevProps.isOpen === nextProps.isOpen &&
    IsSamePrincipal(prevProps.principal, nextProps.principal) &&
    prevProps.onClose === nextProps.onClose
);
InviteAFriendModalComponent.displayName = "InviteAFriendModalComponent";
