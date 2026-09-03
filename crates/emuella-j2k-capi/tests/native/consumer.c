#include "emuella_j2k.h"

#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#define ASSERT_LAYOUT(type, size, alignment) \
  _Static_assert(sizeof(type) == size, #type " size"); \
  _Static_assert(_Alignof(type) == alignment, #type " alignment")
#define ASSERT_OFFSET(type, field, offset) \
  _Static_assert(offsetof(type, field) == offset, #type "." #field " offset")

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

static const uint8_t CODESTREAM[] = {
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

static EmuellaJ2kStatus read_at(void *context, uint64_t offset,
                                uint8_t *destination, size_t length) {
  const uint8_t *bytes = (const uint8_t *)context;
  if (offset > sizeof(CODESTREAM) || length > sizeof(CODESTREAM) - (size_t)offset) {
    return 99;
  }
  memcpy(destination, bytes + (size_t)offset, length);
  return EMUELLA_J2K_STATUS_OK;
}

int main(void) {
  assert(emuella_j2k_abi_version() == EMUELLA_J2K_ABI_VERSION);
  assert(strcmp(emuella_j2k_package_version(), "0.1.0") == 0);
  EmuellaJ2kSourceV0 source = {
      sizeof(source), EMUELLA_J2K_ABI_VERSION, 0, sizeof(CODESTREAM),
      (void *)CODESTREAM, read_at,
  };
  EmuellaJ2kDecoder *decoder = NULL;
  assert(emuella_j2k_decoder_create(&source, &decoder, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kInspection *inspection = NULL;
  assert(emuella_j2k_decoder_inspect(decoder, &inspection, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kImageInfoV0 info = {0};
  assert(emuella_j2k_inspection_image_info(inspection, &info, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(info.width == 4 && info.height == 4 && info.component_count == 1);
  EmuellaJ2kComponentInfoV0 component = {0};
  assert(emuella_j2k_inspection_component_info(inspection, 0, &component, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(component.width == 4 && component.height == 4 && component.bits_per_sample == 8);

  EmuellaJ2kError *error = NULL;
  assert(emuella_j2k_inspection_component_info(inspection, 1, &component, &error) == EMUELLA_J2K_STATUS_INVALID_ARGUMENT);
  size_t message_size = 0;
  assert(emuella_j2k_error_status(error) == EMUELLA_J2K_STATUS_INVALID_ARGUMENT);
  assert(emuella_j2k_error_message_size(error, &message_size) == EMUELLA_J2K_STATUS_OK);
  char message[128];
  assert(message_size <= sizeof(message));
  assert(emuella_j2k_error_message_copy(error, (uint8_t *)message, sizeof(message)) == EMUELLA_J2K_STATUS_OK);
  assert(strstr(message, "out of bounds") != NULL);
  emuella_j2k_error_destroy(error);

  EmuellaJ2kWorkspace *workspace = NULL;
  assert(emuella_j2k_workspace_create(&workspace, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kDecodeRequestV0 request = {
      sizeof(request), EMUELLA_J2K_ABI_VERSION, 0, 0, 0, 1, 1, 2, 2, 0, {0},
  };
  EmuellaJ2kImage *image = NULL;
  assert(emuella_j2k_decode_component_region(decoder, workspace, &request, &image, NULL) == EMUELLA_J2K_STATUS_OK);
  EmuellaJ2kComponentInfoV0 decoded_component = {0};
  assert(emuella_j2k_image_component_info(image, &decoded_component, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(decoded_component.width == 2 && decoded_component.height == 2);
  assert(decoded_component.x_origin == 1 && decoded_component.y_origin == 1);
  uint8_t samples[5];
  memset(samples, 0xa5, sizeof(samples));
  assert(emuella_j2k_image_copy(image, samples, sizeof(samples), 3, NULL) == EMUELLA_J2K_STATUS_OK);
  assert(samples[0] == 5 && samples[1] == 6 && samples[2] == 0xa5);
  assert(samples[3] == 9 && samples[4] == 10);
  emuella_j2k_image_destroy(image);
  emuella_j2k_workspace_destroy(workspace);
  emuella_j2k_inspection_destroy(inspection);
  emuella_j2k_decoder_destroy(decoder);
  return 0;
}
