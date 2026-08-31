#!/usr/bin/env python3
"""Static server for chronochromatic, plus PUT so documents can be edited in the page.

Only files named in WRITABLE may be written, and only at the top level -- the
path is compared as a bare name, so no traversal reaches outside the root.
"""
import http.server, os, pathlib, socketserver

ROOT = pathlib.Path(__file__).parent.resolve()
WRITABLE = {"spec.md"}
MAX_BYTES = 1_000_000


class Server(socketserver.ThreadingTCPServer):
    """One thread per connection, because a browser opens several at once.

    The plain TCPServer this replaced handled exactly one connection at a time,
    so a single slow or abandoned one stalled every other request behind it --
    a tab left half-loaded with no error to explain why. A browser opens up to
    six sockets per host and keeps them open, which is enough to deadlock a
    server that can only hold one.

    daemon_threads so Ctrl-C exits rather than waiting on open sockets;
    block_on_close off for the same reason.
    """
    daemon_threads = True
    block_on_close = False


class Handler(http.server.SimpleHTTPRequestHandler):
    def do_PUT(self):
        name = self.path.lstrip("/")
        if name not in WRITABLE:
            self.send_error(403, "not writable")
            return
        try:
            n = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            self.send_error(400, "bad length")
            return
        if n < 0 or n > MAX_BYTES:
            self.send_error(413, "too large")
            return
        body = self.rfile.read(n)
        target = ROOT / name
        tmp = target.with_suffix(target.suffix + ".tmp")
        tmp.write_bytes(body)
        tmp.replace(target)                      # atomic, never a half-written spec
        self.send_response(204)
        self.end_headers()

    def send_error(self, code, message=None, explain=None):
        """Serve 404.html for a missing page, rather than the stdlib's grey box.

        Only for 404, only for a GET, and only if the file is actually there --
        anything else falls back to the default, because an error handler that
        can itself fail is worse than a plain error.
        """
        if code == 404 and self.command == "GET":
            page = ROOT / "404.html"
            try:
                body = page.read_bytes()
            except OSError:
                body = None
            if body is not None:
                self.send_response(404, message)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
        super().send_error(code, message, explain)

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


def demo():
    """Self-check: the whitelist is the only thing standing between a PUT and the disk."""
    assert "spec.md" in WRITABLE
    for bad in ("../fold.js", "/etc/passwd", "atlas.html", "spec.md/../fold.js", ""):
        assert bad.lstrip("/") not in WRITABLE, bad
    assert issubclass(Server, socketserver.ThreadingMixIn), "the server must handle connections concurrently"
    assert Server.daemon_threads, "threads must not outlive Ctrl-C"
    assert (ROOT / "404.html").exists(), "404.html is missing; send_error would fall back"
    print("serve.py self-check ok")


if __name__ == "__main__":
    import sys
    if "--check" in sys.argv:
        demo()
    else:
        os.chdir(ROOT)
        Server.allow_reuse_address = True
        with Server(("0.0.0.0", 1338), Handler) as s:
            print("chronochromatic on http://localhost:1338")
            s.serve_forever()
