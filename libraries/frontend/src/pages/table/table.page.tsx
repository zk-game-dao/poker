import { memo, useMemo } from 'react';
import { useParams } from 'react-router-dom';

import { Principal } from '@dfinity/principal';
import { TableScreenComponent } from '@lib/table/components/table-screen/table-screen.component';
import { LayoutComponent, OverrideLayoutConfigComponent, PillComponent } from '@zk-game-dao/ui';

import { useFeedbackContext } from '../../context/feedback/feedback.context';

export const TablePageWithPrincipal = memo<{ table_principal: Principal }>(
  ({ table_principal }) => {
    const { openFeedback } = useFeedbackContext();

    return (
      <LayoutComponent
        className="flex flex-col safari-mobile-fs overflow-hidden"
        isFullScreen
        container="large"
      >
        <OverrideLayoutConfigComponent
          isOverlay
          navbarTabs={[]}
          navbarRightSide={
            <PillComponent onClick={openFeedback}>Feedback</PillComponent>
          }
        />
        <TableScreenComponent table_principal={table_principal} />
      </LayoutComponent>
    );
  },
  (prevProps, nextProps) =>
    prevProps.table_principal.toText() === nextProps.table_principal.toText()
);
TablePageWithPrincipal.displayName = 'TablePageWithPrincipal';

export const TablePage = memo(() => {
  const { tableId } = useParams<{ tableId: string }>();

  const table_principal = useMemo(
    () => tableId && Principal.fromText(tableId),
    [tableId],
  );

  if (!table_principal) return <p>No table id found in url</p>;

  return <TablePageWithPrincipal table_principal={table_principal} />;
});
TablePage.displayName = 'TablePage';

export default TablePage;
