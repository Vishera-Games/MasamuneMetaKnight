use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use acmd::{acmd, acmd_func};

static mut _GFX_COUNTER: [i32; 8] = [0; 8];
static mut _SOUND_COUNTER: [i32; 8] = [0; 8];

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_100",
    animcmd = "game_attack100")]
pub fn meta_knight_attack_100(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
        frame(22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_100",
    animcmd = "game_attack100sub")]
pub fn meta_knight_attack_100_sub(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=10, Size=11.0, X=0.0, Y=9.0, Z=12.0, X2=0.0, Y2=9.0, Z2=26.5, Hitlag=0.5, SDI=0.6, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=4.0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_100_end",
    animcmd = "game_attack100end")]
pub fn meta_knight_attack_100_end(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=140, FKB=0, BKB=60, Size=5.0, X=0.0, Y=8.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=140, FKB=0, BKB=60, Size=8.0, X=0.0, Y=8.0, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=140, FKB=0, BKB=60, Size=11.0, X=0.0, Y=8.0, Z=27.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.87)
        frame(15)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
pub fn meta_knight_attack_s31(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=68, KBG=20, FKB=0, BKB=25, Size=7.5, X=0.0, Y=6.4, Z=8.0, X2=0.0, Y2=4.5, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=88, KBG=20, FKB=0, BKB=25, Size=6.5, X=0.0, Y=6.5, Z=28.0, X2=0.0, Y2=6.5, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s2",
    animcmd = "game_attacks3s2")]
pub fn meta_knight_attack_s32(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            CLR_SPEED(FIGHTER_KINETIC_ENERGY_ID_MOTION)
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=60, KBG=20, FKB=0, BKB=20, Size=8.0, X=0.0, Y=6.0, Z=9.0, X2=0.0, Y2=4.5, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=85, KBG=20, FKB=0, BKB=20, Size=6.5, X=0.0, Y=6.0, Z=28.0, X2=0.0, Y2=6.0, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s3",
    animcmd = "game_attacks3s3")]
pub fn meta_knight_attack_s33(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=107, FKB=0, BKB=80, Size=7.0, X=0.0, Y=17.0, Z=6.0, X2=0.0, Y2=6.0, Z2=14.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=107, FKB=0, BKB=80, Size=8.0, X=0.0, Y=28.0, Z=12.0, X2=0.0, Y2=7.0, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=107, FKB=0, BKB=80, Size=10.0, X=0.0, Y=9.2, Z=14.0, X2=0.0, Y2=9.2, Z2=7.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(15)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn meta_knight_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=95, KBG=90, FKB=0, BKB=60, Size=5.5, X=0.0, Y=43.0, Z=0.9, X2=0.0, Y2=23.5, Z2=1.4, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=95, KBG=90, FKB=0, BKB=60, Size=8.0, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=3)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=85, KBG=120, FKB=0, BKB=35, Size=5.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(13)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn meta_knight_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=88, FKB=0, BKB=15, Size=3.2, X=0.0, Y=1.3, Z=30.0, X2=0.0, Y2=2.5, Z2=13.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.25, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=88, FKB=0, BKB=15, Size=4.5, X=0.0, Y=2.1, Z=14.0, X2=0.0, Y2=2.5, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.25, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=0.75)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=22)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
pub fn meta_knight_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=21)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=24)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=108, FKB=0, BKB=30, Size=6.2, X=0.0, Y=5.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=98, FKB=0, BKB=30, Size=6.2, X=0.0, Y=5.0, Z=13.0, X2=0.0, Y2=5.0, Z2=29.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(34)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn meta_knight_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=20.0, Z=-23.8, X2=0.0, Y2=20.0, Z2=13.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=26.0, Z=-10.8, X2=0.0, Y2=26.0, Z2=9.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=130, KBG=100, FKB=30, BKB=30, Size=6.0, X=0.0, Y=13.0, Z=-10.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=15.0, Z=-17.8, X2=0.0, Y2=26.0, Z2=-12.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=4.0, Angle=130, KBG=100, FKB=30, BKB=30, Size=6.0, X=0.0, Y=13.0, Z=8.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=20.0, Z=-23.8, X2=0.0, Y2=20.0, Z2=13.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=26.0, Z=-10.8, X2=0.0, Y2=26.0, Z2=9.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=130, KBG=100, FKB=30, BKB=30, Size=6.0, X=0.0, Y=13.0, Z=-10.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=366, KBG=100, FKB=30, BKB=30, Size=4.5, X=0.0, Y=15.0, Z=-17.8, X2=0.0, Y2=26.0, Z2=-12.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=3.0, Angle=130, KBG=100, FKB=30, BKB=30, Size=6.0, X=0.0, Y=13.0, Z=8.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=148, FKB=0, BKB=65, Size=4.5, X=0.0, Y=30.0, Z=-9.8, X2=0.0, Y2=30.0, Z2=6.2, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=85, KBG=148, FKB=0, BKB=65, Size=4.5, X=0.0, Y=13.0, Z=-10.2, X2=0.0, Y2=13.0, Z2=-27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=85, KBG=148, FKB=0, BKB=65, Size=4.5, X=0.0, Y=13.0, Z=8.6, X2=0.0, Y2=13.0, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=148, FKB=0, BKB=65, Size=4.5, X=0.0, Y=20.0, Z=-22.8, X2=0.0, Y2=20.0, Z2=20.2, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=148, FKB=0, BKB=65, Size=4.5, X=0.0, Y=26.0, Z=-16.8, X2=0.0, Y2=26.0, Z2=13.2, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(40)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn meta_knight_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=35, KBG=77, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.5, Z=11.0, X2=0.0, Y2=11.0, Z2=29.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=78, FKB=0, BKB=50, Size=5.2, X=0.0, Y=6.0, Z=-11.0, X2=0.0, Y2=6.0, Z2=-30.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(20)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn meta_knight_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=10.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=15.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=20.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=23.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=7.5, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=10.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=15.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=20.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=23.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=100, FKB=0, BKB=40, Size=6.0, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(35)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=40)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn meta_knight_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=12.5, X2=0.0, Y2=17.0, Z2=12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=15.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=23.0, X2=0.0, Y2=15.0, Z2=23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=18.0, X2=0.0, Y2=21.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.5, Angle=60, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=12.5, X2=0.0, Y2=17.0, Z2=12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=1.5, Angle=60, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=15.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=1.5, Angle=70, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=23.0, X2=0.0, Y2=15.0, Z2=23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=1.5, Angle=70, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=18.0, X2=0.0, Y2=21.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=12.5, X2=0.0, Y2=17.0, Z2=12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=15.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=23.0, X2=0.0, Y2=15.0, Z2=23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=18.0, X2=0.0, Y2=21.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.5, Angle=60, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=12.5, X2=0.0, Y2=17.0, Z2=12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=1.5, Angle=60, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=15.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=1.5, Angle=70, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=23.0, X2=0.0, Y2=15.0, Z2=23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=1.5, Angle=70, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=18.0, X2=0.0, Y2=21.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=48, KBG=140, FKB=0, BKB=60, Size=6.0, X=0.0, Y=1.0, Z=12.5, X2=0.0, Y2=17.0, Z2=12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=48, KBG=140, FKB=0, BKB=60, Size=4.0, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=15.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=48, KBG=140, FKB=0, BKB=60, Size=6.5, X=0.0, Y=3.0, Z=23.0, X2=0.0, Y2=15.0, Z2=23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=48, KBG=140, FKB=0, BKB=60, Size=6.5, X=0.0, Y=1.0, Z=18.0, X2=0.0, Y2=21.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(28)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=39)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn meta_knight_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=-12.5, X2=0.0, Y2=17.0, Z2=-12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=15.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=55, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=-26.0, X2=0.0, Y2=15.0, Z2=-26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=55, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=-21.0, X2=0.0, Y2=21.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=-12.5, X2=0.0, Y2=17.0, Z2=-12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=15.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=-26.0, X2=0.0, Y2=15.0, Z2=-26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=-21.0, X2=0.0, Y2=21.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=-12.5, X2=0.0, Y2=17.0, Z2=-12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=15.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=55, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=-26.0, X2=0.0, Y2=15.0, Z2=-26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=367, KBG=100, FKB=55, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=-21.0, X2=0.0, Y2=21.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.0, X=0.0, Y=1.0, Z=-12.5, X2=0.0, Y2=17.0, Z2=-12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=15.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=3.0, Z=-26.0, X2=0.0, Y2=15.0, Z2=-26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=1.5, Angle=55, KBG=100, FKB=40, BKB=0, Size=6.5, X=0.0, Y=1.0, Z=-21.0, X2=0.0, Y2=21.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=198, FKB=0, BKB=30, Size=6.0, X=0.0, Y=1.0, Z=-12.5, X2=0.0, Y2=17.0, Z2=-12.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=198, FKB=0, BKB=30, Size=4.0, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=15.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=198, FKB=0, BKB=30, Size=6.5, X=0.0, Y=3.0, Z=-26.0, X2=0.0, Y2=15.0, Z2=-26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=198, FKB=0, BKB=30, Size=6.5, X=0.0, Y=1.0, Z=-21.0, X2=0.0, Y2=21.0, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(37)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=40)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn meta_knight_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=67, KBG=128, FKB=0, BKB=30, Size=6.5, X=0.0, Y=29.0, Z=3.5, X2=0.0, Y2=29.0, Z2=-5.5, Hitlag=1.0, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=67, KBG=128, FKB=0, BKB=30, Size=6.0, X=0.0, Y=19.0, Z=4.5, X2=0.0, Y2=16.0, Z2=9.5, Hitlag=1.0, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=128, FKB=0, BKB=30, Size=6.0, X=0.0, Y=19.0, Z=-6.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.0, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=67, KBG=128, FKB=0, BKB=30, Size=6.0, X=0.0, Y=29.0, Z=4.5, X2=0.0, Y2=21.0, Z2=18.5, Hitlag=1.0, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=128, FKB=0, BKB=30, Size=6.0, X=0.0, Y=29.0, Z=-6.5, X2=0.0, Y2=21.0, Z2=-21.5, Hitlag=1.0, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(10)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=24)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn meta_knight_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=50, KBG=108, FKB=0, BKB=30, Size=6.0, X=0.0, Y=-8.5, Z=3.5, X2=0.0, Y2=-2.5, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=50, KBG=108, FKB=0, BKB=30, Size=6.0, X=0.0, Y=-8.5, Z=-5.5, X2=0.0, Y2=0.0, Z2=-15.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=35, KBG=108, FKB=0, BKB=30, Size=6.0, X=0.0, Y=-16.5, Z=4.5, X2=0.0, Y2=-16.5, Z2=-6.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=50, KBG=108, FKB=0, BKB=30, Size=6.0, X=0.0, Y=-16.5, Z=3.5, X2=0.0, Y2=-5.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=6.0, Angle=50, KBG=108, FKB=0, BKB=30, Size=6.0, X=0.0, Y=-16.5, Z=-5.5, X2=0.0, Y2=-5.0, Z2=-23.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(10)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        frame(Frame=26)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_s_drill",
    animcmd = "game_specialsdrill")]
pub fn meta_knight_special_s_drill(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.1, Angle=363, KBG=50, FKB=45, BKB=50, Size=3.0, X=0.0, Y=13.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.1, Angle=367, KBG=50, FKB=30, BKB=80, Size=3.0, X=0.0, Y=2.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.1, Angle=30, KBG=50, FKB=45, BKB=50, Size=3.0, X=0.0, Y=13.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.1, Angle=30, KBG=50, FKB=45, BKB=80, Size=3.0, X=0.0, Y=2.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=1.1, Angle=367, KBG=50, FKB=0, BKB=25, Size=3.5, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("haver"), Damage=1.1, Angle=365, KBG=50, FKB=0, BKB=25, Size=4.0, X=0.0, Y=5.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("haver"), Damage=1.1, Angle=367, KBG=50, FKB=0, BKB=25, Size=3.5, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("haver"), Damage=1.1, Angle=367, KBG=50, FKB=0, BKB=25, Size=3.5, X=0.0, Y=20.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=8, Part=0, Bone=hash40("haver"), Damage=1.1, Angle=367, KBG=50, FKB=0, BKB=25, Size=3.5, X=0.0, Y=25.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_no_damage_fly_smoke_all(true, false)
        }
        frame(Frame=21)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_s_end",
    animcmd = "game_specialsend")]
pub fn meta_knight_special_s_end(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=8.0, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=16.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=22.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(15)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_s_finish",
    animcmd = "game_specialairsfinish")]
pub fn meta_knight_special_air_s_end(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=8.0, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=16.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=40, KBG=185, FKB=0, BKB=40, Size=6.0, X=0.0, Y=22.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        wait(Frames=1)
        if(is_excute){
            rust{
                if DamageModule::damage(module_accessor, 0) >= 60.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn meta_knight_special_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
        frame(Frame=8)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=86, KBG=120, FKB=125, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=120, FKB=125, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=86, KBG=120, FKB=120, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=120, FKB=120, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=9.0, Angle=86, KBG=120, FKB=120, BKB=0, Size=4.0, X=0.0, Y=24.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=120, FKB=120, BKB=0, Size=4.0, X=0.0, Y=24.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=14.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=2.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=26.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=2 ,false)
            AttackModule::clear(ID=3, false)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=14.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=2.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=26.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            GroundModule::set_passable_check(true)
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=80, KBG=130, FKB=0, BKB=55, Size=7.7, X=0.0, Y=3.5, Z=-2.0, X2=0.0, Y2=22.0, Z2=-2.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=6.0, Angle=80, KBG=130, FKB=0, BKB=55, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear_all()
            GroundModule::set_passable_check(false)
        }
        frame(Frame=48)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_hi_loop",
    animcmd = "game_specialhiloop")]
pub fn meta_knight_special_hi_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=14.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=2.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=86, KBG=120, FKB=100, BKB=0, Size=5.0, X=0.0, Y=26.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=2 ,false)
            AttackModule::clear(ID=3, false)
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=14.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=2.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=84, KBG=100, FKB=68, BKB=0, Size=5.0, X=0.0, Y=26.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
            GroundModule::set_passable_check(true)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=80, KBG=130, FKB=0, BKB=55, Size=7.7, X=0.0, Y=3.5, Z=-2.0, X2=0.0, Y2=22.0, Z2=-2.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=6.0, Angle=80, KBG=130, FKB=0, BKB=55, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
            GroundModule::set_passable_check(false)
        }
        frame(Frame=41)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn meta_knight_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
        JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            //REVERSE_LR()
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_air_lw"))
            //ArticleModule::change_motion()
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw"), false, 0.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=6.5, X=0.0, Y=6.5, Z=-28.3, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_b",
    animcmd = "game_speciallwb")]
pub fn meta_knight_special_lw_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            //REVERSE_LR()
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_lw_b"))
            //ArticleModule::change_motion()
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_lw_b"), false, 0.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=6.5, X=0.0, Y=6.5, Z=-28.3, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_f",
    animcmd = "game_speciallwf")]
pub fn meta_knight_special_lw_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_NO_HOP)
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_air_lw_f"))
            //ArticleModule::change_motion()
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw_f"), false, 0.0)
            JostleModule::set_status(true)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=8.5, X=0.0, Y=7.5, Z=6.0, X2=0.0, Y2=7.5, Z2=22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=5.5, X=0.0, Y=6.5, Z=32.0, X2=0.0, Y2=6.4, Z2=-2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
pub fn meta_knight_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            REVERSE_LR()
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_air_lw"))
            //ArticleModule::change_motion()
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw"), false, 0.0)
            JostleModule::set_status(true)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=11.2, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=6.5, Z=-32.0, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            KineticModule::enable_energy(FIGHTER_KINETIC_ENERGY_ID_GRAVITY)
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_b",
    animcmd = "game_specialairlwb")]
pub fn meta_knight_special_air_lw_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            REVERSE_LR()
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_lw_b"))
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_lw_b"), false, 0.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=11.2, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=6.5, Z=-32.0, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            KineticModule::enable_energy(FIGHTER_KINETIC_ENERGY_ID_GRAVITY)
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_f",
    animcmd = "game_specialairlwf")]
pub fn meta_knight_special_air_lw_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            JostleModule::set_status(false)
        }
        frame(Frame=2)
        if(is_excute){
            //as_hash::const(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, hash40("special_air_lw_f"))
            ArticleModule::change_motion(FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, smash::phx::Hash40::new("special_air_lw_f"), false, 0.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=11.2, X=0.0, Y=7.5, Z=6.0, X2=0.0, Y2=7.5, Z2=22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=6.4, Z=32.0, X2=0.0, Y2=6.4, Z2=-2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
        frame(8)
        if(is_excute){
            JostleModule::set_status(true)
        }
        frame(Frame=21)
        if(is_excute){
            KineticModule::enable_energy(FIGHTER_KINETIC_ENERGY_ID_GRAVITY)
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07 as u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_b",
    animcmd = "game_speciallwsubb")]
pub fn meta_knight_special_lw_sub_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=127, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=127, KBG=98, FKB=0, BKB=40, Size=6.5, X=0.0, Y=6.5, Z=-28.3, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_f",
    animcmd = "game_speciallwsubf")]
pub fn meta_knight_special_lw_sub_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=127, KBG=98, FKB=0, BKB=40, Size=8.5, X=0.0, Y=7.5, Z=6.0, X2=0.0, Y2=7.5, Z2=22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=127, KBG=98, FKB=0, BKB=40, Size=5.5, X=0.0, Y=6.5, Z=32.0, X2=0.0, Y2=6.4, Z2=-2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_b",
    animcmd = "game_speciallwsubairb")]
pub fn meta_knight_special_air_lw_b_sub(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=11.2, X=0.0, Y=7.5, Z=-6.0, X2=0.0, Y2=7.5, Z2=-22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=6.5, Z=-32.0, X2=0.0, Y2=6.5, Z2=2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_f",
    animcmd = "game_speciallwsubairf")]
pub fn meta_knight_special_air_lw_f_sub(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=11.2, X=0.0, Y=7.5, Z=6.0, X2=0.0, Y2=7.5, Z2=22.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=53, KBG=98, FKB=0, BKB=40, Size=7.5, X=0.0, Y=6.4, Z=32.0, X2=0.0, Y2=6.4, Z2=-2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_SITUATION_CHANGE)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_100",
    animcmd = "effect_attack100")]
pub fn meta_knight_effect_attack_100(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_FOLLOW(0x11aa9b40ef as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.6, true)
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        wait(4)
        if(is_excute){
            FOOT_EFFECT(0x0d0da6e3c0 as u64, hash40("top"), -2, 0, 0, 0, 0, 0, 1.20000005, 10, 0, 2, 0, 0, 0, false)
        }
        wait(6)
        if(is_excute){
            FOOT_EFFECT(0x0d0da6e3c0 as u64, hash40("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false)
        }
        wait(5)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), -2, 0, 0, 0, 0, 0, 1.5, 10, 0, 2, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_100_end",
    animcmd = "effect_attack100end")]
pub fn meta_knight_effect_attack_100_end(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_OFF_KIND(0x11aa9b40ef as u64, false, false)
            EffectModule::set_disable_render_offset_last()
        }
        frame(3)
        if(is_excute){
            EFFECT_FOLLOW(0x1517a7efc2 as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.4, true)
            EffectModule::set_disable_render_offset_last()
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        }
        frame(26)
        if(is_excute){
            FOOT_EFFECT(0x13fc11e43e as u64, hash40("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s",
    animcmd = "effect_attacks3")]
pub fn meta_knight_effect_attack_s31(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(4)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), -2, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, false)
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_FOLLOW(0x13951f8fc7 as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(8)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s2",
    animcmd = "effect_attacks3s2")]
pub fn meta_knight_effect_attack_s32(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
            FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.20000005, 2, 0, 0, 0, 0, 0, false)
        }
        frame(2)
        if(is_excute){
            EFFECT_FOLLOW(0x13a4f7955a as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s3_s3",
    animcmd = "effect_attacks3s3")]
pub fn meta_knight_effect_attack_s33(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
            FOOT_EFFECT(0x0d0679b24d as u64, hash40("top"), -6, 0, 0, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, false)
        }
        frame(2)
        if(is_excute){
            EFFECT_FOLLOW(0x1302809eee as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(4)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("haver"), 0, 14, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, true)
        }
        //frame(6)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_hi3",
    animcmd = "effect_attackhi3")]
pub fn meta_knight_effect_attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(7)
        if(is_excute){
            LANDING_EFFECT(0x0fe5950916 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(8)
        if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
            EFFECT_FOLLOW(0x1396b73b0f as u64, hash40("top"), 0, 35, 0, 0, 0, 0, 1.5, true)
        }
        frame(15)
        if(is_excute){
            EFFECT_OFF_KIND(0x1396b73b0f as u64, false, false)
        }
        //frame(17)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
        frame(23)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.699999988, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_lw3",
    animcmd = "effect_attacklw3")]
pub fn meta_knight_effect_attack_lw3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            FOOT_EFFECT(0x0d0679b24d as u64, hash40("top"), -6, 0, 0, 0, 0, 0, 0.600000024, 0, 0, 0, 0, 0, 0, false)
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(3)
        if(is_excute){
            EFFECT(0x146435cbff as u64, hash40("top"), -0.370999992, 1.54900002, 18.0370007, 95, 10, -0.152999997, 1.5, 0, 0, 0, 0, 0, 0, true)
        }
        //frame(8)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_s4_s",
    animcmd = "effect_attacks4")]
pub fn meta_knight_effect_attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(12)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("haver"), 0.0100499997, 9.99973965, -0.0710799992, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        //frame(23)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(24)
        if(is_excute){
            EFFECT_FOLLOW(0x16cd6129e1 as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(25)
        if(is_excute){
            LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_hi4",
    animcmd = "effect_attackhi4")]
pub fn meta_knight_effect_attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(3)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("haver"), 0.0100499997, 9.99973965, -0.0710799992, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(7)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(8)
        if(is_excute){
            EFFECT_FOLLOW(0x186defb564 as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.6, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(18)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
        frame(35)
        if(is_excute){
            FOOT_EFFECT(0x13fc11e43e as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_lw4",
    animcmd = "effect_attacklw4")]
pub fn meta_knight_effect_attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(3)
        if(is_excute){
            FOOT_EFFECT(0x0fe5950916 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(4)
        if(is_excute){
            EFFECT_FOLLOW(0x18ac46221e as u64, hash40("top"), -9.99999975e-06, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(9)
        frame(10)
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_n",
    animcmd = "effect_attackairn")]
pub fn meta_knight_effect_attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("haver"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        //frame(2)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(5)
        if(is_excute){
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 2.400000036, true)
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 1.600000024, true)
        }
        frame(7)
        if(is_excute){
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 2.459999979, true)
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 1.639999986, true)
        }
        frame(9)
        if(is_excute){
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 2.550000072, true)
            EFFECT_FOLLOW(0x0d6dd4defe as u64, hash40("top"), 0, 5, 0, 0, 0, -90, 1.700000048, true)
        }
        //frame(20)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_f",
    animcmd = "effect_attackairf")]
pub fn meta_knight_effect_attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_FOLLOW(0x100937dc21 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.65, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        //frame(19)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_b",
    animcmd = "effect_attackairb")]
pub fn meta_knight_effect_attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_FOLLOW(0x100e5a1838 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(6)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        //frame(26)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_hi",
    animcmd = "effect_attackairhi")]
pub fn meta_knight_effect_attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_FOLLOW(0x11348f3c7d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.6, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(9)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "attack_air_lw",
    animcmd = "effect_attackairlw")]
pub fn meta_knight_effect_attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        //frame(3)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, haver, 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(4)
        if(is_excute){
            EFFECT_FOLLOW(0x11aaecc41a as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.85, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(6)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_s_drill",
    animcmd = "effect_specialsdrill")]
pub fn meta_knight_effect_special_s_drill(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(1)
        if(is_excute){
            EFFECT_FOLLOW(0x14f6f22f78 as u64, hash40("haver"), 0, 37, 0, 0, 0, 0, 2.0, true)
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        for(8 Iterations){
            wait(1)
            if(is_excute){
                EFFECT(0x0f5ea22dd6 as u64, hash40("rot"), 0, 0, 42, 90, 0, 0, 0.7, 0, 0, 10, 20, 15, 360, true)
            }
            wait(1)
            if(is_excute){
                EFFECT(0x0f5ea22dd6 as u64, hash40("rot"), 0, 0, 35, 90, 0, 0, 1.600000024, 0, 0, 10, 20, 15, 360, true)
            }
            wait(1)
            if(is_excute){
                EFFECT(0x0f5ea22dd6 as u64, hash40("rot"), 0, 0, 20, 90, 0, 0, 2.20000004, 0, 0, 10, 20, 15, 360, true)
            }
            wait(1)
            if(is_excute){
                EFFECT(0x0f5ea22dd6 as u64, hash40("rot"), 0, 0, 5, 90, 0, 0, 2.79999996, 0, 0, 10, 20, 15, 360, true)
            }
        }
        frame(43)
        if(is_excute){
            EFFECT_FOLLOW(0x18a25977f3 as u64, hash40("haver"), 0, 37, 0, 0, 0, 0, 2, false)
            EFFECT_FOLLOW_NO_STOP(0x1d7a0e5d19 as u64, hash40("haver"), 0, 37, 0, 0, 0, 0, 2, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_hi",
    animcmd = "effect_specialhi")]
pub fn meta_knight_effect_special_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(5)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(8)
        if(is_excute){
            EFFECT_FOLLOW(0x1714dab75c as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(9)
        if(is_excute){
            EFFECT(0x11345bc2de as u64, hash40("top"), 0, 20, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
        }
        frame(22)
        if(is_excute){
            EFFECT_FOLLOW(0x178dd3e6e6 as u64, hash40("top"), 0, -25, -6, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        //frame(30)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_hi_loop",
    animcmd = "effect_specialhiloop")]
pub fn meta_knight_effect_special_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_FOLLOW(0x1714dab75c as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x11345bc2de as u64, hash40("top"), 0, 20, 15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
        }
        frame(14)
        if(is_excute){
            EFFECT_FOLLOW(0x178dd3e6e6 as u64, hash40("top"), 0, -25, -4, 0, 0, 0, 1.5, true)
            EffectModule::set_disable_render_offset_last()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw",
    animcmd = "effect_speciallw")]
pub fn meta_knight_effect_special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f, false, false)
            EFFECT_FOLLOW(0x1130307fce as u64, hash40("top"), 0, -1.39999998, -0.5, 0, 0, 0, 1.4, true)
			EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45 as u64, hash40("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
        }
        frame(7)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_b",
    animcmd = "effect_speciallwb")]
pub fn meta_knight_effect_special_lw_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
            LANDING_EFFECT(0x17c2314aeb as u64, hash40("top"), 23, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FOLLOW(0x134a85f54d as u64, hash40("top"), 0, 0, -15, 0, 0, 0, 1.4, true)
			EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45 as u64, hash40("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
            LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(7)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_f",
    animcmd = "effect_speciallwf")]
pub fn meta_knight_effect_special_lw_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
            LANDING_EFFECT(0x17c2314aeb as u64, hash40("top"), -22, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT_FOLLOW(0x134de83154 as u64, hash40("top"), 0, 0, 15.3000002, 0, 0, 0, 1.4, true)
			EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45 as u64, hash40("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        frame(4)
        if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
            LANDING_EFFECT(0x0f1f9a3475 as u64, hash40("trans"), 0, 0, 0, 0, 0, 0, 0.800000012, 0, 0, 0, 0, 0, 0, false)
        }
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}     
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw",
    animcmd = "effect_specialairlw")]
pub fn meta_knight_effect_special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
            EFFECT_FOLLOW(0x15c1ab1106 as u64, hash40("top"), 0, -2.5999999, -1, 0, 0, 0, 1.4, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45 as u64, hash40("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_b",
    animcmd = "effect_specialairlwb")]
pub fn meta_knight_effect_special_air_lw_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
            EFFECT_FOLLOW(0x16981ce1dd, hash40("top"), 0, 0, -18, 0, 0, 0, 1.4, true)
            EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45 as u64, hash40("top"), 0, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        //frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_f",
    animcmd = "effect_specialairlwf")]
pub fn meta_knight_effect_special_air_lw_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
            EFFECT_FOLLOW(0x169f7125c4 as u64, hash40("top"), 0, 0, 15.3000002, 0, 0, 0, 1.4, true)
			EffectModule::set_disable_render_offset_last()
        }
        frame(2)
        if(is_excute){
            EFFECT(0x0c47781d45, hash40("top"), 0, 16, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            EFFECT_FOLLOW(0x157d379a4c as u64, hash40("top"), 0, 8, 0, 0, -90, 0, 1, true)
            LAST_EFFECT_SET_RATE(1.5)
        }
        frame(4)
        //if(is_excute){
            //EFFECT_FOLLOW(0x10790b338f as u64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
        //}
        frame(6)
        if(is_excute){
            EFFECT_OFF_KIND(0x157d379a4c as u64, false, false)
        }
        //frame(7)
        //if(is_excute){
            //EFFECT_OFF_KIND(0x10790b338f as u64, false, false)
        //}
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_lw_start",
    animcmd = "effect_speciallwstart")]
pub fn meta_knight_effect_special_lw_start(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_METAKNIGHT, 
    animation = "special_air_lw_start",
    animcmd = "effect_specialairlwstart")]
pub fn meta_knight_effect_special_air_lw_start(fighter: &mut L2CFighterCommon) {
    acmd!({
        
    });
}

pub fn once_per_knight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let kind = smash::app::utility::get_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let motion_frame = MotionModule::frame(module_accessor);

        let gfxcoords  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
          
        if kind == *FIGHTER_KIND_METAKNIGHT {

            if DamageModule::damage(module_accessor, 0) >= 60.0 {
                _GFX_COUNTER[entry_id] += 1;
                _SOUND_COUNTER[entry_id] += 1;
                AttackModule::set_power_up(module_accessor, 1.3);
                if motion_kind == hash40("special_s_start") || motion_kind == hash40("special_air_s_start") {
                    MotionModule::set_rate(module_accessor, 20.0)
                }
                if motion_kind == hash40("special_n_end") {
                    MotionModule::set_rate(module_accessor, 11.0)
                }
                if _SOUND_COUNTER[entry_id] < 2 {
                    let lua_state = fighter.lua_state_agent;
                    acmd!(lua_state, {
                        // acmd code goes here
                        PLAY_SE(0x19346fcd4a as u64)
                        PLAY_STATUS(0x16960a27fd as u64)
                    });
                }
                if _SOUND_COUNTER[entry_id] >= 100 {
                    _SOUND_COUNTER[entry_id] = 2;
                }
            }

            if DamageModule::damage(module_accessor, 0) < 60.0 {
                _SOUND_COUNTER[entry_id] = 0;
            }

            if _GFX_COUNTER[entry_id] >= 6 {
                EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_hit_purple"), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, 0.25, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_hit_purple"), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, 0.25, true, 0, 0, 0, 0, 0, true, true);
                _GFX_COUNTER[entry_id] = 0;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_knight_frame);
    acmd::add_hooks!(
        meta_knight_attack_100,
        meta_knight_attack_100_sub,
        meta_knight_attack_100_end,
        meta_knight_attack_s31,
        meta_knight_attack_s32,
        meta_knight_attack_s33,
        meta_knight_attack_hi3,
        meta_knight_attack_lw3,
        meta_knight_attack_s4,
        meta_knight_attack_hi4,
        meta_knight_attack_lw4,
        meta_knight_attack_air_n,
        meta_knight_attack_air_f,
        meta_knight_attack_air_b,
        meta_knight_attack_air_hi,
        meta_knight_attack_air_lw,
        meta_knight_special_s_drill,
        meta_knight_special_s_end,
        meta_knight_special_air_s_end,
        meta_knight_special_hi,
        meta_knight_special_hi_air,
        meta_knight_special_lw,
        meta_knight_special_lw_b,
        meta_knight_special_lw_f,
        meta_knight_special_air_lw,
        meta_knight_special_air_lw_b,
        meta_knight_special_air_lw_f,
        meta_knight_special_lw_sub_b,
        meta_knight_special_lw_sub_f,
        meta_knight_special_air_lw_b_sub,
        meta_knight_special_air_lw_f_sub,
        meta_knight_effect_attack_100,
        meta_knight_effect_attack_100_end,
        meta_knight_effect_attack_s31,
        meta_knight_effect_attack_s32,
        meta_knight_effect_attack_s33,
        meta_knight_effect_attack_hi3,
        meta_knight_effect_attack_lw3,
        meta_knight_effect_attack_s4,
        meta_knight_effect_attack_hi4,
        meta_knight_effect_attack_lw4,
        meta_knight_effect_attack_air_n,
        meta_knight_effect_attack_air_f,
        meta_knight_effect_attack_air_b,
        meta_knight_effect_attack_air_hi,
        meta_knight_effect_attack_air_lw,
        meta_knight_effect_special_s_drill,
        meta_knight_effect_special_hi,
        meta_knight_effect_special_air_hi,
        meta_knight_effect_special_lw,
        meta_knight_effect_special_lw_b,
        meta_knight_effect_special_lw_f,
        //meta_knight_effect_special_air_lw,
        //meta_knight_effect_special_air_lw_b,
        //meta_knight_effect_special_air_lw_f,
        meta_knight_effect_special_lw_start,
        meta_knight_effect_special_air_lw_start
    );
}
