//! Controlled encounters using production movement, attack, collision and capture code.
use super::*;
use std::fmt::Write;

fn arena(seed: u32) -> World {
    let mut w = World::new(seed);
    w.units.clear();
    w.free_units.clear();
    w.blds.clear();
    w.free_blds.clear();
    w.map.terrain.fill(Terrain::Plain);
    w.cost.fill(1.0);
    w.bmap.fill(0);
    w.owner.fill(0);
    w.land_region.fill(0);
    for n in &mut w.nations {
        n.capital = None;
        n.mil_mul = 1.0;
        n.tm = TraitMods::NONE;
    }
    w.declare_war(MINAMOTO, TAIRA, true);
    w
}

fn force(w: &mut World, n: usize, ut: u8, men: u32, count: usize, x: f32, target: V2) {
    for k in 0..count {
        let pos = v2(x, 120.0 + k as f32 - (count - 1) as f32 / 2.0);
        let h = w.spawn_unit(n, ut, pos, 1.0);
        let u = w.unit_mut(h).unwrap();
        u.establishment = men;
        u.hp = u.max_hp();
        w.order_move(&[h], target, true);
    }
}

#[test]
#[ignore = "Controlled tactical balance measurements; writes KAMAKURA_BALANCE_DIR/tactical.csv"]
fn tactical_balance_audit() {
    let dir = std::env::var("KAMAKURA_BALANCE_DIR").unwrap_or("docs/validation/balance-1".into());
    std::fs::create_dir_all(&dir).unwrap();
    let mut csv =
        "seed,case,elapsed,first_loss,winner,left_personnel,right_personnel,captured\n".to_string();
    for seed in [1, 42, 2026, 12345, 8675309] {
        for (name, ut, men, enemy, count, siege) in [
            ("infantry_100_vs_100", 20, 100, 100, 1, false),
            ("infantry_1000_vs_100", 20, 1000, 100, 1, false),
            ("infantry_1000_vs_1000", 20, 1000, 1000, 1, false),
            ("infantry_1000_vs_ten_100", 20, 1000, 100, 10, false),
            ("archers_100_vs_infantry_100", 21, 100, 100, 1, false),
            ("cavalry_100_vs_infantry_100", 22, 100, 100, 1, false),
            ("infantry_10000_vs_capital", 20, 10000, 0, 0, true),
        ] {
            let mut w = arena(seed);
            force(&mut w, MINAMOTO, ut, men, 1, 110.0, v2(120.0, 120.0));
            let city = if siege {
                let h = w.spawn_building(TAIRA, BKind::Capital, 119, 119, true);
                w.nations[TAIRA].capital = Some(h);
                Some(h)
            } else {
                force(&mut w, TAIRA, 20, enemy, count, 116.0, v2(108.0, 120.0));
                None
            };
            let mut elapsed = 0.0;
            let mut first_loss = None;
            let mut captured = false;
            for tick in 0..(120.0 / DT) as usize {
                w.time += DT;
                w.rebuild_grid();
                w.update_buildings(DT);
                w.update_units(DT);
                w.process_paths();
                if tick % 2 == 0 {
                    w.separate(DT * 2.0);
                }
                let left = w.army_strength(MINAMOTO).personnel;
                let right = w.army_strength(TAIRA).personnel;
                elapsed += DT;
                if first_loss.is_none() && (left < men || right < enemy * count as u32) {
                    first_loss = Some(elapsed);
                }
                captured = city
                    .and_then(|h| w.bld(h))
                    .is_some_and(|b| b.nation as usize == MINAMOTO);
                if left == 0 || captured || (!siege && right == 0) {
                    break;
                }
            }
            let (left, right) = (
                w.army_strength(MINAMOTO).personnel,
                w.army_strength(TAIRA).personnel,
            );
            let winner = if captured || (!siege && right == 0) {
                "left"
            } else if left == 0 {
                "right"
            } else {
                "unresolved"
            };
            writeln!(
                csv,
                "{seed},{name},{elapsed:.3},{:.3},{winner},{left},{right},{captured}",
                first_loss.unwrap_or(-1.0)
            )
            .unwrap();
        }
    }
    std::fs::write(std::path::Path::new(&dir).join("tactical.csv"), csv).unwrap();
}
