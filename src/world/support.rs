//! Political and cultural heroes support their current house without attacking.
use super::*;

pub struct HeroSupport {
    pub hero: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub eco: f32,
    pub upkeep: f32,
    pub defense: f32,
    pub attack: f32,
}

pub const HERO_SUPPORT: &[HeroSupport] = &[
    HeroSupport {
        hero: 54,
        label: "統率",
        description: "生産+5%・軍の攻撃+8%・維持費-10%",
        eco: 1.05,
        upkeep: 0.90,
        defense: 1.0,
        attack: 1.08,
    },
    HeroSupport {
        hero: 55,
        label: "士気",
        description: "軍の攻撃+8%・維持費-5%（本人は非戦闘）",
        eco: 1.0,
        upkeep: 0.95,
        defense: 1.0,
        attack: 1.08,
    },
    HeroSupport {
        hero: 57,
        label: "政務",
        description: "生産+12%・施設の被害-5%",
        eco: 1.12,
        upkeep: 1.0,
        defense: 0.95,
        attack: 1.0,
    },
    HeroSupport {
        hero: 58,
        label: "後援",
        description: "生産+10%・維持費-5%",
        eco: 1.10,
        upkeep: 0.95,
        defense: 1.0,
        attack: 1.0,
    },
    HeroSupport {
        hero: 59,
        label: "結束",
        description: "施設の被害-10%・維持費-5%",
        eco: 1.0,
        upkeep: 0.95,
        defense: 0.90,
        attack: 1.0,
    },
    HeroSupport {
        hero: 60,
        label: "文化",
        description: "生産+8%（本人は非戦闘）",
        eco: 1.08,
        upkeep: 1.0,
        defense: 1.0,
        attack: 1.0,
    },
    HeroSupport {
        hero: 61,
        label: "政務",
        description: "生産+10%",
        eco: 1.10,
        upkeep: 1.0,
        defense: 1.0,
        attack: 1.0,
    },
    HeroSupport {
        hero: 62,
        label: "後援",
        description: "生産+8%・維持費-4%",
        eco: 1.08,
        upkeep: 0.96,
        defense: 1.0,
        attack: 1.0,
    },
];

pub fn support_of(hero: usize) -> Option<&'static HeroSupport> {
    HERO_SUPPORT.iter().find(|s| s.hero as usize == hero)
}

impl World {
    pub(super) fn apply_hero_support(&self, n: usize, m: &mut TraitMods) {
        for u in self
            .units
            .iter()
            .filter(|u| u.alive && u.nation as usize == n)
        {
            if let Some(s) = support_of(u.hero as usize) {
                m.eco *= s.eco;
                m.upkeep *= s.upkeep;
                m.def *= s.defense;
                for k in 1..5 {
                    m.atk[k] *= s.attack;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn women_have_distinct_combat_political_and_cultural_roles() {
        assert_eq!(HEROES[54].name, "北条政子");
        assert_eq!(HEROES[55].name, "静御前");
        assert_eq!(HEROES[56].name, "巴御前");
        assert_eq!(HEROES[56].kind(), UnitKind::Cavalry);
        assert!(support_of(56).is_none());
        for h in [54, 55, 57, 58, 59, 60, 61, 62] {
            assert_eq!(udef(HEROES[h].ut).atk, 0.0);
            assert!(support_of(h).is_some());
        }
    }

    #[test]
    fn masako_support_is_real_and_follows_annexation_and_revival() {
        let mut w = World::new(12345);
        w.refresh_traits();
        assert!(w.nations[HOJO].tm.atk[3] > 1.0);
        let h = w
            .units
            .iter()
            .position(|u| u.alive && u.hero == 54)
            .unwrap();
        w.kill_unit(h);
        w.refresh_traits();
        assert!(w.units[h].alive);
        w.annex(CHIBA, HOJO);
        w.refresh_traits();
        assert_eq!(w.units[h].nation as usize, CHIBA);
        assert!(w.nations[CHIBA].tm.atk[3] > 1.0);
        assert_eq!(w.nations[HOJO].tm.atk[3], 1.0);
    }

    #[test]
    fn shizuka_appears_in_1186_and_dance_is_recorded_once() {
        let mut w = World::new(12345);
        assert!(!w.hero_on_field(55));
        w.hist_skip_to(1186.0 - START_AD + 2.0 / 12.0);
        assert!(w.hero_on_field(55));
        w.refresh_traits();
        assert!(w.nations[MINAMOTO].tm.atk[2] > 1.0);
        w.hist_skip_to(1186.0 - START_AD + 3.0 / 12.0);
        assert_eq!(
            w.chronicle
                .iter()
                .filter(|e| e.title == "静御前の舞")
                .count(),
            1
        );
        w.run_history();
        assert_eq!(
            w.chronicle
                .iter()
                .filter(|e| e.title == "静御前の舞")
                .count(),
            1
        );
    }

    #[test]
    fn tokiko_support_ends_with_historical_death() {
        let mut w = World::new(12345);
        w.refresh_traits();
        assert!(w.nations[TAIRA].tm.def < 1.0);
        w.hist_skip_to(1185.0 - START_AD + 2.0 / 12.0);
        w.refresh_traits();
        assert!(!w.hero_on_field(59));
        assert_eq!(w.nations[TAIRA].tm.def, 1.0);
    }
}
