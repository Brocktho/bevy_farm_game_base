# Fast Paced Farming Game

## Global Terms:
  * Cycles = full day night, takes 2 minutes;
  * Backpack = **6 slots; -> 8 slots; -> 10 slots;**
  * **(!)** denotes a prioritized item for initial release
  * *(?)* denotes subject to change and or low priority item but is considered.

## Primary Goals:
  * Fast paced gameplay where there is always something to do.
  * Personalized experience where choices affect the challenges you face.
  * Progressive enhancement upon gameplay as you continue to play, adding choices and challenge.
  * Create a cohesive world that interests the player, bit of comedy, bit of seriousness.

## Secondary Goals:
  * Create visual consistency
  * Provide information to the player visually through UI, shaders, particles, and sounds.
  * Keep numbers within the game small, helps make balancing easier and understandable.

## Primary Systems

### Main Gameplay Loop
#### Daytime Primary Objectives:
  [] Plant Crops
  [] Maintain Crops
  [] Harvest Crops

#### Nighttime Primary Objectives:
  [] Defend Crops
  [] Maintain Crops
  [] Survive

#### Purchase/Receive Upgrades & Meta Progression From Care Packages:
  [] Character Upgrades **(!)**
  [] Character Unlocks **(!)**
  [] Farm Upgrades **(!)**
  [] Farm Unlocks **(!)**
  [] Farm Main Menu Visual Progression *(?)*
  [] Character Skins *(?)*
  [] End of cycle drops an interactable that pauses gameplay **(!)**
  [] Alternates between purchaseable and free upgrades/boons **(!)**

### Enemies
#### Dictionary: 
  [] HP ( Health ) - Hits it takes to kill *(?)*;
  [] Attracted By / Targets *(?)*;
  [] Damage - Damage it deals to target;
  [x] Max Speed - Fastest it can move ( subject to movement type );
  [x] Acceleration - How quickly it accumulates speed;
  [] Behavior - How it moves;
    [x] Smooth
    [x] Burst
    [] Charge
  [] Focus - How much it focuses it's target, higher number = higher focus on target;
  [] Attack Style - How it attacks;
    [] Melee
    [] Ranged
    [] Support
  [] Attack Range - How far it reaches;
#### Crop Hunters: 
##### Ground Crops:
  [] 🪲
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  [] 🐜
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
##### Trees:
  [] 🦫 
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  [] 🦇 *(?)*
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
#### Player Hunters:
##### Melee Hunters:
  [] 🐺
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  [] 🦀
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
[] 🦊
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
[] 🧟
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
##### Ranged Hunters:
[] 🦂
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
##### Debuff Hunter: *(?)*
[] 🦅 *(?)* ( structure debuffer )
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
[] 🦟 *(?)* ( crop debuffer )
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*

### Upgrades:
#### What Must Be Done To Allow Feature:
  []
#### Dictionary:
  [] HP ( Health );
  [] Cost;
  [] Unlockable - if not unlockable, unlocked by default;
  [] Unlock Cost - if unlockable,
  []  
#### In Game Upgrades: 
[] 🏠 ( homestead ) **outside of game: _visual representation of progress_**
[] 🪵 ( wall/fence )
[] 🐝 ( 🐝 box permanent structure for sub area )
[] 🎇 ( sprinkler boost growth time )
[] 🌱 ( seed mill turn ground crops into seeds ) 1 crop -> 2 seeds;
[] 🟫 ( Farm Grid )

#### Temporary Buffs:
  * 🐝 
  * 🐞 
  * bug spray *( no good emoji 🥺 )* 

#### Crops:
##### Dictionary:
  * TTG *( Time to Grow )*
  * Harvested Amount
  * Boost Effect: TTL *( Time To Live )* on boost
  * Market Price
  * Upkeep
  * Boostable
  * PPS *( Price Per Seed )*
  * Hot Sale: prices up for specific crops;

##### Action Times:
  [] planting ( *?🌱?* ): **4.0 seconds**;
  [] watering ( 💧 ): **0.5 seconds**;
  [] fertilizer ( 💩 ): **2.0 seconds**;
  [] water refill ( 🪣 ): **2.5 seconds**; *(?)*
  [] harvesting ( 🧑‍🌾 ): **0.5 seconds**;

##### Ground Crops: 
[] 🌽  
  * TTG: 3 cycle;
  * Harvest Amount: 1;
  * Boost Effect: 1 max hp heal; 
  * Market Price: 2 credit; 
  * Upkeep: Set and forget; 
  * Boostable: 💧 💩 🐞;
        
[] 🍓  
  * TTG: 2 cycle;
  * Harvest Amount: 1;
  * Boost Effect: *(?)*;
  * Market Price: *(?)*;
  * Upkeep: *(?)*;
  * Boostable: 💩 🐝;

[] 🍉  
  * TTG: *(?)*;
  * Harvest Amount: 1;
  * Boost Effect:*(?)*; 
  * Market Price:*(?)*; 
  * Upkeep:*(?)*;
  * Boostable: 💩 🐞 🐝;
        
[] 🧇 (wheat) 
  * TTG: 1 cycle;
  * Harvest Amount: 1; 
  * Boost Effect: 1 hp heal; 
  * Market Price: 1 credit; 
  * Upkeep: Set and forget;
  * Boostable: 💧 💩 🐞;
        
[] 🫑  
  * TTG: 1.5 cycle; 
  * Harvest Amount: 1;
  * Boost Effect: dmg boost; 
  * Market Price: 3 credit; 
  * Upkeep: 💧 .4 cycles; 
  * Boostable: 🐞 🐝 💩

##### Trees:
[] 🍎  
  * TTG: 5 cycle (harvest every 2);
  * Harvest Amount: 4;
  * Boost Effect: Harvest time boost; 50% *(?)* .25 cycles; **loooking for like 20-30 seconds**
  * Market Price: 1 credit;
  * Upkeep: 💧 1 cycle; 💧 + 🐝 3 cycles; 
  * Boostable: 💩 🐞 🐝;

[] 🥑  
  * TTG: 3 cycle (harvest every 3);
  * Harvested Amount: 2; 
  * Boost Effect: temp hp +1 per fruit;
  * Market Price: 2; 
  * Upkeep: 💧 + 💩 .5 cycle; 1.5 cycle upkeep after;
  * Boostable: 🐞 🐝;
        
[] 🍌  
  * TTG: 4 cycle (harvest every 3); 
  * Harvest Amount: 3;
  * Boost Effect: speed boost; **10%** *(?)* 
  * Market Price: 2; 
  * Upkeep: 💩 .5 cycle; 1.5 cycle upkeep after;
  * Boostable: 🐞 🐝; 

pay to win: 
  * weapon;
  * character; 
  * farmhouse upgrade; 
  * cosmetic main menu upgrades;

just receive buff: 
  * seeds; 
  * money; 
  * temp boost; 
  * 50% perm upgrade compared to pay to win || separate buff not pay to win;



Player: 
  * 🐰
