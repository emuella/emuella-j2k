/* Project-authored native C/C++ journey over a generated reversible-MCT fixture. */
#include <stdio.h>
#include <stdlib.h>

struct MultiSourceContext {
  const uint8_t *bytes;
  size_t length;
};

static EmuellaJ2kStatus multi_read_at(void *opaque, uint64_t offset,
                                    uint8_t *destination, size_t length)
#ifdef __cplusplus
    noexcept
#endif
{
  const struct MultiSourceContext *source = (const struct MultiSourceContext *)opaque;
  if (offset > source->length || length > source->length - (size_t)offset)
    return EMUELLA_J2K_STATUS_SOURCE_IO;
  memcpy(destination, source->bytes + (size_t)offset, length);
  return EMUELLA_J2K_STATUS_OK;
}

static uint8_t rgb_expected(uint16_t component, uint32_t x, uint32_t y) {
  switch (component) {
  case 0: return (uint8_t)((x * 13 + y * 7 + x * y * 3 + 17) & 255);
  case 1: return (uint8_t)((x * 5 + y * 19 + (x ^ y) * 11 + 29) & 255);
  default: return (uint8_t)((x * 23 + y * 3 + x * y * 5 + 41) & 255);
  }
}

static void test_multi_component(const char *fixture_path) {
  FILE *file = fopen(fixture_path, "rb");
  assert(file != NULL);
  assert(fseek(file, 0, SEEK_END) == 0);
  long file_length = ftell(file);
  assert(file_length > 0);
  assert(fseek(file, 0, SEEK_SET) == 0);
  uint8_t *bytes = (uint8_t *)malloc((size_t)file_length);
  assert(bytes != NULL);
  assert(fread(bytes, 1, (size_t)file_length, file) == (size_t)file_length);
  assert(fclose(file) == 0);
  struct MultiSourceContext context = {bytes, (size_t)file_length};
  EmuellaJ2kSourceV0 source = {sizeof(source), EMUELLA_J2K_ABI_VERSION, 0,
                             (uint64_t)file_length, &context, multi_read_at};
  EmuellaJ2kDecoder *decoder = NULL;
  EmuellaJ2kWorkspace *workspace = NULL;
  assert(emuella_j2k_decoder_create(&source, &decoder, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(emuella_j2k_workspace_create(&workspace, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kDecodeComponentsRequestV0 request;
  memset(&request, 0, sizeof(request));
  request.struct_size = sizeof(request);
  request.component_count = 3;
  request.components[0] = 2;
  request.components[1] = 0;
  request.components[2] = 1;
  request.x = 61; request.y = 63; request.width = 7; request.height = 5;
  request.collect_work = 1;
  EmuellaJ2kDecodeWorkV0 combined;
  memset(&combined, 0, sizeof(combined));
  for (unsigned repetition = 0; repetition < 2; ++repetition) {
    EmuellaJ2kImage *image = NULL;
    assert(emuella_j2k_decode_components_region(decoder, workspace, &request, &image, NULL) == EMUELLA_J2K_STATUS_OK);
    EmuellaJ2kImageInfoV0 info;
    assert(emuella_j2k_image_info(image, &info, NULL) == EMUELLA_J2K_STATUS_OK);
    assert(info.component_count == 3 && info.width == 7 && info.height == 5);
    EmuellaJ2kDecodeWorkV0 work;
    assert(emuella_j2k_image_decode_work(image, &work, NULL) == EMUELLA_J2K_STATUS_OK);
    assert(work.preparation_count == 1 && work.code_blocks_decoded > 0);
    assert(work.output_allocation_bytes == 105 && work.output_allocation_count == 3);
    assert(work.output_capacity_bytes >= work.output_allocation_bytes);
    if (repetition == 0) combined = work;
    else assert(work.workspace_retained_heap_bytes == combined.workspace_retained_heap_bytes);
    for (uint16_t index = 0; index < 3; ++index) {
      EmuellaJ2kComponentInfoV0 component;
      assert(emuella_j2k_image_component_info_at(image, index, &component, NULL) == EMUELLA_J2K_STATUS_OK);
      assert(component.source_component == request.components[index]);
      uint8_t actual[45];
      memset(actual, 0xa5, sizeof(actual));
      assert(emuella_j2k_image_copy_component(image, index, actual, sizeof(actual), 9, NULL) == EMUELLA_J2K_STATUS_OK);
      for (uint32_t y = 0; y < 5; ++y) {
        for (uint32_t x = 0; x < 7; ++x)
          assert(actual[y * 9 + x] == rgb_expected(request.components[index], x + 61, y + 63));
        assert(actual[y * 9 + 7] == 0xa5 && actual[y * 9 + 8] == 0xa5);
      }
      uint8_t sentinel[45];
      memcpy(sentinel, actual, sizeof(actual));
      assert(emuella_j2k_image_copy_component(image, 3, actual, sizeof(actual), 9, NULL) == EMUELLA_J2K_STATUS_INVALID_ARGUMENT);
      assert(memcmp(sentinel, actual, sizeof(actual)) == 0);
      assert(emuella_j2k_image_copy_component(image, index, actual, 42, 9, NULL) == EMUELLA_J2K_STATUS_INVALID_ARGUMENT);
      assert(memcmp(sentinel, actual, sizeof(actual)) == 0);
    }
    emuella_j2k_image_destroy(image);
  }
  request.component_count = 1;
  request.components[0] = 0; request.components[1] = 0; request.components[2] = 0;
  EmuellaJ2kImage *image = NULL;
  assert(emuella_j2k_decode_components_region(decoder, workspace, &request, &image, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kDecodeWorkV0 single;
  assert(emuella_j2k_image_decode_work(image, &single, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(single.code_blocks_decoded == combined.code_blocks_decoded);
  assert(single.tier1_coefficients == combined.tier1_coefficients);
  assert(single.synthesis_lifting_updates == combined.synthesis_lifting_updates);
  assert(single.synthesis_output_samples == combined.synthesis_output_samples);
  emuella_j2k_image_destroy(image);
  request.component_count = 2; /* Duplicate selection is rejected before publication. */
  image = NULL;
  assert(emuella_j2k_decode_components_region(decoder, workspace, &request, &image, NULL) == EMUELLA_J2K_STATUS_INVALID_ARGUMENT);
  assert(image == NULL);
  emuella_j2k_workspace_destroy(workspace);
  emuella_j2k_decoder_destroy(decoder);
  free(bytes);
}
