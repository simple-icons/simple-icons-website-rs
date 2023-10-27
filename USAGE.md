# Simple Icons website client usage

### Global URL parameters

You can pass the following parameters to the website to change the behaviour on load:

| Option         | Description                                                                                                                                                                                                                        | Default                                                                                                                                                                                                         | Example               |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| `lang`         | [Unicode language identifier] of the language to use by default. It will be saved as your preferred language in local storage.                                                                                                     | The preferred language is discovered using [`navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages) based on your browser preference, using English (`en-US`) as fallback. | `?lang=es`            |
| `color-scheme` | Color scheme to use by default. Possible values are `light`, `dark` and `system`.                                                                                                                                                  | By default is system and will be inferred from your browser settings.                                                                                                                                           | `?color-scheme=light` |
| `modal`        | Show modal to open when load. Possible values are `languages` (opens the language selector), `extensions` (open the third party extensions modal) and `icon` (open the details of the first icon, combine with `q` to specify it). | By default no modals are opened.                                                                                                                                                                                | `?modal=icon&q=3m`    |

## Index at `/`

Grid with all brand icons.

### URL parameters

| Option          | Description                                                                                                                     | Default                                                                         | Example              |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | -------------------- |
| `q`             | Value to set in the search bar.                                                                                                 | The search bar is empty by default and the alphabetical order mode is selected. | `?q=simpleicons`     |
| `download-type` | The type of download to set by controls. This affect to the download button of icon cards. Possible values are `svg` and `pdf`. | The download type is `svg` by default.                                          | `?download-type=pdf` |
| `layout`        | Icons grid layout to use. Possible values are `comfortable` and `compact`.                                                      | The layout is `comfortable` by default.                                         | `?layout=compact`    |

[Unicode language identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier

## Preview Generator at `/preview`

Used to include an image of the icon showing a preview of it in pull requests.

### Keyboard shortcuts

- <kbd>Ctrl</kbd> + <kbd>S</kbd>: Save preview
- <kbd>Ctrl</kbd> + <kbd>C</kbd>: Copy preview
- <kbd>Ctrl</kbd> + <kbd>↑</kbd>: Upload SVG
- <kbd>Ctrl</kbd> + <kbd>↓</kbd>: Download SVG
