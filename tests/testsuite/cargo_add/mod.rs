mod add_basic;
mod add_multiple;
mod add_no_vendored_package_with_alter_registry;
mod add_no_vendored_package_with_vendor;
mod add_normalized_name_external;
mod add_toolchain;
mod build;
mod build_prefer_existing_version;
mod change_rename_target;
mod cyclic_features;
mod default_features;
mod deprecated_default_features;
mod deprecated_section;
mod detect_workspace_inherit;
mod detect_workspace_inherit_features;
mod detect_workspace_inherit_fuzzy;
mod detect_workspace_inherit_optional;
mod detect_workspace_inherit_public;
mod dev;
mod dev_build_conflict;
mod dev_prefer_existing_version;
mod dry_run;
mod empty_dep_name;
mod features;
mod features_activated_over_limit;
mod features_deactivated_over_limit;
mod features_empty;
mod features_multiple_occurrences;
mod features_preserve;
mod features_spaced_values;
mod features_unknown;
mod features_unknown_no_features;
mod git;
mod git_branch;
mod git_conflicts_namever;
mod git_dev;
mod git_inferred_name;
mod git_inferred_name_multiple;
mod git_multiple_names;
mod git_multiple_packages_features;
mod git_normalized_name;
mod git_registry;
mod git_rev;
mod git_tag;
mod help;
mod infer_prerelease;
mod invalid_arg;
mod invalid_git_name;
mod invalid_key_inherit_dependency;
mod invalid_key_overwrite_inherit_dependency;
mod invalid_key_rename_inherit_dependency;
mod invalid_manifest;
mod invalid_name_external;
mod invalid_path;
mod invalid_path_name;
mod invalid_path_self;
mod invalid_target_empty;
mod invalid_vers;
mod list_features;
mod list_features_path;
mod list_features_path_no_default;
mod locked_changed;
mod locked_unchanged;
mod lockfile_updated;
mod manifest_path_package;
mod merge_activated_features;
mod multiple_conflicts_with_features;
mod multiple_conflicts_with_rename;
mod namever;
mod no_args;
mod no_default_features;
mod no_optional;
mod no_public;
mod offline_empty_cache;
mod optional;
mod overwrite_default_features;
mod overwrite_default_features_with_no_default_features;
mod overwrite_features;
mod overwrite_git_with_path;
mod overwrite_inherit_features_noop;
mod overwrite_inherit_noop;
mod overwrite_inherit_optional_noop;
mod overwrite_inline_features;
mod overwrite_name_dev_noop;
mod overwrite_name_noop;
mod overwrite_no_default_features;
mod overwrite_no_default_features_with_default_features;
mod overwrite_no_optional;
mod overwrite_no_optional_with_optional;
mod overwrite_no_public;
mod overwrite_no_public_with_public;
mod overwrite_optional;
mod overwrite_optional_with_no_optional;
mod overwrite_optional_with_optional;
mod overwrite_path_noop;
mod overwrite_path_with_version;
mod overwrite_preserves_inline_table;
mod overwrite_public;
mod overwrite_public_with_no_public;
mod overwrite_rename_with_no_rename;
mod overwrite_rename_with_rename;
mod overwrite_rename_with_rename_noop;
mod overwrite_version_with_git;
mod overwrite_version_with_path;
mod overwrite_with_rename;
mod overwrite_workspace_dep;
mod overwrite_workspace_dep_features;
mod path;
mod path_dev;
mod path_inferred_name;
mod path_inferred_name_conflicts_full_feature;
mod path_normalized_name;
mod preserve_dep_std_table;
mod preserve_features_sorted;
mod preserve_features_table;
mod preserve_features_unsorted;
mod preserve_sorted;
mod preserve_unsorted;
mod public;
mod quiet;
mod registry;
mod rename;
mod require_weak;
mod rust_version_ignore;
mod rust_version_incompatible;
mod rust_version_latest;
mod rust_version_older;
mod rustc_ignore;
mod rustc_incompatible;
mod rustc_latest;
mod rustc_older;
mod sorted_table_with_dotted_item;
mod target;
mod target_cfg;
mod unknown_inherited_feature;
mod vers;
mod workspace_name;
mod workspace_path;
mod workspace_path_dev;
