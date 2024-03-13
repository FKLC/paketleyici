# Executable Spec

## Packaged Runner File Contents
- Runner
- Config Size (u16 / 2 bytes)
- Config
- Tarball Size (u64 / 8 bytes)
- Tarball
- Runner Size (u32 / 4 bytes)

## Config
All the fields are self-explanatory except args_pos and append_path. Here's an explanation.

- `args_pos`: If it is 0, arguments passed to runner are concatenated with the specified `command`, i.e. `packaged_runner.exe executionTimeArg1 executionTimeArg2` turns into `["<command to run>", "argument1", "argument2", "executionTimeArg1", "executionTimeArg2"]`. For values other than 0, arguments passed to runner are concatenated with the nth argument (one-indexed), this is especially useful when you use `["sh", "-c", "<command to run>"]` or something similar. For example, if we assume `command` is `["sh", "-c", "echo "]` and args_pos is `3`, then running `packaged_runner.exe Hello World` will turn into `["sh", "-c", "echo Hello World"]`
- `append_path`: if it is true, path of the executable is appended to the 1st element of command to run.

```json
{
  "paket": "<path to runner>",
  "tarball": "<path to tarball>",
  "folder": "<folder name to be used in temp directory>",
  "command": ["<command to run>", "argument1", "argument2"],
  "args_pos": 0,
  "append_path": true
}
```