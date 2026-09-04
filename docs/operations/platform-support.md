# Prepare PocketStation on macOS, Windows, and Linux

PocketStation uses the same `Session` and `Source` declarations on every
desktop platform. Native dependencies, capture permissions, and available
source mechanisms remain platform-specific.

## Check permission without guessing

`microphone_permission_observation()` reads the current authorization state
without prompting. `Denied`, `Restricted`, and `Revoked` are actionable host
states. `NotDetermined` means the host has not decided. `NotObservable` means
the platform cannot provide an authoritative preflight result; it does not mean
capture is allowed.

PocketStation does not own operating-system consent UI. The host application
must explain why capture is needed and start capture from an appropriate user
action. Opening the Source is authoritative: a typed permission or source-open
failure is not an empty audio stream. Restart the host application after a
permission change when the operating system does not update the running
process.

Desktop-output capture and microphone capture use separate permissions.
Request only the Sources the workflow needs. Use `Source::application` for one
selected application and `Source::system_audio` for the complete output mix.

## Reuse a discovered source safely

Discovery returns an immutable snapshot. Each `CaptureSource` includes a
`StableSourceId`, `SourceIdentityStrength`, and
`SelectorPersistenceScope`. Inspect the persistence scope before storing the
selector:

```rust,no_run
use pocketstation::{discover_sources, Session, Source, SourceKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let discovered = discover_sources()
    .into_iter()
    .find(|source| {
        source.stable_id.kind == SourceKind::Application
            && source.name == "Zoom"
    })
    .ok_or("Zoom is not running")?;

println!("persistence={:?}", discovered.selector_persistence_scope());
let session = Session::new();
let application = session.capture(Source::application(discovered.stable_id))?;
# let _ = application;
# Ok(())
# }
```

Use the scope as the storage policy:

| Persistence scope | Safe reuse |
|---|---|
| `ApplicationIdentity` | Reopen the application identity after a normal restart, then verify discovery still resolves it. |
| `DeviceIdentity` | Reopen the same native device while it remains installed and permitted. |
| `PlatformIdentity` | Reuse the platform-owned identity on that platform. |
| `ProcessLifetime` | Reuse only while that exact process is alive. Rediscover after exit. |
| `SessionDefaultDevice` | Follow the host default for each new Session; do not treat it as a pinned device. |
| unavailable | Do not persist the selector. Rediscover before the next Session. |

A `SourceId` identifies captured lineage; it is not a portable account or
cloud identifier. Persist the platform, source kind, stable key, and declared
persistence scope when the application needs to remember a selection.

## Recover after a source changes

PocketStation does not silently switch applications or devices during a
Session. Source disappearance and backend failure are typed Session events.
When the event reports `ExplicitRediscoveryAndNewSession`, stop or cancel the
current Session, discover again, let the user confirm any changed selection,
and create a new Session. The next source generation and discontinuity remain
visible instead of being presented as uninterrupted media.

Choose fallback behavior in the application:

- use an exact application identity first, then require user confirmation
  before falling back to an exact display name;
- use a microphone device ID when the device must stay pinned, or
  `microphone_default()` when following the host default is intended;
- reject ambiguous application matches instead of choosing the first result;
- attach separate Connector values for primary and backup destinations so each
  has its own queue, failure, and shutdown outcome; and
- keep retry and reconnect finite. Core reports Connector recovery state but
  does not invent a provider retry policy.

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

After a source disappears, do not restart capture inside a callback or reuse a
stale process ID. Stop capture, discover the application again, and create a
new Session.

The 10 ms and 20 ms profiles describe PocketStation's normalized frame cadence.
They do not promise end-to-end latency below that duration. Report capture,
queue, transport, receiver, and acoustic measurements separately for each
qualified environment.
