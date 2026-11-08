//! 英霊召喚画面と、地図上の英霊の名札

use super::*;

impl Game {
    pub(super) fn render_summon(&mut self) {
        let (w, h) = (self.gfx.width as f32, self.gfx.height as f32);
        let pw = 1120.0f32.min(w - 20.0);
        let ph = 680.0f32.min(h - 30.0);
        let (x0, y0) = ((w - pw) / 2.0, (h - ph) / 2.0);
        self.gfx.fill_rect(0.0, 0.0, w, h, [0.0, 0.0, 0.0, 0.65]);
        self.art
            .draw_stone_panel(&self.gfx, x0, y0, x0 + pw, y0 + ph);
        self.gfx.text_shadow(
            "英雄図鑑 — 63人",
            x0 + 20.0,
            y0 + 10.0,
            300.0,
            38.0,
            Font::Big,
            [1.0, 0.85, 0.5, 1.0],
        );
        self.gfx.text(
            "各勢力に複数の英雄。敗れても史実の死去時期までは拠点で復活",
            x0 + 310.0,
            y0 + 16.0,
            pw - 420.0,
            26.0,
            Font::Small,
            [0.9, 0.86, 0.77, 1.0],
        );
        self.add_button(
            x0 + pw - 90.0,
            y0 + 10.0,
            x0 + pw - 12.0,
            y0 + 44.0,
            "閉じる",
            "",
            0,
            true,
            Act::CloseOverlay,
            "",
        );
        let per = 9;
        let pages = (HEROES.len() + per - 1) / per;
        self.hero_page = self.hero_page.min(pages - 1);
        let cw = (pw - 32.0) / 3.0;
        let rh = (ph - 112.0) / 3.0;
        for (local, (k, d)) in HEROES
            .iter()
            .enumerate()
            .skip(self.hero_page * per)
            .take(per)
            .enumerate()
        {
            let x = x0 + 12.0 + (local % 3) as f32 * cw;
            let y = y0 + 56.0 + (local / 3) as f32 * rh;
            self.art
                .draw_inset_box(&self.gfx, x, y, x + cw - 8.0, y + rh - 8.0);
            let sz = (rh - 16.0).min(140.0);
            if let Some(bmp) = self.art.period_hero(k, false) {
                self.gfx.draw_bitmap(bmp, x + 4.0, y + 4.0, x + sz, y + sz);
            }
            let tx = x + sz + 6.0;
            let tw = cw - sz - 22.0;
            self.gfx.text(
                d.name,
                tx,
                y + 14.0,
                tw,
                27.0,
                Font::Big,
                [1.0, 0.87, 0.58, 1.0],
            );
            self.gfx.text(
                d.era,
                tx,
                y + 46.0,
                tw,
                44.0,
                Font::Small,
                [0.85, 0.84, 0.77, 1.0],
            );
            let owner = self
                .world
                .units
                .iter()
                .find(|u| u.alive && u.hero as usize == k)
                .map(|u| u.nation as usize);
            let (home, entry, death) = HERO_LIVES[k];
            let ad = START_AD + self.world.years;
            let state = if let Some(n) = owner {
                let recovering = self.world.units.iter().find(|u| u.alive && u.hero as usize == k)
                    .map_or(0.0, |u| u.recovery_seconds());
                if recovering > 0.0 {
                    format!("療養中：あと{:.0}秒　{}", recovering.ceil(), self.world.nations[n].name)
                } else { format!("所属：{}", self.world.nations[n].name) }
            } else if ad < entry {
                format!("{}年に登場", entry.floor() as i32)
            } else if ad >= death {
                "史実の死去により退場".into()
            } else {
                "拠点へ帰還中".into()
            };
            self.gfx.text(
                &state,
                tx,
                y + 92.0,
                tw,
                23.0,
                Font::Small,
                [0.7, 0.9, 0.85, 1.0],
            );
            if let Some(role) = support_of(k) {
                self.gfx.text(
                    role.description,
                    tx,
                    y + 116.0,
                    tw,
                    38.0,
                    Font::Small,
                    [0.8, 0.86, 1.0, 1.0],
                );
            }
            self.gfx.text(
                &format!("出自：{}", NATIONS[home].name),
                tx,
                y + rh - 30.0,
                tw,
                23.0,
                Font::Small,
                [0.76, 0.75, 0.69, 1.0],
            );
        }
        self.add_button(
            x0 + 20.0,
            y0 + ph - 46.0,
            x0 + 140.0,
            y0 + ph - 12.0,
            "◀ 前の九人",
            "",
            0,
            self.hero_page > 0,
            Act::HeroPage(-1),
            "",
        );
        self.gfx.text(
            &format!("{} / {}", self.hero_page + 1, pages),
            x0 + pw / 2.0 - 60.0,
            y0 + ph - 44.0,
            120.0,
            28.0,
            Font::NormalCenter,
            [1.0, 0.85, 0.5, 1.0],
        );
        self.add_button(
            x0 + pw - 140.0,
            y0 + ph - 46.0,
            x0 + pw - 20.0,
            y0 + ph - 12.0,
            "次の九人 ▶",
            "",
            0,
            self.hero_page + 1 < pages,
            Act::HeroPage(1),
            "",
        );
    }

    /// 英霊の名札（地図上）
    pub(super) fn render_hero_labels(&mut self) {
        let (w, h) = (self.gfx.width as f32, self.gfx.height as f32);
        let pl = self.world.player;
        let z = self.zoom;
        let mut labels = Vec::new();
        for u in self.world.units.iter() {
            if !u.alive
                || (u.hero as usize) >= HEROES.len()
                || (u.nation as usize != pl && !self.seen(u.pos))
            {
                continue;
            }
            let (sx, sy) = self.world_to_screen(u.pos);
            if sx < -80.0 || sy < -40.0 || sx > w + 80.0 || sy > h + 40.0 {
                continue;
            }
            labels.push((
                sx,
                sy,
                crate::lang::tx(HEROES[u.hero as usize].name),
                self.world.nations[u.nation as usize].color,
            ));
        }
        for (sx, sy, name, c) in labels {
            let ty = sy - (z * 1.3).clamp(14.0, 34.0) - 22.0;
            self.gfx.fill_round(
                sx - 40.0,
                ty,
                sx + 40.0,
                ty + 20.0,
                4.0,
                [0.10, 0.07, 0.02, 0.85],
            );
            self.gfx.stroke_round(
                sx - 40.0,
                ty,
                sx + 40.0,
                ty + 20.0,
                4.0,
                1.4,
                [1.0, 0.82, 0.35, 0.95],
            );
            self.gfx.text(
                name,
                sx - 40.0,
                ty + 1.0,
                80.0,
                18.0,
                Font::NormalCenter,
                [1.0, 0.92, 0.6, 1.0],
            );
            let _ = c;
        }
    }
}
