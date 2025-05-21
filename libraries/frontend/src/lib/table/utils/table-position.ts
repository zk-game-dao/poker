export type TableSeatPosition = {
  vertical: "top" | "bottom";
  horizontal: "left" | "right";
};

export type TablePosition = {
  left?: string;
  right?: string;
  top?: string;
  bottom?: string;
  transform: string;
  positionContext: TableSeatPosition;
};

export type PositionCalculationProps = {
  seat: number;
  maxSeats: number;
  width: number;
  height: number;
};

export const calculatePortraitPosition = ({
  seat,
  maxSeats,
  width,
  height,
}: PositionCalculationProps): TablePosition => {
  const angle = Math.PI / (maxSeats - 1);
  const x = (width / 2) * Math.cos(seat * angle);
  const y = (height / 2) * Math.sin(seat * angle);
  const normLeft = 50 + (x / (width / 2)) * 50;
  const normTop = 50 - (y / (height / 2)) * 50;

  return {
    left: `${normLeft}%`,
    right: normLeft < 50 ? `${100 - normLeft}%` : undefined,
    top: `${normTop}%`,
    bottom: undefined,
    transform: "translate(-50%, -50%)",
    positionContext: {
      vertical: "top",
      horizontal: "left",
    },
  };
};

const LandscapeWidth = 1040;
const LandscapeHeight = 600;

export const LandscapeTableBackgroundRatio = LandscapeWidth / LandscapeHeight;
export const PortraitTableBackgroundRatio = 384.85 / 570.24;

/** Cheap implementation, create a circle and push the edges together to form a quasi pill shape */
export const calculateLandscapePosition = ({
  seat,
  maxSeats,
  width,
  height,
}: PositionCalculationProps): TablePosition => {
  const percentage = seat / maxSeats;
  const bottomSpace = 0.2;
  const normalPercentage = (bottomSpace + percentage * (1 - bottomSpace) + bottomSpace) % 1;
  const angle = (2 * Math.PI) * normalPercentage;

  // Calculate normalized positions
  const x = ((width / 2) * Math.cos(angle)) / (width / 2);
  const y = ((height / 2) * Math.sin(angle)) / (height / 2);

  // Apply "amplifying" effect using an inverted exponent
  const exponent = 1.2; // Change this value to control the amplification effect
  const squishedX = Math.sign(x) * (1 - Math.pow(1 - Math.abs(x), exponent));
  const squishedY = Math.sign(y) * (1 - Math.pow(1 - Math.abs(y), exponent));

  // Map to percentage space
  const percX = (squishedX + 1) / 2;
  const percY = (squishedY + 1) / 2;

  const normLeft = percX * 100;
  const normTop = percY * 100;

  const vertical = normTop < 50 ? "top" : "bottom";
  const horizontal = normLeft < 50 ? "left" : "right";

  return {
    left: normLeft >= 50 ? `${normLeft}%` : undefined,
    right: normLeft < 50 ? `${100 - normLeft}%` : undefined,
    top: normTop >= 50 ? `${normTop}%` : undefined,
    bottom: normTop < 50 ? `${100 - normTop}%` : undefined,
    transform: `
      translate(
        ${percX < 0.2 ? -100 : percX > 0.8 ? 0 : -50}%,
        ${percY < 0.2 ? -100 : percY > 0.8 ? 0 : -50}%
      )
    `,
    positionContext: {
      vertical,
      horizontal,
    },
  };
};

export const calculatePosition = (
  props: PositionCalculationProps
): TablePosition => {
  if (props.width < props.height) {
    return calculatePortraitPosition(props);
  } else {
    return calculateLandscapePosition(props);
  }
};
