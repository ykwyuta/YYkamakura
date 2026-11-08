//! 史実イベント：英雄・名将・政治指導者の登場、反乱・簒奪による君主家の交代、
//! 草原の統一（服属）、宋の弱体化とモンゴルの強大化
//!
//! 年表は西暦の年で並べ、その年が来たら発生する。対象国が滅亡（または未成立）していれば省略する。

use super::*;

/// 各国の始まりの君主家。NATIONS と同じ並び
pub const INITIAL_HOUSE: &[&str] = &[
    "源氏（頼朝）",
    "平家",
    "木曽源氏",
    "奥州藤原氏",
    "甲斐源氏",
    "北条氏",
    "三浦氏",
    "千葉氏",
    "上総氏",
    "比企氏",
    "畠山氏",
    "和田氏",
    "小山氏",
    "宇都宮氏",
    "佐竹氏",
    "那須氏",
    "菊池氏",
    "緒方氏",
    "原田氏",
    "朝廷",
    "河野氏",
    "熊谷氏",
    "安達氏",
    "土肥氏",
    "佐々木氏",
    "浅利氏",
    "城氏",
    "渋谷氏",
    "新田氏",
    "足利氏",
    "大庭氏",
    "湯浅氏",
    "熊野水軍",
    "能登平氏",
    "妹尾氏",
    "田口氏",
    "山鹿氏",
    "秋月氏",
    "阿蘇氏",
    "平賀氏",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HKind {
    /// 英雄・名将・名君
    Hero,
    /// 学者・宗教者
    Sage,
    /// 政治指導者（宰相・改革者）
    Minister,
    /// 反乱・内乱
    Revolt,
    /// 簒奪・乗っ取り（君主家の交代）
    Usurp,
    /// 旧君主家の復辟
    Restore,
    /// 大敗・災厄
    Disaster,
}

impl HKind {
    pub fn label(self) -> &'static str {
        match self {
            HKind::Hero => t!("英雄", "Hero", "英雄"),
            HKind::Sage => t!("賢者", "Sage", "贤者"),
            HKind::Minister => t!("政治指導者", "Statesman", "政治家"),
            HKind::Revolt => t!("反乱", "Revolt", "叛乱"),
            HKind::Usurp => t!("簒奪", "Usurpation", "篡位"),
            HKind::Restore => t!("復辟", "Restoration", "复辟"),
            HKind::Disaster => t!("大敗", "Disaster", "大败"),
        }
    }
    /// 国にとって不利な出来事か
    pub fn bad(self) -> bool {
        matches!(self, HKind::Revolt | HKind::Usurp | HKind::Disaster)
    }
    pub fn color(self) -> [f32; 3] {
        if self.bad() {
            [1.0, 0.55, 0.45]
        } else {
            [0.75, 0.9, 1.0]
        }
    }
}

/// イベントの効果（対象は原則としてイベントの国）
#[derive(Clone, Copy, Debug)]
pub enum Fx {
    /// 人物が仕える：生産（税・採集）と兵の攻撃力を一定年数だけ高める（負なら混乱）
    Figure(&'static str, f32, f32, f32),
    /// 人物が去る
    Dismiss(&'static str),
    /// 資源を得る
    Grant(f32, f32, f32),
    /// 全兵の能力を上げる（時代上限まで）
    Train(f32),
    /// 都に戈兵を召集
    Levy(u32),
    /// 兵の一部が離反・戦死する
    Purge(f32),
    /// 蓄えの一部を失う
    Plunder(f32),
    /// 君主家（国姓）が変わる
    House(&'static str),
    /// 指定国を標的とした反〇〇同盟（隣国が結ぶ）
    Hezong(usize),
    /// 自国を標的とする連合を解散させる（連衡）
    Breakup,
    /// a と b が休戦する
    Truce(usize, usize, f32),
    /// a が b に宣戦する
    War(usize, usize),
    /// 生産能力バイアスを掛ける（国の盛衰）
    Bias(f32),
    /// 指定国を服属させる（城邑と兵を併せる）
    Annex(usize),
    /// 指定国が成立する
    Found(usize),
    /// 靖康の変（宋が南宋に）
    Jingkang,
    /// A named hero changes allegiance at a historical date.
    DefectHero(u8, usize),
    /// 神風（日本の近海の敵船が沈む）
    Kamikaze,
}

pub struct HEvent {
    pub ad: i32,
    pub month: u8,
    pub nation: usize,
    pub kind: HKind,
    pub title: &'static str,
    pub text: &'static str,
    pub fx: &'static [Fx],
}

macro_rules! ev {
    ($bc:expr, $month:expr, $n:expr, $k:ident, $t:expr, $x:expr, [$($fx:expr),* $(,)?]) => {
        HEvent { ad: $bc, month: $month, nation: $n, kind: HKind::$k, title: $t, text: $x, fx: &[$($fx),*] }
    };
}

use Fx::*;

/// 史実年表（西暦の昇順）。tools/gendata.py が生成する
pub const HISTORY: &[HEvent] = &[
    ev!(
        1180,
        5,
        0,
        Hero,
        "以仁王の令旨",
        "以仁王が平家追討を呼びかける。東国の武士たちが動き始める。",
        [War(MINAMOTO, TAIRA)]
    ),
    ev!(
        1180,
        8,
        0,
        Hero,
        "頼朝挙兵",
        "頼朝が挙兵。北条・土肥・佐々木が結ぶ。",
        [
            Truce(MINAMOTO, HOJO, 41.0),
            Truce(MINAMOTO, DOI, 41.0),
            Truce(MINAMOTO, SASAKI, 41.0)
        ]
    ),
    ev!(
        1180,
        9,
        0,
        Hero,
        "房総の結集",
        "千葉常胤・上総広常・三浦義澄が頼朝に合流。",
        [
            Truce(MINAMOTO, CHIBA, 41.0),
            Truce(MINAMOTO, KAZUSA, 3.0),
            Truce(MINAMOTO, MIURA, 41.0)
        ]
    ),
    ev!(
        1180,
        10,
        4,
        Hero,
        "富士川と甲斐源氏",
        "甲斐源氏が頼朝と共同して平家に対抗。",
        [Truce(MINAMOTO, TAKEDA, 6.0), War(TAKEDA, TAIRA)]
    ),
    ev!(
        1181,
        1,
        2,
        Hero,
        "横田河原への戦い",
        "木曽義仲と越後の城氏が対立する。",
        [War(KISO, JO)]
    ),
    ev!(
        1183,
        3,
        0,
        Hero,
        "頼朝と義仲の和議",
        "義仲が嫡子義高を鎌倉へ送り、頼朝と和議を結ぶ。",
        [Truce(MINAMOTO, KISO, 0.7)]
    ),
    ev!(
        1183,
        7,
        19,
        Hero,
        "平家都落ち",
        "義仲が入京し、平家は西国へ。朝廷は義仲と接近する。",
        [Truce(COURT, KISO, 0.3), War(KISO, TAIRA)]
    ),
    ev!(
        1183,
        11,
        2,
        Hero,
        "法住寺合戦",
        "義仲が後白河院を幽閉。朝廷との関係が決裂する。",
        [War(KISO, COURT), War(MINAMOTO, KISO)]
    ),
    ev!(
        1183,
        12,
        0,
        Hero,
        "上総広常の誅殺",
        "広常が誅殺され、上総勢は鎌倉に組み込まれる。",
        [Annex(KAZUSA)]
    ),
    ev!(
        1184,
        1,
        0,
        Hero,
        "宇治川・粟津",
        "頼朝軍が義仲軍を破る。義仲の死去は英雄の史実退場として処理。",
        [War(MINAMOTO, KISO)]
    ),
    ev!(
        1185,
        2,
        17,
        Hero,
        "緒方氏の転陣",
        "緒方惟義が平家から源氏方へ転じる。",
        [Truce(OGATA, MINAMOTO, 36.0), War(OGATA, TAIRA)]
    ),
    ev!(
        1185,
        3,
        32,
        Hero,
        "壇ノ浦と熊野水軍",
        "熊野水軍が源氏方に加勢。平家との決戦。拠点の帰属は戦況で決まる。",
        [Truce(KUMANO, MINAMOTO, 36.0), War(KUMANO, TAIRA)]
    ),
    ev!(
        1185,
        11,
        0,
        Hero,
        "頼朝と義経の決裂",
        "後白河院が頼朝追討の院宣を出し、兄弟の関係が決裂する。",
        [War(MINAMOTO, COURT), DefectHero(35, COURT)]
    ),
    ev!(1186, 4, MINAMOTO, Sage, "静御前の舞", "静が鶴岡八幡宮で舞い、政子がその心情に理解を示す。文化による結束をゲーム上の効果として表す。", [Figure("鶴岡八幡宮の舞", 0.05, 0.08, 1.0)]),
    ev!(
        1187,
        2,
        FUJIWARA,
        Hero,
        "義経の平泉入り",
        "義経が秀衡を頼って奥州に入る。",
        [DefectHero(35, FUJIWARA)]
    ),
    ev!(
        1189,
        4,
        3,
        Hero,
        "泰衡の離反・衣川",
        "藤原泰衡が義経を襲撃。義経は史実の時期に退場。",
        [War(FUJIWARA, MINAMOTO)]
    ),
    ev!(
        1189,
        7,
        0,
        Hero,
        "奥州合戦",
        "頼朝が奥州へ進軍。平泉の領有は実際の攻略で決まる。",
        [War(MINAMOTO, FUJIWARA)]
    ),
    ev!(
        1192,
        7,
        0,
        Hero,
        "征夷大将軍",
        "頼朝が征夷大将軍に任じられる。朝廷と鎌倉は和議。",
        [Truce(MINAMOTO, COURT, 28.8), House("源氏将軍家")]
    ),
    ev!(
        1199,
        1,
        0,
        Hero,
        "頼朝死去と合議",
        "頼朝が死去。幕府内部の合議と御家人の競合が始まる。",
        [House("源氏将軍家・宿老合議")]
    ),
    ev!(
        1203,
        9,
        5,
        Hero,
        "比企能員の変",
        "北条氏と比企氏の対立が決裂する。",
        [War(HOJO, HIKI)]
    ),
    ev!(
        1205,
        6,
        5,
        Hero,
        "畠山重忠の乱",
        "北条氏と畠山氏が敵対。重忠は史実の時期に退場する。",
        [War(HOJO, HATAKEYAMA)]
    ),
    ev!(
        1205,
        7,
        5,
        Hero,
        "牧氏事件",
        "北条時政が失脚し、義時が執権となる。",
        [House("北条義時")]
    ),
    ev!(
        1213,
        5,
        5,
        Hero,
        "和田合戦",
        "和田義盛が挙兵し、北条義時と戦う。",
        [War(HOJO, WADA)]
    ),
    ev!(
        1219,
        1,
        0,
        Hero,
        "実朝暗殺",
        "実朝が公暁に暗殺され、源氏将軍が断絶する。",
        [House("北条執権・摂家将軍へ")]
    ),
    ev!(
        1221,
        5,
        19,
        Hero,
        "承久の乱",
        "後鳥羽院が北条義時追討を命じる。朝廷の滅亡禁止が解除される。",
        [War(COURT, HOJO), War(COURT, MINAMOTO)]
    ),
    ev!(1221, 5, HOJO, Minister, "北条政子の訴え", "政子が頼朝以来の恩義を説き、御家人たちの結束を促す。", [Figure("政子の結束", 0.0, 0.15, 0.2)]),
    ev!(
        1221,
        6,
        5,
        Hero,
        "宇治川の決戦",
        "幕府方が京都へ進軍。最終月まで拠点を争う。",
        [War(HOJO, COURT)]
    ),
    ev!(
        1221,
        7,
        19,
        Hero,
        "承久の乱終結",
        "後鳥羽院らの配流。全国の固定拠点の支配で最終結果を判定する。",
        []
    ),
];

/// 在世の人物（効果は年限つき）
pub struct Person {
    pub name: &'static str,
    pub eco: f32,
    pub mil: f32,
    pub until: f32,
}

/// 史書の記録
pub struct Chron {
    pub ad: i32,
    pub month: u8,
    pub nation: usize,
    pub kind: HKind,
    pub title: &'static str,
    pub text: &'static str,
    /// 対象国が滅んでいて起こらなかった
    pub skipped: bool,
}

impl World {
    /// 年が改まるごとに史実イベントを発生させる
    pub(super) fn run_history(&mut self) {
        // 人物の退場
        let years = self.years;
        for n in self.nations.iter_mut() {
            n.figures.retain(|f| f.until > years);
        }
        while self.hist_i < HISTORY.len()
            && START_AD + self.years + 0.0001
                >= HISTORY[self.hist_i].ad as f32 + (HISTORY[self.hist_i].month - 1) as f32 / 12.0
        {
            let e = &HISTORY[self.hist_i];
            self.hist_i += 1;
            self.fire_event(e);
        }
        self.refresh_mods();
    }

    fn fire_event(&mut self, e: &'static HEvent) {
        let n = self.successor_of(e.nation);
        // 新しい国の成立は、その国がまだ無くても起こる
        let founding = e.fx.iter().any(|f| matches!(*f, Fx::Found(x) if x == n));
        let skipped = false;
        let _ = founding;
        self.chronicle.push(Chron {
            ad: e.ad,
            month: e.month,
            nation: n,
            kind: e.kind,
            title: e.title,
            text: e.text,
            skipped,
        });
        if skipped {
            return;
        }
        // チュートリアル中はプレイヤーに不利な出来事を起こさない
        let shield = self.protect_player && n == self.player;
        let mut notes: Vec<String> = Vec::new();
        for fx in e.fx.iter() {
            match *fx {
                DefectHero(h, target) => {
                    let target = self.successor_of(target);
                    if let Some(i) = self.units.iter().position(|u| u.alive && u.hero == h) {
                        if self.nations[target].alive {
                            self.units[i].nation = target as u8;
                            self.units[i].order = Order::Idle;
                            self.units[i].path.clear();
                        }
                    }
                }
                Figure(name, eco, mil, yrs) => {
                    if shield && (eco < 0.0 || mil < 0.0) {
                        continue;
                    }
                    let until = self.years + yrs;
                    let nat = &mut self.nations[n];
                    nat.figures.retain(|f| f.name != name);
                    nat.figures.push(Person {
                        name,
                        eco,
                        mil,
                        until,
                    });
                    let mut s = String::from(tx(name));
                    if eco != 0.0 {
                        s.push_str(&tf!(
                            " 生産{:+.0}%",
                            " production {:+.0}%",
                            " 生产{:+.0}%",
                            eco * 100.0
                        ));
                    }
                    if mil != 0.0 {
                        s.push_str(&tf!(
                            " 武{:+.0}%",
                            " military {:+.0}%",
                            " 武{:+.0}%",
                            mil * 100.0
                        ));
                    }
                    s.push_str(&tf!(
                        "（{}年）",
                        " ({} years)",
                        "（{}年）",
                        yrs.min(END_AD - START_AD) as i32
                    ));
                    notes.push(s);
                }
                Dismiss(name) => {
                    let nat = &mut self.nations[n];
                    if nat.figures.iter().any(|f| f.name == name) {
                        nat.figures.retain(|f| f.name != name);
                        notes.push(tf!("{} が去る", "{} departs", "{}离去", tx(name)));
                    }
                }
                Grant(a, b, c) => {
                    let r = &mut self.nations[n].res;
                    r[0] += a;
                    r[1] += b;
                    r[2] += c;
                    notes.push(tf!(
                        "糧{:.0} 木{:.0} 鉄{:.0}",
                        "food {:.0}, wood {:.0}, iron {:.0}",
                        "粮{:.0} 木{:.0} 铁{:.0}",
                        a,
                        b,
                        c
                    ));
                }
                Train(x) => {
                    let cap = self.power_cap();
                    let mut k = 0;
                    for u in self.units.iter_mut() {
                        if u.alive && u.nation as usize == n && u.kind.military() && u.power < cap {
                            let ratio = u.hp / u.max_hp();
                            u.power = (u.power + x).min(cap);
                            u.hp = ratio * u.max_hp();
                            k += 1;
                        }
                    }
                    if k > 0 {
                        notes.push(tf!(
                            "兵{}人の能力{:+.1}",
                            "{} soldiers' power {:+.1}",
                            "{}名士兵能力{:+.1}",
                            k,
                            x
                        ));
                    }
                }
                Levy(cnt) => {
                    let Some(c) = self.nations[n].capital else {
                        continue;
                    };
                    let pw = (1.0 + self.years / (END_AD - START_AD) * 2.0).min(self.power_cap());
                    let ut = best_land_ut(n);
                    let mut k = 0;
                    for _ in 0..cnt {
                        if let Some(sp) = self.spawn_point(c) {
                            self.spawn_unit(n, ut, sp, pw);
                            k += 1;
                        }
                    }
                    if k > 0 {
                        notes.push(tf!(
                            "{}{}人が馳せ参じる",
                            "{1} {0} rally to the capital",
                            "{}{}名赶来效命",
                            tx(udef(ut).name),
                            k
                        ));
                    }
                }
                Bias(f) => {
                    if shield && f < 1.0 {
                        continue;
                    }
                    let nat = &mut self.nations[n];
                    let old = nat.bias;
                    nat.bias = (nat.bias * f).clamp(0.4, 2.4);
                    notes.push(tf!(
                        "国力 ×{:.2} → ×{:.2}",
                        "national strength ×{:.2} → ×{:.2}",
                        "国力 ×{:.2} → ×{:.2}",
                        old,
                        nat.bias
                    ));
                }
                Annex(t) => {
                    if let Some(s) = self.annex(n, t) {
                        notes.push(s);
                    }
                }
                Found(t) => {
                    if let Some(s) = self.found_nation(t) {
                        notes.push(s);
                    }
                }
                Jingkang => {
                    if let Some(s) = self.jingkang() {
                        notes.push(s);
                    }
                }
                Kamikaze => {
                    if let Some(s) = self.kamikaze() {
                        notes.push(s);
                    }
                }
                Purge(f) => {
                    if shield {
                        continue;
                    }
                    let mut idx: Vec<usize> = (0..self.units.len())
                        .filter(|&i| {
                            self.units[i].alive
                                && self.units[i].nation as usize == n
                                && self.units[i].kind.military()
                        })
                        .collect();
                    // 都から遠い兵ほど離反しやすい
                    let home = self.nations[n].home;
                    idx.sort_by(|&a, &b| {
                        self.units[b]
                            .pos
                            .dist(home)
                            .partial_cmp(&self.units[a].pos.dist(home))
                            .unwrap()
                    });
                    let k = (idx.len() as f32 * f).round() as usize;
                    for &i in idx.iter().take(k) {
                        self.kill_unit(i);
                    }
                    if k > 0 {
                        notes.push(tf!("兵{}人を失う", "{} soldiers lost", "损失{}名士兵", k));
                    }
                }
                Plunder(f) => {
                    if shield {
                        continue;
                    }
                    for r in self.nations[n].res.iter_mut() {
                        *r *= 1.0 - f;
                    }
                    notes.push(tf!(
                        "蓄えの{:.0}%を失う",
                        "{:.0}% of stores lost",
                        "损失{:.0}%的储备",
                        f * 100.0
                    ));
                }
                House(h) => {
                    if self.nations[n].house != h {
                        notes.push(tf!(
                            "君主家：{} → {}",
                            "ruling house: {} → {}",
                            "君主家：{} → {}",
                            tx(self.nations[n].house),
                            tx(h)
                        ));
                        self.nations[n].house = h;
                    }
                }
                Hezong(t) => {
                    if self.nations[t].alive
                        && !self.coalitions.iter().any(|c| c.target == t)
                        && self.start_coalition_ex(CoalKind::Hezong, t, false)
                    {
                        let c = self.coalitions.last().unwrap();
                        notes.push(tf!(
                            "隣国{}か国が反{}同盟を結ぶ",
                            "{} neighbours form the Anti-{} League",
                            "{}个邻国结成反{}同盟",
                            c.members.len(),
                            tx(self.nations[t].name)
                        ));
                    }
                }
                Breakup => {
                    let years = self.years;
                    let mut any = false;
                    for c in self.coalitions.iter_mut() {
                        if c.target == n && c.end > years {
                            c.end = years;
                            any = true;
                        }
                    }
                    if any {
                        notes.push(tf!(
                            "{}を討つ連合が瓦解",
                            "the coalition against {} collapses",
                            "讨伐{}的联军瓦解",
                            tx(self.nations[n].name)
                        ));
                    }
                }
                Truce(a, b, yrs) => {
                    let (a, b) = (self.successor_of(a), self.successor_of(b));
                    if !self.nations[a].alive || !self.nations[b].alive || a == b {
                        continue;
                    }
                    self.war[a][b] = false;
                    self.war[b][a] = false;
                    let t = self.years + yrs;
                    self.truce[a][b] = self.truce[a][b].max(t);
                    self.truce[b][a] = self.truce[b][a].max(t);
                    notes.push(tf!(
                        "{}と{}が{}年の休戦",
                        "{} and {} agree a {}-year truce",
                        "{}与{}休战{}年",
                        tx(self.nations[a].name),
                        tx(self.nations[b].name),
                        yrs as i32
                    ));
                }
                War(a, b) => {
                    if self.historical_war(a, b) {
                        notes.push(tf!(
                            "{}が{}に攻め込む",
                            "{} invades {}",
                            "{}进攻{}",
                            tx(self.nations[a].name),
                            tx(self.nations[b].name)
                        ));
                    }
                }
            }
        }
        self.refresh_mods();
        self.territory_dirty = true;
        let name = tx(self.nations[n].name);
        let head = tf!(
            "【{}・{}】{}",
            "[{} · {}] {}",
            "【{}・{}】{}",
            name,
            e.kind.label(),
            tx(e.title)
        );
        // 本文はバナーと歴史に任せ、記録欄は見出しと効果だけにする
        let mut msg = head;
        if !notes.is_empty() {
            msg.push_str(&tf!(
                "（{}）",
                " ({})",
                "（{}）",
                notes.join(t!("、", "; ", "；"))
            ));
        }
        self.push_msg(msg, e.kind.color());
        // 大きな出来事はバナーで知らせる（小国の出来事は記録のみ）
        let involves_player = n == self.player
            || e.fx.iter().any(|f| match *f {
                Hezong(t) | Annex(t) => t == self.player,
                War(a, b) | Truce(a, b, _) => a == self.player || b == self.player,
                _ => false,
            });
        let annexes_player =
            e.fx.iter()
                .any(|f| matches!(*f, Annex(t) if t == self.player));
        if involves_player || annexes_player || NATIONS[n].major || n == MONGOL {
            let mut text = tx(e.text).to_string();
            if !notes.is_empty() {
                text.push_str(&format!("\n{}", notes.join(t!("　", "; ", "；"))));
            }
            let title = format!("{}　{}", name, tx(e.title));
            self.queue_banner(Banner {
                title,
                text,
                t: 7.0,
            });
        }
    }

    /// 在世の人物の効果を集計する
    pub(super) fn refresh_mods(&mut self) {
        let years = self.years;
        for n in self.nations.iter_mut() {
            let (mut e, mut m) = (1.0f32, 1.0f32);
            for f in n.figures.iter().filter(|f| f.until > years) {
                e += f.eco;
                m += f.mil;
            }
            n.eco_mul = e.clamp(0.4, 3.0);
            n.mil_mul = m.clamp(0.5, 2.5);
        }
    }

    /// 時を進めて、その年までの史実の出来事を起こす（ユニットは動かさない。テスト・撮影用）
    pub fn hist_skip_to(&mut self, years: f32) {
        while self.years < years {
            self.years = (self.years + 1.0).min(years);
            if self.era == Era::Rise && START_AD + self.years >= EMPIRE_AD {
                self.begin_empire();
            }
            self.run_history();
            self.update_campaign_heroes();
            self.refresh_traits();
            if self.territory_dirty {
                self.recompute_territory();
            }
            self.update_stats();
        }
    }

    /// いまの告知を閉じて、次の告知へ
    pub fn dismiss_banner(&mut self) {
        self.banner = self.banner_queue.pop_front();
    }

    /// 待っている告知の数（いま出ているものを除く）
    pub fn banners_waiting(&self) -> usize {
        self.banner_queue.len()
    }

    /// 告知をすべて閉じる（記録欄と歴史には残る）
    pub fn dismiss_all_banners(&mut self) {
        self.banner = None;
        self.banner_queue.clear();
    }

    pub fn queue_banner(&mut self, b: Banner) {
        if self.banner.is_none() {
            self.banner = Some(b);
        } else {
            self.banner_queue.push_back(b);
        }
    }
}
