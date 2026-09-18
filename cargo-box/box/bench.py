# Driven by `cargo box runner` for benches: lldb is attached only to keep
# the iOS launch watchdog from killing a process that runs longer than ~20s
# without becoming a UI app.
import sys
import time

import lldb


def run(debugger):
    target = debugger.GetSelectedTarget()
    process = target.GetProcess()

    # `device process attach` returns before the attach completes
    deadline = time.time() + 30
    while process.GetState() != lldb.eStateStopped:
        if time.time() > deadline:
            sys.stderr.write("cargo box: lldb attach timed out\n")
            return
        time.sleep(0.05)

    debugger.SetAsync(False)

    # The app starts in `/`, which is read only. Move to Documents of the app
    # container, so relative outputs (`target/criterion`) can be written and
    # copied back to the host.
    bp = target.BreakpointCreateByName("main", target.GetExecutable().GetFilename())
    process.Continue()
    if process.GetState() == lldb.eStateStopped:
        res = target.EvaluateExpression(
            '(int)chdir((const char *)getenv("HOME")) + (int)chdir("Documents")'
        )
        if not res.GetError().Success() or res.GetValueAsSigned() != 0:
            sys.stderr.write("cargo box: can't chdir to app Documents\n")
        else:
            # the runner looks for this line in lldb output
            print("cargo box: resumed in Documents")
        target.BreakpointDelete(bp.GetID())
        process.Continue()

    if process.GetState() == lldb.eStateStopped:
        # crashed: show where, lldb is not interactive here
        stream = lldb.SBStream()
        process.GetSelectedThread().GetDescription(stream)
        sys.stderr.write(stream.GetData())
        for frame in process.GetSelectedThread():
            sys.stderr.write(f"  {frame}\n")
        process.Kill()
