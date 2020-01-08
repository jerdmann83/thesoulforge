import logging, os, re, strformat, strutils, parseopt
import illwill

type
  inputMode = enum
    Main,
    Filter,
    Save,

type
  trimLoc = enum
    Begin,
    End,

type
  fnavContext = ref object
    tb: TerminalBuffer

    # file handles and begin/end positions for each
    files: seq[File]
    bpos: seq[int64]
    epos: seq[int64]

    # begin and end seq indexes relative to what's visible on the screen
    # for example, suppose we have files from 0-8, and files 0-2 are
    # visible with the current set of filters running.  bfile and efile are 0
    # and 2 respectively
    bidx: int
    eidx: int

    # buffer of visible lines
    screenBuf: seq[string]

    # active filters
    filters: seq[Regex]

    # filters removed via the undo command
    # can be later reapplied via (surprise) redo
    rmfilters: seq[Regex]

    highlights: seq[Regex]

    # input stuff
    inBuf: seq[char]
    inMode: inputMode
    inPrompt: string

    # couple ui elements
    status: string
    bufPos: string

    log: FileLogger

    dirty: bool
    wrap: bool

proc newFnavContext() : fnavContext
# var ctx = newFnavContext()

proc exitProc() {.noconv.} =
  try:
    illwillDeinit()
  except:
    discard
  showCursor()
  quit(0)

proc fnavControlCHook() {.noconv.} = discard

proc bail(msg: string) =
  echo("error: ", msg)
  exitProc()

# Total terminal buffer height minus 1.  This is both where the status row is
# drawn and how much stuff we can fit on the screen at any given time.
proc mainHeight(ctx: fnavContext) : int = terminalHeight() - 1

proc fnavLog(ctx: fnavContext, a: varargs[string, `$`])

proc addFile(ctx: fnavContext, file: string) =
  let f = open(file)
  ctx.files.add(f)
  ctx.bpos.add(0)
  ctx.epos.add(0)

proc newFnavContext() : fnavContext =
  var ctx = new fnavContext
  ctx.tb = newTerminalBuffer(terminalWidth(), terminalHeight())
  ctx.inMode = inputMode.Main
  ctx.wrap = true

  hideCursor()
  illwillInit(fullscreen=true)
  setControlCHook(fnavControlCHook)
  return ctx

#--highlight=herpderp

proc loadRc(ctx: fnavContext, rcfile: string) =
  let f = open(rcfile)
  defer: f.close()

  for l in f.lines:
    let toks = l.split('=')
    if len(toks) != 2:
      continue
    case toks[0]:
      of "--highlight":
        ctx.highlights.add(re(toks[1]))
      else:
        discard

proc raiseEof() =
  var e: ref EOFError
  new(e)
  e.msg = "out of files"
  raise e

proc anyContains(rexes: seq[Regex], s: string): bool =
  for rex in rexes:
    if s.contains(rex):
      return true
  return false

# Return the next line from the next available source.
# TODO: exceptions are annoying.  Learn to Optional type.
proc nextLine(ctx: fnavContext): string =
  var buf : string

  # Scrolling down.  Start at the last visible file index.
  var i = ctx.eidx
  var currfile = ctx.files[i]
  currfile.setFilePos(ctx.bpos[i])

  while true:
    if currfile.endOfFile:
      i += 1
      if i >= ctx.files.len:
        ctx.fnavLog("eof i=", $i, " no more files")
        break
      ctx.fnavLog("eof i=", $i, " next file")

      ctx.eidx = i
      currfile = ctx.files[i]
      currfile.setFilePos(0)
      continue

    buf = currfile.readLine()
    # Set the new endpos for this file.
    ctx.epos[i] = currfile.getFilePos()

    # Push the bpos forward a line as well.  We have to actually read a line to
    # get the correct byte position.
    currfile.setFilePos(ctx.bpos[i])
    ctx.fnavLog("after set ", currfile.getFilePos)
    discard currfile.readline()
    # ctx.fnavLog("after read ", currfile.getFilePos)
    ctx.bpos[i] = currfile.getFilePos()

    return buf

  raiseEof()

# Return the previous line, working backwards from the "top" file.
proc prevLine(ctx: fnavContext): string =
  result = ""
  # Scrolling up.  Start at the first visible file index.
  var i = ctx.bidx
  var currfile = ctx.files[i]
  currfile.setFilePos(ctx.bpos[i])

  var isFirst = false
  var pchar = ' '
  while true:
    if currfile.getFilePos() == 0:
      i -= 1
      if i < 0:
        break

      ctx.bidx = i
      currfile = ctx.files[i]
      continue

    currfile.setFilePos(currfile.getFilePos() - 1)
    error("next ", currfile.getFilePos)

    var pos = currfile.getFilePos()
    pchar = currfile.readChar()
    currfile.setFilePos(max(pos - 1, 0))

    # Skip the first newline we find.  For example, if we're currently at:
    # \n Line1
    # \n Line2
    # \n [!here!]Line3
    # In order to get back to Line2, we need to jump over that first \n, then
    # stop at the next one.
    if pchar == '\n':
      if isFirst:
        isFirst = false
        continue

      # Going backwards is weird.  Grab the current file pos.  Read the line,
      # then "snap" the pos back to the actual starting point again.
      let pos = currfile.getFilePos() + 1
      let buf = currfile.readLine()
      currfile.setFilePos(pos)

      # The pre-read pos is now also the epos for this file.
      ctx.epos[i] = pos

      return buf

  raiseEof()

proc trimScreenBuf(ctx: fnavContext, loc: trimLoc) =
  var pos = 0
  if loc == trimLoc.End:
    pos = ctx.mainHeight()
  while ctx.screenBuf.len > ctx.mainHeight():
    ctx.screenBuf.delete(pos)

proc scrollDown(ctx: fnavContext, num: int) =
  ctx.dirty = true
  # Read n new lines that match the active filters.
  var remain = num
  var buf : string
  while remain > 0:
    try:
      buf = ctx.nextLine()
    except:
      break

    if anyContains(ctx.filters, buf):
      continue

    ctx.screenBuf.add(buf)
    remain -= 1

  ctx.trimScreenBuf(trimLoc.Begin)

proc fnavLogState(ctx: fnavContext, a: varargs[string, `$`]) =
  if not ctx.dirty:
    return

  ctx.dirty = false
  error("bidx=", $ctx.bidx,
      " eidx=", $ctx.eidx,
      " bpos=", $ctx.bpos,
      " epos=", $ctx.epos,
      " (", $ctx.screenBuf.len, "/", $ctx.mainHeight(), ")")

proc fnavLog(ctx: fnavContext, a: varargs[string, `$`]) =
  error(a)

proc scrollUp(ctx: fnavContext, num: int) =
  ctx.dirty = true
  # Read n lines backwards that match the active filters.
  var buf : string

  # Seek backwards until we either satisfy the request, or hit the beginning of
  # the file.  To do this, we will "bloat" the screenBuf beyond the display
  # limit, then trim the trailing edge off so it fits again.
  while ctx.screenBuf.len < ctx.mainHeight + num:
    try:
      buf = ctx.prevLine()
    except:
      break

    # Keep going backwards if this is a filtered line.
    if anyContains(ctx.filters, buf):
      continue

    ctx.screenBuf.add(buf)

  ctx.trimScreenBuf(trimLoc.End)

proc drawUi(ctx: fnavContext) =
  # Clear the bottom row.  We'll always draw something here whether it's an
  # input box or the main status row.
  ctx.tb.setCursorPos(0, ctx.mainHeight())
  ctx.tb.write(resetStyle, " ".repeat(terminalWidth()))
  ctx.tb.setCursorPos(0, ctx.mainHeight())

  ctx.tb.setForegroundColor(fgWhite, true)
  case ctx.inMode:
    of inputMode.Main:
      ctx.tb.write(fgWhite, ctx.status)

    of inputMode.Filter, inputMode.Save:
      ctx.tb.setCursorPos(0, ctx.mainHeight())
      ctx.tb.write(fgWhite, ctx.inPrompt, ctx.inBuf.join(), "_")

  var idx : int = 0
  var remain: int = 0
  while idx < ctx.screenBuf.len:
    ctx.tb.setCursorPos(0, idx)
    ctx.tb.write(fgWhite, ctx.screenBuf[idx])
    # if bufIdx < ctx.fileBuf.len:
    #   num = ctx.tb.write(fgWhite, ctx.fileBuf[bufIdx])
    # else:
    #   num = ctx.tb.write(fgWhite, "~")
    remain = terminalWidth() - ctx.screenBuf[idx].len
    if remain > 0:
      ctx.tb.write(fgWhite, " ".repeat(remain))
    idx += 1

  for i in idx..ctx.mainHeight() - 1:
    ctx.tb.setCursorPos(0, i)
    ctx.tb.write(fgWhite, "~")
    ctx.tb.write(fgWhite, " ".repeat(terminalWidth()))

  # if bufIdx >= ctx.fileBuf.len:
  #   ctx.bufPos = "Bot"
  # else:
  #   let pct : int = int(100 * ctx.rowOff / ctx.fileBuf.len)
  #   ctx.bufPos = $pct & "%"

  ctx.tb.setCursorPos(terminalWidth()-5, ctx.mainHeight())
  ctx.tb.write(fgWhite, ctx.bufPos)

  ctx.tb.display()
  ctx.fnavLogState()

proc handleTextInput(ctx: fnavContext, key: Key) =
  case key
  of Key.None: discard
  of Key.Escape:
    ctx.inMode = inputMode.Main
  of Key.Backspace:
    if ctx.inBuf.len > 0:
      ctx.inBuf.delete(ctx.inBuf.len)
  else:
    let ckey = char(key)
    ctx.inBuf.add(ckey)

proc saveFile(ctx: fnavContext, file: string) =
  # try:
  #   let f = open(file, fmWrite)
  #   defer: f.close()

  #   var bufIdx = 0
  #   var filtLines = 0
  #   var totLines = 0
  #   while bufIdx < ctx.fileBuf.len:
  #     totLines += 1
  #     if anyContains(ctx.filters, ctx.fileBuf[bufIdx]):
  #       bufIdx += 1
  #       filtLines += 1
  #       continue
  #     f.write(ctx.fileBuf[bufIdx])
  #     f.write("\n")
  #     bufIdx += 1

  #   let writLines = totLines - filtLines
  #   ctx.status = fmt"wrote {file} ({writLines} of {totLines} lines written)"
  # except:
  #   ctx.status = fmt"failed to write {file}"
  ctx.status = fmt"TODO: write {file}"

proc runForever(ctx: fnavContext) =
  # On startup, our begin and end files are the same.  These will move around as
  # we scroll around and/or filter stuff out.
  ctx.bidx = 0
  ctx.eidx = 0

  # Populate the initial screen contents.
  ctx.scrollDown(ctx.mainHeight())
  ctx.files[0].setFilePos(0)

  # Single event loop for all input handling.
  while true:
    var key = getKey()
    # Dispatch the input depending on what mode we're currently in.
    case ctx.inMode:
      of inputMode.Main:
        case key:
        of Key.None: discard
        of Key.Escape, Key.Q: exitProc()

        # Switch to input buffering mode to live enter new filters.
        of Key.F:
          ctx.inMode = inputMode.Filter
          ctx.inPrompt = "filter: "

        of Key.H: discard # TODO help

        # Various flavors of scrolling.
        of Key.J, Key.Down:
          ctx.scrollDown(1)
          # if ctx.rowOff < ctx.fileBuf.len:
          #   ctx.rowOff += 1
        of Key.K, Key.Up:
          ctx.scrollUp(1)
          # if ctx.rowOff > 0:
          #   ctx.rowOff -= 1
        # of Key.PageUp, Key.CtrlB:
        #   ctx.rowOff -= ctx.mainHeight()
        #   if ctx.rowOff < 0: ctx.rowOff = 0
        # of Key.PageDown, Key.CtrlF:
        #   ctx.rowOff += ctx.mainHeight()
        #   if ctx.rowOff > ctx.fileBuf.len-1:
        #     ctx.rowOff = ctx.fileBuf.len-1

        # # Half-page scrolling a-la vim/less
        # of Key.CtrlU:
        #   ctx.rowOff -= int((ctx.mainHeight()) / 2)
        #   if ctx.rowOff < 0: ctx.rowOff = 0
        # of Key.CtrlD:
        #   ctx.rowOff += int((ctx.mainHeight()) / 2)
        #   if ctx.rowOff > ctx.fileBuf.len-1:
        #     ctx.rowOff = ctx.fileBuf.len-1

        # # Jump to begin/end of buffer.
        # of Key.Home, Key.G:
        #   ctx.rowOff = 0
        # of Key.End, Key.ShiftG:
        #   ctx.rowOff = ctx.fileBuf.len - terminalHeight() + 1

        # Pop a filter off the stack, eg "undo it"
        of Key.U:
          if ctx.filters.len > 0:
            ctx.rmfilters.add(ctx.filters[ctx.filters.len-1])
            ctx.filters.delete(ctx.filters.len-1)
            ctx.status = $(len(ctx.filters)) & " filters active"

        # Reapply filters from the last-removed stack
        of Key.R:
          if ctx.rmfilters.len > 0:
            ctx.filters.add(ctx.rmfilters[ctx.rmfilters.len-1])
            ctx.rmfilters.delete(ctx.rmfilters.len-1)
            ctx.status = $(len(ctx.filters)) & " filters active"

        of Key.S:
          ctx.inMode = inputMode.Save
          ctx.inPrompt = "save to: "

        of Key.C:
          discard

        else: discard

      # Once in input mode, most keys just update the filter input buffer.
      of inputMode.Filter, inputMode.Save:
        var inputDone = false
        case key:
        of Key.Escape:
          inputDone = true
        of Key.Enter:
          inputDone = true
          if ctx.inBuf.len > 0:
            if ctx.inMode == inputMode.Filter:
              ctx.filters.add(re(ctx.inBuf.join))
              ctx.status = $(len(ctx.filters)) & " filters active"
            elif ctx.inMode == inputMode.Save:
              let outf = ctx.inBuf.join
              ctx.saveFile(outf):

        else:
          ctx.handleTextInput(key)

        if inputDone:
          ctx.inBuf = @[]
          ctx.inMode = inputMode.Main

    try:
      ctx.drawUi()
    except:
      ctx.fnavLog("caught ", getCurrentExceptionMsg())
      bail("except out!")
    sleep(20)

when isMainModule:
  var log = newFileLogger("/tmp/fnav")
  addHandler(log)
  error("========== new run ===========")

  var files: seq[string] = @[]
  var rc = "~/.fnavrc"

  var p = initOptParser()
  while true:
    p.next()
    case p.kind:
      of cmdArgument:
        add(files, p.key)
      of cmdLongOption:
        if p.key == "rc":
          rc = p.val
        else:
          bail("unknown option")
      of cmdEnd:
        break
      else:
        break

  if len(files) == 0:
    bail("no files")

  var fnav = newFnavContext()
  try:
    fnav.loadRc(rc)
  except:
    discard

  for file in files:
    fnav.addFile(file)

  fnav.runForever()
