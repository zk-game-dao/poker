import classNames from 'classnames';
import { memo, useRef } from 'react';

import { useTableSeat } from '../../context/table-seat.context';
import { useRegisterTableSeat } from '../../context/table-ui.context';
import { DealerButtonComponent } from './dealer-button.component';

export const TableChipStackComponent = memo(() => {
  const { seatIndex, isDealer, position: { horizontal } } = useTableSeat()
  const ref = useRef<HTMLDivElement>(null);

  useRegisterTableSeat(seatIndex, ref);

  return (
    <div
      className={classNames(
        "w-full h-full items-center justify-center pointer-events-auto hidden lg:flex",
        horizontal === "left" ? "flex-row" : "flex-row-reverse",
      )}
    >
      {isDealer && <DealerButtonComponent />}
      <div
        ref={ref}
        className={classNames(
          "w-full h-full absolute top-10",
          horizontal === "left" ? "left-14" : "right-12",
        )}
      />
    </div>
  );
});
TableChipStackComponent.displayName = "TableChipStackComponent";
