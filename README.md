# fhistory - security forensics tool
- that title may sound like a lot, but the only purpose of this is to recursively check your filesystem for changes. It will return all the files changed today, so you can `grep` it and report it. All you need to run is `fhistory` (preferably with root).

  ## Warning
  This will create an extremely big file if you do not filter any directories or keywords, it's not intended to output straight to `stdout`.
