# myw

`myw` ("my week") is an esoteric daylog/timetracker-hybrid built to work with plain markdown files.

⚠️ This is **very** _work in progress_. Please **don't use this yet**. Current [plans](#plans) include cli commands for editing and reporting, as well as an interactive calendar TUI.

## Example

Let's pretend it's still 2024-02-13 and we're keeping a week log in [`2024-w07.md`](./resources/2024-w07-example.md):

```md
# What a week

Note: this is the [example file from the `README`](./README.md#example).

## 2024-02-12

Let's pretend this is what my day looked like:

- 07:30 - 09:00: ABC
- 09:00 - 10:00: DEF (let's pretend all project names are this simple)
- 10:00 - 12:30: ABC everything after the first word is treated as "notes"
  - I'm not sure yet what to do with "notes"
  - oh and sub-lists like this are currently ignored entirely
- 12:30 - 13:00: **ABC** extra _formatting_ is ignored
- unrelated top-level items are ignored too
- 14 - 16: GHI (minutes are optional)
- 16 17 GHI (as is most syntax)
- 1700 1730 "Some project" quoted project names like this will work at some point
  - or maybe I'll stop supporting inline notes, and treat sub-lists as notes?

Well that was it for this lovely monday.

## 2024-02-13 (putting the date somewhere in the heading is important)

Oh, and dates need to be formatted like `yyyy-mm-dd`.

- 0800 1100: ABC let's start with some ABC again
- 1100 1300: DEF
  - I'm getting tired of writing this example so let's stop here
```

Now that we've logged something we can generate a report:

<pre><code>$ myw report 2024-w07.md
<strong>2024-02-12</strong>: 9
ABC: 4.5
DEF: 1
GHI: 3
Some project: 0.5

<strong>2024-02-13</strong>: 5
ABC: 3
DEF: 2

<strong>Total</strong>: 14
</code></pre>

## Plans

The short term goal is reliable basic cli commands for reporting. The **v1** milestone goals include cli commands for editing, as well as some form of interactive calendar. For v1 this can be a TUI. Perhaps something like this, but with better glyphs:

<pre><code>$ myw
<strong>2024-02-13</strong>
  9:00        10:00       11:00       12:00       13:00       14:00       15:00
  v  .  .  .  v  .  .  .  v  .  .  .  v  .  .  .  v  .  .  .  v  .  .  .  v  .
  -------------------------------------------------
  |                       |            |>          |
  |         VLA           |    MGO     |>   █      |
  |_______________________|____________|>__________|</code></pre>

Hotkeys could be something like:

- `c`: create
- `jkhl`: change selection
- `JKHL`: move selected entry's "from" or "until"
- `space`: toggle _move_ between "from" and "until"
- `enter`: edit entry's project

Long term goals could include a GUI, perhaps a system tray widget, if that's not too much work for a "nice to have".

### TODO

- [x] Parse entries (time)
- [x] Parse entries (datetime)
- [x] Report sum per day, sum per project and total (`myw r <file>`)
- [ ] Detect and warn about overlap
- [ ] Config, default directory
  - [ ] Onboarding flow to get path
  - [ ] Default to file containing current day (`myw r`)
  - [ ] Interactive file select if multiple files match date / have entries for date
- [x] Quoted project names
- [ ] Better project name handling
  - [ ] Case insensitivity
  - [ ] Known project names (canonical names) in config
  - [ ] Configurable project sort order
  - [ ] Project groups (config)
- [ ] Edit command (`myw e`, `myw e yyyy-mm-dd`)
  - Opens file that has entries for given date in `$EDITOR`
  - [ ] Interactive file select if multiple files match
  - [ ] Support relative date arguments (`myw e w-1`)
- [ ] Add command (`myw a hhmm hhmm project`, `myw a yyyy-mm-dd hhmm hhmm project`)
  - [ ] Insert after existing entries for day
    - Add after closest preceding entry's list item (after sub-list, if any)
    - Detect list bullet character from preceding list item
  - [ ] Insert as new list after existing heading for day (if there are no entries yet)
  - [ ] Insert with new heading at same level as existing heading day headings before closest later date (if any) or end of file, or add new h2(?) before end of file
  - [ ] Project name autocomplete
- [ ] Gracefully handle nested date headings
- [ ] Better error handling
- [ ] Interactive edit (`myw i`)
  - [ ] Calendar TUI for single day (🏆 **v1 milestone**)
  - [ ] Calendar TUI for all days in file
  - [ ] Calendar TUI for all days in given range (myw i `2024-W07`, `myw i 2024-02`)
    - [ ] Automatically create file(s) based on the given range and the existing file(s)
  - [ ] Project name autocomplete
- [ ] Timezone support
- [ ] Support entries across day boundaries
- [ ] Pretty output
- [ ] Support time-only mode (generate per-project report for file without date headings and without date in filename)

### Musings on dates and date ranges

Some commands might take a date-like argument. You could imagine such an argument to look ISO8601-inspired and to be either absolute or relative (`2024-02-12` or `d-1` for yesterday, `2024-W06` or `w-1` for last week).

Here are some ideas on how that could work:

- date:
  - absolute: 2024-02-12, 2024-2-12
  - relative: d+5, d-5
- month:
  - absolute: 2024-02, 2024-2
  - relative: m+1, m-1
- year:
  - absolute: 2024
  - relative: y+1, y-1
- week (ISO week number obviously):
  - absolute, current year: w07, w7
  - absolute with year: 2023-w51, 2023w51
  - relative: w+1, w-1

You could also imagine an argument like this implying a date range (a week, month or year), depending on which command it's used with. For example, `myw report m-1` could collect all entries for last month from across multiple files to collate a single report for that month.
