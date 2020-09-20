# selinux-rs
libselinux bindings for Rust

![](https://github.com/nibon7/selinux-rs/workflows/Build/badge.svg)
[![Build Status](https://travis-ci.org/nibon7/selinux-rs.svg?branch=master)](https://travis-ci.org/nibon7/selinux-rs)

## Requires

```sh
$ sudo dnf install libselinux-devel
```

## Building selinux-rs

```sh
$ git clone https://github.com/nibon7/selinux-rs
$ cd selinux-rs
$ cargo build
```

## TODO
 - [ ] avc_add_callback
 - [ ] avc_audit
 - [ ] avc_av_stats
 - [ ] avc_cache_stats
 - [ ] avc_cleanup
 - [ ] avc_compute_create
 - [ ] avc_compute_member
 - [ ] avc_context_to_sid
 - [ ] avc_context_to_sid_raw
 - [ ] avc_destroy
 - [ ] avc_get_initial_sid
 - [ ] avc_has_perm
 - [ ] avc_has_perm_noaudit
 - [ ] avc_init
 - [ ] avc_netlink_acquire_fd
 - [ ] avc_netlink_check_nb
 - [ ] avc_netlink_close
 - [ ] avc_netlink_loop
 - [ ] avc_netlink_open
 - [ ] avc_netlink_release_fd
 - [ ] avc_open
 - [ ] avc_reset
 - [ ] avc_sid_stats
 - [ ] avc_sid_to_context
 - [ ] avc_sid_to_context_raw
 - [ ] checkPasswdAccess
 - [ ] context_free
 - [x] context_new
 - [x] context_range_get
 - [x] context_range_set
 - [x] context_role_get
 - [x] context_role_set
 - [x] context_str
 - [x] context_type_get
 - [x] context_type_set
 - [x] context_user_get
 - [x] context_user_set
 - [x] fgetfilecon
 - [x] fgetfilecon_raw
 - [ ] fini_selinuxmnt
 - [ ] freecon
 - [ ] freeconary
 - [x] fsetfilecon
 - [x] fsetfilecon_raw
 - [x] getcon
 - [x] getcon_raw
 - [ ] get_default_context
 - [ ] get_default_context_with_level
 - [ ] get_default_context_with_role
 - [ ] get_default_context_with_rolelevel
 - [ ] get_default_type
 - [x] getexeccon
 - [x] getexeccon_raw
 - [x] getfilecon
 - [x] getfilecon_raw
 - [x] getfscreatecon
 - [x] getfscreatecon_raw
 - [x] getkeycreatecon
 - [x] getkeycreatecon_raw
 - [ ] get_ordered_context_list
 - [ ] get_ordered_context_list_with_level
 - [x] getpeercon
 - [x] getpeercon_raw
 - [x] getpidcon
 - [ ] getpidcon_raw
 - [x] getprevcon
 - [x] getprevcon_raw
 - [ ] getseuser
 - [ ] getseuserbyname
 - [x] getsockcreatecon
 - [x] getsockcreatecon_raw
 - [ ] is_context_customizable
 - [x] is_selinux_enabled
 - [x] is_selinux_mls_enabled
 - [x] lgetfilecon
 - [x] lgetfilecon_raw
 - [x] lsetfilecon
 - [x] lsetfilecon_raw
 - [ ] manual_user_enter_context
 - [ ] map_class
 - [ ] map_decision
 - [ ] map_perm
 - [ ] matchmediacon
 - [ ] matchpathcon
 - [ ] matchpathcon_checkmatches
 - [ ] matchpathcon_filespec_add
 - [ ] matchpathcon_filespec_destroy
 - [ ] matchpathcon_filespec_eval
 - [ ] matchpathcon_fini
 - [ ] matchpathcon_index
 - [ ] matchpathcon_init
 - [ ] matchpathcon_init_prefix
 - [ ] mode_to_security_class
 - [ ] print_access_vector
 - [ ] query_user_context
 - [ ] realpath_not_final
 - [ ] rpm_execcon
 - [ ] security_av_perm_to_string
 - [ ] security_av_string
 - [ ] security_canonicalize_context
 - [ ] security_canonicalize_context_raw
 - [ ] security_check_context
 - [ ] security_check_context_raw
 - [ ] security_class_to_string
 - [ ] security_commit_booleans
 - [ ] security_compute_av
 - [ ] security_compute_av_flags
 - [ ] security_compute_av_flags_raw
 - [ ] security_compute_av_raw
 - [ ] security_compute_create
 - [ ] security_compute_create_name
 - [ ] security_compute_create_name_raw
 - [ ] security_compute_create_raw
 - [ ] security_compute_member
 - [ ] security_compute_member_raw
 - [ ] security_compute_relabel
 - [ ] security_compute_relabel_raw
 - [ ] security_compute_user
 - [ ] security_compute_user_raw
 - [ ] security_deny_unknown
 - [ ] security_disable
 - [ ] security_get_boolean_active
 - [ ] security_get_boolean_names
 - [ ] security_get_boolean_pending
 - [ ] security_get_checkreqprot
 - [ ] security_getenforce
 - [ ] security_get_initial_context
 - [ ] security_get_initial_context_raw
 - [ ] security_load_booleans
 - [ ] security_load_policy
 - [ ] security_policyvers
 - [ ] security_reject_unknown
 - [ ] security_set_boolean
 - [ ] security_set_boolean_list
 - [ ] security_setenforce
 - [ ] selabel_close
 - [ ] selabel_cmp
 - [ ] selabel_digest
 - [ ] selabel_lookup
 - [ ] selabel_lookup_best_match
 - [ ] selabel_lookup_best_match_raw
 - [ ] selabel_lookup_raw
 - [ ] selabel_open
 - [ ] selabel_partial_match
 - [ ] selabel_stats
 - [ ] selinux_binary_policy_path
 - [ ] selinux_booleans_path
 - [ ] selinux_booleans_subs_path
 - [ ] selinux_boolean_sub
 - [ ] selinux_check_access
 - [ ] selinux_check_passwd_access
 - [ ] selinux_check_securetty_context
 - [ ] selinux_colors_path
 - [ ] selinux_contexts_path
 - [ ] selinux_current_policy_path
 - [ ] selinux_customizable_types_path
 - [ ] selinux_default_context_path
 - [ ] selinux_default_type_path
 - [ ] selinux_failsafe_context_path
 - [ ] selinux_file_context_cmp
 - [ ] selinux_file_context_homedir_path
 - [ ] selinux_file_context_local_path
 - [ ] selinux_file_context_path
 - [ ] selinux_file_context_subs_dist_path
 - [ ] selinux_file_context_subs_path
 - [ ] selinux_file_context_verify
 - [ ] selinuxfs_exists
 - [ ] selinux_get_callback
 - [ ] selinux_getenforcemode
 - [ ] selinux_getpolicytype
 - [ ] selinux_homedir_context_path
 - [ ] selinux_init_load_policy
 - [ ] selinux_lsetfilecon_default
 - [ ] selinux_lxc_contexts_path
 - [ ] selinux_media_context_path
 - [ ] selinux_mkload_policy
 - [ ] selinux_netfilter_context_path
 - [ ] selinux_openrc_contexts_path
 - [ ] selinux_openssh_contexts_path
 - [ ] selinux_path
 - [ ] selinux_policy_root
 - [ ] selinux_raw_context_to_color
 - [ ] selinux_raw_to_trans_context
 - [ ] selinux_removable_context_path
 - [ ] selinux_reset_config
 - [ ] selinux_restorecon
 - [ ] selinux_restorecon_default_handle
 - [ ] selinux_restorecon_set_alt_rootpath
 - [ ] selinux_restorecon_set_exclude_list
 - [ ] selinux_restorecon_set_sehandle
 - [ ] selinux_restorecon_xattr
 - [ ] selinux_securetty_types_path
 - [ ] selinux_sepgsql_context_path
 - [ ] selinux_set_callback
 - [ ] selinux_set_mapping
 - [ ] selinux_set_policy_root
 - [ ] selinux_snapperd_contexts_path
 - [ ] selinux_status_close
 - [ ] selinux_status_deny_unknown
 - [ ] selinux_status_getenforce
 - [ ] selinux_status_open
 - [ ] selinux_status_policyload
 - [ ] selinux_status_updated
 - [ ] selinux_systemd_contexts_path
 - [ ] selinux_translations_path
 - [ ] selinux_trans_to_raw_context
 - [ ] selinux_user_contexts_path
 - [ ] selinux_usersconf_path
 - [ ] selinux_users_path
 - [ ] selinux_virtual_domain_context_path
 - [ ] selinux_virtual_image_context_path
 - [ ] selinux_x_context_path
 - [x] setcon
 - [x] setcon_raw
 - [x] setexeccon
 - [x] setexeccon_raw
 - [x] setexecfilecon
 - [x] setfilecon
 - [x] setfilecon_raw
 - [x] setfscreatecon
 - [x] setfscreatecon_raw
 - [x] setkeycreatecon
 - [x] setkeycreatecon_raw
 - [ ] set_matchpathcon_canoncon
 - [ ] set_matchpathcon_flags
 - [ ] set_matchpathcon_invalidcon
 - [ ] set_matchpathcon_printf
 - [ ] set_selinuxmnt
 - [x] setsockcreatecon
 - [x] setsockcreatecon_raw
 - [ ] sidget
 - [ ] sidput
 - [ ] string_to_av_perm
 - [ ] string_to_security_class
 - [ ] unmap_class
 - [ ] unmap_perm
