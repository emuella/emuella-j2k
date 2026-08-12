"""Canonical legal-file policy for completed Cargo source packages."""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from typing import Mapping

APACHE = "Apache-2.0"
COMBINED = "Apache-2.0 AND BSD-2-Clause"
APACHE_2_0_SHA256 = "c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4"
OPENJPH_COMMIT = "2d0a033a135fb58dab87ea9551db8870e5b68548"
STALE_ATTRIBUTIONS = ("openjpeg", "hayro")

COMMON_OPENJPH_HOLDERS = (
    "Aous Naman",
    "Kakadu Software Pty Ltd, Australia",
    "The University of New South Wales",
)
EXTENDED_OPENJPH_HOLDERS = COMMON_OPENJPH_HOLDERS + (
    "Intel Corporation",
    "Osamu Watanabe",
)

HT_UPSTREAM_PATHS = (
    "src/core/coding/ojph_block_encoder.cpp",
    "src/core/coding/table0.h",
    "src/core/coding/table1.h",
    "src/core/coding/ojph_block_common.cpp",
    "src/core/coding/ojph_block_decoder32.cpp",
    "src/core/coding/ojph_block_decoder64.cpp",
    "src/core/coding/ojph_block_decoder_avx2.cpp",
    "LICENSE",
)
PYTHON_UPSTREAM_PATHS = HT_UPSTREAM_PATHS[:-1] + (
    "src/core/codestream/ojph_codestream_gen.cpp",
    "LICENSE",
)


@dataclass(frozen=True)
class OpenJphRequirements:
    licence_file: str
    required_upstream_paths: tuple[str, ...]
    required_holders: tuple[str, ...]


@dataclass(frozen=True)
class PackageLegalPolicy:
    license_expression: str
    legal_file_sha256: Mapping[str, str]
    openjph: OpenJphRequirements | None = None


def apache_only_policy() -> PackageLegalPolicy:
    return PackageLegalPolicy(
        license_expression=APACHE,
        legal_file_sha256={"LICENSE-APACHE-2.0": APACHE_2_0_SHA256},
    )


PACKAGE_POLICY: Mapping[str, PackageLegalPolicy] = {
    "emuella-j2k": apache_only_policy(),
    "emuella-j2k-accel": PackageLegalPolicy(
        license_expression=COMBINED,
        legal_file_sha256={
            "LICENSE-APACHE-2.0": APACHE_2_0_SHA256,
            "LICENSE-BSD-2-CLAUSE": "04c48b9bf08012b048477afc189a35db9f81a5ec580351434e71f3f8563b973d",
            "NOTICE": "27b2471df45cb29898fe03a3fa6e7b06a0d96134c9cc2474588301a8e90292cd",
            "THIRD_PARTY.md": "694040904a2159d573f22c05f0bfa006274a1f019612cd383e354d8a8751e3d7",
        },
        openjph=OpenJphRequirements(
            licence_file="LICENSE-BSD-2-CLAUSE",
            required_upstream_paths=("src/core/coding/ojph_block_decoder_avx2.cpp",),
            required_holders=EXTENDED_OPENJPH_HOLDERS,
        ),
    ),
    "emuella-j2k-cli": apache_only_policy(),
    "emuella-j2k-codestream": PackageLegalPolicy(
        license_expression=COMBINED,
        legal_file_sha256={
            "LICENSE-APACHE-2.0": APACHE_2_0_SHA256,
            "LICENSE-BSD-2-CLAUSE": "d0f04cb1604c54b27e8eac8a13b53f59d3502816ca78da95b0d8281d02519b87",
            "NOTICE": "2d399a5ffcfa3d50dbba64ececec46e9f43ee1b706e9d2df4e265a37b237fd87",
            "THIRD_PARTY.md": "db4d157ee30a00fbb35cd867eb4df73963716fb75fc20426fd20450bb1ad4d02",
        },
        openjph=OpenJphRequirements(
            licence_file="LICENSE-BSD-2-CLAUSE",
            required_upstream_paths=(
                "src/core/coding/ojph_block_decoder32.cpp",
                "src/core/codestream/ojph_codestream_gen.cpp",
            ),
            required_holders=COMMON_OPENJPH_HOLDERS,
        ),
    ),
    "emuella-j2k-container": apache_only_policy(),
    "emuella-j2k-core": apache_only_policy(),
    "emuella-j2k-ht": PackageLegalPolicy(
        license_expression=COMBINED,
        legal_file_sha256={
            "LICENSE-APACHE-2.0": APACHE_2_0_SHA256,
            "LICENSE-BSD-2-CLAUSE": "cf85d9844ce5b731dc74042dd26339c2a910e1c4063c059059eb2cac0620b08d",
            "NOTICE": "230c99b40b1df0f10bbfbd6b2e9ec2950feadb889b1736c1d259831930de85f8",
            "THIRD_PARTY.md": "a581741727d3cd8e620a551a9663c31631c8a26e86843ee1b7304a44b1c8cbea",
        },
        openjph=OpenJphRequirements(
            licence_file="LICENSE-BSD-2-CLAUSE",
            required_upstream_paths=HT_UPSTREAM_PATHS,
            required_holders=EXTENDED_OPENJPH_HOLDERS,
        ),
    ),
    "emuella-j2k-python": PackageLegalPolicy(
        license_expression=COMBINED,
        legal_file_sha256={
            "LICENSE-APACHE-2.0": APACHE_2_0_SHA256,
            "LICENSE-OPENJPH-BSD-2-CLAUSE": "cf85d9844ce5b731dc74042dd26339c2a910e1c4063c059059eb2cac0620b08d",
            "NOTICE": "09341dcd301814ca800cbc6d1596ebb95cc4a230d8b84bb9c2a89becbba5cc49",
            "THIRD_PARTY.md": "3b08ee4efaca58aadd815759abefb8f7943a4b33fdc054d1ab6f407a9320ba75",
        },
        openjph=OpenJphRequirements(
            licence_file="LICENSE-OPENJPH-BSD-2-CLAUSE",
            required_upstream_paths=PYTHON_UPSTREAM_PATHS,
            required_holders=EXTENDED_OPENJPH_HOLDERS,
        ),
    ),
    "emuella-j2k-test-support": apache_only_policy(),
    "emuella-j2k-tier1": apache_only_policy(),
    "emuella-j2k-transform": apache_only_policy(),
}


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def legal_content_errors(
    package_name: str,
    legal_files: Mapping[str, bytes],
) -> list[str]:
    """Validate exact legal bytes and package-specific provenance semantics."""
    policy = PACKAGE_POLICY[package_name]
    expected_names = set(policy.legal_file_sha256)
    actual_names = set(legal_files)
    errors: list[str] = []
    missing = expected_names - actual_names
    unexpected = actual_names - expected_names
    if missing:
        errors.append(f"missing legal files: {', '.join(sorted(missing))}")
    if unexpected:
        errors.append(f"unexpected legal files: {', '.join(sorted(unexpected))}")

    decoded: dict[str, str] = {}
    for name in sorted(expected_names & actual_names):
        content = legal_files[name]
        actual_hash = sha256_bytes(content)
        expected_hash = policy.legal_file_sha256[name]
        if actual_hash != expected_hash:
            errors.append(
                f"legal file hash differs: {name} "
                f"(expected {expected_hash}, got {actual_hash})"
            )
        try:
            decoded[name] = content.decode("utf-8")
        except UnicodeDecodeError:
            errors.append(f"legal file is not UTF-8: {name}")

    for name, text in decoded.items():
        lowered = text.casefold()
        for stale in STALE_ATTRIBUTIONS:
            if stale in lowered:
                errors.append(f"stale {stale} attribution in {name}")

    requirements = policy.openjph
    if requirements is None:
        return errors

    third_party = decoded.get("THIRD_PARTY.md", "")
    expected_commit_record = f"Pinned upstream commit: `{OPENJPH_COMMIT}`"
    if expected_commit_record not in third_party:
        errors.append("THIRD_PARTY.md omits the pinned OpenJPH commit")
    for path in requirements.required_upstream_paths:
        if f"`{path}`" not in third_party:
            errors.append(f"THIRD_PARTY.md omits upstream source path: {path}")

    licence = decoded.get(requirements.licence_file, "")
    for holder in requirements.required_holders:
        if holder not in licence:
            errors.append(
                f"{requirements.licence_file} omits copyright holder: {holder}"
            )

    notice = " ".join(decoded.get("NOTICE", "").split())
    if "OpenJPH" not in notice or "BSD 2-Clause" not in notice:
        errors.append("NOTICE omits the OpenJPH BSD 2-Clause attribution")
    return errors
