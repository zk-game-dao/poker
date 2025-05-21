import { ButtonComponent, Modal, ModalFooterPortal, TitleTextComponent } from "@zk-game-dao/ui";
import { memo } from "react";

export const NoPlayersActiveModal = memo<{ isOpen: boolean; onClose(): void; onCreate(): void; }>(({ isOpen, onClose, onCreate }) => (
  <Modal
    open={isOpen}
    title="No active tables found"
    onClose={onClose}
  >

    <TitleTextComponent title="It looks like the tables are empty right now."
      text="We're still a growing platform, so traffic can be light at times.
            You can join an existing table or create your own with rake-sharing and invite your friends.
            For updates on community events and to stay in the loop, follow us on Twitter or join our Discord."
    />
    <p className='text-center type-title '>
      Thanks for your support as we build our player base!
    </p>
    <ModalFooterPortal>
      <ButtonComponent
        href='https://t.co/KjUWEKc3aa'
        variant="naked"
      >
        Check our linktree
      </ButtonComponent>
      <ButtonComponent
        onClick={onCreate}
      >
        Create table
      </ButtonComponent>
    </ModalFooterPortal>
  </Modal>
));
NoPlayersActiveModal.displayName = "NoPlayersActiveModal";
