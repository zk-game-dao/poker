import classNames from 'classnames';
import { memo, useMemo, useRef } from 'react';

import { CurrencyComponent } from '@zk-game-dao/currency';
import { LoadingAnimationComponent } from '@zk-game-dao/ui';

import { PositionOnTable } from '../../context/table-seat.context';
import { ProvideTableUIContext, useTableRect } from '../../context/table-ui.context';
import { useTable } from '../../context/table.context';
import { PotComponent } from './pot/pot.component';
import { SidePotComponent } from './side-pot.component';
import { TableBackgroundComponent } from './table-background/table-background.component';
import { TableChipStackComponent } from './table-chip-stack.component';
import { TablePlayer } from './table-player/table-player.component';

export const TableComponent = memo<{ className?: string; }>(({ className }) => {
  const { table, users, currencyType: currency } = useTable();
  const remainingSeats = useMemo(
    () => table.config.seats - users.length,
    [table.seats, users.length],
  );

  const containerRef = useRef<HTMLDivElement>(null);
  const tableRef = useRef<HTMLDivElement>(null);
  const tableSize = useTableRect(containerRef);
  const chipsTableSize = useMemo(
    () => ({ width: tableSize.width * 0.75, height: tableSize.height * 0.75 }),
    [tableSize],
  );
  const orientation = useMemo(
    () => (tableSize.width > tableSize.height ? "landscape" : "portrait"),
    [tableSize],
  );
  const renderPotAnimations = useMemo(
    () => orientation === "landscape" && tableSize.width > 800,
    [tableSize.width, orientation],
  );

  const seats = useMemo(
    () => Array.from({ length: table.config.seats }, (_, i) => i),
    [table.config.seats],
  );

  return (
    <ProvideTableUIContext
      tableRef={tableRef}
      animatePots={renderPotAnimations}
      orientation={orientation}
    >
      <div
        className={classNames(
          "flex flex-col justify-center items-center z-40",
          className
        )}
        ref={containerRef}
      >
        {tableSize && (
          <div
            className="relative w-full flex"
            style={tableSize}
            ref={tableRef}
          >
            {seats.map((seatIndex, _, all) => (
              <PositionOnTable
                key={seatIndex}
                seat={seatIndex}
                seatAmount={all.length}
                {...tableSize}
              >
                <TablePlayer />
              </PositionOnTable>
            ))}

            <div
              className="absolute z-1 pointer-events-none"
              style={{
                ...chipsTableSize,
                left: (tableSize.width - chipsTableSize.width) / 2,
                top: (tableSize.height - chipsTableSize.height) / 2,
              }}
            >
              {seats.map((seatIndex, _, all) => (
                <PositionOnTable
                  key={seatIndex}
                  seat={seatIndex}
                  seatAmount={all.length}
                  {...chipsTableSize}
                >
                  <TableChipStackComponent />
                </PositionOnTable>
              ))}
            </div>

            <TableBackgroundComponent
              className="flex flex-1"
              community_cards={table.community_cards}
              tableSize={tableSize}
            >
              <div className={classNames("flex flex-row")}>
                {!renderPotAnimations ? (
                  <CurrencyComponent
                    currencyType={currency}
                    className="py-1"
                    size="big"
                    currencyValue={table.pot}
                  />
                ) : (
                  <PotComponent table={table} />
                )}
                {table.side_pots.filter((v) => v.confirmed_pot > 0).length >
                  0 && (
                    <>
                      <div className="w-2 rounded-full bg-material-main-2 h-full ml-4 mr-3" />
                      <SidePotComponent />
                    </>
                  )}
              </div>
              {remainingSeats > 0 && "Fresh" in table.deal_stage && (
                <LoadingAnimationComponent variant="shimmer">
                  {remainingSeats}{" "}
                  {remainingSeats > 1 ? "seats are " : "seat is "} still open
                </LoadingAnimationComponent>
              )}
            </TableBackgroundComponent>
          </div>
        )}
      </div>
    </ProvideTableUIContext>
  );
});
TableComponent.displayName = "TableComponent";
