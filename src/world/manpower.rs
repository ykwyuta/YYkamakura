//! People are independent of quality, hero identity and tactical sprite count.
use super::*;

pub const COMPANY_MEN: u32 = 100;
pub const MAX_FORMATION_MEN: u32 = 100_000;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ArmyStrength {
    pub personnel: u32,
    pub establishment: u32,
    pub companies: u32,
    pub commanders: u32,
    pub ships: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn company(w: &World, n: usize) -> Handle {
        w.units
            .iter()
            .enumerate()
            .find(|(_, u)| u.alive && u.nation as usize == n && u.is_company())
            .map(|(i, _)| w.uh(i))
            .unwrap()
    }

    #[test]
    fn opening_benchmark_and_normal_recruitment_are_people_not_sprites() {
        let mut w = World::new(42);
        assert_eq!(
            [MINAMOTO, HOJO, DOI]
                .iter()
                .map(|&n| w.army_strength(n).personnel)
                .sum::<u32>(),
            300
        );
        assert_eq!(w.army_strength(OBA).personnel, 3000);
        let h = w.spawn_unit(CHIBA, 20, w.nations[CHIBA].home, 1.0);
        assert_eq!(w.unit(h).unwrap().personnel(), 100);
        assert_eq!(w.unit(h).unwrap().establishment, 100);
    }

    #[test]
    fn eastern_mobilisation_is_thirty_thousand_once() {
        let mut w = World::new(42);
        w.years = 1180.0 + 9.0 / 12.0 - START_AD;
        w.run_history();
        assert_eq!(w.allied_personnel(MINAMOTO), 30_000);
        let units = w.units.len();
        w.run_history();
        assert_eq!(w.units.len(), units);
        assert_eq!(w.allied_personnel(MINAMOTO), 30_000);
    }

    #[test]
    fn casualties_reduce_people_and_firepower_and_training_creates_nobody() {
        let mut w = World::new(42);
        let h = company(&w, CHIBA);
        let max = w.unit(h).unwrap().max_hp();
        w.damage_unit(h.idx as usize, max * 0.5, None, TAIRA);
        assert_eq!(w.unit(h).unwrap().personnel(), 50);
        assert!((w.unit(h).unwrap().fighting_mass() - 0.5).abs() < 0.001);
        w.nations[CHIBA].res = [100000.0; 3];
        w.upgrade_unit(h).unwrap();
        assert_eq!(w.unit(h).unwrap().personnel(), 50);
        assert_eq!(w.unit(h).unwrap().establishment, 100);
    }

    #[test]
    fn replacements_need_equipment_food_and_an_owned_base() {
        let mut w = World::new(42);
        let h = company(&w, CHIBA);
        w.unit_mut(h).unwrap().hp *= 0.5;
        w.nations[CHIBA].res = [0.0; 3];
        w.reinforce_companies(10.0);
        assert_eq!(w.unit(h).unwrap().personnel(), 50);
        w.nations[CHIBA].res = [100.0; 3];
        w.reinforce_companies(10.0);
        assert_eq!(w.unit(h).unwrap().personnel(), 60);
        assert!(w.nations[CHIBA].res[0] < 100.0 && w.nations[CHIBA].res[2] < 100.0);
        w.unit_mut(h).unwrap().pos = v2(0.5, 0.5);
        w.reinforce_companies(10.0);
        assert_eq!(w.unit(h).unwrap().personnel(), 60);
    }

    #[test]
    fn large_formations_cannot_be_trained_for_one_company_price() {
        let mut w = World::new(42);
        w.muster_house(CHIBA, 10_000);
        let h = company(&w, CHIBA);
        w.nations[CHIBA].res = [50.0; 3];
        assert!(w.upgrade_unit(h).is_err());
        assert_eq!(w.unit(h).unwrap().power, 1.0);
    }

    #[test]
    fn levy_keeps_losses_and_never_recruits_a_conquered_house() {
        let mut w = World::new(42);
        let h = company(&w, CHIBA);
        w.unit_mut(h).unwrap().hp *= 0.5;
        let before = w.army_strength(CHIBA);
        w.muster_house(CHIBA, 5000);
        let after = w.army_strength(CHIBA);
        assert_eq!(after.personnel, 5000);
        assert_eq!(
            after.establishment - after.personnel,
            before.establishment - before.personnel
        );
        let total = w.army_strength(TAIRA).personnel + after.personnel;
        w.annex(TAIRA, CHIBA);
        assert_eq!(w.army_strength(TAIRA).personnel, total);
        assert_eq!(w.muster_house(CHIBA, 5000), 0);
        assert_eq!(w.army_strength(CHIBA).personnel, 0);
    }

    #[test]
    fn jokyu_levies_match_source_scale_without_new_seats_or_thousands_of_sprites() {
        let mut w = World::new(42);
        w.years = 1221.0 + 4.0 / 12.0 - START_AD;
        w.run_history();
        assert_eq!(w.allied_personnel(HOJO), 190_000);
        assert_eq!(w.army_strength(COURT).personnel, 23_000);
        assert!(w.units.iter().filter(|u| u.alive).count() < 600);
        assert_eq!(
            w.blds.iter().filter(|b| b.alive && b.kind.city()).count(),
            46
        );
        for n in 0..NATIONS.len() {
            assert!(w.nations[n].soldiers + w.nations[n].q_soldiers <= w.nations[n].mil_cap());
        }
        let before = w.allied_personnel(HOJO);
        w.run_history();
        assert_eq!(w.allied_personnel(HOJO), before);
    }

    #[test]
    fn embarkation_and_hero_revival_do_not_duplicate_people() {
        let mut w = World::new(42);
        let h = company(&w, MINAMOTO);
        let before = w.army_strength(MINAMOTO).personnel;
        let ship = w.spawn_unit(MINAMOTO, 23, w.nations[MINAMOTO].home, 1.0);
        w.unit_mut(h).unwrap().aboard = Some(ship);
        w.unit_mut(ship).unwrap().cargo.push(h);
        assert_eq!(w.army_strength(MINAMOTO).personnel, before);
        let hero = w.hero_of(MINAMOTO).unwrap();
        w.units[hero].hp = 0.0;
        w.kill_unit(hero);
        assert_eq!(w.army_strength(MINAMOTO).personnel, before);
        assert_eq!(w.units[hero].personnel(), 0);
    }

    #[test]
    fn save_preserves_large_formations_losses_and_event_once_state() {
        let mut w = World::new(42);
        w.years = 1221.0 + 4.0 / 12.0 - START_AD;
        w.run_history();
        let h = company(&w, COURT);
        w.unit_mut(h).unwrap().hp *= 0.5;
        let expected = w.army_strength(COURT);
        let mut writer = crate::save::Writer::new();
        w.save_to(&mut writer);
        let mut restored =
            World::load_from(42, &mut crate::save::Reader::new(&writer.buf)).unwrap();
        assert_eq!(restored.army_strength(COURT), expected);
        restored.run_history();
        assert_eq!(restored.army_strength(COURT), expected);
    }

    #[test]
    fn version_two_saves_migrate_to_hundred_person_companies() {
        let mut w = World::new(42);
        for u in &mut w.units {
            if u.alive && u.is_company() {
                u.establishment = 100;
                u.hp = u.max_hp() * 0.5;
            }
        }
        let mut writer = crate::save::Writer::new();
        w.save_to_version(&mut writer, 2);
        let restored =
            World::load_from_version(42, &mut crate::save::Reader::new(&writer.buf), 2).unwrap();
        assert_eq!(restored.army_strength(MINAMOTO).personnel, 150);
        assert_eq!(
            restored.army_strength(MINAMOTO).commanders,
            w.army_strength(MINAMOTO).commanders
        );
    }
}

impl Unit {
    pub fn is_company(&self) -> bool {
        self.hero == NONE && self.kind.military() && !self.kind.naval()
    }

    pub fn formation_mass(&self) -> f32 {
        if self.is_company() {
            self.establishment.max(1) as f32 / COMPANY_MEN as f32
        } else {
            1.0
        }
    }

    /// Effective soldiers, including temporarily unfit losses in the HP abstraction.
    pub fn personnel(&self) -> u32 {
        if !self.alive || !self.is_company() || self.hp <= 0.0 {
            return 0;
        }
        ((self.establishment as f32 * (self.hp / self.max_hp()).clamp(0.0, 1.0)).round() as u32)
            .max(1)
            .min(self.establishment)
    }

    pub fn fighting_mass(&self) -> f32 {
        if self.is_company() {
            self.formation_mass() * (self.hp / self.max_hp()).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }
}

impl World {
    pub fn army_strength(&self, n: usize) -> ArmyStrength {
        let mut a = ArmyStrength::default();
        for u in self
            .units
            .iter()
            .filter(|u| u.alive && u.nation as usize == n)
        {
            if u.hero != NONE {
                a.commanders += 1;
            } else if u.kind.naval() {
                a.ships += 1;
            } else if u.is_company() {
                a.personnel += u.personnel();
                a.establishment += u.establishment;
                a.companies += 1;
            }
        }
        a
    }

    pub fn allied_personnel(&self, n: usize) -> u32 {
        (0..NATIONS.len())
            .filter(|&o| o == n || self.campaign_allied(n, o))
            .map(|o| self.army_strength(o).personnel)
            .sum()
    }

    pub(super) fn initialize_manpower(&mut self) {
        for n in 0..NATIONS.len() {
            // Only the 300-vs-3,000 opening is a numeric historical benchmark.
            // Other houses' opening strengths are explicit scenario estimates.
            let target = match n {
                MINAMOTO | HOJO | DOI => 100,
                OBA => 3_000,
                TAIRA | FUJIWARA => 6_000,
                KISO | TAKEDA | COURT => 1_000,
                _ => 300,
            };
            let hs: Vec<_> = self
                .units
                .iter()
                .enumerate()
                .filter(|(_, u)| u.alive && u.nation as usize == n && u.is_company())
                .map(|(i, _)| i)
                .collect();
            for (k, &i) in hs.iter().enumerate() {
                let size = target / hs.len() as u32 + u32::from(k < target as usize % hs.len());
                self.units[i].establishment = size;
                self.units[i].hp = self.units[i].max_hp();
            }
        }
    }

    /// One dated levy, at existing bases. Does not heal/resize the army every frame.
    /// Returns the number actually recruited; extinct houses never receive a levy.
    pub(super) fn muster_house(&mut self, n: usize, target: u32) -> u32 {
        if !self.nations[n].alive {
            return 0;
        }
        let before = self.army_strength(n).personnel;
        if before >= target {
            return 0;
        }
        let Some(cap) = self.nations[n].capital else {
            return 0;
        };
        let mut hs: Vec<_> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.alive && u.nation as usize == n && u.is_company())
            .map(|(i, _)| i)
            .collect();
        // Large armies are grouped into formations, rather than thousands of new sprites.
        let ships = self.army_strength(n).ships;
        let available = self.nations[n]
            .mil_cap()
            .saturating_sub(ships + self.nations[n].q_soldiers);
        if hs.is_empty() && available == 0 {
            return 0;
        }
        let desired = target
            .div_ceil(1000)
            .min(available)
            .max(hs.len() as u32)
            .max(1);
        while hs.len() < desired as usize && self.army_strength(n).personnel < target {
            let Some(pos) = self.spawn_point(cap) else {
                break;
            };
            let h = self.spawn_unit(n, native_ut(n, UnitKind::Infantry), pos, 1.0);
            // Start empty so adding a formation cannot overshoot a small remaining levy.
            self.units[h.idx as usize].establishment = 0;
            self.units[h.idx as usize].hp = 0.0;
            hs.push(h.idx as usize);
        }
        if hs.is_empty() {
            return 0;
        }
        let mut remaining = target.saturating_sub(self.army_strength(n).personnel);
        let count = hs.len();
        for (k, i) in hs.into_iter().enumerate() {
            let u = &mut self.units[i];
            let add = remaining
                .div_ceil((count - k) as u32)
                .min(MAX_FORMATION_MEN.saturating_sub(u.establishment));
            // Preserve previous losses; only newly levied people add HP.
            u.establishment += add;
            u.hp = (u.hp + u.def().hp * u.power * add as f32 / COMPANY_MEN as f32).min(u.max_hp());
            remaining = remaining.saturating_sub(add);
        }
        let recruited = self.army_strength(n).personnel.saturating_sub(before);
        // Campaign baggage/requisitions: these resources must pay upkeep and replacements.
        self.nations[n].res[0] += recruited as f32 * 0.25;
        self.nations[n].res[2] += recruited as f32 * 0.04;
        self.update_stats();
        recruited
    }

    pub(super) fn muster_campaign(&mut self, n: usize, target: u32, allies: bool) -> u32 {
        let members: Vec<_> = (0..NATIONS.len())
            .filter(|&o| {
                self.nations[o].alive && (o == n || (allies && self.campaign_allied(n, o)))
            })
            .collect();
        let seats = |w: &World, o: usize| {
            w.blds
                .iter()
                .filter(|b| b.alive && b.done() && b.kind.city() && b.nation as usize == o)
                .count() as u32
        };
        let total: u32 = members.iter().map(|&o| seats(self, o)).sum();
        if total == 0 {
            return 0;
        }
        let mut recruited = 0;
        let mut allocated = 0;
        for (k, &o) in members.iter().enumerate() {
            let quota = if k + 1 == members.len() {
                target - allocated
            } else {
                (target as u64 * seats(self, o) as u64 / total as u64) as u32
            };
            allocated += quota;
            recruited += self.muster_house(o, quota);
        }
        recruited
    }

    /// Replacement men need food, equipment and a controlled base. Quality alone creates nobody.
    pub(super) fn reinforce_companies(&mut self, dt: f32) {
        for i in 0..self.units.len() {
            let u = &self.units[i];
            if !u.alive
                || !u.is_company()
                || u.aboard.is_some()
                || u.order != Order::Idle
                || u.cooldown > 0.0
                || u.hp >= u.max_hp()
            {
                continue;
            }
            let n = u.nation as usize;
            let (x, y) = u.pos.tile();
            if !Map::inb(x, y) || self.owner[Map::idx(x, y)] as usize != n {
                continue;
            }
            if !self.blds.iter().any(|b| {
                b.alive
                    && b.done()
                    && b.kind.city()
                    && b.nation as usize == n
                    && b.center().dist(u.pos) <= 12.0
            }) {
                continue;
            }
            let missing = u.establishment as f32 * (1.0 - u.hp / u.max_hp()).max(0.0);
            let men = (u.establishment as f32 * 0.01 * dt)
                .min(missing)
                .min(self.nations[n].res[0] / 0.05)
                .min(self.nations[n].res[2] / 0.01)
                .max(0.0);
            self.nations[n].res[0] -= men * 0.05;
            self.nations[n].res[2] -= men * 0.01;
            let u = &mut self.units[i];
            u.hp = (u.hp + u.def().hp * u.power * men / COMPANY_MEN as f32).min(u.max_hp());
        }
    }
}
