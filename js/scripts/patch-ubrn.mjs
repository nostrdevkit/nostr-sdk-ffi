import { readFile, writeFile } from "node:fs/promises";

const manifests = [
  {
    path: "../node_modules/uniffi-bindgen-react-native/Cargo.toml",
    replacements: [
      [
        '    "xtask",\n]\n\nresolver = "2"',
        '    "xtask",\n]\nexclude = ["crates/uniffi-runtime-javascript"]\n\nresolver = "2"',
      ],
      ['uniffi = "=0.31.0"', 'uniffi = "=0.31.2"'],
      ['uniffi_bindgen = "=0.31.0"', 'uniffi_bindgen = "=0.31.2"'],
      [
        'uniffi_core = { version = "=0.31.0", default-features = false }',
        'uniffi_core = { version = "=0.31.2", default-features = false }',
      ],
      ['uniffi_meta = "=0.31.0"', 'uniffi_meta = "=0.31.2"'],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/uniffi-runtime-javascript/Cargo.toml",
    replacements: [
      [
        'uniffi_core = { version = "=0.31.0", default-features = false }',
        'uniffi_core = { version = "=0.31.2", default-features = false }',
      ],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/ubrn_bindgen/src/bindings/gen_typescript/api_module/mod.rs",
    replacements: [
      [
        "pub(crate) use self::nodes::{",
        "pub(crate) use self::type_helpers::arg_name;\n\npub(crate) use self::nodes::{",
      ],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/ubrn_bindgen/src/bindings/gen_typescript/api_module/type_helpers.rs",
    replacements: [
      [
        "pub(super) fn arg_name(nm: &str) -> String {",
        "pub(crate) fn arg_name(nm: &str) -> String {",
      ],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/ubrn_bindgen/src/bindings/gen_typescript/ffi_module_player/builder.rs",
    replacements: [
      [
        "use crate::bindings::gen_typescript::config::TsConfig;",
        "use crate::bindings::gen_typescript::api_module::arg_name;\nuse crate::bindings::gen_typescript::config::TsConfig;",
      ],
      [
        "name: arg.name.to_lower_camel_case(),",
        "name: arg_name(&arg.name.to_lower_camel_case()),",
      ],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/ubrn_bindgen/src/bindings/gen_typescript/ffi_module/builder.rs",
    replacements: [
      [
        "use super::type_mapping::{ffi_type_to_ts, ffi_type_to_ts_native};",
        "use super::type_mapping::{ffi_type_to_ts, ffi_type_to_ts_native};\nuse crate::bindings::gen_typescript::api_module::arg_name;",
      ],
      [
        "name: arg.name.to_lower_camel_case(),\n                type_name: type_mapper(&arg.ty.ty),",
        "name: arg_name(&arg.name.to_lower_camel_case()),\n                type_name: type_mapper(&arg.ty.ty),",
      ],
      [
        "name: arg.name.to_lower_camel_case(),\n                type_name: ffi_type_to_ts(&arg.ty.ty),",
        "name: arg_name(&arg.name.to_lower_camel_case()),\n                type_name: ffi_type_to_ts(&arg.ty.ty),",
      ],
    ],
  },
  {
    path: "../node_modules/uniffi-bindgen-react-native/crates/ubrn_cli/templates/jsi/android/CMakeLists.txt",
    replacements: [
      [
        `cmake_path(
  SET MY_RUST_LIB
  \${CMAKE_SOURCE_DIR}/{{ jni_libs_dir }}/\${ANDROID_ABI}/{{ self.config.rust_crate.library_file(Some("android"), Some(*self.config.project.android.use_shared_library)) }}
  NORMALIZE
)`,
        `set(ANDROID_RUST_PACKAGE
  "@nostrdevkit/nostr-sdk-react-native-android-\${ANDROID_ABI}"
)
execute_process(
  COMMAND node -p "require.resolve('\${ANDROID_RUST_PACKAGE}/package.json')"
  WORKING_DIRECTORY \${CMAKE_SOURCE_DIR}
  RESULT_VARIABLE ANDROID_RUST_PACKAGE_RESULT
  OUTPUT_VARIABLE ANDROID_RUST_PACKAGE_MANIFEST
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
if(NOT ANDROID_RUST_PACKAGE_RESULT EQUAL 0)
  message(FATAL_ERROR "Unable to resolve \${ANDROID_RUST_PACKAGE}")
endif()
get_filename_component(ANDROID_RUST_PACKAGE_PATH
  "\${ANDROID_RUST_PACKAGE_MANIFEST}"
  DIRECTORY
)`,
      ],
      [
        "set_target_properties(my_rust_lib PROPERTIES IMPORTED_LOCATION ${MY_RUST_LIB})",
        `set_target_properties(my_rust_lib PROPERTIES
  IMPORTED_LOCATION "\${ANDROID_RUST_PACKAGE_PATH}/libnostr_sdk_ffi.a"
)`,
      ],
    ],
  },
];

for (const manifest of manifests) {
  const path = new URL(manifest.path, import.meta.url);
  let source = await readFile(path, "utf8");

  for (const [current, patched] of manifest.replacements) {
    if (source.includes(patched)) {
      continue;
    }

    if (!source.includes(current)) {
      throw new Error(`Unsupported UBRN manifest: ${manifest.path}`);
    }

    source = source.replace(current, patched);
  }

  await writeFile(path, source);
}
