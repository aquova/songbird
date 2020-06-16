# Blargg Test Results

The Blargg tests are a widely-used set of test ROMs designed to test the edge cases of various operations.

Note that even many widely used emulators don't pass all the tests, but using the tests is still a good way to track my progress and potential issues.

## Legend

:white_check_mark: - Test passes

:x: - Test fails

:warning: - Functionality unimplemented

| Test number | cgb_sound | cpu_instrs         | dmg_sound | instr_timing | interrupt_time | mem_timing | mem_timing-2 | oam_bug            |
| ----------- | --------- | ----------         | --------- | ------------ | -------------- | ---------- | ------------ | -------            |
| 1           | :warning: | :x:                | :warning: | :x:          | :x:            | :x:        | :x:          | :x:                |
| 2           | :warning: | :white_check_mark: | :warning: |              |                | :x:        | :x:          | :x:                |
| 3           | :warning: | :white_check_mark: | :warning: |              |                | :x:        | :x:          | :white_check_mark: |
| 4           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 5           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 6           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :white_check_mark: |
| 7           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 8           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 9           | :warning: | :white_check_mark: | :warning: |              |                |            |              |                    |
| 10          | :warning: | :white_check_mark: | :warning: |              |                |            |              |                    |
| 11          | :warning: | :x:                | :warning: |              |                |            |              |                    |
| 11          | :warning: |                    | :warning: |              |                |            |              |                    |
