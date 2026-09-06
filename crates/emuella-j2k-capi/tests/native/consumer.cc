#include "emuella_j2k.h"

#include <atomic>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <thread>
#include <vector>

#define ASSERT_LAYOUT(type, size, alignment) \
  static_assert(sizeof(type) == size, #type " size"); \
  static_assert(alignof(type) == alignment, #type " alignment")
#define ASSERT_OFFSET(type, field, offset) \
  static_assert(offsetof(type, field) == offset, #type "." #field " offset")

ASSERT_LAYOUT(EmuellaJ2kSourceV0, 40, 8);
ASSERT_OFFSET(EmuellaJ2kSourceV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kSourceV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kSourceV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kSourceV0, length, 16);
ASSERT_OFFSET(EmuellaJ2kSourceV0, context, 24);
ASSERT_OFFSET(EmuellaJ2kSourceV0, read_at, 32);

ASSERT_LAYOUT(EmuellaJ2kImageInfoV0, 40, 8);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, width, 16);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, height, 20);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, component_count, 24);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, bits_per_sample, 26);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, is_signed, 27);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, byte_order, 28);
ASSERT_OFFSET(EmuellaJ2kImageInfoV0, reserved_bytes, 29);

ASSERT_LAYOUT(EmuellaJ2kComponentInfoV0, 40, 8);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, source_component, 16);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, bits_per_sample, 18);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, is_signed, 19);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, byte_order, 20);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, horizontal_separation, 21);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, vertical_separation, 22);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, reserved_byte, 23);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, width, 24);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, height, 28);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, x_origin, 32);
ASSERT_OFFSET(EmuellaJ2kComponentInfoV0, y_origin, 36);

ASSERT_LAYOUT(EmuellaJ2kDecodeRequestV0, 48, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, component, 16);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, max_quality_layers, 18);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, x, 20);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, y, 24);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, width, 28);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, height, 32);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, discard_levels, 36);
ASSERT_OFFSET(EmuellaJ2kDecodeRequestV0, reserved_bytes, 37);

ASSERT_LAYOUT(EmuellaJ2kDecodeComponentsRequestV0, 56, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, component_count, 16);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, max_quality_layers, 18);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, components, 20);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, x, 28);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, y, 32);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, width, 36);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, height, 40);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, discard_levels, 44);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, collect_work, 45);
ASSERT_OFFSET(EmuellaJ2kDecodeComponentsRequestV0, reserved_bytes, 46);

ASSERT_LAYOUT(EmuellaJ2kDecodeWorkV0, 176, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, struct_size, 0);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, abi_version, 8);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, reserved, 12);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, preparation_count, 16);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, code_blocks_decoded, 24);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, tier1_coefficients, 32);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, dwt_samples, 40);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, synthesis_coefficients_loaded, 48);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, synthesis_horizontal_values, 56);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, synthesis_vertical_values, 64);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, synthesis_lifting_updates, 72);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, synthesis_output_samples, 80);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, windowed_synthesis_component_tiles, 88);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, full_synthesis_component_tiles, 96);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, output_allocation_bytes, 104);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, output_capacity_bytes, 112);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, coefficient_capacity, 120);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, segment_capacity, 128);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, transform_capacity, 136);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, full_coefficient_plane_capacity, 144);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, full_transform_scratch_capacity, 152);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, output_allocation_count, 160);
ASSERT_OFFSET(EmuellaJ2kDecodeWorkV0, workspace_retained_heap_bytes, 168);

#include "multi-component.h"

static const std::uint8_t CODESTREAM[] = {
    0xff, 0x4f, 0xff, 0x51, 0x00, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x01, 0xff, 0x52, 0x00,
    0x0c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x04, 0x04, 0x00, 0x01, 0xff,
    0x5c, 0x00, 0x04, 0x40, 0x40, 0xff, 0x90, 0x00, 0x0a, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x20, 0x00, 0x01, 0xff, 0x93, 0xdf, 0x80, 0x78, 0x12, 0x0b,
    0xd3, 0x5f, 0x4b, 0x93, 0x20, 0x7d, 0xb9, 0xec, 0x3b, 0xc2, 0x15, 0xc0,
    0x37, 0xff, 0xd9,
};

struct SourceContext {
  const std::uint8_t *bytes;
  std::size_t length;
  std::atomic<bool> concurrent_mode{false};
  std::atomic<int> barrier_entries{0};
  std::atomic<int> active_callbacks{0};
  std::atomic<bool> overlap_observed{false};
};

static EmuellaJ2kStatus read_at(void *opaque, std::uint64_t offset,
                                std::uint8_t *destination, std::size_t length) noexcept {
  SourceContext *context = static_cast<SourceContext *>(opaque);
  bool active_incremented = false;
  try {
    int previous_active = context->active_callbacks.fetch_add(1);
    active_incremented = true;
    if (previous_active != 0) {
      context->overlap_observed.store(true);
    }
    if (context->concurrent_mode.load()) {
      int ticket = context->barrier_entries.fetch_add(1);
      if (ticket < 2) {
        while (context->barrier_entries.load() < 2) {
          std::this_thread::yield();
        }
      }
    }
    if (offset > context->length ||
        length > context->length - static_cast<std::size_t>(offset)) {
      context->active_callbacks.fetch_sub(1);
      return 98;
    }
    std::memcpy(destination,
                context->bytes + static_cast<std::size_t>(offset), length);
    context->active_callbacks.fetch_sub(1);
    return EMUELLA_J2K_STATUS_OK;
  } catch (...) {
    if (active_incremented) {
      context->active_callbacks.fetch_sub(1);
    }
    return EMUELLA_J2K_STATUS_INTERNAL;
  }
}

struct ConcurrentDecode {
  EmuellaJ2kDecoder *decoder{};
  EmuellaJ2kWorkspace *workspace{};
  EmuellaJ2kDecodeComponentsRequestV0 request{};
  EmuellaJ2kStatus status{EMUELLA_J2K_STATUS_INTERNAL};
  EmuellaJ2kImage *image{};
  std::vector<std::uint8_t> samples = std::vector<std::uint8_t>(4);
};

static void decode_concurrently(ConcurrentDecode *decode) {
  decode->status = emuella_j2k_decode_components_region(
      decode->decoder, decode->workspace, &decode->request, &decode->image, nullptr);
  if (decode->status == EMUELLA_J2K_STATUS_OK) {
    decode->status = emuella_j2k_image_copy(
        decode->image, decode->samples.data(), decode->samples.size(), 2, nullptr);
  }
}

int main(int argc, char **argv) {
  assert(argc == 2);
  test_multi_component(argv[1]);
  SourceContext source_context{CODESTREAM, sizeof(CODESTREAM)};
  EmuellaJ2kSourceV0 source{sizeof(source), EMUELLA_J2K_ABI_VERSION, 0,
                            sizeof(CODESTREAM), &source_context, read_at};
  EmuellaJ2kDecoder *decoder = nullptr;
  assert(emuella_j2k_decoder_create(&source, &decoder, nullptr) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kInspection *inspection = nullptr;
  assert(emuella_j2k_decoder_inspect(decoder, &inspection, nullptr) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kComponentInfoV0 component{};
  assert(emuella_j2k_inspection_component_info(inspection, 0, &component, nullptr) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kWorkspace *workspace = nullptr;
  assert(emuella_j2k_workspace_create(&workspace, nullptr) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kDecodeRequestV0 request{sizeof(request), EMUELLA_J2K_ABI_VERSION, 0,
                                    0, 0, 1, 1, 2, 2, 0, {0}};
  EmuellaJ2kImage *image = nullptr;
  assert(emuella_j2k_decode_component_region(decoder, workspace, &request, &image, nullptr) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kComponentInfoV0 decoded_component{};
  assert(emuella_j2k_image_component_info(image, &decoded_component, nullptr) == EMUELLA_J2K_STATUS_OK);
  assert(decoded_component.width == 2 && decoded_component.height == 2);
  assert(decoded_component.x_origin == 1 && decoded_component.y_origin == 1);
  std::vector<std::uint8_t> samples(4);
  assert(emuella_j2k_image_copy(image, samples.data(), samples.size(), 2, nullptr) == EMUELLA_J2K_STATUS_OK);
  assert((samples == std::vector<std::uint8_t>{5, 6, 9, 10}));
  emuella_j2k_image_destroy(image);
  emuella_j2k_workspace_destroy(workspace);

  ConcurrentDecode decodes[2];
  for (auto &decode : decodes) {
    assert(emuella_j2k_workspace_create(&decode.workspace, nullptr) ==
           EMUELLA_J2K_STATUS_OK);
    decode.decoder = decoder;
    decode.request.struct_size = sizeof(EmuellaJ2kDecodeComponentsRequestV0);
    decode.request.component_count = 1;
    decode.request.x = 1; decode.request.y = 1;
    decode.request.width = 2; decode.request.height = 2;
    decode.request.collect_work = 1;
  }
  source_context.concurrent_mode.store(true);
  std::thread threads[2] = {
      std::thread(decode_concurrently, &decodes[0]),
      std::thread(decode_concurrently, &decodes[1]),
  };
  for (std::size_t index = 0; index < 2; ++index) {
    threads[index].join();
    assert(decodes[index].status == EMUELLA_J2K_STATUS_OK);
    assert((decodes[index].samples == std::vector<std::uint8_t>{5, 6, 9, 10}));
    EmuellaJ2kDecodeWorkV0 work;
    assert(emuella_j2k_image_decode_work(decodes[index].image, &work, NULL) == EMUELLA_J2K_STATUS_OK);
    assert(work.preparation_count == 1 && work.output_allocation_bytes == 4);
    assert(work.code_blocks_decoded == 1);
    emuella_j2k_image_destroy(decodes[index].image);
    emuella_j2k_workspace_destroy(decodes[index].workspace);
  }
  assert(source_context.overlap_observed.load());
  assert(source_context.active_callbacks.load() == 0);

  emuella_j2k_inspection_destroy(inspection);
  emuella_j2k_decoder_destroy(decoder);
}
