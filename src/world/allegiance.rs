//! Scenario allegiance: houses retain their seats, troops and heroes while sharing wars.
//! Dates are campaign abstractions, not claims that every oath occurred that month.
use super::*;

impl World {
    /// Original political actors only: conquest does not turn a conqueror into a historical ally.
    pub fn campaign_leader(&self, n: usize) -> Option<usize> {
        if n >= NATIONS.len() || !self.nations[n].alive {
            return None;
        }
        let ad = START_AD + self.years;
        let joined = match n {
            MINAMOTO | HOJO | DOI | SASAKI => ad >= 1180.0 + 7.0 / 12.0,
            CHIBA | MIURA | KAZUSA => ad >= 1180.0 + 8.0 / 12.0,
            HIKI | HATAKEYAMA | WADA | OYAMA | ADACHI | KUMAGAI | SHIBUYA | NITTA | ASHIKAGA
            | UTSUNOMIYA | HIRAGA => ad >= 1180.0 + 9.0 / 12.0,
            OGATA => ad >= 1185.0 + 1.0 / 12.0,
            KUMANO => ad >= 1185.0 + 2.0 / 12.0,
            _ => false,
        };
        let rebel = (n == HIKI && ad >= 1203.0 + 8.0 / 12.0)
            || (n == HATAKEYAMA && ad >= 1205.0 + 5.0 / 12.0)
            || (n == WADA && ad >= 1213.0 + 4.0 / 12.0);
        if !joined || rebel {
            return None;
        }
        let leader = if ad >= 1205.0 + 6.0 / 12.0 && self.nations[HOJO].alive {
            HOJO
        } else {
            MINAMOTO
        };
        self.nations[leader].alive.then_some(leader)
    }

    pub fn campaign_allied(&self, a: usize, b: usize) -> bool {
        a != b
            && self.campaign_leader(a).is_some()
            && self.campaign_leader(a) == self.campaign_leader(b)
    }

    /// Both negotiated and dated historical settlements bind the mobilised houses.
    pub(super) fn settle_campaign_peace(&mut self, a: usize, b: usize, until: f32) {
        let (a_leader, b_leader) = (self.campaign_leader(a), self.campaign_leader(b));
        for x in 0..NATIONS.len() {
            if x != a && !(a_leader.is_some() && self.campaign_leader(x) == a_leader) {
                continue;
            }
            for y in 0..NATIONS.len() {
                if y == b || (b_leader.is_some() && self.campaign_leader(y) == b_leader) {
                    self.war[x][y] = false;
                    self.war[y][x] = false;
                    self.truce[x][y] = self.truce[x][y].max(until);
                    self.truce[y][x] = self.truce[y][x].max(until);
                }
            }
        }
    }

    /// Symmetric collective defence and shared mobilisation; never transfers ownership.
    pub(super) fn update_campaign_allegiance(&mut self) {
        let members: Vec<_> = (0..NATIONS.len())
            .filter(|&n| self.campaign_leader(n).is_some())
            .collect();
        let enemies: Vec<_> = (0..NATIONS.len())
            .filter(|&e| {
                self.nations[e].alive
                    && !members.contains(&e)
                    && members.iter().any(|&n| self.at_war(n, e))
            })
            .collect();
        for &a in &members {
            for &b in &members {
                self.war[a][b] = false;
                self.truce[a][b] = self.truce[a][b].max(self.years + 0.1);
            }
            for &e in &enemies {
                self.declare_war(a, e, true);
            }
        }
    }

    /// Shared strategic objective; tactical movement and combat still resolve normally.
    pub fn campaign_target(&self, n: usize) -> Option<usize> {
        let ad = START_AD + self.years;
        if self.campaign_leader(n).is_some() {
            // Resolve current internal conflicts before continuing an old distant expedition.
            for (target, from) in [
                (COURT, 1221.0 + 4.0 / 12.0),
                (WADA, 1213.0 + 4.0 / 12.0),
                (HATAKEYAMA, 1205.0 + 5.0 / 12.0),
                (HIKI, 1203.0 + 8.0 / 12.0),
                (FUJIWARA, 1189.0 + 3.0 / 12.0),
                (KISO, 1183.0 + 10.0 / 12.0),
                (TAIRA, 1180.0),
            ] {
                if ad >= from && self.nations[target].alive && self.at_war(n, target) {
                    return Some(target);
                }
            }
            None
        } else {
            (n == KISO && self.nations[TAIRA].alive && self.at_war(n, TAIRA)).then_some(TAIRA)
        }
    }

    /// Give short, geographically large campaigns enough real time for marching and battle.
    pub fn campaign_calendar_scale(&self) -> f32 {
        let ad = START_AD + self.years;
        if (1221.0 + 4.0 / 12.0..END_AD).contains(&ad) {
            24.0
        } else if (1189.0 + 6.0 / 12.0..1189.0 + 9.0 / 12.0).contains(&ad) {
            6.0
        } else if (1183.5..1185.0 + 3.0 / 12.0).contains(&ad) {
            3.0
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mobilisation_preserves_houses_and_breaks_at_political_conflicts() {
        let mut w = World::new(42);
        w.years = 1180.0 + 10.0 / 12.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        for n in [HOJO, CHIBA, MIURA, DOI, SASAKI] {
            assert!(w.at_war(n, TAIRA));
            assert!(!w.declare_war(n, MINAMOTO, false));
            assert!(w.nations[n].alive);
        }
        w.make_peace(HOJO, TAIRA, 1.0);
        w.update_campaign_allegiance();
        assert!(!w.at_war(MINAMOTO, TAIRA));
        assert!(!w.at_war(CHIBA, TAIRA));
        w.historical_war(MINAMOTO, TAIRA);
        w.update_campaign_allegiance();
        assert!(w.at_war(MIURA, TAIRA));
        w.years = 1213.0 + 4.0 / 12.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        assert!(w.at_war(MIURA, WADA));
        assert!(!w.at_war(MIURA, HOJO));
        assert_eq!(w.campaign_target(HOJO), Some(WADA));
        assert_eq!(w.campaign_leader(WADA), None);
    }
    #[test]
    fn conquest_does_not_inherit_historical_allegiance() {
        let mut w = World::new(42);
        w.annex(TAIRA, MINAMOTO);
        w.years = 1181.0 - START_AD;
        assert_eq!(w.campaign_leader(HOJO), None);
        assert_eq!(w.campaign_leader(TAIRA), None);
    }

    #[test]
    fn historical_court_settlement_stops_the_entire_coalition_war() {
        let mut w = World::new(42);
        w.years = 1186.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        assert!(w.at_war(MIURA, COURT));
        w.years = 1192.0 + 6.0 / 12.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        for n in [MINAMOTO, HOJO, MIURA, CHIBA] {
            assert!(!w.at_war(n, COURT));
            assert!(w.truce[n][COURT] > w.years);
        }
    }
    #[test]
    fn jokyu_calendar_allows_long_distance_march() {
        let mut w = World::new(42);
        w.years = 1221.0 + 4.0 / 12.0 - START_AD;
        assert!((END_AD - (START_AD + w.years)) * w.sec_per_year() > 180.0);
    }

    #[test]
    fn allegiance_and_calendar_resume_from_save_without_new_state() {
        let mut w = World::new(42);
        w.years = 1221.0 + 4.0 / 12.0 - START_AD;
        w.run_history();
        w.update_campaign_allegiance();
        let mut saved = crate::save::Writer::new();
        w.save_to(&mut saved);
        let restored = World::load_from(42, &mut crate::save::Reader::new(&saved.buf)).unwrap();
        assert_eq!(restored.campaign_leader(MIURA), Some(HOJO));
        assert!(restored.at_war(MIURA, COURT));
        assert_eq!(restored.sec_per_year(), w.sec_per_year());
    }

    #[test]
    fn extinct_actor_is_recorded_as_skipped_without_reassigning_political_power() {
        let mut w = World::new(42);
        w.annex(TAIRA, MINAMOTO);
        w.years = 1199.0 - START_AD;
        w.run_history();
        let event = w
            .chronicle
            .iter()
            .find(|e| e.title == "頼朝死去と合議")
            .unwrap();
        assert_eq!(event.nation, MINAMOTO);
        assert!(event.skipped);
        assert_eq!(w.nations[TAIRA].house, history::INITIAL_HOUSE[TAIRA]);
    }
}
