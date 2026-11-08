//! Kamakura fixed-base campaign. Dates use a twelve-month historical calendar.
use super::*;
pub const HERO_LIVES: &[(usize, f32, f32)] = &[
    (0, 1179.0, 1199.0),
    (1, 1179.0, 1181.0 + 1.0 / 12.0),
    (2, 1179.0, 1184.0),
    (3, 1179.0, 1187.0 + 9.0 / 12.0),
    (4, 1179.0, 1186.0 + 2.0 / 12.0),
    (5, 1179.0, 1215.0),
    (6, 1179.0, 1180.0 + 7.0 / 12.0),
    (7, 1179.0, 1201.0 + 2.0 / 12.0),
    (8, 1179.0, 1183.0 + 11.0 / 12.0),
    (9, 1179.0, 1203.0 + 8.0 / 12.0),
    (10, 1179.0, 1205.0 + 5.0 / 12.0),
    (11, 1179.0, 1213.0 + 4.0 / 12.0),
    (12, 1179.0, 1199.0),
    (13, 1179.0, 1204.0 + 7.0 / 12.0),
    (14, 1179.0, 1183.0),
    (15, 1179.0, f32::INFINITY),
    (16, 1179.0, 1185.0 + 2.0 / 12.0),
    (17, 1179.0, f32::INFINITY),
    (18, 1179.0, f32::INFINITY),
    (19, 1179.0, 1192.0 + 2.0 / 12.0),
    (20, 1179.0, 1181.0),
    (21, 1179.0, 1207.0 + 8.0 / 12.0),
    (22, 1179.0, 1200.0 + 3.0 / 12.0),
    (23, 1179.0, 1191.0 + 6.0 / 12.0),
    (24, 1179.0, 1184.0 + 7.0 / 12.0),
    (25, 1179.0, 1221.0 + 8.0 / 12.0),
    (26, 1179.0, 1181.0 + 1.0 / 12.0),
    (27, 1179.0, f32::INFINITY),
    (28, 1179.0, 1202.0),
    (29, 1179.0, 1199.0 + 2.0 / 12.0),
    (30, 1179.0, 1180.0 + 9.0 / 12.0),
    (31, 1179.0, 1195.0),
    (32, 1179.0, 1198.0 + 10.0 / 12.0),
    (33, 1179.0, 1185.0 + 2.0 / 12.0),
    (1, 1181.0 + 2.0 / 12.0, 1185.0 + 5.0 / 12.0),
    (0, 1180.0 + 9.0 / 12.0, 1189.0 + 3.0 / 12.0),
    (3, 1187.0 + 10.0 / 12.0, 1189.0 + 8.0 / 12.0),
    (5, 1180.0 + 7.0 / 12.0, 1224.0 + 5.0 / 12.0),
    (5, 1202.0, 1242.0),
    (19, 1198.0, 1239.0),
    (6, 1180.0 + 8.0 / 12.0, 1200.0),
    (6, 1200.0, 1239.0),
    (20, 1181.0, 1223.0),
    (0, 1199.0, 1204.0 + 6.0 / 12.0),
    (0, 1203.0 + 8.0 / 12.0, 1219.0),
    (34, 1179.0, 1183.0 + 9.0 / 12.0),
    (35, 1179.0, f32::INFINITY),
    (36, 1179.0, f32::INFINITY),
    (37, 1179.0, f32::INFINITY),
    (38, 1179.0, f32::INFINITY),
    (39, 1179.0, 1207.0),
    (1, 1179.0, 1185.0 + 2.0 / 12.0),
    (1, 1179.0, 1185.0 + 5.0 / 12.0),
    (1, 1179.0, 1179.0 + 7.0 / 12.0),
];
pub const BASE_NAMES: &[(usize, &str, f32, f32)] = &[
    (0, "蛭ヶ小島", 138.95, 35.05),
    (1, "六波羅", 135.78, 34.99),
    (2, "木曽福島", 137.69, 35.85),
    (3, "平泉", 141.11, 38.99),
    (4, "逸見", 138.42, 35.82),
    (5, "北条館", 138.94, 35.08),
    (6, "衣笠", 139.65, 35.23),
    (7, "亥鼻", 140.13, 35.6),
    (8, "大柳館", 140.3, 35.3),
    (9, "比企館", 139.36, 36.04),
    (10, "菅谷館", 139.33, 36.04),
    (11, "和田館", 139.63, 35.19),
    (12, "祇園城", 139.8, 36.31),
    (13, "宇都宮館", 139.89, 36.56),
    (14, "太田", 140.53, 36.54),
    (15, "神田城", 140.01, 36.78),
    (16, "菊之池", 130.81, 32.98),
    (17, "緒方荘", 131.47, 32.97),
    (18, "高祖", 130.25, 33.55),
    (19, "平安京", 135.75, 35.03),
    (20, "風早", 132.77, 33.96),
    (21, "熊谷館", 139.39, 36.15),
    (22, "盛長館", 139.48, 36.07),
    (23, "土肥郷", 139.1, 35.15),
    (24, "佐々木荘", 136.12, 35.13),
    (25, "大里", 138.56, 35.61),
    (26, "鳥坂", 139.43, 38.08),
    (27, "渋谷荘", 139.47, 35.43),
    (28, "新田荘", 139.29, 36.28),
    (29, "足利荘", 139.45, 36.34),
    (30, "大庭御厨", 139.45, 35.37),
    (31, "湯浅", 135.18, 34.03),
    (32, "田辺", 135.38, 33.73),
    (33, "七尾", 136.97, 37.04),
    (0, "鎌倉", 139.55, 35.32),
    (1, "福原", 135.18, 34.68),
    (1, "屋島", 134.1, 34.36),
    (1, "彦島", 130.92, 33.93),
    (3, "胆沢", 141.13, 39.14),
    (3, "多賀国府", 141.0, 38.31),
    (34, "妹尾荘", 133.85, 34.58),
    (35, "桜庭館", 134.48, 34.08),
    (36, "山鹿荘", 130.67, 33.89),
    (37, "秋月荘", 130.69, 33.47),
    (38, "阿蘇神社", 131.12, 32.95),
    (39, "平賀館", 138.49, 36.24),
];

impl World {
    /// A conquered house follows the owner of its surviving original seat.
    pub fn successor_of(&self, n: usize) -> usize {
        let mut next = n;
        for _ in 0..NATIONS.len() {
            if self.nations[next].alive {
                return next;
            }
            let parent = self.successors[next] as usize;
            if parent >= NATIONS.len() {
                return next;
            }
            next = parent;
        }
        n
    }

    pub(super) fn historical_war(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.successor_of(a), self.successor_of(b));
        if a == b {
            return false;
        }
        self.truce[a][b] = 0.0;
        self.truce[b][a] = 0.0;
        // Earlier coalitions must not silently forbid a dated betrayal.
        self.coalitions
            .retain(|c| !(c.members.contains(&a) && c.members.contains(&b)));
        self.declare_war(a, b, true)
    }

    pub(super) fn update_campaign_heroes(&mut self) {
        let ad = START_AD + self.years;
        for (h, &(home, entry, death)) in HERO_LIVES.iter().enumerate() {
            if ad < entry {
                continue;
            }
            let living = self
                .units
                .iter()
                .position(|u| u.alive && u.hero as usize == h);
            if ad >= death {
                if let Some(i) = living {
                    self.units[i].hero = NONE;
                    self.kill_unit(i);
                    self.push_msg(
                        format!("{}：史実の死去時期により退場", HEROES[h].name),
                        [0.8, 0.7, 0.6],
                    );
                }
                continue;
            }
            if living.is_some() {
                continue;
            }
            let n = self.successor_of(home);
            if !self.nations[n].alive {
                continue;
            }
            if let Some(cap) = self.nations[n].capital {
                if let Some(p) = self.spawn_point(cap) {
                    let d = &HEROES[h];
                    let handle = self.spawn_unit(n, d.ut, p, d.mult);
                    self.unit_mut(handle).unwrap().hero = h as u8;
                }
            }
        }
    }

    /// Preserve the hero's handle across defeat, so saves need no separate resurrection queue.
    pub(super) fn revive_campaign_hero(&mut self, i: usize) -> bool {
        let h = self.units[i].hero as usize;
        let Some(&(_, _, death)) = HERO_LIVES.get(h) else {
            return false;
        };
        if START_AD + self.years >= death {
            return false;
        }
        let n = self.units[i].nation as usize;
        let cap = self.nations[n]
            .capital
            .filter(|&c| self.bld(c).map_or(false, |b| b.nation as usize == n))
            .or_else(|| {
                self.blds
                    .iter()
                    .position(|b| b.alive && b.kind.city() && b.nation as usize == n)
                    .map(|b| self.bh(b))
            });
        let Some(p) = cap.and_then(|c| self.spawn_point(c)) else {
            return false;
        };
        let me = self.uh(i);
        if let Some(ship) = self.units[i].aboard.take() {
            if let Some(s) = self.unit_mut(ship) {
                s.cargo.retain(|&u| u != me);
            }
        }
        let u = &mut self.units[i];
        u.pos = p;
        u.hp = u.max_hp();
        u.order = Order::Idle;
        u.path.clear();
        u.build_queue.clear();
        u.last_attacker = None;
        u.cooldown = 8.0;
        self.push_msg(format!("{}が拠点で復活", HEROES[h].name), [0.8, 0.9, 0.65]);
        true
    }

    pub(super) fn campaign_finished(&mut self) {
        if START_AD + self.years < END_AD || self.game_over.is_some() {
            return;
        }
        self.update_stats();
        let p = self.player;
        let mine = self
            .blds
            .iter()
            .filter(|b| b.alive && b.kind.city() && b.nation as usize == p)
            .count();
        let best = (0..NATIONS.len())
            .map(|n| {
                self.blds
                    .iter()
                    .filter(|b| b.alive && b.kind.city() && b.nation as usize == n)
                    .count()
            })
            .max()
            .unwrap_or(0);
        self.game_over = Some((
            mine == best && mine > 0,
            format!(
                "承久の乱終結 — {}の支配拠点：{}。全国最多：{}。",
                self.nations[p].name, mine, best
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::{Reader, Writer};

    fn date(w: &mut World, year: u32, month: u32) {
        w.hist_skip_to(year as f32 - START_AD + (month - 1) as f32 / 12.0);
    }
    fn cities(w: &World) -> usize {
        w.blds.iter().filter(|b| b.alive && b.kind.city()).count()
    }

    #[test]
    fn all_fixed_seats_and_native_heroes_exist() {
        let w = World::new(12345);
        assert_eq!(cities(&w), BASE_NAMES.len());
        for n in 0..NATIONS.len() {
            assert!(
                w.nations[n].capital.is_some(),
                "{} has no seat",
                NATIONS[n].name
            );
            assert!(w.hero_of(n).is_some(), "{} has no hero", NATIONS[n].name);
        }
    }

    #[test]
    fn multiple_heroes_and_non_kanto_houses_are_playable() {
        let w = World::new(12345);
        assert!(
            w.units
                .iter()
                .filter(|u| u.alive && u.nation as usize == TAIRA && u.hero != NONE)
                .count()
                >= 4
        );
        for n in [
            FUJIWARA, KISO, JO, SENO, TAGUCHI, YAMAGA, AKIZUKI, ASO, KIKUCHI, KONO, KUMANO,
        ] {
            assert!(w.nations[n].capital.is_some(), "{}", NATIONS[n].name);
            assert!(w.hero_of(n).is_some(), "{}", NATIONS[n].name);
        }
    }

    #[test]
    fn fixed_seats_cannot_be_built_razed_or_demolished() {
        let mut w = World::new(12345);
        let cap = w.nations[TAIRA].capital.unwrap();
        let before = cities(&w);
        for kind in [BKind::Town, BKind::Capital] {
            assert!(w.can_place(TAIRA, kind, 200, 200).is_err());
            assert!(w.place_building(TAIRA, kind, 200, 200).is_err());
        }
        assert!(w.can_raze(cap.idx as usize, TAIRA).is_err());
        assert_eq!(w.raze_city(cap.idx as usize, TAIRA), [0.0; 3]);
        w.remove_building(cap.idx as usize);
        assert_eq!(cities(&w), before);
    }

    #[test]
    fn hero_revives_with_same_identity_until_death_and_never_returns_after() {
        let mut w = World::new(12345);
        let h = w.hero_of(MINAMOTO).unwrap();
        let handle = w.uh(h);
        for _ in 0..3 {
            w.units[h].hp = -1.0;
            w.kill_unit(h);
            assert!(w.unit(handle).is_some());
            assert_eq!(w.units[h].hp, w.units[h].max_hp());
        }
        date(&mut w, 1199, 1);
        assert!(w.unit(handle).is_none());
        w.update_campaign_heroes();
        assert!(!w.hero_on_field(0));
    }

    #[test]
    fn capturing_last_seat_annexes_heroes_and_player_and_preserves_base_count() {
        let mut w = World::new(12345);
        w.player = HIKI;
        let hero = w.hero_of(HIKI).unwrap();
        let handle = w.uh(hero);
        let cap = w.nations[HIKI].capital.unwrap();
        let before = cities(&w);
        w.capture_city(cap.idx as usize, HOJO);
        assert!(!w.nations[HIKI].alive);
        assert_eq!(w.player, HOJO);
        assert_eq!(w.unit(handle).unwrap().nation as usize, HOJO);
        assert_eq!(cities(&w), before);
        w.kill_unit(hero);
        assert_eq!(w.unit(handle).unwrap().nation as usize, HOJO);
        assert_eq!(w.successor_of(HIKI), HOJO);
    }

    #[test]
    fn capturing_one_seat_does_not_annex_house_with_another_seat() {
        let mut w = World::new(12345);
        let cap = w.nations[MINAMOTO].capital.unwrap();
        w.capture_city(cap.idx as usize, TAIRA);
        assert!(w.nations[MINAMOTO].alive);
        assert!(w.hero_of(MINAMOTO).is_some());
        assert_ne!(w.nations[MINAMOTO].capital, Some(cap));
    }

    #[test]
    fn court_protection_covers_capture_annex_and_elimination_until_may_1221() {
        let mut w = World::new(12345);
        let c = w.nations[COURT].capital.unwrap().idx as usize;
        date(&mut w, 1221, 4);
        assert!(w.immortal(COURT));
        assert!(w.annex(HOJO, COURT).is_none());
        w.eliminate(COURT, HOJO);
        w.capture_city(c, HOJO);
        assert!(w.nations[COURT].alive);
        assert_eq!(w.blds[c].nation as usize, COURT);
        date(&mut w, 1221, 5);
        assert!(!w.immortal(COURT));
        w.capture_city(c, HOJO);
        assert!(!w.nations[COURT].alive);
    }

    #[test]
    fn dated_alliance_and_betrayal_apply_even_to_the_player() {
        let mut w = World::new(12345);
        w.player = MINAMOTO;
        w.declare_war(MINAMOTO, HOJO, true);
        date(&mut w, 1180, 7);
        assert!(w.at_war(MINAMOTO, HOJO));
        date(&mut w, 1180, 8);
        assert!(!w.at_war(MINAMOTO, HOJO));
        assert!(!w.declare_war(MINAMOTO, HOJO, false));
        date(&mut w, 1183, 10);
        assert!(!w.at_war(MINAMOTO, KISO));
        date(&mut w, 1183, 11);
        assert!(w.at_war(MINAMOTO, KISO));
        let count = w.chronicle.len();
        w.run_history();
        assert_eq!(w.chronicle.len(), count);
    }

    #[test]
    fn yoshitsune_changes_allegiance_and_dies_in_april_1189() {
        let mut w = World::new(12345);
        date(&mut w, 1185, 10);
        let h = w
            .units
            .iter()
            .position(|u| u.alive && u.hero == 35)
            .unwrap();
        assert_eq!(w.units[h].nation as usize, MINAMOTO);
        date(&mut w, 1185, 11);
        assert_eq!(w.units[h].nation as usize, COURT);
        date(&mut w, 1187, 2);
        assert_eq!(w.units[h].nation as usize, FUJIWARA);
        date(&mut w, 1189, 4);
        assert!(!w.hero_on_field(35));
    }

    #[test]
    fn annexation_chain_and_death_state_survive_save_load() {
        let mut w = World::new(42);
        w.annex(HOJO, HIKI);
        w.annex(TAIRA, HOJO);
        date(&mut w, 1181, 3);
        assert_eq!(w.successor_of(HIKI), TAIRA);
        assert!(!w.hero_on_field(1));
        let mut out = Writer::new();
        w.save_to(&mut out);
        let mut restored = World::load_from(42, &mut Reader::new(&out.buf)).unwrap();
        assert_eq!(restored.successor_of(HIKI), TAIRA);
        let events = restored.chronicle.len();
        restored.run_history();
        restored.update_campaign_heroes();
        assert_eq!(events, restored.chronicle.len());
        assert!(!restored.hero_on_field(1));
        assert!(World::load_from(42, &mut Reader::new(&out.buf[..out.buf.len() / 2])).is_err());
    }

    #[test]
    fn campaign_ends_in_july_1221_and_clock_stops() {
        let mut w = World::new(12345);
        date(&mut w, 1221, 6);
        w.campaign_finished();
        assert!(w.game_over.is_none());
        date(&mut w, 1221, 7);
        w.campaign_finished();
        assert!(w.game_over.is_some());
        assert!(w.descent.is_none());
        let years = w.years;
        w.step();
        assert_eq!(w.years, years);
        assert_eq!(w.chronicle.len(), HISTORY.len());
    }

    #[test]
    fn month_table_is_sorted_and_each_hero_has_an_original_house() {
        for es in HISTORY.windows(2) {
            assert!((es[0].ad, es[0].month) <= (es[1].ad, es[1].month));
        }
        assert_eq!(HEROES.len(), HERO_LIVES.len());
        for &(n, entry, death) in HERO_LIVES {
            assert!(n < NATIONS.len());
            assert!(entry < death);
        }
    }

    /// Full production simulation through 1221; bounded even when the end screen is reached.
    #[test]
    fn long_sim() {
        let mut w = World::new(12345);
        w.ai_player = true;
        let before = cities(&w);
        let max_steps = ((END_AD - START_AD + 1.0) * SEC_PER_YEAR / DT) as usize;
        for _ in 0..max_steps {
            if w.game_over.is_some() {
                break;
            }
            w.step();
        }
        assert!(
            w.game_over.is_some(),
            "campaign failed to finish at {}",
            w.years
        );
        assert!(w.descent.is_none());
        assert_eq!(cities(&w), before);
        assert_eq!(w.chronicle.len(), HISTORY.len());
        assert!(w
            .units
            .iter()
            .filter(|u| u.alive)
            .all(|u| w.nations[u.nation as usize].alive));
    }
}
