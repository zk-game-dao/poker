# Understanding Our Rake System

## What is Rake?
Rake is a small fee taken from poker pots to cover the operational costs of running the poker platform. Our rake system is designed to be fair, transparent, and competitive with industry standards.

## Rake Structure
The rake amount is calculated based on several factors:
- The stakes being played
- The number of players at the table
- The size of the pot

### Rake Percentages by Stakes (No Limit Games)
- Micro Stakes (ICP 0.0001/0.0002 to ICP 0.0099/0.0198): 4.5% rake
- Mini Stakes (ICP 0.01/0.02 to ICP 0.24/0.48): 4.5% rake
- Low Stakes (ICP 0.25/0.50 to ICP 0.99/1.98): 4.0% rake
- Mid Stakes (ICP 1/2 to ICP 2.99/5.98): 3.5% rake
- High Stakes (ICP 3/6 to ICP 4.99/9.98): 3.0% rake
- Higher Stakes (ICP 5/10+): 2.5% rake

### Rake Caps
To ensure fairness, we implement rake caps that vary based on the number of players:
- Tables with 2-3 players: Lower rake cap
- Tables with 4+ players: Higher rake cap

The specific cap amounts scale with the stake levels. For example:
- At mini stakes (ICP 0.01/0.02 to ICP 0.24/0.48):
  - 2-3 players: Cap ranges from ICP 0.05 to ICP 0.20
  - 4+ players: Cap ranges from ICP 0.10 to ICP 0.50

## Important Rake Collection Details

### Rake Payout Schedule
**Please Note**: To optimize for network efficiency and minimize transaction costs:
- Rake is only collected from the platform every 10 rounds
- Collection only occurs when the accumulated rake exceeds 3 times the default ICP transaction fee
- This batching system helps reduce overall transaction costs while maintaining efficient operations

### Rake Sharing Program
We offer a unique rake sharing option for table creators:
- Cost: 1 ICP to enable rake sharing when creating a table
- Benefit: Table creators can earn a portion of the rake generated at their table
- This option must be selected during table creation and cannot be modified later
- The specific revenue share percentage is determined by the platform based on various factors

## Fixed Limit Games
Fixed Limit games follow a similar structure with slightly different ranges:
- Micro Stakes (ICP 0.0001/0.0002 to ICP 0.029/0.058): 4.5% rake
- Mini Stakes (ICP 0.03/0.06 to ICP 0.24/0.48): 4.5% rake
- Low Stakes (ICP 0.25/0.50 to ICP 0.99/1.98): 4.0% rake
- Mid Stakes (ICP 1/2 to ICP 2.99/5.98): 3.5% rake
- High Stakes (ICP 3/6 to ICP 4.99/9.98): 3.0% rake
- Higher Stakes (ICP 5/10+): 2.5% rake

## Legal and Compliance Notice
- All rake collected is used for platform operations, maintenance, and development
- Rake calculations are performed automatically by smart contracts to ensure fairness
- The rake structure may be subject to change with appropriate notice to users
- Users can verify rake amounts in their game histories
- Our rake system complies with standard industry practices and relevant regulations

## Transparency
You can always view:
- The current rake percentage for your table
- The applicable rake cap
- Your accumulated rake contributions in your game history
- Rake share earnings (if you've created a table with rake sharing enabled)

## Questions or Concerns?
If you have any questions about our rake system or need clarification on any aspect, please contact our support team. We're committed to maintaining full transparency about our rake structure and ensuring all players understand how it works.