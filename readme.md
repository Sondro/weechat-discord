Weechat Discord
===============

Currently a work in progress. I deeply apologize to anyone who may stumble upon this repository in hopes of finding an elusive weechat-discord bridge before it is finished, or worse, stumble upon it and find it abandoned.

### Building

The makefile should give enough information for build commands. Here's the essentials:

    cd weechat-discord # or wherever you cloned it
    cargo build --release

This will produce a shared object called `target/release/libweecord.so`. Place it in your weechat plugins directory, which is probably located at `~/.weechat/plugins` (may need to be created)

The Makefile has a tiny bit of automation that helps with development:

    make # (same as make all) just runs that `cargo build --release` command, produces weecord.so
    make install # builds and copies the .so to ~/.weechat/plugins, creating the dir if required
    make run # installs and runs `weechat -a` (-a means "don't autoconnect to servers")

Maybe important note: The previous version of this project, written in Go, used to get **really upset** when the .so was modified during the same weechat session, even if unloaded. When developing, make sure to completely quit weechat when updating the .so, just to be sure (otherwise you might get a SIGSEGV and hard crash).

### Using

Due to some idiocracy on Discord's part, [you will need to obtain a login token](https://github.com/hammerandchisel/discord-api-docs/issues/69#issuecomment-223886862). The wonderful tip by the Discord devs on how to do that, though, doesn't work: for me, and only for chromium, I went into inspector (ctrl+shift+i, because Discord *also* decided to do the *wonderful* thing of ripping out right-click menus), Application tab, Local Storage on left, discordapp.com, token entry.

Set that token with

    /discord token 123456789ABCDEF

Then, connect with

    /discord connect