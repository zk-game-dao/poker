import { memo, useEffect, useMemo, useState } from 'react';

import {
  ButtonComponent, List, ListItem, Modal, ModalFooterPortal, SwitchInputComponent, TabsComponent,
  TitleTextComponent, useToast
} from '@zk-game-dao/ui';

import { useNotifications } from '../context/notifications.context';

const BrowserInstructions = {
  Chrome: [
    'Click the "i" icon in the address bar.',
    "Switch 'Notifications' to 'Allow'.",
    "Reload the page.",
  ],
  Firefox: [
    "Click the padlock icon next to the URL in the address bar.",
    "Click the right arrow next to 'Connection secure' to expand the menu.",
    "Find 'Permissions' and set 'Send Notifications' to 'Allow'.",
  ],
  Safari: [
    "Click 'Safari' in the menu bar and select 'Settings for This Website'.",
    "In the pop-up, find 'Notifications'.",
    "Choose 'Allow' to enable notifications.",
  ],
};

type Browser = keyof typeof BrowserInstructions;

export const NotificationsModalComponent = memo<{
  isOpen: boolean;
  onClose(): void;
}>(({ isOpen, onClose }) => {
  const [browser, setBrowser] = useState<Browser>("Chrome");
  const {
    isConnecting,
    connect,
    isConnected,
    isLoadingEnabled,
    enabled,
    setEnabled,
  } = useNotifications();

  const [enabledState, setEnabledState] = useState(enabled);
  const { addToast } = useToast();

  useEffect(() => setEnabledState(enabled), [enabled]);

  const instructions = useMemo(
    () => BrowserInstructions[browser] ?? [],
    [browser],
  );

  return (
    <Modal open={isOpen} onClose={onClose}>
      <TitleTextComponent
        title="Notifications"
        text="Enable notifications to receive updates when it's your turn to play."
      />

      {isConnected ? (
        <SwitchInputComponent
          label="Notifications"
          disabled={!isConnected}
          onChange={setEnabledState}
          checked={enabledState}
        />
      ) : (
        <>
          <TabsComponent
            value={browser}
            tabs={Object.keys(BrowserInstructions).map((browser) => ({
              label: browser,
              value: browser,
            }))}
            onChange={(browser) => setBrowser(browser as Browser)}
          />

          <List>
            {instructions.map((instruction, i) => (
              <ListItem
                key={i}
                icon={
                  <span className="type-button-3 text-material-medium-3 pl-3 pr-2">
                    {i + 1}.
                  </span>
                }
              >
                <p className="type-button-2 text-material-medium-3">
                  {instruction}
                </p>
              </ListItem>
            ))}
          </List>
        </>
      )}

      <ModalFooterPortal>
        <ButtonComponent variant="naked" onClick={onClose}>
          Close
        </ButtonComponent>
        {isConnected ? (
          <ButtonComponent
            onClick={async () => {
              await setEnabled(enabledState);
              addToast({
                children: enabledState
                  ? "Notifications enabled"
                  : "Notifications disabled",
              });
              onClose();
            }}
            isLoading={isLoadingEnabled}
          >
            Save
          </ButtonComponent>
        ) : (
          <ButtonComponent isLoading={isConnecting} onClick={connect}>
            Connect
          </ButtonComponent>
        )}
      </ModalFooterPortal>
    </Modal>
  );
});
NotificationsModalComponent.displayName = 'NotificationsModalComponent';
