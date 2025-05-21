import { Fragment, memo } from 'react';

import { LayoutComponent } from '@zk-game-dao/ui';
import { useIsBTC } from '@zk-game-dao/currency';
import { useWording } from '../../hooks/wording';

const HOUSE_RULES = [
  {
    title: "House Rules for {{brand}}",
    content:
      "These house rules govern the conduct of all participants on our online poker platform, {{brand}}. They are designed to ensure fair play and a positive experience for all users.",
  },

  {
    title: "General Conduct",
    content: [
      [
        "Respect",
        "All players must treat each other with respect. Any form of harassment, abusive language, or inappropriate behavior will not be tolerated.",
      ],
      [
        "Fair Play",
        "Players must play fairly and honestly. Collusion, cheating, or use of any external software or bots is strictly prohibited.",
      ],
    ],
  },

  {
    title: "Gameplay Rules",
    content: [
      [
        "Age Requirement",
        "Players must be at least 18 years old to participate in games.",
      ],
      [
        "Account",
        "Each player is allowed one account. Creating multiple accounts for any reason is prohibited.",
      ],
      [
        "Game Integrity",
        "The integrity of the game must be maintained at all times. Any suspicious activity should be reported to the support team immediately.",
      ],
      [
        "Connection Issues",
        "Players are responsible for their own internet connection. The platform is not liable for any losses due to connection issues.",
      ],
      [
        "Disconnections",
        "In the event of a disconnection, players will still be considered active in the game and bets will be handled according to the platform's disconnection policy.",
      ],
    ],
  },

  {
    title: "Betting and Financial Rules",
    content: [
      [
        "Minimum and Maximum Bets",
        "All games will have clearly stated minimum and maximum bets. Players must adhere to these limits.",
      ],
      [
        "Deposits and Withdrawals",
        "Players must only use their own funds for deposits. Withdrawals will be processed according to the platform's withdrawal policy.",
      ],
      [
        "Chips",
        "Virtual chips have no real-world value and cannot be exchanged for real money.",
      ],
    ],
  },
  {
    title: "Privacy and Security",
    content: [
      [
        "Account Security",
        "Players are responsible for keeping their account information secure. Sharing account details with others is prohibited.",
      ],
      [
        "Personal Information",
        "Players must provide accurate information during registration. False information may lead to account suspension.",
      ],
    ],
  },
  {
    title: "Violations and Penalties",
    content: [
      [
        "Rule Violations",
        "Any violation of these rules may result in warnings, temporary suspensions, or permanent bans, depending on the severity of the infraction.",
      ],
      [
        "Reporting",
        "Players can report any rule violations or suspicious activities to the support team for investigation.",
      ],
    ],
  },

  {
    title: "Community and Support",
    content: [
      [
        "Support",
        "Our support team is available to help with any issues or questions. Players can reach out via the support section on the platform.",
      ],
      [
        "Feedback",
        "We encourage players to provide feedback to help us improve the platform. Constructive suggestions are always welcome.",
      ],
    ],
  },

  {
    title: "Conclusion",
    content:
      "These house rules are designed to ensure a fair and enjoyable experience for all players. By playing on {{brand}}, you agree to abide by these rules and promote a positive community.",
  },
];

export const HouseRulesPage = memo(() => {
  const { product: brand } = useWording();
  return (
    <LayoutComponent
      footer
      hero={{
        title: "House Rules",
        subTitle: `Guidelines to ensure fair and enjoyable gameplay on ${brand}.`,
      }}
    >
      {HOUSE_RULES.map(({ title, content }) => (
        <Fragment key={title}>
          <h2 className="text-material-heavy-1 type-top max-w-[650px] mx-auto mt-8 text-left w-full">
            {title.replace(/\{\{brand\}\}/g, brand)}
          </h2>
          {typeof content === "string" ? (
            <p className="text-material-heavy-1 mt-3 max-w-[650px] mx-auto">
              {content.replace(/\{\{brand\}\}/g, brand)}
            </p>
          ) : (
            <ul className="list-decimal pl-8">
              {content.map(([subTitle, subContent]) => (
                <li
                  key={subTitle}
                  className="text-white mt-3 max-w-[650px] mx-auto"
                >
                  <span className="type-headline mr-2 text-white">
                    {subTitle.replace(/\{\{brand\}\}/g, brand)}:
                  </span>
                  <span className="text-material-heavy-1">{subContent.replace(/\{\{brand\}\}/g, brand)}</span>
                </li>
              ))}
            </ul>
          )}
        </Fragment>
      ))}
    </LayoutComponent>
  )
});
HouseRulesPage.displayName = "HouseRulesPage";

export default HouseRulesPage;
