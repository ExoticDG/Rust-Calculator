# Rust-Calculator

A simple calculator I made in Rust.

## Usage

[Dropbox File Download](https://www.dropbox.com/scl/fo/v65uvyfaletwsikna5m57/AE5xTG4bKB2_DL0ocXvFZkg?rlkey=4gbqwfq8ofolgu4hbzg99wz8z&st=21n8n7hd&dl=0)

To use this calculator on Windows (tested on windows 11) download the Windows executable and left click it twice (installer not available).

To use the Linux version (tested on kubuntu) download the Linux executable and right click on the file. you then want to select run in Konsole (on different distributions of Linux this might say console or terminal). For newer versions (8.0.0 and up) just left click it twice  for the exactable or do the same thing for the debian package and follow the installation instructions look for the program in the start menu and launch it by clicking it.

Feel free to make suggestions in the discussions tab and report any errors or bugs the the issues tab.

## Dev

* Compile & run dev mode Linux

    ```bash
    > cargo run
    ```

* Compile Android app & run in emulator

    * Start Android Studio (`studio.sh`). Go to 'Virtual Device Manager'.  Select a device to emulate and start the device (play button).

        ```bash
        > cargo apk run --target aarch64-linux-android --lib
        ```

## Dev Setup

### Pre-req

```bash
> sudo apt update
> sudo apt install build-essential curl 
```

### Install Rust

```bash
> curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Add Android build target

```bash
> rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### Install Cargo-Apk

https://github.com/rust-mobile/cargo-apk

```bash
> cargo install cargo-apk
```

## Setup Android SDK Environment

https://developer.android.com/studio

1. Download & install Android Studio
    1. Unpack the Android Studio distribution archive that you downloaded where you wish to install the program. Extract `android-studio` to `$HOME`.
    2. To start the application, open a console, cd into `~/android-studio/bin` and type: `./studio.sh`
      This will initialize various configuration files in the configuration directory: `~/.config/Google/AndroidStudio2024.1.`
    3. Go to SDK Manager & install the following:
        * Android SDK Platform
        * Android SDK Platform-Tools
        * NDK (Side by side)
        * Android SDK Build-Tools
        * Android SDK Command-line Tools
    4. Go to Virtual Device Manager & add a device to emulate.
    5. Edit `~./bashrc`. (modified to make Slint compile to android too)
        ```bash
        export PATH=$PATH:$HOME/android-studio/bin:$HOME/android-studio/jbr/bin
        export JAVA_HOME=$HOME/android-studio/jbr
        export ANDROID_HOME="$HOME/Android/Sdk"
        export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
        export ANDROID_NDK_ROOT="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
        ```
    6. Restart shell to re-load `~./bashrc`
2. Unknown if this is actually needed
    ```
    > sudo apt-get install adb
    > sudo agp-get install google-android-platform-tools-installer
    ```
