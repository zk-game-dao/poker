import { createContext, memo, ReactNode, useCallback, useContext, useEffect, useMemo } from 'react';

import { useUser } from '@lib/user';
import { UnwrapOptional } from '@zk-game-dao/ui';

import { ConvertNat16ToPerc } from '../lib/utils/nat16';
import { DefaultSoundVolume } from '../lib/utils/sound';

const SOUNDS = [
  "deal-card",
  "chips-increase",
  "turn-notification",
  "win",
] as const;

type Sound = (typeof SOUNDS)[number];

type SoundContextType = {
  play: (sound: Sound) => void;
};

const SoundContext = createContext<SoundContextType>({
  play: () => { },
});

export const ProvideSound = memo<{ children: ReactNode }>(({ children }) => {
  const { user } = useUser();
  const volume = useMemo(() => {
    const volume = UnwrapOptional(user?.volume_level);
    if (volume === undefined)
      return DefaultSoundVolume;
    return ConvertNat16ToPerc(volume);
  }, [user?.volume_level[0]]);

  const sounds = useMemo(
    () =>
      SOUNDS.reduce(
        (acc, sound) => {
          const audio = new Audio(`/sounds/${sound}.wav`);
          return {
            ...acc,
            [sound]: audio,
          };
        },
        {} as Record<Sound, HTMLAudioElement>,
      ),
    [],
  );

  useEffect(() => {
    Object.entries(sounds).forEach(([, audio]) => {
      audio.volume = volume;
    });
  }, [sounds, volume]);

  const play = useCallback((sound: Sound) => sounds[sound].play(), [sounds]);

  return (
    <SoundContext.Provider value={{ play }}>{children}</SoundContext.Provider>
  );
});
ProvideSound.displayName = "SoundProvider";

export const useSound = () => useContext(SoundContext);
