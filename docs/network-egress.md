# What this app does with the network

PdfCraft opens, edits, signs, prints and saves documents without contacting anything. There is no
telemetry, no analytics, no crash reporting, no licence check and no check for updates at start.
This page lists every place the code you are running can move bytes off the computer, or hand an
address to another program, and what limits each one.

The list is not prose in a document that can drift: it lives in
[`network-egress.toml`](network-egress.toml) and
[`check_egress.py`](implementation/check_egress.py) fails when a source file starts using the
network without an entry, when a listed place stops having the code its entry describes, or when
a crate gains an HTTP client nobody explained. CI runs that check.

Verify it yourself:

```bash
python3 docs/implementation/check_egress.py --check      # the inventory still matches the code
python3 -m unittest discover -s docs/implementation -p 'test_egress.py'   # and the check works
```

## The place list

| File | Kind | What it does |
|---|---|---|
| `apps/pdfcraft/src/updates.rs` | update-check | Help ▸ Check for updates asks GitHub for the latest release: one HTTPS GET to api.github.com. |
| `crates/ui-egui/src/lib.rs` | browser | An address handed to the browser or mail program: a link the user clicks, a button's URI action, or `app.launchURL` from a script. |
| `crates/ui-egui/src/control.rs` | loopback | The agent control channel: a JSON-RPC listener on 127.0.0.1 that lets a script drive the real interface. |
| `apps/pdfcraft-cli/src/main.rs` | loopback | `pdfcraft-cli ui` connects to that control channel to inspect and drive the running app. |
| `crates/print/src/spool.rs` | no-socket | A printer address is parsed as an IPv4 host, to tell a queue name from a host name. |

Kinds: **update-check** is a request made only when you ask for one; **browser** hands an address
to a program you already trust, when you act; **loopback** never leaves the computer;
**no-socket** only names a network type and connects to nothing.

## What each one means in practice

- **Check for updates** runs only when you choose it from the Help menu. It asks GitHub for the
  latest release tag, uses a 10-second timeout, reads at most 1 MiB, and only offers a download
  page under the project's releases page. PdfCraft never downloads or installs anything itself.
  If you do not use the command, the editor makes no request at all.
- **Links in a document** are never fetched by the editor. Clicking one shows a dialog with the
  full address, and only `http`, `https` and `mailto` addresses that a URL parser accepts can go
  ahead; anything else (a local path, a script, a malformed address) is refused. If you allow it,
  the address goes to your browser — which is then the program talking to the network.
- **The control channel** exists only when the app is started with `--control FILE`, which is a
  developer and test feature. It binds `127.0.0.1` on a random port and refuses a connection that
  does not present the random token from the owner-only control file. Nothing else on your
  machine, and nothing on the network, can reach it.
- **Signing, timestamping and revocation** use no transport of their own: the sign crate's
  timestamp authority is a trait the caller supplies, and the app supplies none, so a time-stamp
  request is never sent anywhere. Revocation material is whatever you hand the app.
- **Printing** goes through the platform's spooler. On Linux a job is fed to `lp` on standard
  input, never through a file in a shared folder.

## What this page does not claim

- It describes the source in this repository, checked statically. It is not a runtime sandbox: a
  library the app links could change behaviour in a future version, which is why the dependency
  rule fails the check when a new HTTP client appears.
- The browser, the mail program and your operating system's own update and telemetry services are
  not covered — once an address is handed to the browser, that program's policy applies.
- A document can contain a form's "submit" action or a script's network call; the app does not
  perform either. JavaScript runs in a sandbox with no network API (see `crates/js`).
- The agent control channel lets a local script drive the interface when you start the app with
  `--control`. That is a deliberate feature for automation, and it is off otherwise.
