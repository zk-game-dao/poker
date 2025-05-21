import 'react-json-view-lite/dist/index.css';

import { isEqual } from 'lodash';
import { memo, useState } from 'react';
import { allExpanded, darkStyles, JsonView } from 'react-json-view-lite';

import { PublicTable, User } from '@declarations/table_canister/table_canister.did';
import { ButtonComponent, IsDev, Modal, ModalFooterPortal, PillComponent } from '@zk-game-dao/ui';

export const TableScreenDebugModalButton = memo<{ table: PublicTable; user?: User }>(({ table, user }) => {
  const [showDebug, setShowDebug] = useState(false);

  if (!IsDev) return null;

  return (
    <>
      <Modal title="Debug" open={showDebug} onClose={() => setShowDebug(false)}>
        {user && (
          <>
            <p className="type-title mb-2">User</p>
            <div className="rounded-sm overflow-hidden">
              <JsonView
                data={user}
                shouldExpandNode={allExpanded}
                style={darkStyles}
              />
            </div>
          </>
        )}
        <p className="type-title my-2">Table</p>
        <div className="rounded-sm overflow-hidden">
          <JsonView
            data={table}
            shouldExpandNode={allExpanded}
            style={darkStyles}
          />
        </div>
        <ModalFooterPortal>
          <ButtonComponent
            variant="naked"
            onClick={() => setShowDebug(false)}
          >
            Close
          </ButtonComponent>
          <ButtonComponent
            onClick={() =>
              navigator.clipboard.writeText(JSON.stringify(table))
            }
          >
            Copy to clipboard
          </ButtonComponent>
        </ModalFooterPortal>
      </Modal>
      {IsDev && (
        <PillComponent
          className="absolute left-24 lg:left-4 top-14 lg:bottom-4 lg:z-100"
          onClick={() => setShowDebug(true)}
        >
          Debug
        </PillComponent>
      )}
    </>
  );
}, isEqual);
TableScreenDebugModalButton.displayName = 'TableScreenDebugModalButton';

export default TableScreenDebugModalButton;
