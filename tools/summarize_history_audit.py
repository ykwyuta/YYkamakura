"""Summarize an observational audit without changing game behavior (stdlib only)."""
from pathlib import Path
import argparse
import collections
import csv
import json

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--seeds', default='1,42,2026,12345,8675309')
args = parser.parse_args()
seeds = [int(x) for x in args.seeds.split(',')]
out = ROOT / 'docs' / 'validation'
raw = out / 'raw'
run = json.loads((raw / 'run.json').read_text(encoding='utf-8'))
assert seeds == run['seeds'], 'Requested seeds differ from the recorded run; regenerate the audit first'

def read(seed, kind):
    with (raw / f'seed-{seed}-{kind}.csv').open(encoding='utf-8', newline='') as f:
        return list(csv.DictReader(f))

checks = collections.defaultdict(list)
finals = {}
annexations = collections.defaultdict(list)
actor_changes = []
for seed in seeds:
    states = read(seed, 'factions')
    assert len(states) == 15 * 40
    for row in read(seed, 'checks'):
        checks[row['check']].append(row)
    finals[seed] = [row for row in states if row['year'] == '1221' and row['month'] == '7']
    initial = [row for row in states if row['year'] == '1179' and row['month'] == '12']
    owned = {row['faction']: int(row['seats']) for row in initial}
    for row in read(seed, 'conquests'):
        old, new = row['previous_owner'], row['new_owner']
        owned[old] -= 1
        owned[new] += 1
        assert owned[old] >= 0
        if owned[old] == 0:
            annexations[old].append({'seed': seed, 'ad': float(row['actual_ad']), 'by': new})
    actual = {row['faction']: int(row['seats']) for row in finals[seed]}
    assert owned == actual, f'Conquest trace does not reproduce the final seat counts: {seed}'
    events = read(seed, 'events')
    assert len(events) == 28
    actor_changes.extend(row for row in events if row['actor_matches'] != 'true')

rates = {key: {'passed': sum(row['pass'] == 'true' for row in rows), 'trials': len(rows),
               'observations': rows} for key, rows in checks.items()}
summary = {'seeds': seeds, 'simulation': 'World::step, DT=1/30, ai_player=true, 1179-01 to 1221-07',
           'baseline_game_commit': run['source_head'],
           'checks': rates, 'annexations': annexations, 'changed_event_actors': actor_changes,
           'final_factions': finals}
if (raw / 'travel.json').exists():
    summary['travel'] = json.loads((raw / 'travel.json').read_text(encoding='utf-8'))
(out / 'summary.json').write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding='utf-8')

labels = [
    ('1180_alliance_5', '1180年：頼朝と北条氏の同盟'),
    ('1180_alliance_7', '1180年：頼朝と千葉氏の同盟'),
    ('1180_alliance_6', '1180年：頼朝と三浦氏の同盟'),
    ('1183_taira_leave_rokuhara', '1183年8月：平家が六波羅を失う'),
    ('1184_kiso_defeated', '1184年2月：木曽勢が独立勢力として消滅'),
    ('1185_taira_defeated', '1185年4月：平家が独立勢力として消滅'),
    ('1187_yoshitsune_with_fujiwara', '1187年3月：義経が奥州藤原氏に所属'),
    ('1189_fujiwara_defeated', '1189年10月：奥州藤原氏が独立勢力として消滅'),
    ('1189_hiraizumi_kamakura_side', '1189年10月：鎌倉方が平泉を領有'),
    ('1199_yoritomo_dead', '1199年2月：頼朝が退場'),
    ('1203_hiki_defeated', '1203年10月：比企氏が独立勢力として消滅'),
    ('1205_hatakeyama_defeated', '1205年7月：畠山氏が独立勢力として消滅'),
    ('1213_wada_defeated', '1213年6月：和田氏が独立勢力として消滅'),
    ('1219_sanetomo_dead', '1219年2月：実朝が退場'),
    ('1221_court_survives_before_war', '1221年4月：朝廷が存続'),
    ('1221_court_hojo_actual_war', '1221年5月：朝廷と北条氏が交戦'),
    ('1221_kyoto_kamakura_side', '1221年7月：鎌倉方が平安京を領有'),
]
text = ['# 史実再現の検証：実測表', '',
        f'対象はゲーム挙動のコミット `{run["source_head"][:7]}`。今回の検証では監査用コードだけを追加し、ルール・AI・年表の効果・時間・地形は変更していない。', '',
        f'乱数は {", ".join(map(str, seeds))} の{len(seeds)}通り。全勢力AIで全期間を実際に進め、15時点で40勢力と63英雄を記録した。', '',
        '「成立」は表に記した時点の一致を意味する。日付が決まっている人物退場や初期領有による一致を、AIが歴史的展開を再現した証拠には数えない。', '',
        '| 判定項目 | 成立した試行 |', '|---|---:|']
for key, label in labels:
    r = rates[key]
    text.append(f'| {label} | {r["passed"]}/{r["trials"]} |')
text += ['', '## 1221年7月の支配拠点数', '',
         '| 乱数 | 源氏 | 北条 | 平家 | 奥州藤原 | 朝廷 | 存続勢力数 |', '|---|---:|---:|---:|---:|---:|---:|']
for seed, rows in finals.items():
    counts = {int(row['faction_id']): int(row['seats']) for row in rows}
    alive = sum(row['alive'] == 'true' for row in rows)
    text.append(f'| {seed} | {counts[0]} | {counts[5]} | {counts[1]} | {counts[3]} | {counts[19]} | {alive} |')
text += ['', '## 滅亡・併合の実際の時期', '',
         '小数年はゲーム内の年の位置。各記録の元データは `raw/seed-*-conquests.csv`。全拠点の奪取履歴から最終拠点の喪失時点を復元し、最終領有数と一致することも照合した。', '',
         '| 勢力 | 史実の基準年・月 | 実測の最初〜最後 | 実際の併合者（試行数） |', '|---|---|---|---|']
for name, expected in [('木曽源氏', '1184年1月'), ('比企氏', '1203年9月'), ('畠山氏', '1205年6月'), ('和田氏', '1213年5月')]:
    rows = annexations[name]
    years = [row['ad'] for row in rows]
    who = collections.Counter(row['by'] for row in rows)
    text.append(f'| {name} | {expected} | {min(years):.3f}〜{max(years):.3f}年 | ' + '、'.join(f'{n} {c}' for n, c in who.items()) + ' |')
text += ['', '## 実行主体が変わったイベント', '']
for row in actor_changes:
    text.append(f'- 乱数{row["seed"]}：{row["year"]}年{row["month"]}月「{row["event"]}」は{row["original_actor"]}から{row["actual_actor"]}へ置き換わった。')
if not actor_changes:
    text.append('なし。')
if 'travel' in summary:
    t = summary['travel']
    text += ['', '## 移動時間の下限', '',
             f'初期地図の鎌倉〜平安京は直線で約{t["distance_tiles"]:.1f}マス。騎馬武者の速度は{t["cavalry_speed_tiles_per_second"]:.1f}マス/秒なので、地形・戦闘・経路探索を無視しても約{t["unopposed_straight_line_seconds"]:.1f}秒（約{t["travel_game_months"]:.1f}か月）が必要。', '',
             f'1221年5月の開戦から7月の終了までは約{t["may_to_july_seconds"]:.1f}秒で、到達できるのは約{t["distance_reachable_tiles"]:.1f}マス。これは5月に鎌倉を出発する軍の比較であり、既に前進配置された軍や英雄の復活による移動まで含めた到達不可能性の証明ではない。']
text += ['', '## 注意', '',
         '- この検証の成功は「史実に一致した」という意味ではなく、シミュレーションを最後まで実行し証拠を出力できたという意味。',
         '- 5試行は挙動確認用の標本で、全乱数に対する確率推定ではない。',
         '- 「鎌倉方」は源氏・北条・千葉・三浦・小山・宇都宮・安達・土肥・佐々木・新田・足利・平賀のいずれかが実際に領有する場合と定義した。主従制度の実装がないため、この分類自体は検証用の整理。',
         '- 同盟は現在のゲームでは休戦。共同で対外戦をする仕組みの成立を保証しない。',
         '- 朝廷の負けを平安京の攻略で判定するのはゲーム内の代理指標。史実の朝廷制度そのものの消滅を意味しない。', '']
(out / 'measurements.md').write_text('\n'.join(text), encoding='utf-8')
print(f'Wrote {out / "measurements.md"}; {len(seeds)} seeds, {len(rates)} checks per seed')
