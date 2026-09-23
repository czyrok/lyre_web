# Space motion

The spacing between elements shrinks progressively as the page is scrolled: the
top of a page is the roomy state, and everything tightens up over the first
viewport of scroll.

## Mechanism

Everything hangs off a single registered custom property declared in
`tailwind/root.css`, and a scale derived from it:

```css
@property --space-motion-progress {
  syntax: "<number>";
  inherits: true;
  initial-value: 0;
}

:root {
  --space-motion-scale: calc(1 - 0.4 * var(--space-motion-progress));
}
```

Registering it is what makes it interpolable — an unregistered custom property
is a plain token stream and CSS can only flip it from one keyframe to the next,
never tween it.

The split between the two matters: `--space-motion-progress` is a raw `0 → 1`
ramp, and the whole compaction curve lives in the one `calc()` that turns it into
a scale. Anything that can produce a progress figure can drive the motion —
which is what lets the Firefox fallback below reuse the curve instead of
restating it.

The `level` spacing scale comes in two halves — a static one holding the design
value, and an animated one multiplying it by the scale:

```css
--spacing-fixed-level1: 1rem;
--spacing-level1: calc(var(--spacing-fixed-level1) * var(--space-motion-scale));
```

Tailwind follows `var()` references when tree-shaking `@theme`, so the
`fixed-level` tokens survive even at the levels no utility names directly.

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
`initial-value: 0` of the property, i.e. the untouched design spacing.

`prefers-reduced-motion: no-preference` opts out the same way.

## Firefox

Firefox does not ship scroll-driven animations — MDN's compatibility data puts
`animation-timeline`, `scroll()` and `view()` at `preview`, meaning Nightly
only. The `@supports` guard therefore does its job and Firefox gets no motion at
all, which is correct but not what we want.

`assets/polyfills/space-motion.js` fills the gap, loaded from the polyfill block
in `src/system/route/shell.rs` under the same feature test the CSS uses:

```js
if (!CSS.supports('animation-timeline', 'scroll()')) {
    import('/polyfills/space-motion.js');
}
```

It sets `--space-motion-progress` on `<html>` from
`scrollY / innerHeight`, clamped to 1, on a `requestAnimationFrame`-throttled
passive scroll listener. Because it feeds the same `0 → 1` property the keyframes
animate, the compaction curve, the compacted value and the vertical-only rule all
stay defined once, in CSS — the fallback contributes a number and nothing else.
The one thing it does restate is the range: `scrollY / innerHeight` is the JS
spelling of `animation-range: 0 100vh`.

It honours `prefers-reduced-motion` itself, by clearing the property rather than
setting it, since the CSS `@media` guard only gates the animation it replaces.

## Tuning

| Knob | Location | Effect |
| --- | --- | --- |
| `0.4` in `--space-motion-scale` | `:root` rule | How tight the compacted state gets (`0.4` = 40% tighter) |
| `animation-range` | `:root` rule | Over how much scroll the compaction happens (mirror it in the polyfill) |
| `linear` | `:root` rule | The easing of the compaction against scroll distance |

## The motion is vertical only

Compaction applies to the block axis and never to the inline one. Tightening a
wrapped row sideways as the page scrolls drags buttons, tags and project cards
toward each other horizontally, which reads as the layout collapsing rather than
as rhythm.

`gap-levelN` is therefore safe **only on a `flex-col` container**, where the
`column-gap` half of the shorthand is inert. Anywhere the gap can manifest
horizontally — a `flex-wrap` row, a `flex-row` at some breakpoint, a grid with
columns — the two axes must be named separately:

```css
.section-projects {
  @apply flex flex-wrap justify-center;
  @apply gap-y-level2 gap-x-fixed-level2;
}
```

The sites that currently need the split are the wrapped action/tag rows
(`.section-actions`, `.top-part-actions`, `.details-tags`, `.more-part-actions`,
`.bottom-part-settings`, `.bottom-part-legal`, the selector row in
`ordered_project_context_filter.rs`), the wrapped card lists
(`.section-projects`, `.middle-part-list`), the timeline
(`.section-text-timeline`, `.timeline-item`), and `.intro-details`, which was
already inline-only via `gap-x-`.

**Adding a horizontal gap means reaching for `fixed-level`, not `level`.** A bare
`gap-levelN` on a new `flex-wrap` row is the easy mistake — it looks right until
someone scrolls.

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
