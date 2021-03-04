# OSX Tray

Can't live my global microphone mute tray thingy.

WIP at the moment there are some issues
~~including the code does not setup new listeners when devices changes (includes headphones plug/unplug)~~
Still need to clean up the old listeners on hardware change :D

There is still some issue where with two devices osxtray might report mute, but Zoom is looking at the
other mic. I'll probably change the toggle (Apple Script) code to just mute all devices instead of targeting the primary.

## Install / Run
```shell
$ git clone https://github.com/iphands/osxtray.git
$ cd osxtray
$ cargo run --release
```

## How to toggle mute from keyboard

I used this [AppleScript + Automator + keyboard shortcut](https://blog.fosketts.net/2010/08/09/assign-keyboard-shortcut-applescript-automator-service/) guide for now.

And this [helpful hint](https://superuser.com/a/397770) for the AppleScript

## Screenshots
<table>
 <tr>
  <td><img src="https://raw.githubusercontent.com/iphands/osxtray/main/assets/demo.gif" alt="screenshot"></td>
 </tr>
</table>
