
Rust callers must uphold each unsafe export’s documented contract. The two
version queries remain callable from safe Rust.

```rust
use emuella_j2k_capi::*;
let _: u32 = emuella_j2k_abi_version();
let _: *const std::ffi::c_char = emuella_j2k_package_version();
```

Each pointer-dependent export requires an unsafe call site, independently.
The paired compile-only controls validate the same imports and arguments;
null arguments do not perform a useful operation.

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decoder_create;
emuella_j2k_decoder_create(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decoder_create;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_decoder_create(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decoder_create_indexed;
emuella_j2k_decoder_create_indexed(std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decoder_create_indexed;
// SAFETY: null pointers are rejected before access.
unsafe {
    emuella_j2k_decoder_create_indexed(std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decoder_destroy;
emuella_j2k_decoder_destroy(std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decoder_destroy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_decoder_destroy(std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decoder_inspect;
emuella_j2k_decoder_inspect(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decoder_inspect;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_decoder_inspect(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_inspection_destroy;
emuella_j2k_inspection_destroy(std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_inspection_destroy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_inspection_destroy(std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_inspection_image_info;
emuella_j2k_inspection_image_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_inspection_image_info;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_inspection_image_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_inspection_component_info;
emuella_j2k_inspection_component_info(std::ptr::null(), 0, std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_inspection_component_info;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_inspection_component_info(std::ptr::null(), 0, std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_workspace_create;
emuella_j2k_workspace_create(std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_workspace_create;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_workspace_create(std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_workspace_destroy;
emuella_j2k_workspace_destroy(std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_workspace_destroy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_workspace_destroy(std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decode_component_region;
emuella_j2k_decode_component_region(std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decode_component_region;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_decode_component_region(std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_destroy;
emuella_j2k_image_destroy(std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_destroy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_image_destroy(std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_info;
emuella_j2k_image_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_info;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_image_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_component_info;
emuella_j2k_image_component_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_component_info;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_image_component_info(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_copy;
emuella_j2k_image_copy(std::ptr::null(), std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_copy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_image_copy(std::ptr::null(), std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_error_destroy;
emuella_j2k_error_destroy(std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_error_destroy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_error_destroy(std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_error_status;
emuella_j2k_error_status(std::ptr::null());
```
```no_run
use emuella_j2k_capi::emuella_j2k_error_status;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_error_status(std::ptr::null());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_error_message_size;
emuella_j2k_error_message_size(std::ptr::null(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_error_message_size;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_error_message_size(std::ptr::null(), std::ptr::null_mut());
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_error_message_copy;
emuella_j2k_error_message_copy(std::ptr::null(), std::ptr::null_mut(), 0);
```
```no_run
use emuella_j2k_capi::emuella_j2k_error_message_copy;
// SAFETY: null pointers are accepted for destruction or rejected before access.
unsafe {
    emuella_j2k_error_message_copy(std::ptr::null(), std::ptr::null_mut(), 0);
}
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_decode_components_region;
emuella_j2k_decode_components_region(std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_decode_components_region;
// SAFETY: Null pointers are rejected before access.
unsafe { emuella_j2k_decode_components_region(std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut()); }
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_component_info_at;
emuella_j2k_image_component_info_at(std::ptr::null(), 0, std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_component_info_at;
// SAFETY: Null pointers are rejected before access.
unsafe { emuella_j2k_image_component_info_at(std::ptr::null(), 0, std::ptr::null_mut(), std::ptr::null_mut()); }
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_copy_component;
emuella_j2k_image_copy_component(std::ptr::null(), 0, std::ptr::null_mut(), 0, 0, std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_copy_component;
// SAFETY: Null pointers are rejected before access.
unsafe { emuella_j2k_image_copy_component(std::ptr::null(), 0, std::ptr::null_mut(), 0, 0, std::ptr::null_mut()); }
```

```compile_fail,E0133
use emuella_j2k_capi::emuella_j2k_image_decode_work;
emuella_j2k_image_decode_work(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut());
```
```no_run
use emuella_j2k_capi::emuella_j2k_image_decode_work;
// SAFETY: Null pointers are rejected before access.
unsafe { emuella_j2k_image_decode_work(std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut()); }
```
