//! Balance rules keep headcount intact while constraining siege frontage and repeated hero screens.
use super::*;

pub const HERO_RECOVERY_SECONDS: f32 = 12.0;
pub const HERO_RECOVERY_HP: f32 = 0.2;
const SIEGE_FRONTAGE_COMPANIES: f32 = 10.0;

impl Unit {
    /// Normal weapon cooldown never exceeds def.cd; the extra interval is saved recovery time.
    pub fn is_recovering(&self) -> bool {
        self.hero != NONE && self.cooldown > self.def().cd + 0.0001
    }

    pub fn recovery_seconds(&self) -> f32 {
        if self.is_recovering() {
            (self.cooldown - self.def().cd).max(0.0)
        } else {
            0.0
        }
    }

    /// A whole army cannot attack one gate simultaneously. Field firepower stays proportional.
    pub fn siege_mass(&self) -> f32 {
        if !self.is_company() {
            return 1.0;
        }
        let mass = self.formation_mass();
        let frontage = if mass > SIEGE_FRONTAGE_COMPANIES {
            SIEGE_FRONTAGE_COMPANIES + (mass - SIEGE_FRONTAGE_COMPANIES).sqrt()
        } else {
            mass
        };
        frontage.min(mass) * (self.hp / self.max_hp()).clamp(0.0, 1.0)
    }
}

impl World {
    pub(super) fn fund_starting_armies(&mut self) {
        for n in &mut self.nations {
            if n.alive {
                n.res[0] = n.res[0].max(n.upkeep * SEC_PER_YEAR * 2.0);
            }
        }
    }

    /// A neighbour's allies can respond to a new war. Dated historical wars bypass this estimate.
    pub fn defensive_strength(&self, n: usize) -> f32 {
        (0..NATIONS.len())
            .filter(|&o| self.nations[o].alive && (o == n || self.campaign_allied(n, o)))
            .map(|o| self.nations[o].strength)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_blocks_targeting_and_support_until_health_has_returned() {
        let mut w = World::new(42);
        let hero = w.hero_of(MINAMOTO).unwrap();
        let handle = w.uh(hero);
        w.kill_unit(hero);
        let start_hp = w.units[hero].hp;
        assert!(w.units[hero].alive && !w.units[hero].on_map());
        assert!((start_hp / w.units[hero].max_hp() - 0.2).abs() < 0.001);
        assert!(w.target_info(Target::Unit(handle)).is_none());
        w.damage_unit(hero, 1_000_000.0, None, TAIRA);
        assert_eq!(w.units[hero].hp, start_hp);
        for _ in 0..(6.0 / DT) as usize {
            w.update_units(DT);
        }
        assert!(w.units[hero].is_recovering());
        assert!(w.units[hero].hp > start_hp && w.units[hero].hp < w.units[hero].max_hp());
        for _ in 0..(6.1 / DT) as usize {
            w.update_units(DT);
        }
        assert!(w.units[hero].on_map());
        assert!(w.target_info(Target::Unit(handle)).is_some());
        assert_eq!(w.units[hero].hp, w.units[hero].max_hp());
    }

    #[test]
    fn recovery_survives_save_annexation_and_still_obeys_death_date() {
        let mut w = World::new(42);
        let hero = w.hero_of(MINAMOTO).unwrap();
        w.kill_unit(hero);
        for _ in 0..(3.0 / DT) as usize {
            w.update_units(DT);
        }
        let mut writer = crate::save::Writer::new();
        w.save_to(&mut writer);
        let mut restored =
            World::load_from(42, &mut crate::save::Reader::new(&writer.buf)).unwrap();
        assert!(
            (restored.units[hero].recovery_seconds() - w.units[hero].recovery_seconds()).abs()
                < 0.001
        );
        assert_eq!(restored.units[hero].hp, w.units[hero].hp);
        restored.annex(HOJO, MINAMOTO);
        assert_eq!(restored.units[hero].nation as usize, HOJO);
        assert!(restored.units[hero].is_recovering());
        restored.years = 1199.0 - START_AD;
        restored.update_campaign_heroes();
        assert!(!restored.units[hero].alive);
        assert!(!restored.hero_on_field(0));
    }

    #[test]
    fn ordinary_weapon_cooldown_is_not_recovery() {
        let mut w = World::new(42);
        let hero = w.hero_of(MINAMOTO).unwrap();
        w.units[hero].cooldown = w.units[hero].def().cd;
        assert!(w.units[hero].on_map());
        assert_eq!(w.units[hero].recovery_seconds(), 0.0);
    }

    #[test]
    fn siege_frontage_does_not_nerf_field_headcount_or_ordinary_companies() {
        let mut w = World::new(42);
        let h = w.spawn_unit(CHIBA, 20, w.nations[CHIBA].home, 1.0);
        assert_eq!(w.unit(h).unwrap().siege_mass(), 1.0);
        let u = w.unit_mut(h).unwrap();
        u.establishment = 10_000;
        u.hp = u.max_hp();
        assert_eq!(u.personnel(), 10_000);
        assert_eq!(u.fighting_mass(), 100.0);
        assert!(u.siege_mass() > 10.0 && u.siege_mass() < 25.0);
    }

    #[test]
    fn contested_bases_cannot_replace_casualties_during_combat() {
        let mut w = World::new(42);
        let i = w
            .units
            .iter()
            .position(|u| u.alive && u.nation as usize == CHIBA && u.is_company())
            .unwrap();
        w.units[i].hp *= 0.5;
        let pos = w.units[i].pos;
        w.spawn_unit(TAIRA, 20, pos + v2(2.0, 0.0), 1.0);
        w.declare_war(CHIBA, TAIRA, true);
        w.nations[CHIBA].res = [100.0; 3];
        let before = w.units[i].hp;
        w.reinforce_companies(10.0);
        assert_eq!(w.units[i].hp, before);
        assert_eq!(w.nations[CHIBA].res, [100.0; 3]);
        w.make_peace(CHIBA, TAIRA, 1.0);
        w.reinforce_companies(10.0);
        assert!(w.units[i].hp > before);
    }

    #[test]
    fn starting_food_covers_large_armies_and_load_does_not_refill_it() {
        let w = World::new(42);
        assert!(w.nations[TAIRA].res[0] > 200.0);
        for n in &w.nations {
            if n.alive {
                assert!(n.res[0] >= n.upkeep * SEC_PER_YEAR * 2.0);
            }
        }
        let mut w = w;
        w.nations[TAIRA].res[0] = 10.0;
        let mut writer = crate::save::Writer::new();
        w.save_to(&mut writer);
        let restored = World::load_from(42, &mut crate::save::Reader::new(&writer.buf)).unwrap();
        assert_eq!(restored.nations[TAIRA].res[0], 10.0);
    }

    #[test]
    fn internal_conflict_takes_priority_over_a_distant_old_expedition() {
        let mut w = World::new(42);
        w.years = 1203.0 + 8.0 / 12.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        assert_eq!(w.campaign_target(HOJO), Some(HIKI));
        w.make_peace(HOJO, HIKI, 1.0);
        assert_eq!(w.campaign_target(HOJO), Some(FUJIWARA));
    }
}
