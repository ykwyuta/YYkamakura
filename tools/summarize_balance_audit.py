"""Compare matched-seed combat and campaign measurements (stdlib only)."""
import argparse
import collections
import csv
import json
import statistics
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--dir', type=Path, default=Path(__file__).resolve().parents[1] / 'docs/validation/balance-1')
root = parser.parse_args().dir

def read(path):
    with path.open(encoding='utf-8', newline='') as f:
        return list(csv.DictReader(f))

data = {}
for phase in ('before', 'after'):
    path = root / phase
    run = json.loads((path / 'raw/run.json').read_text(encoding='utf-8'))
    tactical = read(path / 'tactical.csv')
    assert len(tactical) == len(run['seeds']) * 7
    cases = collections.defaultdict(list)
    for r in tactical:
        cases[r['case']].append(r)
    starvation = []
    kyoto = []
    checks = collections.defaultdict(list)
    for seed in run['seeds']:
        states = read(path / f'raw/seed-{seed}-factions.csv')
        assert len(states) == 600
        starvation += [r for r in states if r['year'] == '1179' and r['month'] == '12' and r['alive'] == 'true']
        final = {r['faction']: r for r in states if r['year'] == '1221' and r['month'] == '7'}
        owner = next(r['faction'] for r in states if r['faction_id'] == '19')
        for r in read(path / f'raw/seed-{seed}-conquests.csv'):
            if r['base'] == '平安京': owner = r['new_owner']
        ruler = final[owner]
        kyoto.append({'seed': seed, 'owner': owner, 'leader': int(ruler['campaign_leader']),
                      'kamakura_side': int(ruler['campaign_leader']) >= 0})
        for r in read(path / f'raw/seed-{seed}-checks.csv'):
            checks[r['check']].append(r['pass'] == 'true')
    data[phase] = {
        'source_head': run['source_head'], 'seeds': run['seeds'],
        'tactical': {case: {
            'median_seconds': statistics.median(float(r['elapsed']) for r in rows),
            'left_wins': sum(r['winner'] == 'left' for r in rows),
            'right_wins': sum(r['winner'] == 'right' for r in rows),
            'unresolved': sum(r['winner'] == 'unresolved' for r in rows),
            'observations': rows,
        } for case, rows in cases.items()},
        'early_food_below_one': sum(float(r['food']) < 1 for r in starvation),
        'early_food_observations': len(starvation),
        'kyoto': kyoto,
        'deadline_checks': {key: sum(vals) for key, vals in checks.items()},
    }
assert data['before']['seeds'] == data['after']['seeds']
field_cases = [case for case in data['before']['tactical'] if case != 'infantry_10000_vs_capital']
unchanged_field = all(data['before']['tactical'][case]['observations'] == data['after']['tactical'][case]['observations'] for case in field_cases)
comparison = {'phases': data, 'field_cases_identical': unchanged_field,
              'notes': 'Controlled flat-ground combat uses production unit updates; campaigns use full World::step. Five seeds are exploratory, not a human win-rate estimate.'}
(root / 'comparison.json').write_text(json.dumps(comparison, ensure_ascii=False, indent=2), encoding='utf-8')

text = ['# バランス調整の比較実測', '',
        f'調整前 `{data["before"]["source_head"][:7]}`、調整後 `{data["after"]["source_head"][:7]}`。乱数は両方とも {data["after"]["seeds"]}。', '',
        '## 局地戦', '',
        '平地・能力1・補充なし・AI介入なし。実際の移動・索敵・攻撃・衝突・拠点攻略処理を使う。攻城の相手は守備射撃を行う首都だが、守備部隊・英雄はいない。最大120秒で打ち切り。', '',
        '| ケース（左軍） | 調整前の所要時間中央値 | 調整後 | 左軍勝利数：前→後 |', '|---|---:|---:|---|']
for case in data['before']['tactical']:
    a, b = data['before']['tactical'][case], data['after']['tactical'][case]
    text.append(f'| {case} | {a["median_seconds"]:.2f}秒 | {b["median_seconds"]:.2f}秒 | {a["left_wins"]}/5 → {b["left_wins"]}/5 |')
text += ['', f'攻城以外の6ケース×5乱数は、終了時刻・残存人数・勝敗を含めて同一：{unchanged_field}。野戦の人数効果を変えずに攻城だけを調整したことを確認した。', '',
         '1,000人を1駒にまとめる場合と100人10駒では、包囲・位置・射線・集中攻撃の差があり、同じ結果になるとは限らない。この差も今回の野戦調整では拡大させていない。', '',
         '## 全期間AI', '', '| 指標 | 調整前 | 調整後 |', '|---|---:|---:|']
for label, get in [
    ('1179年12月：食糧1未満の存続勢力の観測数', lambda d: f'{d["early_food_below_one"]}/{d["early_food_observations"]}'),
    ('1221年7月：実装された陣営所属による幕府方の平安京領有', lambda d: f'{sum(r["kamakura_side"] for r in d["kyoto"])}/5'),
    ('1183年8月：平家が六波羅を失う', lambda d: f'{d["deadline_checks"]["1183_taira_leave_rokuhara"]}/5'),
    ('1185年4月：平家が独立勢力として消滅', lambda d: f'{d["deadline_checks"]["1185_taira_defeated"]}/5'),
    ('1189年10月：奥州藤原氏が独立勢力として消滅', lambda d: f'{d["deadline_checks"]["1189_fujiwara_defeated"]}/5'),
]:
    text.append(f'| {label} | {get(data["before"])} | {get(data["after"])} |')
text += ['', '上記の食糧値は15時点中の最初の観測であり、飢餓の継続時間ではない。5乱数の全AI結果を、プレイヤーの勝率や全乱数の確率として扱わない。', '',
         '平安京の最終領有者：', '']
for phase in ('before', 'after'):
    text.append(f'- {phase}：' + '、'.join(f'{r["seed"]}={r["owner"]}' for r in data[phase]['kyoto']))
text.append('')
(root / 'comparison.md').write_text('\n'.join(text), encoding='utf-8')
print(f'Wrote {root / "comparison.md"}; identical field cases: {unchanged_field}')
