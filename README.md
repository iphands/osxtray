# OSX Tray

Can't live my global microphone mute tray thingy.

WIP at the moment there are some issues including the code does
not setup new listeners when devices changes (includes headphones plug/unplug)

## Install / Run
```shell
$ git clone https://github.com/iphands/osxtray.git
$ cd osxtray
$ mkdir -p /tmp/osxtray && cp ./assets/*png /tmp/osxtray/
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
