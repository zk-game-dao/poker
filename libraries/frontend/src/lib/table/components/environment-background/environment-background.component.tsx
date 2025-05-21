import { EnvironmentColor } from "@/src/models/table-color.model";
import { useIsBTC } from "@zk-game-dao/currency";
import classNames from "classnames";
import { PropsWithChildren, memo } from "react";

export const EnvironmentBackgroundComponent = memo<
  PropsWithChildren<{ className?: string; color?: EnvironmentColor }>
>(({ className, color = EnvironmentColor.Purple, children }) => {
  const isBTC = useIsBTC();
  return (
    <div
      className={classNames(
        className,
        "overflow-hidden",
        !isBTC ? "bg-linear-to-b" : 'bg-linear-to-br',
        {
          "from-[#BF5AF2] to-[#6E348C]": color === EnvironmentColor.Purple,
          "from-[#666666] to-[#000000]": color === EnvironmentColor.Black && !isBTC,
          "from-[#333] to-[#000]": color === EnvironmentColor.Black && isBTC,
          "from-[#CAA90F] to-[#665508]": color === EnvironmentColor.Yellow,
          "from-[#41A7CF] to-[#18475A]": color === EnvironmentColor.Blue,
          "from-[#F83A2E] to-[#92221B]": color === EnvironmentColor.Red,
          "from-[#1DB542] to-[#053B12]": color === EnvironmentColor.Green,
          relative:
            !className ||
            (className.indexOf("absolute") === -1 &&
              className.indexOf("fixed") === -1),
        })}
    >
      {!isBTC && (
        <div className="absolute inset-0 flex flex-col">
          <img src="/table-background.svg" className="object-cover" />
          <div className="flex flex-1 bg-black/[0.08]" />
          <img
            className="absolute -inset-x-[43px] top-8 max-w-none w-[calc(100%+43px*2)] object-cover"
            src="/table-background-overlay.svg"
          />
        </div>
      )}
      {children}
    </div>
  );
},
  (prevProps, nextProps) =>
    prevProps.className === nextProps.className &&
    prevProps.color === nextProps.color &&
    prevProps.children === nextProps.children
);
EnvironmentBackgroundComponent.displayName =
  "EnvironmentBackgroundComponent";
