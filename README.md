DayZ Linux GUI Launcher
====

## This launcher relies on [bastimeyer's dayz-linux-cli-launcher][dayz-linux-cli-launcher-gh]
and uses an unmodified version of that base shell-script.
## About

This is an experimental launcher for DayZ standalone on Linux when running the game via Proton.

Proton is currently unable to start the game's own regular launcher application which sets up mods and launch parameters for the game client. The game however does work fine when launching the client directly, so mods can be set up and configured manually, which is what this Launcher does, similar to what the launcher would do.

Automatic Steam workshop mod downloads are currently unsupported due to a limitation of Steam's CLI. Workshop mods will therefore need to be subscribed manually via the web browser.

Please see the "[Install DayZ](#install-dayz)" section down below on how to get the game running on Linux.

## Install

To install the launcher script, simply clone the git repository:

```sh
# do stuff will update later
```

## Install DayZ

[Support for BattlEye anti-cheat for Proton on Linux has been officially announced by Valve on 2021-12-03.][battleye-announcement]

In order to get the game running on Linux, you first have to install the Steam beta client (see Steam's settings menu). Then install `Proton Experimental` and the `Proton BattlEye Runtime` (filter by "tools" in your games library). After that, set the "Steam play compatibility tool" for DayZ to "Proton Experimental" (right-click the game and go to properties).

### Important notes (Usually not necessary)

In order for the game to actually run on Linux via Proton, the [`vm.max_map_count`][vm.max_map_count] kernel parameter needs to be increased, because otherwise the game will freeze while loading the main menu or after playing for a couple of minutes. Some custom kernels like TK-Glitch for example already increase this value from its [default value of `64*1024-6`][vm.max_map_count-default] to [`512*1024`][tkg-kernel-patch], but even this won't work reliably. Increasing it to `1024*1024` seems to work.

```sh
​sudo sysctl -w vm.max_map_count=1048576
```

Or apply it permanently:

```sh
​echo 'vm.max_map_count=1048576' | sudo tee /etc/sysctl.d/vm.max_map_count.conf
```


  [battleye-announcement]: https://store.steampowered.com/news/group/4145017/view/3104663180636096966
  [vm.max_map_count]: https://github.com/torvalds/linux/blob/v5.15/Documentation/admin-guide/sysctl/vm.rst#max_map_count
  [vm.max_map_count-default]: https://github.com/torvalds/linux/blob/v5.15/include/linux/mm.h#L185-L202
  [tkg-kernel-patch]: https://github.com/Frogging-Family/linux-tkg/blob/db405096bd7fb52656fc53f7c5ee87e7fe2f99c9/linux-tkg-patches/5.15/0003-glitched-base.patch#L477-L534
  [dayz-linux-cli-launcher-gh]: https://github.com/bastimeyer/dayz-linux-cli-launcher
