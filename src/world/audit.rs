//! Reproducible historical-outcome audit. This observes the unmodified production simulation.
use super::*;
use std::fmt::Write;
use std::path::Path;

fn output_dir() -> std::path::PathBuf {
    std::env::var_os("KAMAKURA_AUDIT_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/validation/raw"))
}

const CHECKPOINTS: &[(i32, u8)] = &[
    (1179, 12),
    (1180, 11),
    (1183, 8),
    (1184, 2),
    (1185, 4),
    (1187, 3),
    (1189, 10),
    (1199, 2),
    (1203, 10),
    (1205, 7),
    (1213, 6),
    (1219, 2),
    (1221, 4),
    (1221, 5),
    (1221, 7),
];

fn csv(value: impl ToString) -> String {
    format!("\"{}\"", value.to_string().replace('"', "\"\""))
}

fn hero_owner(w: &World, h: u8) -> Option<usize> {
    w.units
        .iter()
        .find(|u| u.alive && u.hero == h)
        .map(|u| u.nation as usize)
}

fn seat_owner(w: &World, name: &str) -> usize {
    w.blds
        .iter()
        .enumerate()
        .find(|(i, b)| b.alive && b.kind.city() && w.city_place(*i) == name)
        .map(|(_, b)| b.nation as usize)
        .expect("fixed historical seat missing")
}

fn kamakura_side(n: usize) -> bool {
    [
        MINAMOTO, HOJO, CHIBA, MIURA, OYAMA, UTSUNOMIYA, ADACHI, DOI, SASAKI, NITTA, ASHIKAGA,
        HIRAGA,
    ]
    .contains(&n)
}

fn counts(w: &World) -> Vec<usize> {
    (0..NATIONS.len())
        .map(|n| {
            w.blds
                .iter()
                .filter(|b| b.alive && b.kind.city() && b.nation as usize == n)
                .count()
        })
        .collect()
}

fn snapshot(
    w: &World,
    seed: u32,
    y: i32,
    m: u8,
    states: &mut String,
    heroes: &mut String,
    checks: &mut String,
) {
    let cities = counts(w);
    for n in 0..NATIONS.len() {
        let army = w.army_strength(n);
        assert!(army.personnel <= army.establishment);
        if !w.nations[n].alive {
            assert_eq!(army.personnel, 0);
        }
        let foes = (0..NATIONS.len())
            .filter(|&o| w.at_war(n, o))
            .map(|o| w.nations[o].name)
            .collect::<Vec<_>>()
            .join(" / ");
        writeln!(
            states,
            "{seed},{y},{m},{n},{},{},{},{},{},{:.3},{},{},{},{},{},{},{},{}",
            csv(w.nations[n].name),
            w.nations[n].alive,
            cities[n],
            w.successor_of(n),
            csv(w.nations[w.successor_of(n)].name),
            w.nations[n].strength,
            w.nations[n].soldiers,
            csv(foes),
            army.personnel,
            army.establishment,
            army.ships,
            army.commanders,
            w.campaign_leader(n).map_or(-1, |leader| leader as i32),
            w.allied_personnel(n)
        )
        .unwrap();
    }
    for (h, d) in HEROES.iter().enumerate() {
        let owner = hero_owner(w, h as u8);
        writeln!(
            heroes,
            "{seed},{y},{m},{h},{},{},{}",
            csv(d.name),
            owner.map_or(-1, |o| o as i32),
            csv(owner.map_or("absent", |o| w.nations[o].name))
        )
        .unwrap();
    }
    let mut add = |key: &str, pass: bool, detail: String| {
        writeln!(checks, "{seed},{y},{m},{},{pass},{}", csv(key), csv(detail)).unwrap();
    };
    let state = |n: usize| {
        format!(
            "alive={} seats={} successor={}",
            w.nations[n].alive,
            cities[n],
            w.nations[w.successor_of(n)].name
        )
    };
    match (y, m) {
        (1179, 12) => add(
            "pre_uprising_minamoto_survives",
            w.nations[MINAMOTO].alive,
            state(MINAMOTO),
        ),
        (1180, 11) => {
            add(
                "1180_kamakura_minamoto_control",
                seat_owner(w, "鎌倉") == MINAMOTO,
                w.nations[seat_owner(w, "鎌倉")].name.into(),
            );
            for n in [HOJO, CHIBA, MIURA] {
                let (a, b) = (w.successor_of(MINAMOTO), w.successor_of(n));
                add(
                    &format!("1180_alliance_{n}"),
                    a == b || (!w.at_war(a, b) && w.truce[a][b] > w.years),
                    format!("{} / {}", w.nations[a].name, w.nations[b].name),
                );
            }
        }
        (1183, 8) => add(
            "1183_taira_leave_rokuhara",
            seat_owner(w, "六波羅") != TAIRA,
            w.nations[seat_owner(w, "六波羅")].name.into(),
        ),
        (1184, 2) => add("1184_kiso_defeated", !w.nations[KISO].alive, state(KISO)),
        (1185, 4) => add("1185_taira_defeated", !w.nations[TAIRA].alive, state(TAIRA)),
        (1187, 3) => add(
            "1187_yoshitsune_with_fujiwara",
            hero_owner(w, 35) == Some(FUJIWARA),
            format!("{:?}", hero_owner(w, 35).map(|o| w.nations[o].name)),
        ),
        (1189, 10) => {
            add(
                "1189_fujiwara_defeated",
                !w.nations[FUJIWARA].alive,
                state(FUJIWARA),
            );
            add(
                "1189_hiraizumi_kamakura_side",
                kamakura_side(seat_owner(w, "平泉")),
                w.nations[seat_owner(w, "平泉")].name.into(),
            );
        }
        (1199, 2) => {
            add(
                "1199_yoritomo_dead",
                hero_owner(w, 0).is_none(),
                format!("{:?}", hero_owner(w, 0)),
            );
            add(
                "1199_kamakura_side_controls_kamakura",
                kamakura_side(seat_owner(w, "鎌倉")),
                w.nations[seat_owner(w, "鎌倉")].name.into(),
            );
        }
        (1203, 10) => add("1203_hiki_defeated", !w.nations[HIKI].alive, state(HIKI)),
        (1205, 7) => add(
            "1205_hatakeyama_defeated",
            !w.nations[HATAKEYAMA].alive,
            state(HATAKEYAMA),
        ),
        (1213, 6) => add("1213_wada_defeated", !w.nations[WADA].alive, state(WADA)),
        (1219, 2) => {
            add(
                "1219_sanetomo_dead",
                hero_owner(w, 44).is_none(),
                format!("{:?}", hero_owner(w, 44)),
            );
            add("1219_hojo_survives", w.nations[HOJO].alive, state(HOJO));
        }
        (1221, 4) => add(
            "1221_court_survives_before_war",
            w.nations[COURT].alive,
            state(COURT),
        ),
        (1221, 5) => {
            add(
                "1221_hojo_exists_at_declaration",
                w.nations[HOJO].alive,
                state(HOJO),
            );
            add(
                "1221_court_hojo_actual_war",
                w.nations[COURT].alive && w.nations[HOJO].alive && w.at_war(COURT, HOJO),
                format!(
                    "court={} hojo={} direct_war={} successor_hojo={}",
                    w.nations[COURT].alive,
                    w.nations[HOJO].alive,
                    w.at_war(COURT, HOJO),
                    w.nations[w.successor_of(HOJO)].name
                ),
            );
        }
        (1221, 7) => {
            let kyoto = seat_owner(w, "平安京");
            add(
                "1221_kyoto_kamakura_side",
                kamakura_side(kyoto),
                w.nations[kyoto].name.into(),
            );
            add(
                "1221_kyoto_minamoto_or_hojo",
                [MINAMOTO, HOJO].contains(&kyoto),
                w.nations[kyoto].name.into(),
            );
        }
        _ => {}
    }
}

fn simulate(seed: u32, dir: &Path) {
    let mut w = World::new(seed);
    w.ai_player = true;
    let mut states="seed,year,month,faction_id,faction,alive,seats,successor_id,successor,strength,soldiers,enemies,personnel,establishment,fleets,commanders,campaign_leader,allied_personnel\n".to_string();
    let mut heroes = "seed,year,month,hero_id,hero,owner_id,owner\n".to_string();
    let mut checks = "seed,year,month,check,pass,observed\n".to_string();
    let mut events =
        "seed,year,month,event,original_actor,actual_actor,actor_matches,skipped\n".to_string();
    let mut changes = "seed,actual_ad,base,previous_owner,new_owner\n".to_string();
    let mut owned = w.blds.iter().map(|b| b.nation).collect::<Vec<_>>();
    let mut cp = 0;
    let mut event_i = 0;
    let limit = ((END_AD - START_AD + 12.0) * SEC_PER_YEAR / DT) as usize;
    for _ in 0..limit {
        w.step();
        while event_i < w.chronicle.len() {
            let event = &w.chronicle[event_i];
            let original = &HISTORY[event_i];
            writeln!(
                events,
                "{seed},{},{},{},{},{},{},{}",
                event.ad,
                event.month,
                csv(event.title),
                csv(w.nations[original.nation].name),
                csv(w.nations[event.nation].name),
                original.nation == event.nation,
                event.skipped
            )
            .unwrap();
            event_i += 1;
        }
        for (i, b) in w.blds.iter().enumerate() {
            if i < owned.len() && b.alive && b.kind.city() && owned[i] != b.nation {
                writeln!(
                    changes,
                    "{seed},{:.6},{},{},{}",
                    START_AD + w.years,
                    csv(w.city_place(i)),
                    csv(w.nations[owned[i] as usize].name),
                    csv(w.nations[b.nation as usize].name)
                )
                .unwrap();
            }
        }
        owned = w.blds.iter().map(|b| b.nation).collect();
        while let Some(&(y, m)) = CHECKPOINTS.get(cp) {
            if START_AD + w.years + 0.0001 < y as f32 + (m - 1) as f32 / 12.0 {
                break;
            }
            snapshot(&w, seed, y, m, &mut states, &mut heroes, &mut checks);
            cp += 1;
        }
        if w.game_over.is_some() {
            break;
        }
    }
    assert_eq!(
        cp,
        CHECKPOINTS.len(),
        "seed={seed}: campaign did not reach final checkpoint"
    );
    assert_eq!(w.chronicle.len(), HISTORY.len());
    assert_eq!(
        w.blds.iter().filter(|b| b.alive && b.kind.city()).count(),
        campaign::BASE_NAMES.len()
    );
    for (suffix, content) in [
        ("factions", states),
        ("heroes", heroes),
        ("checks", checks),
        ("events", events),
        ("conquests", changes),
    ] {
        std::fs::write(dir.join(format!("seed-{seed}-{suffix}.csv")), content).unwrap();
    }
    println!(
        "AUDIT seed={seed} complete AD={:.4}, alive={}, seats={}",
        START_AD + w.years,
        w.nations
            .iter()
            .take(NATIONS.len())
            .filter(|n| n.alive)
            .count(),
        campaign::BASE_NAMES.len()
    );
}

#[test]
#[ignore = "Multi-seed historical validation writes reproducible evidence to docs/validation/raw"]
fn historical_campaign_audit() {
    let dir = output_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let seeds = std::env::var("KAMAKURA_AUDIT_SEEDS")
        .unwrap_or("1,42,2026,12345,8675309".into())
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let revision = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or("unknown".into());
    std::fs::write(dir.join("run.json"),format!("{{\"source_head\":\"{revision}\",\"seeds\":{seeds:?},\"checkpoints\":15,\"factions\":{},\"heroes\":{},\"ai_player\":true,\"dt\":{DT},\"notes\":\"Source HEAD is recorded; record any uncommitted gameplay edits separately\"}}\n",NATIONS.len(),HEROES.len())).unwrap();
    let threads = seeds
        .into_iter()
        .map(|seed| {
            let dir = dir.clone();
            std::thread::spawn(move || simulate(seed, &dir))
        })
        .collect::<Vec<_>>();
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
#[ignore = "Measures the campaign calendar against actual map-space movement"]
fn historical_travel_audit() {
    let dir = output_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let mut w = World::new(12345);
    w.years = 1221.0 + 4.0 / 12.0 - START_AD;
    let position = |name: &str| {
        w.blds
            .iter()
            .enumerate()
            .find(|(i, b)| b.alive && b.kind.city() && w.city_place(*i) == name)
            .unwrap()
            .1
            .center()
    };
    let distance = position("鎌倉").dist(position("平安京"));
    let speed = udef(22).speed;
    let available = (END_AD - (1221.0 + 4.0 / 12.0)) * w.sec_per_year();
    let result=format!("{{\"seed\":12345,\"distance_tiles\":{distance},\"cavalry_speed_tiles_per_second\":{speed},\"unopposed_straight_line_seconds\":{},\"may_to_july_seconds\":{available},\"travel_game_months\":{},\"distance_reachable_tiles\":{},\"notes\":\"Hypothetical army departing Kamakura in May; calendar at May 1221; ignores terrain, combat, pathfinding and transport delays\"}}\n",distance/speed,distance/speed/w.sec_per_year()*12.0,available*speed);
    std::fs::write(dir.join("travel.json"), &result).unwrap();
    println!("{result}");
    assert!(distance.is_finite() && distance > 0.0 && speed > 0.0);
}
