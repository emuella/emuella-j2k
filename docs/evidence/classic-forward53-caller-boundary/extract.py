#!/usr/bin/env python3
"""Reuse the bounded Emuella extractor for three prospective ordinary ELF arms."""
import importlib.util
import json
from pathlib import Path
import re
import sys

RECIPE = Path(__file__).resolve().parent.parent / 'classic-forward53-serial-attribution' / 'extract.py'
spec = importlib.util.spec_from_file_location('serial_attribution_extract', RECIPE)
recipe = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recipe)
ORIGINAL_NAMES = dict(recipe.NAMES)
OWNING_HELPER = 'emuella_j2k_codestream::scalable_lossless::transform_forward53_parallel::<false>'


def extract(binary, arm, output):
    names = dict(ORIGINAL_NAMES)
    symbols = {
        match[1] for line in recipe.run('nm', '-S', '-C', '--defined-only', str(binary)).splitlines()
        if (match := re.fullmatch(r'[0-9a-f]+ [0-9a-f]+ [tT] (.+)', line))
    }
    absent = {}
    if arm == 'A':
        assert names['prepare'] not in symbols, 'A must not contain panel preparation'
        absent['prepare'] = names.pop('prepare')
    if arm == 'C':
        names['owning_helper'] = OWNING_HELPER
        # Preparation may be inlined into the owning helper. Its absence is
        # recorded, never substituted with a different source instantiation.
        if names['prepare'] not in symbols:
            absent['prepare'] = names.pop('prepare')
    else:
        assert OWNING_HELPER not in symbols, 'only C may contain the owning helper'
        absent['owning_helper'] = OWNING_HELPER
    recipe.NAMES = names
    try:
        result = recipe.extract(binary, arm, output)
    finally:
        recipe.NAMES = ORIGINAL_NAMES
    result['functions'].update({key: None for key in absent})
    result['absent_symbols'] = absent
    if arm == 'C':
        # This one retained indirect component call loads its target into r14.
        # Resolve its ELF slot explicitly rather than treating masked RIP data
        # in the normalised listing as proof of target identity.
        raw = (output / 'C-owning_helper.asm.txt').read_text()
        loads = re.findall(r'^\s*([0-9a-f]+):[^\n]*\bmov\s+[^\n]*,%r14\s+# ([0-9a-f]+)', raw, re.MULTILINE)
        calls = re.findall(r'^\s*([0-9a-f]+):[^\n]*\bcall\s+\*%r14$', raw, re.MULTILINE)
        assert len(loads) == len(calls) == 1, 'owning component call must remain bounded and identifiable'
        load, slot = loads[0]
        matches = re.findall(r'^0*' + slot + r'\s+\S+\s+R_X86_64_RELATIVE\s+([0-9a-f]+)$',
                             recipe.run('readelf', '-rW', str(binary)), re.MULTILINE)
        assert len(matches) == 1
        target = int(matches[0], 16)
        planned = 'emuella_j2k_transform::analysis53::forward_reversible_5_3_planned_bounded'
        assert re.search(r'^0*' + format(target, 'x') + r' [0-9a-f]+ [tT] ' + re.escape(planned) + '$',
                         recipe.run('nm', '-S', '-C', '--defined-only', str(binary)), re.MULTILINE)
        result['owning_component_call'] = {
            'load': hex(int(load, 16)), 'slot': hex(int(slot, 16)),
            'call': hex(int(calls[0], 16)), 'target': hex(target), 'symbol': planned,
        }
    return result


def main(argv):
    if len(argv) != 5:
        raise SystemExit('usage: extract.py A_ELF B_ELF C_ELF OUTPUT_DIRECTORY')
    output = Path(argv[4])
    output.mkdir(parents=True, exist_ok=True)
    result = {
        'schema': 'classic-forward53-caller-boundary/static-v1',
        'reused_extractor_sha256': recipe.sha(RECIPE.read_bytes()),
        'tools': {tool: recipe.run(tool, '--version').splitlines()[0]
                  for tool in ('nm', 'objdump', 'readelf')},
        'arms': {arm: extract(Path(binary), arm, output)
                 for arm, binary in zip(('A', 'B', 'C'), argv[1:4])},
    }
    result['normalised_comparisons'] = {
        name: {
            key: (left['normalised_sha256'] == right['normalised_sha256']
                  if left is not None and right is not None else None)
            for key in result['arms'][reference]['functions']
            for left, right in [(result['arms'][reference]['functions'][key],
                                 result['arms'][candidate]['functions'][key])]
        }
        for name, reference, candidate in [('C/B', 'B', 'C'), ('B/A', 'A', 'B'), ('C/A', 'A', 'C')]
    }
    (output / 'static.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main(sys.argv)
