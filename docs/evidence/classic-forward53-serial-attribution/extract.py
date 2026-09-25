#!/usr/bin/env python3
"""Bounded static extraction; never executes either worker. Python 3, GNU binutils."""
import hashlib
import json
import re
import struct
import subprocess
import sys
from pathlib import Path

NAMES = {
    'worker_encode': 'classic_compare_worker::encode',
    'facade': 'emuella_j2k_core::scalable_lossless::encode_with_limits',
    'cleanup': 'emuella_j2k_tier1::cleanup_pass_encode::<false>',
    'significance': 'emuella_j2k_tier1::significance_propagation_pass_encode::<false>',
    'magnitude': 'emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>',
    'mq_bit': '<emuella_j2k_tier1::mq::Encoder>::write_bit',
    'mq_byte': '<emuella_j2k_tier1::mq::Encoder>::byte_out',
    'writer': 'emuella_j2k_codestream::scalable_lossless::encode_lossless_d2',
    'components': 'emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}',
    'prepare': 'emuella_j2k_codestream::scalable_lossless::prepare_forward53',
    'levels': 'emuella_j2k_codestream::forward_reversible_5_3_levels_with_scratch',
    'bounded53': 'emuella_j2k_transform::forward_reversible_5_3_bounded',
    'low_line': 'emuella_j2k_transform::transform_line_forward_first_low_bounded',
    'high_line': 'emuella_j2k_transform::transform_line_forward_first_high_bounded',
    'subband': 'emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>',
    'tier1_strided': 'emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch',
    'tier1_prepared': 'emuella_j2k_tier1::encode_prepared_baseline_code_block',
    'packet_header': 'emuella_j2k_codestream::write_component_packet_header',
    'main_header': 'emuella_j2k_codestream::write_native_main_header',
    'reserve_output': 'emuella_j2k_codestream::scalable_lossless::reserve_output',
}


def run(*args):
    return subprocess.check_output(args, text=True)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def extract(binary, arm, output):
    data = binary.read_bytes()
    assert data[:6] == b'\x7fELF\x02\x01', 'requires little-endian ELF64'
    header = struct.unpack_from('<16sHHIQQQIHHHHHH', data)
    sections = [struct.unpack_from('<IIQQQQIIQQ', data, header[6] + i * header[11])
                for i in range(header[12])]
    strings = sections[header[13]]
    names = data[strings[4]:strings[4] + strings[5]]
    section_map = {}
    for section in sections:
        name = names[section[0]:].split(b'\0', 1)[0].decode()
        section_map[name] = section
    symbols = {}
    for line in run('nm', '-S', '-C', '--defined-only', str(binary)).splitlines():
        match = re.fullmatch(r'([0-9a-f]+) ([0-9a-f]+) [tT] (.+)', line)
        if match:
            symbols[match[3]] = (int(match[1], 16), int(match[2], 16))
    relocations = {}
    for line in run('readelf', '-rW', str(binary)).splitlines():
        match = re.match(r'([0-9a-f]+)\s+\S+\s+R_X86_64_RELATIVE\s+([0-9a-f]+)$', line)
        if match:
            relocations[int(match[1], 16)] = int(match[2], 16)
    owned_addresses = {a: n for n, (a, _) in symbols.items()
                       if n.startswith(('emuella_', '<emuella_', 'classic_compare_worker::'))}
    result = {'whole_sha256': sha(data), 'file_bytes': len(data), 'sections': {}, 'functions': {}}
    for name in ('.text', '.rodata', '.data.rel.ro', '.eh_frame', '.gcc_except_table', '.debug_line', '.debug_info'):
        section = section_map[name]
        result['sections'][name] = {'address': hex(section[3]), 'bytes': section[5],
                                    'sha256': sha(data[section[4]:section[4] + section[5]])}
    text = section_map['.text']
    for key, name in NAMES.items():
        if name not in symbols:
            assert arm == 'baseline' and key == 'prepare', (arm, key)
            result['functions'][key] = None
            continue
        address, size = symbols[name]
        raw = run('objdump', '-d', '-C', '--insn-width=16',
                  f'--start-address={address}', f'--stop-address={address + size}', str(binary))
        # Remove only the input filename, retaining every address and instruction byte.
        raw = '\n'.join(line for line in raw.splitlines() if 'file format' not in line).strip() + '\n'
        (output / f'{arm}-{key}.asm.txt').write_text(raw)
        instructions = []
        decoded_bytes = bytearray()
        for line in raw.splitlines():
            match = re.match(r'\s*([0-9a-f]+):\s+((?:[0-9a-f]{2} )+)\s+(.+)', line)
            if match:
                assert int(match[1], 16) == address + len(decoded_bytes)
                decoded_bytes.extend(bytes.fromhex(match[2]))
                instructions.append((int(match[1], 16), match[3]))
        # This comparison intentionally does not establish data-target identity:
        # mask RIP displacements/comments and label instruction-index branch targets.
        indices = {addr: i for i, (addr, _) in enumerate(instructions)}
        normal = []
        for _, instruction in instructions:
            instruction = re.sub(r'[-]?(?:0x)?[0-9a-f]+\(%rip\)', 'RIP_DATA(%rip)', instruction)
            instruction = instruction.split(' # ', 1)[0].rstrip()
            match = re.search(r'\b([0-9a-f]+) <(.+)>', instruction)
            if match:
                destination = int(match[1], 16)
                target = f'I{indices[destination]}' if destination in indices else match[2]
                instruction = instruction[:match.start()] + '<' + target + '>'
            normal.append(re.sub(r'\s+', ' ', instruction))
        normal_text = '\n'.join(normal) + '\n'
        (output / f'{arm}-{key}.normalised.txt').write_text(normal_text)
        offset = text[4] + address - text[3]
        assert bytes(decoded_bytes) == data[offset:offset + size], (arm, key)
        result['functions'][key] = {
            'symbol': name, 'address': hex(address), 'bytes': size,
            'address_mod_64': address % 64,
            'sha256': sha(data[offset:offset + size]),
            'normalised_sha256': sha(normal_text.encode()),
            'instructions': len(instructions),
            'rsp_operand_instructions': sum('(%rsp)' in s for _, s in instructions),
            'stack_subtractions': [s for _, s in instructions if re.match(r'sub\s+\$0x[0-9a-f]+,%rsp$', s)],
            'own_indirect_call_targets': {
                hex(a): owned_addresses[relocations[int(m[1], 16)]]
                for a, instruction in instructions
                if instruction.startswith('call')
                and (m := re.search(r'# ([0-9a-f]+)', instruction))
                and int(m[1], 16) in relocations
                and relocations[int(m[1], 16)] in owned_addresses},
            'calls': [{'address': hex(a), 'instruction': s} for a, s in instructions if s.startswith('call')],
        }
    return result


if __name__ == '__main__':
    if len(sys.argv) != 4:
        raise SystemExit('usage: extract.py BASELINE_ELF CANDIDATE_ELF OUTPUT_DIRECTORY')
    output = Path(sys.argv[3])
    output.mkdir(parents=True, exist_ok=True)
    result = {'schema': 'classic-forward53-serial-attribution/static-v1',
              'tools': {tool: run(tool, '--version').splitlines()[0] for tool in ('nm', 'objdump', 'readelf')},
              'arms': {arm: extract(Path(binary), arm, output)
                       for arm, binary in zip(('baseline', 'candidate'), sys.argv[1:3])}}
    (output / 'static.json').write_text(json.dumps(result, indent=2) + '\n')
