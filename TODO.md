    https://github.com/devkitPro/pacman/blob/a7dbe4635b4af9b5bd927ec0e7a211d1f6c1b0f2/src/pacman/sync.c#L734

`pacman/sync.c/sync_prepare_execute()` is called after `alpm_trans_init` and `alpm_sync_sys_upgrade` are called

* Convert `pacman/sync.c/sync_prepare_execute()` function to `Kea.update_alpm_pkgs()`
*  Impliment aur package update 

NOTE: aur outdated package decector already done in `repo::upgrade::get_outdated_pkgs()`