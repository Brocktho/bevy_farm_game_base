# Fast Paced Farming Game

## Global Terms:
  * TTI ( Time To Implement );
  * Cycles = full day night, takes 2 minutes;
  * Backpack = **6 slots; -> 8 slots; -> 10 slots;**
  * **(!)** denotes a prioritized item for initial release;
  * *(?)* denotes subject to change and or low priority item but is considered;
  * When discussing Features that need to be made a score will be provided for difficulty, effort, and confidence;
  * Difficulty: Pretty straight forward, how it appears to me to implement, Higher difficulty makes TTI wildly vary;
  * Effort: How much work is required to implement, effort is most likely a linear TTI scale;
  * Confidence: How confident I am in the previous two metrics, High confidence means ratings are probably correct;
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
#### What Must Be Done To Allow Feature:
  ** These are very different difficulty and effort values compared to others **
  - [ ] Complete Enemies Tab;
  * Difficulty: 6/10, Effort: 8/10, Confidence: 2/10;
  - [ ] Complete Crops Tab;
  * Difficulty: 7/10, Effort: 8/10, Confidence: 2/10;
  - [ ] Complete Player Tab;
  * Difficulty: 8/10, Effort: 8/10, Confidence: 2/10;
  - [ ] Complete Upgrades Tab;
  * Difficulty: 9/10, Effort: 9/10, Confidence: 5/10;
  - [ ] Complete Sprite Work;
  * Difficulty: 9/10, Effort: 10/10, Confidence: 7/10;
  - [ ] Tune Gameplay;
  * Difficulty: 3/10, Effort: 10/10, Confidence: 5/10;
#### Daytime Primary Objectives:
  - [ ] Plant Crops
  - [ ] Maintain Crops
  - [ ] Harvest Crops

#### Nighttime Primary Objectives:
  - [ ] Defend Crops
  - [ ] Maintain Crops
  - [ ] Survive

#### Purchase/Receive Upgrades & Meta Progression From Care Packages:
  - [ ] Character Upgrades **(!)**
  - [ ] Character Unlocks **(!)**
  - [ ] Farm Upgrades **(!)**
  - [ ] Farm Unlocks **(!)**
  - [ ] Farm Main Menu Visual Progression *(?)*
  - [ ] Character Skins *(?)*
  - [ ] End of cycle drops an interactable that pauses gameplay **(!)**
  - [ ] Alternates between purchaseable and free upgrades/boons **(!)**

### Enemies
#### What Must Be Done To Allow Feature:
  - [ ] Enemy sprites to be made, desired animations: Walking, Attacking;
  * Difficulty: 8/10, Effort: 10/10, Confidence: 6/10;
  - [ ] Research/Document/Test Spawn Mechanics;
  * Difficulty: 3/10, Effort: 5/10, Confidence: 8/10;
  - [ ] Research/Test Movement Mechanics ( Behaviors, Acceleration, & Max Speed);
  * Difficulty: 3/10, Effort: 6/10, Confidence: 8/10;
  - [ ] Research/Test Attack Mechanics ( Attack Style, Range, & Damage );
  * Difficulty: 5/10, Effort: 8/10, Confidence: 6/10;
  - [ ] Research/Test Enemy Focus;
  * Difficulty: 7/10, Effort: 6/10, Confidence: 5/10;
#### Dictionary: 
  - [ ] HP ( Health ) - Hits it takes to kill *(?)*;
  - [ ] Attracted By / Targets *(?)*;
  - [ ] Damage - Damage it deals to target;
  - [x] Max Speed - Fastest it can move ( subject to movement type );
  - [x] Acceleration - How quickly it accumulates speed;
  - [ ] Behavior - How it moves;
    - [x] Smooth
    - [x] Burst
    - [ ] Charge
  - [ ] Focus - How much it focuses it's target, higher number = higher focus on target;
  - [ ] Attack Style - How it attacks;
    - [ ] Melee
    - [ ] Ranged
    - [ ] Support
  - [ ] Attack Range - How far it reaches;
#### Crop Hunters: 
##### Ground Crops:
  - [ ] 🪲
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🐜
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
  - [ ] 🦫 
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🦇 *(?)*
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
  - [ ] 🐺
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🦀
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🦊
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🧟
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
  - [ ] 🦂
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
  - [ ] 🦅 *(?)* ( structure debuffer )
  * HP *(?)*
  * Attracted By *(?)*
  * Damage *(?)*
  * Max Speed *(?)*
  * Acceleration *(?)*
  * Behavior *(?)*
  * Focus *(?)*
  * Attack Style *(?)*
  * Attack Range *(?)*
  - [ ] 🦟 *(?)* ( crop debuffer )
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
  - [ ] Need to be able to save player progress;
  * Difficulty: 7/10, Effort: 7/10, Confidence: 8/10;
  - [ ] Create values for each object;
  * Difficulty: 4/10, Effort: 7/10, Confidence: 7/10;
  - [ ] Think of additional objects;
  * Difficulty: 5/10, Effort: 8/10, Confidence: 8/10;
  - [ ] Figure out UI/UX for creating, moving, using buildings & upgrades;
  * Difficulty: 7/10, Effort: 9/10, Confidence: 7/10;
  - [ ] Figure out UI/UX for upgrade menu, meta progression, etc;
  * Difficulty: 7/10, Effort: 8/10, Confidence: 5/10;
#### Dictionary:
  - [ ] HP ( Health );
  - [ ] Cost;
  - [ ] Unlockable - if not unlockable, unlocked by default;
  - [ ] Unlock Cost - if unlockable;
  - [ ] TTD ( Time To Deploy ) *(?)*;
  - [ ] TTR ( Time To Repair ) *(?)*;
  - [ ] COR ( Cost Of Repair ) *(?)*;
  - [ ] Function - What the building does
#### In Game Upgrades: 
  - [ ] 🏠 Homestead 
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: **outside of game: _visual representation of progress_**;
  - [ ] 🪵 Wall/Fence
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Block Enemies from coming a specific direction, forcing them to hit through or go around.
  - [ ] 🐝  Bee Box
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Permanent structure to bee boost small area;
  - [ ] 🎇 Sprinkler
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Boost growth time; 
  - [ ] 🌱 Seed Mill
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Turn ground crops into seeds 1 crop -> 2 seeds;
  - [ ] 🟫 ( Farm Grid )
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Plant, Maintain & Harvest crops;

#### Temporary Buffs:
  - [ ] 🐝 
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Plant, Maintain & Harvest crops;
  - [ ] 🐞 
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Plant, Maintain & Harvest crops;
  - [ ] bug spray *( no good emoji 🥺 )*
  * HP: *(?)*
  * Cost: *(?)*
  * Unlockable: *(?)*
  * Unlock Cost: *(?)*
  * TTD: *(?)*
  * TTR: *(?)*
  * COR: *(?)*
  * Function: Plant, Maintain & Harvest crops;

#### Crops:
#### What Must Be Done To Allow Feature:
  - [ ] Player must cast their focus in front of them to show where the player intends to perform an action. Highlighting specific target they have access to.
  * Difficulty: 7/10, Work Required: 7/10, Confidence: 6/10 
  - [ ] PLayer must be able to interact ( Press E ? ) once focused.
  * Difficulty: 1/10, Work Required: 4/10, Confidence: 8/10
##### Dictionary:
  - [ ] TTG *( Time to Grow )*
  - [ ] Harvested Amount
  - [ ] Boost Effect: TTL *( Time To Live )* on boost
  - [ ] Market Price
  - [ ] Upkeep
  - [ ] Boostable
  - [ ] PPS *( Price Per Seed )*
  - [ ] Hot Sale: prices up for specific crops;

##### Action Times:
  - [ ] planting ( *?🌱?* ): **4.0 seconds**;
  - [ ] watering ( 💧 ): **0.5 seconds**;
  - [ ] fertilizer ( 💩 ): **2.0 seconds**;
  - [ ] water refill ( 🪣 ): **2.5 seconds**; *(?)*
  - [ ] harvesting ( 🧑‍🌾 ): **0.5 seconds**;

##### Ground Crops: 
  - [ ] 🌽  
  * TTG: 3 cycle;
  * Harvest Amount: 1;
  * Boost Effect: 1 max hp heal; 
  * Market Price: 2 credit; 
  * Upkeep: Set and forget; 
  * Boostable: 💧 💩 🐞;
        
  - [ ] 🍓  
  * TTG: 2 cycle;
  * Harvest Amount: 1;
  * Boost Effect: *(?)*;
  * Market Price: *(?)*;
  * Upkeep: *(?)*;
  * Boostable: 💩 🐝;

  - [ ] 🍉  
  * TTG: *(?)*;
  * Harvest Amount: 1;
  * Boost Effect:*(?)*; 
  * Market Price:*(?)*; 
  * Upkeep:*(?)*;
  * Boostable: 💩 🐞 🐝;
        
  - [ ] 🧇 (wheat) 
  * TTG: 1 cycle;
  * Harvest Amount: 1; 
  * Boost Effect: 1 hp heal; 
  * Market Price: 1 credit; 
  * Upkeep: Set and forget;
  * Boostable: 💧 💩 🐞;
        
  - [ ] 🫑  
  * TTG: 1.5 cycle; 
  * Harvest Amount: 1;
  * Boost Effect: dmg boost; 
  * Market Price: 3 credit; 
  * Upkeep: 💧 .4 cycles; 
  * Boostable: 🐞 🐝 💩

##### Trees:
  - [ ] 🍎  
  * TTG: 5 cycle (harvest every 2);
  * Harvest Amount: 4;
  * Boost Effect: Harvest time boost; 50% *(?)* .25 cycles; **loooking for like 20-30 seconds**
  * Market Price: 1 credit;
  * Upkeep: 💧 1 cycle; 💧 + 🐝 3 cycles; 
  * Boostable: 💩 🐞 🐝;

  - [ ] 🥑  
  * TTG: 3 cycle (harvest every 3);
  * Harvested Amount: 2; 
  * Boost Effect: temp hp +1 per fruit;
  * Market Price: 2; 
  * Upkeep: 💧 + 💩 .5 cycle; 1.5 cycle upkeep after;
  * Boostable: 🐞 🐝;
        
  - [ ] 🍌  
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
