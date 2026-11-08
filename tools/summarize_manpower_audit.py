"""Aggregate recorded people and alliances, without reconstructing allegiance rules."""
import argparse
import csv
import json
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--dir', type=Path, default=Path(__file__).resolve().parents[1] / 'docs/validation/manpower-1')
out = parser.parse_args().dir
raw = out / 'raw'
run = json.loads((raw / 'run.json').read_text(encoding='utf-8'))
samples = []
for seed in run['seeds']:
    with (raw / f'seed-{seed}-factions.csv').open(encoding='utf-8', newline='') as f:
        states = list(csv.DictReader(f))
    dates = sorted({(int(r['year']), int(r['month'])) for r in states})
    for year, month in dates:
        rows = {int(r['faction_id']): r for r in states if int(r['year']) == year and int(r['month']) == month}
        assert len(rows) == 40
        for r in rows.values():
            assert 0 <= int(r['personnel']) <= int(r['establishment'])
            if r['alive'] == 'false':
                assert int(r['personnel']) == 0
            leader = int(r['campaign_leader'])
            if leader >= 0:
                total = sum(int(x['personnel']) for x in rows.values() if int(x['campaign_leader']) == leader)
                assert total == int(r['allied_personnel']), (seed, year, month, leader)
        bloc = next((rows[n] for n in (5, 0) if int(rows[n]['campaign_leader']) >= 0), None)
        samples.append({
            'seed': seed, 'year': year, 'month': month,
            'national_personnel': sum(int(r['personnel']) for r in rows.values()),
            'national_establishment': sum(int(r['establishment']) for r in rows.values()),
            'land_formations': sum(int(r['soldiers']) - int(r['fleets']) for r in rows.values()),
            'fleets': sum(int(r['fleets']) for r in rows.values()),
            'heroes': sum(int(r['commanders']) for r in rows.values()),
            'kamakura_leader': None if bloc is None else rows[int(bloc['campaign_leader'])]['faction'],
            'kamakura_personnel': 0 if bloc is None else int(bloc['allied_personnel']),
            'court_personnel': int(rows[19]['personnel']),
            'hojo_alive': rows[5]['alive'] == 'true',
        })
assert len(samples) == 15 * len(run['seeds'])
(out / 'manpower.json').write_text(json.dumps({'source_head': run['source_head'], 'samples': samples}, ensure_ascii=False, indent=2), encoding='utf-8')

text = ['# 兵員数の実測', '', f'本番挙動 `{run["source_head"][:7]}`。記録した陣営IDと兵員を用いて集計し、所属規則を集計側で推測していない。', '',
        '英雄・労働者・船団を通常兵に加算せず、乗船中の兵員は通常兵に含める。定員は満員時の人数、現在人数は損耗を反映した値。', '',
        '## 承久の乱開始（1221年5月）', '',
        '| 乱数 | 幕府方の記録上の盟主 | 幕府方の兵員 | 朝廷の兵員 | 全国の陸上部隊数 | 船団数 | 英雄数（別枠） |',
        '|---|---|---:|---:|---:|---:|---:|']
for s in samples:
    if (s['year'], s['month']) == (1221, 5):
        text.append(f'| {s["seed"]} | {s["kamakura_leader"] or "不在"} | {s["kamakura_personnel"]:,} | {s["court_personnel"]:,} | {s["land_formations"]} | {s["fleets"]} | {s["heroes"]} |')
text += ['', '招集目標は幕府方190,000人・朝廷23,000人。表は月境界を越えた本番ステップの後の観測で、既に行った損耗・補充も含む。既存軍が割当を上回る場合は削減せず、滅亡した家の再建もしない。戦場への到着人数を示す表ではない。', '',
         '## 全国の通常兵員の推移', '',
         '| 観測時点 | 最少人数 | 最多人数 | 陸上部隊数の範囲 |', '|---|---:|---:|---|']
for date in sorted({(s['year'], s['month']) for s in samples}):
    ss = [s for s in samples if (s['year'], s['month']) == date]
    text.append(f'| {date[0]}年{date[1]}月 | {min(s["national_personnel"] for s in ss):,} | {max(s["national_personnel"] for s in ss):,} | {min(s["land_formations"] for s in ss)}〜{max(s["land_formations"] for s in ss)} |')
text += ['', '各観測で「兵員≤定員」「滅亡勢力の兵員0」「同じ盟主IDを持つ各家の兵員合計＝ゲームが出力した陣営人数」を照合した。', '',
         '初期状態から史実の人数・全戦役の勝敗を再現できたことを意味しない。各家の招集配分と史料値の扱いは[兵員設計](../../manpower.md)を参照。', '']
(out / 'manpower.md').write_text('\n'.join(text), encoding='utf-8')
print(f'Wrote {out / "manpower.md"}; {len(samples)} checkpoints')
