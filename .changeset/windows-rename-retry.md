---
"fnm": patch
---

Retry the post-extraction directory rename on Windows when an A/V handle blocks it

On Windows, `fnm install` could fail with `Can't download the requested binary: Access is denied. (os error 5)` even though the download and the extraction both succeeded. Windows denies a directory rename while any file inside the tree has an open handle that does not grant `FILE_SHARE_DELETE`, which is what a real-time A/V scanner holds on the freshly extracted `node.exe`.

Both renames that follow extraction are now retried with bounded backoff. The budget defaults to 5000ms and can be tuned with `FNM_RENAME_RETRY_TIMEOUT_MS` (`0` disables retrying). Filesystem failures after the download also stopped reporting themselves as download errors.

Refs #1583, #1193
