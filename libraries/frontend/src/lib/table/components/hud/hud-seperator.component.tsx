import classNames from "classnames";
import { memo } from "react";

export const HudSeperator = memo<{ mobileOnly?: boolean; desktopOnly?: boolean; }>(({
  mobileOnly = false,
  desktopOnly = false
}) => (
  <div className={classNames(
    "bg-material-main-1 w-0.5 rounded-full self-stretch flex mx-1 lg:mx-2",
    { 'lg:hidden': mobileOnly, 'hidden lg:flex': desktopOnly }
  )} />
));
HudSeperator.displayName = "HudSeperator";
