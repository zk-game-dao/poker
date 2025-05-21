import { AnimatePresence, motion } from 'framer-motion';
import { memo, ReactNode, useEffect, useState } from 'react';

export const SpinnerComponent = memo<{
  symbol: ReactNode;
  result: number;
  onFinish(): void;
}>(({ symbol, result, onFinish }) => {

  const strdigit = Math.floor(result).toString();
  const numDigits = strdigit.split('').map(Number);

  const amountOfDigits = strdigit.length;
  const numberOfCycles = 6;
  const digitsPerCycle = 10;

  const heightOfAllDigits = (-16 * digitsPerCycle * 4);


  const [countdownNumber, setCountdownNumber] = useState(4);

  useEffect(() => {
    if (countdownNumber === 0) return;
    const n = setTimeout(() => setCountdownNumber(v => Math.max(0, v - 1)), 1000);
    return () => clearTimeout(n);
  }, [countdownNumber]);

  return (
    <motion.div
      variants={{
        hidden: { opacity: 0, y: -16 },
        visible: { opacity: 1, y: 0 },
      }}
      initial="hidden"
      animate="visible"
      exit="hidden"
      transition={{
        type: "spring",
        stiffness: 50,
        damping: 10
      }}
      className="flex justify-center items-center gap-1 bg-black/50 backdrop-blur-[4px] p-4 fixed inset-0 z-[50]"
    >
      <AnimatePresence>
        {countdownNumber && countdownNumber < 4 && (
          <motion.p
            variants={{
              hidden: { opacity: 0, scale: 0.5 },
              visible: { opacity: 1, scale: 2 },
            }}
            initial="hidden"
            animate="visible"
            exit="hidden"
            className='type-display absolute inset-0 flex justify-center items-center z-[2]'
            key={countdownNumber}
          >
            {countdownNumber}
          </motion.p>
        )}
      </AnimatePresence>
      <div className='material rounded-[16px] p-2 flex flex-col gap-3 z-[1] relative'>
        <div className='flex flex-row flex-wrap type-top gap-1 relative z-[1]'>
          <div className="size-16 flex justify-center items-center flex-grow-0 flex-shrink-0">{symbol}</div>
          {Array.from({ length: amountOfDigits }, (_, digit) => (
            <div
              key={digit}
              className="size-16 flex flex-col overflow-hidden rounded-[12px] bg-material-main-1"
              title={`${digit}: ${(numberOfCycles + (numDigits[digit] / 10))}`}
            >
              <motion.div
                className="w-16"
                variants={{
                  waiting: { y: 0 },
                  moving: { y: [0, heightOfAllDigits * (numberOfCycles + (numDigits[digit] / 10))], },
                }}
                animate={countdownNumber === 0 ? 'moving' : 'waiting'}
                transition={{
                  duration: numberOfCycles + Math.pow(digit * 0.1, 0.7),
                  ease: 'linear',
                  delay: Math.pow(digit * 0.3, 0.7),
                }}
                onAnimationComplete={() => {
                  if (digit !== amountOfDigits - 1 || countdownNumber > 0) return;
                  setTimeout(() => onFinish(), 2000);
                }}
              >
                {Array.from({ length: (numberOfCycles + 1) * digitsPerCycle + 1 }, (_, value) => (
                  <div
                    key={value}
                    className="size-16 flex justify-center items-center flex-grow-0 flex-shrink-0"
                  >
                    {value % 10}
                  </div>
                ))}
              </motion.div>
            </div>
          ))}
        </div>
      </div>
    </motion.div>
  );
});
SpinnerComponent.displayName = "SpinnerComponent";