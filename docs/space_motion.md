# Space motion

The spacing between elements shrinks progressively as the page is scrolled: the
top of a page is the roomy state, and everything tightens up over the first
viewport of scroll.

## Mechanism

Everything hangs off a single registered custom property declared in
`tailwind/root.css`:

```css
@property --space-motion-scale {
  syntax: "<number>";
  inherits: true;
  initial-value: 1;
}
```

Registering it is what makes it interpolable — an unregistered custom property
is a plain token stream and CSS can only flip it from one keyframe to the next,
never tween it.

The `level` spacing scale multiplies its design value by that scale:

```css
--spacing-level1: calc(1rem * var(--space-motion-scale));
```

`--spacing-level*` is substituted lazily, at the point of use, so every
`gap-level2`, `py-level6`, `mt-level2` in the tree picks up the animated value
inherited from `:root`. Nothing needs to opt in.

Driving it is a scroll-progress timeline on the root element:

```css
:root {
  animation: space-motion-compaction linear both;
  animation-timeline: scroll(root block);
  animation-range: 0 100vh;
}
```

## Why the range is absolute

`animation-range: 0 100vh` is deliberate, not a stylistic choice. The default
`normal` range maps progress to a *percentage* of the scrollable length — but
compacting the spacing shortens the document, which raises the percentage at a
fixed scroll position, which compacts it further. That feedback loop reads as
jitter while scrolling.

An absolute range maps `scrollTop ∈ [0, 100vh]` onto the animation regardless of
how tall the document currently is, so the loop never closes. Any change to the
range must stay in absolute units for the same reason.

## Why the `@supports` guard

`animation-timeline` has no effect in a browser that does not implement
scroll-driven animations, but `animation-name` still does. The animation would
then run on the default *time* timeline with a `0s` duration and, because of
`both`, settle on the `to` keyframe on the first frame — the page would render
permanently compacted instead of simply not animating.

`@supports (animation-timeline: scroll())` keeps those browsers on the
`initial-value: 1` of the property, i.e. the untouched design spacing.

`prefers-reduced-motion: no-preference` opts out the same way.

## Tuning

| Knob | Location | Effect |
| --- | --- | --- |
| `to { --space-motion-scale }` | `@keyframes space-motion-compaction` | How tight the compacted state is (`0.6` = 40% tighter) |
| `animation-range` | `:root` rule | Over how much scroll the compaction happens |
| `linear` | `:root` rule | The easing of the compaction against scroll distance |

## Scope

Only the `level` scale moves. The rest of `--spacing-*` also feeds widths and
heights (`w-41`, `min-w-33`, `max-w-103`), so scaling it would shrink the
contact photo and the project thumbnails along with the gaps.

Spacing that should take part in the motion therefore has to be expressed with a
`level` token. `.landing-page-content` is the precedent: its `py-25` / `gap-16`
became `py-level6` / `gap-level5`, which is what `--spacing-level5: 8rem` and
`--spacing-level6: 12.5rem` exist for.

Padding *inside* a component — `.landing-page-section-container`'s
`p-4 sm:p-6 md:p-8`, `.landing-page-highlighted-section`'s `py-10`,
`.secondary-page-layout-intro`'s `pt-16 pb-4` — is left static on purpose: it is
the breathing room of a block, not the distance between two of them.
