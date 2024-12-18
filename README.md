<div align="center">
  <img style="height: 300px; width: 300px;" src="./src-tauri/icons/logo.png" />
  <h1>wledTR</h1>
  <p>A cross-platform <a href="https://github.com/tauri-apps/tauri">Tauri</a> application for controlling <a href="https://github.com/Aircoookie/WLED">WLED</a> lights.</p>
</div>

## Why another App?

The [official app](https://github.com/Aircoookie/WLED-App) is essentially just an embedding of the web interface that is hosted on the actual WLED-Device.

I find this suboptimal in most cases since it is not at all reliable for me and going through the JSON-API, which [WLED does provide](https://kno.wled.ge/interfaces/json-api/), makes the experience a lot better.

> I think a tiny microcontroller should not be used to provide a web interface at all. An extensive API is about all that it should need and leave the "heavy-lifting" and data presentation to the client.
> This is of course my opinion and the WLED devs may have radically different views on this and that is fine. It is an awesome project nonetheless.

Unfortunately not all features are accessible via the API and there is also zero authentication required which is also something I dislike.

## Features

> [!NOTE]  
> This project is in a very early development stage.<br>
> Supposed to be a learner project to get used to SvelteKit + Tauri 2.0
> 
> Expect bugs and unfinished stuff

- Save Multiple Devices
- Toggle light ON/OFF
- Change color via color-wheel
- Control Brightness via slider

## How to Build

Initializing and Building is handled through `npm` scripts.

The following instructions are for building on a Linux machine.

### Linux

Follow the instructions [provided by Tauri](https://beta.tauri.app/guides/prerequisites/#linux), then run:

```sh
# install
npm install
# initialize
npm run make:init
# build for linux
npm run make:linux
```

### Android

Follow the instructions [provided by Tauri](https://beta.tauri.app/guides/prerequisites/#android).

This project uses a fixed set of libraries, make sure to install the following:

```bash
# using the sdkmanager utlitity
sdkmanager --install "build-tools;34.0.0"
sdkmanager --install "platforms;android-34"
sdkmanager --install "ndk/27.2.12479018"
sdkmanager --install "emulator"
```

Then run:

```sh
# install
npm install
# initialize
npm run make:init
# make sure $ANDROID_HOME is set
export ANDROID_HOME="/home/$USER/Android/Sdk"
# build for android
npm run make:android
```

## Checklist

- [x] Basic Light Selection
- [x] Device State Recovery
- [x] Device Saving
- [x] Multiple Devices
- [ ] Preset switching
- [ ] Light Auto-Discover
