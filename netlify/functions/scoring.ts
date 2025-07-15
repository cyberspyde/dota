import { Build, Hero } from '../../src/types';

// =================================================================
// SCORING ALGORITHM CONSTANTS
// These can be tuned to adjust the scoring logic.
// =================================================================

const BASE_SCORE = 3.0; // Starting score for any build

// --- Mood & Role Synergy ---
const ROLE_MOOD_MODIFIERS: Record<string, Record<string, number>> = {
  Carry:      { aggressive: 0.5, defensive: -0.2, experimental: 0.1, creative: 0.1, chaos: 0.2 },
  Support:    { aggressive: -0.2, defensive: 0.5, experimental: 0.2, creative: 0.3, chaos: 0.1 },
  Mid:        { aggressive: 0.4, defensive: 0.0,  experimental: 0.3, creative: 0.3, chaos: 0.4 },
  Initiator:  { aggressive: 0.5, defensive: 0.1,  experimental: 0.1, creative: 0.2, chaos: 0.5 },
};

// --- Item Synergy ---
const CORE_ITEM_BONUS = 0.5; // Bonus for having a solid number of core items
const SITUATIONAL_ITEM_BONUS = 0.2; // Bonus for including situational options
const LUXURY_ITEM_BONUS = 0.3; // Bonus for good luxury items

// --- Content Quality ---
const DESCRIPTION_LENGTH_BONUS = 0.1; // Bonus for detailed descriptions
const PLAYSTYLE_COMPLETENESS_BONUS = 0.3; // Bonus for having a full set of dos/donts/tips
const GAMEPLAN_COMPLETENESS_BONUS = 0.2; // Bonus for detailed gameplan

/**
 * Calculates a legitimacy score for a given build based on a set of rules.
 * @param hero The hero associated with the build.
 * @param build The build to be scored.
 * @returns A score between 0 and 5.
 */
export function calculateBuildScore(hero: Hero, build: Build): number {
  let score = BASE_SCORE;

  // 1. Mood and Role Synergy
  const roleModifiers = ROLE_MOOD_MODIFIERS[hero.role] || {};
  score += roleModifiers[build.mood] || 0;

  // 2. Item Build Quality
  const coreItems = build.items.filter(item => item.priority === 'Core').length;
  const situationalItems = build.items.filter(item => item.priority === 'Situational').length;
  const luxuryItems = build.items.filter(item => item.priority === 'Luxury').length;

  if (coreItems >= 3) {
    score += CORE_ITEM_BONUS;
  }
  if (situationalItems > 0) {
    score += SITUATIONAL_ITEM_BONUS;
  }
  if (luxuryItems > 0 && hero.role === 'Carry') {
    score += LUXURY_ITEM_BONUS;
  }
  
  // 3. Content Quality and Completeness
  if (build.items.every(item => item.description.length > 10)) {
    score += DESCRIPTION_LENGTH_BONUS;
  }
  if (build.playstyle.dos.length >= 3 && build.playstyle.donts.length >= 3 && build.playstyle.tips.length >= 3) {
    score += PLAYSTYLE_COMPLETENESS_BONUS;
  }
  if (build.gameplan.early.length > 20 && build.gameplan.mid.length > 20 && build.gameplan.late.length > 20) {
    score += GAMEPLAN_COMPLETENESS_BONUS;
  }
  
  // Clamp the score between 0 and 5
  return Math.max(0, Math.min(5, score));
} 