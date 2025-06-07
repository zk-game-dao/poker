# Changelog

## WIP

### Fixed
- Internet identity idle timeout increased to 1 day.
- Rendering of token icons now also shows in-line.
- `"No EIP-6963 wallets found"` warning message improved.
- Update `ic-cdk` to 0.18.3.
- Make dev principals log viewers and not controllers in prep for the SNS.

## [2.3.1] - 2025-05-26

### Added
- Add DAO announcement banner.
- Add upgrade canister functions.

### Fixed
- Leaderboard will only pay out to POH verified players.

## [2.3.0] - 2025-04-11

### Added

- Add min max buttons for some token inputs.
- Add new inline chat system.

### Fixed

- Fix addon to pause at the correct time.
- Fix tournament leaderboard.
- Fix tournament addons.
- Fix timeout bug not working with multiple inactive players.
- Fix multiple performance issues related to ui.
- Fix hud rendering issues.

---

## [2.2.0] - 2025-04-01

### Added

- Add custom token support

### Changed

- Wallet ui overhaul

## [2.1.0] - 2025-03-22

### Added

- Add Spin and Go tournament type.
- Add Multi Table Tournaments.
- Add global quick join for cash games.

### Fixed

- Add rebuy and addons to tournament prize pool.

---

## [2.0.0] - 2025-02-21

### Added

- Add single table buy in tournaments.

---

## [1.4.10] - 2025-01-23

### Added
- Add rake and revenue share functionality.
- Add support for USDT and ETH along with ck counterparts.

### Changed
- Improve in game ui by moving cards to the HUD.

---

## [1.4.9] - 2025-01-05

### Added
- Add tracing to better locate where errors originate.

### Changed
- Move inter canister calls out of library when kicking inactive players.

---

## [1.4.8] - 2025-01-02

### Fixed
- Chat button not clickable

---

## [1.4.7] - 2024-12-12

### Changed
- General qol improvements on ui

### Fixed
- Visual player distribution on table

### Changed
- Update description on leaderboard and new XP mechanics.

---

## [1.4.6] - 2024-12-06

### Fixed
- Fix turn ordering at post flop to start with the player to the left of the dealer.
- Add refund functionality when deposit and join table fails.
- Add retry functionality when querying blocks.

---

## [1.4.5] - 2024-11-29

### Added
- Add wallet installation instructions.
- Sort get tables by amount of users on the table.
- Kick user for being seated out too long functionality.
- Add experience points system.

### Changed
- Remove fold button when its not the users turn.
- Put the users player tag on top for mobile.
- Introduce max buy in of 10x the big blind.

### Fixed
- Enhance mobile user interface throughout the platform.
- Fix join table modal by improving error transparency.

---

## [1.4.4] - 2024-11-21

### Changed
- Set default new round timer to 7s
- Remove small and big blind buttons

### Fixed
- Fix leave table causing pot split bug.

---

## [1.4.3] - 2024-11-20

### Changed
- Move landing page to `/about-us`
- Move lobby to `/`

### Fixed
- Auto sit out inactive user.
- Rotate gameplay clockwise.

---

## [1.4.2] - 2024-11-19

### Fixed
- Check if external wallet principal or account id has been set.

---

## [1.4.1] - 2024-11-19

### Changed
- Add extra confirmation step for withdrawals.
- Add dynamic switch between principal or account id for withdrawals.

### Fixed
- Fixed `Failed to fetch dynamically imported module` error.
- Fixed withdrawing to external account.

---

## [1.4.0] - 2024-11-16

### Added
- Add USDC.
- Add mobile UI for games.
- Add ZKP wallet functionality to Internet Identity users.

### Changed
- Overhaul navigational ui elements for desktop and mobile.
- Use std panic catch unwind to make upgrades safer.

---

## [1.3.0] - 2024-10-29

### Added
- Add windoge to ic-assets.
- Add the option to play with fake money.
- Add functionality to be able to filter tables.
- Add transaction history.
- Add changelog page.
- Add roadmap page.
- Add authentication.

---

## [1.2.3] - 2024-10-26

### Fixed
- Fix frozen table bug.

---

## [1.2.2] - 2024-10-25

### Fixed
- Fix auto checking instead of auto folding bug.
- Fix create user cycle check bug.

---

## [1.2.1] - 2024-10-22

### Fixed
- Request for cycles from cycle dispenser fixed.

---

## [1.2.0] - 2024-10-21

### Added
- Add call to store action logs in log store canister.
- Add deposit and withdraw functionality to the table canister.
- Add withdraw legacy funds panel in wallet ui.
- Add functionallity for web3auth users to deposit into their zkp wallets using plug and bitfinity.
- Add sidepot creation log entry.
- Add log grouping by hand.
- Add pot limit.
- Add pre fold button.
- Add volume level to user object.
- Add volume level slider in profile UI.
- Add functionality to sit out inactive players.

### Changed
- Update ui to allow users to directly interface with their wallets.
- Rename wording for transfers from balance to wallet funds.

### Removed
- Remove references to user balances in the ui.

---

## [1.1.0] - 2024-10-07

### Added
- Add CHANGELOG.
- Add enlarge text toggle to user object.
- Add `web3auth` for authentication.
- Add ZKP wallet for `web3auth` accounts.

### Changed
- Unopiniated tansfer methods in balance modal.

### Fixed
- Fix balances on user tags to represent real time value.

---

## [1.0.0] - 2024-08-20

### Added
- Final test before deployment, ensuring all features are functioning correctly.
- Implemented "leave table" feature that triggers the next player to act.
- Re-added wallet plug-in feature for frontend integration.
- Introduced storable structure for table management.
- Canister per table architecture for better scalability.
- Authentication system integrated with backend and frontend.
- Mucking feature added, allowing users who fold to choose whether to show their cards.
- Force end games upon deployment to maintain game integrity.
- Clean-up process initiated when the last user leaves the table.

### Changed
- Updated to use `e8s` instead of floats for backend calculations.
- Timer set to auto-check when it runs out, streamlining gameplay.

### Fixed
- Fixed issue with timers not functioning correctly across multiple tables.

### Removed
- Old method of displaying winning hands replaced, ensuring only relevant cards are shown.

---

## [0.2.0] - 2024-08-05

### Added
- New opponent interaction feature displaying opponents' cards.
- Copy link functionality for easy table sharing upon table creation.
- "Support us" functionality integrating both frontend and backend.
- Action logs overhaul for better clarity and tracking.
- Chat toggle option for user communication during gameplay.
- UI for the "no tables found" scenario in the lobby, providing a more user-friendly experience.

### Changed
- Updated feedback pop-up design for enhanced user interaction.
- Redesigned the switch component to improve UX.
- Time circles and bars now count down, not up, for better game tracking.

### Fixed
- Resolved backend bug where checking after raising was not functioning correctly.
- Fixed an issue where minimum balance limits prevented betting on low balances.
- The bug causing issues with the "turn" feature was addressed.

### Security
- Improved user object fetching from the backend for displaying notifications, enhancing data security.

---

## [0.1.0] - 2024-07-08

### Added
- Implement core game mechanics for No Limit, Spread Limit and Fixed Limit Poker.
- Initial setup for Usergeek production.
- "Add possible hands" modal to provide better user insights.
- New HUD design implemented for improved user interface.
- Added logging for transitioning to the next round.
- Round ticker added to table for enhanced round tracking.
- Authentication and profile pop-up integrated with backend and frontend.

### Changed
- Email updated for "Contact Us" to reflect new communication guidelines.
- Moved feedback modal to function correctly in the new UX flow.

### Fixed
- Fixed issue where spamming the call button resulted in multiple calls.
- Cards no longer hide chips at certain screen sizes.
- Community card animation bug resolved for smoother gameplay.
- Addressed bug preventing feedback pop-ups from functioning as intended.
- Corrected issue where certain buttons didn’t adapt to the UI changes.

### Removed
- Removed outdated API endpoints and legacy structures from the initial implementation.

