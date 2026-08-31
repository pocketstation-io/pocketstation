# Prepare PocketStation on macOS, Windows, and Linux

PocketStation uses the same `Session` and `Source` declarations on every
desktop platform. Native dependencies, capture permissions, and available
source mechanisms remain platform-specific.

## macOS

Install Rust 1.95 or newer and the Xcode command-line tools. Application audio
capture requires the operating-system screen and system-audio recording
permission. Microphone capture requires microphone permission.

The first capture attempt may trigger consent UI. If permission changes while
the application is running, restart the application before retrying capture.
Select applications by exact display name, bundle identifier, process ID, or a
discovered stable source identity.

## Windows

Install Rust 1.95 or newer with the MSVC toolchain, Visual Studio Build Tools,
and a Windows SDK. PocketStation uses WASAPI process-loopback and microphone
capture. A process ID identifies only the current process instance; discover
the source again after an application restarts.

VM results establish the tested guest configuration. Qualify physical devices
and latency on the hardware you support.

## Debian and Ubuntu

Install the native build and audio development packages:

```bash
sudo apt install build-essential cmake pkg-config \
  libasound2-dev libpipewire-0.3-dev
```

Application and system capture use PipeWire. Microphone capture uses ALSA.
The desktop session must expose the requested PipeWire nodes to the process;
container or service accounts do not automatically inherit a logged-in
desktop user's audio session.

## Diagnose setup failures

Before starting a Session:

1. call `discover_sources()` and confirm the intended application or device is
   present;
2. resolve an exact name, identifier, process instance, or stable identity;
3. request only the sources the workflow needs;
4. treat permission and source-open failures as setup failures, not empty
   audio;
5. inspect Session events and route metrics after startup.

The 10 ms and 20 ms profiles describe PocketStation's normalized frame cadence.
They do not promise end-to-end latency below that duration. Report capture,
queue, transport, receiver, and acoustic measurements separately for each
qualified environment.
