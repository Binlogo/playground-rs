// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

#pragma once

#include <stdbool.h>
#include <stdint.h>

// The following structs are used to implement the lowest level
// of the FFI, and thus useful to multiple uniffied crates.
// We ensure they are declared exactly once, with a header guard, UNIFFI_SHARED_H.
#ifdef UNIFFI_SHARED_H
    // We also try to prevent mixing versions of shared uniffi header structs.
    // If you add anything to the #else block, you must increment the version in UNIFFI_SHARED_HEADER_V1
    #ifndef UNIFFI_SHARED_HEADER_V1
        #error Combining helper code from multiple versions of uniffi is not supported
    #endif // ndef UNIFFI_SHARED_HEADER_V1
#else
#define UNIFFI_SHARED_H
#define UNIFFI_SHARED_HEADER_V1
// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version in all instance of UNIFFI_SHARED_HEADER_V1 in this file.                   ⚠️

typedef struct RustBuffer
{
    int32_t capacity;
    int32_t len;
    uint8_t *_Nullable data;
    // Ref https://github.com/mozilla/uniffi-rs/issues/334 for this weird "padding" field.
    int64_t padding;
} RustBuffer;

typedef struct ForeignBytes
{
    int32_t len;
    const uint8_t *_Nullable data;
    // Ref https://github.com/mozilla/uniffi-rs/issues/334 for these weird "padding" fields.
    int64_t padding;
    int32_t padding2;
} ForeignBytes;

// Error definitions
// Each error has an error code enum, and a struct
typedef struct NativeRustError {
    int32_t code;
    char *_Nullable message;
} NativeRustError;

// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version in all instance of UNIFFI_SHARED_HEADER_V1 in this file.                   ⚠️
#endif // def UNIFFI_SHARED_H
  
uint32_t uni_me_db85_add(
      uint32_t a,uint32_t b
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer ffi_uni_me_db85_rustbuffer_alloc(
      int32_t size
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer ffi_uni_me_db85_rustbuffer_from_bytes(
      ForeignBytes bytes
    ,NativeRustError *_Nonnull out_err
    );
void ffi_uni_me_db85_rustbuffer_free(
      RustBuffer buf
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer ffi_uni_me_db85_rustbuffer_reserve(
      RustBuffer buf,int32_t additional
    ,NativeRustError *_Nonnull out_err
    );
void ffi_uni_me_db85_string_free(
      const char*_Nonnull cstr
    ,NativeRustError *_Nonnull out_err
    );
