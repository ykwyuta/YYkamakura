//! 国ごとの史実に基づく特徴

use super::*;

/// 特徴による補正（1.0＝補正なし）
#[derive(Clone, Copy, Debug)]
pub struct TraitMods {
    /// 生産（城邑の税・採集）
    pub eco: f32,
    /// 兵の大分類ごとの攻撃（民・歩兵・弓兵・騎兵・船）
    pub atk: [f32; 5],
    /// 兵の大分類ごとの訓練費
    pub cost: [f32; 5],
    /// 軍の維持費
    pub upkeep: f32,
    /// 城邑・建物が受ける損害
    pub def: f32,
    /// 商人の集まりやすさ
    pub attract: f32,
    /// 市場の値
    pub market: f32,
    /// 部族の略奪で失う蓄え
    pub plunder: f32,
    /// 鍛錬・教化の費用
    pub upgrade: f32,
}

impl TraitMods {
    pub const NONE: TraitMods = TraitMods {
        eco: 1.0,
        atk: [1.0; 5],
        cost: [1.0; 5],
        upkeep: 1.0,
        def: 1.0,
        attract: 1.0,
        market: 1.0,
        plunder: 1.0,
        upgrade: 1.0,
    };
}

pub struct Trait {
    pub nation: usize,
    pub name: &'static str,
    pub desc: &'static str,
    /// この年（西暦）から有効
    pub from_ad: i32,
    /// この年（西暦）まで有効
    pub to_ad: i32,
    pub fx: fn(&mut TraitMods),
}

const ALL: (i32, i32) = (1115, 1294);
const INF: usize = 1;
const ARC: usize = 2;
const CAVK: usize = 3;

macro_rules! tr {
    ($n:expr, $name:expr, $desc:expr, $span:expr, $f:expr) => {
        Trait {
            nation: $n,
            name: $name,
            desc: $desc,
            from_ad: $span.0,
            to_ad: $span.1,
            fx: $f,
        }
    };
}

fn horse(m: &mut TraitMods) {
    m.atk[CAVK] *= 1.2;
    m.cost[CAVK] *= 0.8;
}

pub const TRAITS: &[Trait] = &[
    tr!(
        MINAMOTO,
        "御家人の結束",
        "東国武士の軍役。騎兵の維持費を軽減",
        (1179, 1221),
        |m| m.upkeep *= 0.85
    ),
    tr!(
        TAIRA,
        "西国の海路",
        "瀬戸内海の海運。船の攻撃と生産を強化",
        (1179, 1221),
        |m| {
            m.atk[4] *= 1.2;
            m.eco *= 1.15;
        }
    ),
    tr!(
        FUJIWARA,
        "奥州の産金",
        "平泉の富。生産を強化",
        (1179, 1221),
        |m| m.eco *= 1.2
    ),
    tr!(
        COURT,
        "朝廷の権威",
        "承久の乱発生まで最後の拠点を失わない",
        (1179, 1221),
        |_| {}
    ),
];

/// その年に有効な特徴
pub fn traits_of(n: usize, ad: i32) -> impl Iterator<Item = &'static Trait> {
    TRAITS
        .iter()
        .filter(move |t| t.nation == n && ad >= t.from_ad && ad <= t.to_ad)
}

/// 年代を問わずその国の特徴すべて
pub fn all_traits_of(n: usize) -> impl Iterator<Item = &'static Trait> {
    TRAITS.iter().filter(move |t| t.nation == n)
}

impl World {
    /// 特徴の補正を計算し直す（毎年）
    pub(super) fn refresh_traits(&mut self) {
        let ad = self.year_ad();
        for i in 0..NATIONS.len() {
            let mut m = TraitMods::NONE;
            for t in traits_of(i, ad) {
                (t.fx)(&mut m);
            }
            self.nations[i].tm = m;
        }
    }

    /// アッバース朝は1258年（バグダードの陥落の年）まで、モンゴルは1206年（チンギス・カンの即位）までは滅びない
    pub fn immortal(&self, n: usize) -> bool {
        n == COURT && START_AD + self.years < 1221.0 + 4.0 / 12.0
    }

    /// その国の兵種の訓練費
    pub fn train_cost(&self, n: usize, ut: u8) -> [f32; 3] {
        let d = udef(ut);
        let m = if n < NATIONS.len() {
            self.nations[n].tm.cost[d.class as usize]
        } else {
            1.0
        };
        let c = d.cost;
        [(c[0] * m).round(), (c[1] * m).round(), (c[2] * m).round()]
    }

    /// 市場の値
    pub fn market_price(&self, n: usize, k: usize) -> f32 {
        (MARKET_PRICE[k] * self.nations[n].tm.market).round()
    }
}
