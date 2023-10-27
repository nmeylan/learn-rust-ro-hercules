// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::weapon::Weapon;
use models::item::NormalInventoryItem;

use crate::{Skill, SkillRequirementResult, DelegateSkill};

use crate::skills::*;
// MG_SRECOVERY
pub struct IncreaseSpRecovery {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for IncreaseSpRecovery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_SIGHT
pub struct Sight {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Sight {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp >= 10 { Ok(10) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_NAPALMBEAT
pub struct NapalmBeat {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for NapalmBeat {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 9 { return Ok(9) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_SAFETYWALL
pub struct SafetyWall {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SafetyWall {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_SOULSTRIKE
pub struct SoulStrike {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for SoulStrike {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 38 { return Ok(38) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_COLDBOLT
pub struct ColdBolt {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for ColdBolt {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_FROSTDIVER
pub struct FrostDiver {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FrostDiver {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_STONECURSE
pub struct StoneCurse {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for StoneCurse {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_FIREBALL
pub struct FireBall {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FireBall {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp >= 25 { Ok(25) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_FIREWALL
pub struct FireWall {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FireWall {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp >= 40 { Ok(40) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_FIREBOLT
pub struct FireBolt {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for FireBolt {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_LIGHTNINGBOLT
pub struct LightningBolt {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for LightningBolt {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_THUNDERSTORM
pub struct Thunderstorm {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for Thunderstorm {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if character_sp >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 2 {
            if character_sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 3 {
            if character_sp >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 4 {
            if character_sp >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 5 {
            if character_sp >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 6 {
            if character_sp >= 54 { return Ok(54) } else {return Err(())}
        }
        if self.level == 7 {
            if character_sp >= 59 { return Ok(59) } else {return Err(())}
        }
        if self.level == 8 {
            if character_sp >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 9 {
            if character_sp >= 69 { return Ok(69) } else {return Err(())}
        }
        if self.level == 10 {
            if character_sp >= 74 { return Ok(74) } else {return Err(())}
        }
        Err(())
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}
// MG_ENERGYCOAT
pub struct EnergyCoat {
    level: u8,
    delegate: Option<Box<dyn DelegateSkill>>,
}
impl Skill for EnergyCoat {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, delegate: None })
    }
    fn level(&self) -> u8 {
        self.level
    }
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &self.delegate
    }
    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32> {
        if character_sp >= 30 { Ok(30) } else {Err(())}
    }
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>> {
        Ok(None)
    }
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()> {
         Ok(())
    }
    fn hit_count(&self) -> u8 {
        1
    }
    fn cast_delay(&self) -> u32 {
        0
    }
    fn after_cast_delay(&self) -> u32 {
        0
    }
}