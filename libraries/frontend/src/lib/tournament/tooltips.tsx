import { TooltipComponent } from '@zk-game-dao/ui';
import { JSX, memo, MemoExoticComponent } from 'react';

const BuildTooltipMemo = <Name extends string>(name: Name, fn: () => JSX.Element):
  Record<Name, MemoExoticComponent<() => JSX.Element>> => {
  const comp = memo(fn);
  comp.displayName = `TooltipComponent(${name})`;

  return ({ [name]: comp }) as Record<Name, MemoExoticComponent<() => JSX.Element>>;
}

export const Tooltips = {
  ...BuildTooltipMemo('freezout', () =>
    <TooltipComponent className='ml-1 z-60' title="Freezout Tournaments">
      <p>A freezout is the most straightforward type of poker tournament.</p>
      <p>Players start with a fixed amount of chips, and once they lose all their chips, they're eliminated.</p>
      <p>There are no opportunities to buy more chips - when you're out, you're out.</p>
      <p>This creates a pure competitive environment where chip preservation and strategic play are crucial.</p>
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('addon', () =>
    <TooltipComponent className='ml-1 z-60' title="Addon Tournaments">
      <p>An addon is a one-time opportunity to purchase additional chips, typically offered at the end of the rebuy period. The configurable options for addons include:</p>
      <p>
        - A specific time window when they become available<br />
        - A set price and the correspond chip amount the player will recieve<br />
        - A one-time-only limitation per player
      </p>
      <p>Addons are different from rebuys because:</p>
      <p>
        1. They're available to all players, regardless of chip stack size<br />
        2. They can typically only be purchased once<br />
        3. They're typically offered at a specific time (usually when the rebuy period ends)
      </p>
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('addon_chips', () =>
    <TooltipComponent>
      The number of additional chips a player receives when purchasing an addon.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('addon_price', () =>
    <TooltipComponent>
      The cost of purchasing the addon, defined in the tournament’s selected currency.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('max_addons', () =>
    <TooltipComponent>
      The maximum number of times a player can purchase an addon in this tournament.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('addon_start_time', () =>
    <TooltipComponent>
      The date and time when addon purchases become available.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('addon_end_time', () =>
    <TooltipComponent>
      The deadline for purchasing an addon.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('rebuy', () =>
    <TooltipComponent className='ml-1 z-60' title="Rebuy Tournaments">
      <p>Rebuy tournaments allow players to purchase additional chips if they lose all their chips within a specific time frame. The configurable options for rebuys include:</p>
      <p>
        - A time limit for when rebuys are available<br />
        - A set price and the corresponding chip amount the player receives for each rebuy<br />
        - A maximum number of rebuys allowed per player
      </p>
      <p>Rebuy tournaments are popular in low-stakes events and provide players with a second chance to continue playing after losing their initial</p>
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('max_rebuys', () =>
    <TooltipComponent>
      The maximum number of times a player can rebuy in this tournament.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('rebuy_chips', () =>
    <TooltipComponent>
      The number of chips received for each rebuy.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('rebuy_price', () =>
    <TooltipComponent>
      The cost of each rebuy, defined in the tournament’s selected currency.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('rebuy_end_timestamp', () =>
    <TooltipComponent>
      The deadline for purchasing a rebuy.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('min_chips_for_rebuy', () =>
    <TooltipComponent>
      The minimum number of chips required to be eligible for a rebuy.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('rebuy_window_seconds', () =>
    <TooltipComponent>
      The time window during which a player can rebuy after being eliminated.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('reentry', () =>
    <TooltipComponent className='ml-1 z-60' title="Reentry Tournaments">
      <p>Reentry tournaments allow players who have been eliminated to re-enter the game. The configurable options for reentry include:</p>
      <p>
        - A maximum number of reentries allowed per player<br />
        - A time limit for when reentries are available<br />
        - A set price and the corresponding chip amount the player receives for each reentry
      </p>
      <p>Reentry tournaments are similar to rebuy tournaments but typically have a higher buy-in cost and are more common in high-stakes events.</p>
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('max_reentries', () =>
    <TooltipComponent>
      The maximum number of times a player can re-enter in this tournament.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('reentry_chips', () =>
    <TooltipComponent>
      The number of chips received for each re-entry.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('reentry_price', () =>
    <TooltipComponent>
      The cost of each re-entry, defined in the tournament’s selected currency.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('reentry_end_timestamp', () =>
    <TooltipComponent>
      The deadline for purchasing a re-entry.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('buy_in_starting_chips', () =>
    <TooltipComponent>
      The number of chips each player starts with. Chips have no fixed currency value and are used purely for gameplay.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('buy_in', () =>
    <TooltipComponent>
      The amount required to enter the tournament. This is measured in the selected currency and does not correspond directly to the chip value.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('speed_type', () =>
    <TooltipComponent title="Speed Types">
      <p className="whitespace-normal">
        The speed type determines the pace of the tournament and how quickly the blinds increase.
      </p>

      <p className="type-header whitespace-normal">Regular</p>
      <p className="whitespace-normal">
        A standard-paced structure where blinds increase at a moderate rate. Level duration is set to 15 minutes, and antes start from level 5 at 10% of the big blind. This mode is suitable for players looking for a well-balanced tournament experience.
      </p>

      <p className="type-header whitespace-normal">Turbo</p>
      <p className="whitespace-normal">
        A faster-paced tournament structure with 10-minute level durations and a blind multiplier of 1.75. Antes start earlier at level 4, set at 12% of the big blind. This format accelerates gameplay while maintaining a structured progression.
      </p>

      <p className="type-header whitespace-normal">Hyper Turbo</p>
      <p className="whitespace-normal">
        An ultra-fast tournament with 3-minute levels and a blind multiplier of 2.0, doubling blinds every level. Antes begin at level 3, set at 15% of the big blind. This format is designed for players who prefer quick, high-action games with rapid blind escalations.
      </p>

      <p className="type-header whitespace-normal">Custom</p>
      <p className="whitespace-normal">
        Allows players to set their own tournament structure with customizable parameters, including level duration, blind multiplier, ante start level, ante percentage, initial blind percentage, and max levels. This provides flexibility to tailor the tournament experience to specific preferences.
      </p>
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('level_duration', () =>
    <TooltipComponent>
      The duration of each blind level in seconds. Determines how long each stage lasts before blinds increase.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('ante_start_level', () =>
    <TooltipComponent>
      The level at which antes begin. Antes are additional forced bets that increase action in later stages.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('ante_percentage', () =>
    <TooltipComponent>
      The percentage of the big blind used for the ante. If set to 10, the ante will be 10% of the big blind.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('blind_multiplier', () =>
    <TooltipComponent>
      The rate at which blinds increase per level. A multiplier of 1.5 means blinds increase by 50% each level.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('blind_max_levels', () =>
    <TooltipComponent>
      The total number of blind levels before the tournament ends. More levels allow for longer play.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('initial_blind_percentage', () =>
    <TooltipComponent>
      The percentage of the starting stack used to determine the small blind at level 1. If set to 2, the small blind is 2% of the starting stack.
    </TooltipComponent>
  ),
  ...BuildTooltipMemo('blind_levels', () =>
    <TooltipComponent title="Levels">
      The total number of blind levels for this tournament structure.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('tournament_type', () =>

    <TooltipComponent title="Tournament Type">
      Determines the tournament format

      <span className='type-header'>Buy-In</span>
      Players pay an entry fee for chips.
      <span className='type-header'>Sit & Go</span>
      Starts when a set number of players join.
      <span className='type-header'>Freeroll</span>
      Free to enter with real money prizes.
      <span className='type-header'>Spin & Go</span>
      A fast-paced tournament with random prize pools. Players spin a wheel to determine the prize pool multiplier before the game starts. The prize pool can range from 2x to 1,000x the buy-in, with varying probabilities for each multiplier.
    </TooltipComponent>
  ),

  ...BuildTooltipMemo('token', () =>
    <TooltipComponent title="Token">
      The tournaments token is used for the prize pool and for buy-in (if it is'nt a free-roll tournament).
    </TooltipComponent>
  ),
}
