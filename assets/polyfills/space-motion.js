const root = document.documentElement;
const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');

let scheduled = false;

const apply = () => {
  scheduled = false;

  if (reducedMotion.matches) {
    root.style.removeProperty('--space-motion-progress');

    return;
  }

  //// Mirrors `animation-range: 0 100vh` on the CSS side
  const progress = Math.min(window.scrollY / window.innerHeight, 1);

  root.style.setProperty('--space-motion-progress', String(progress));
};

const schedule = () => {
  if (scheduled) return;

  scheduled = true;

  requestAnimationFrame(apply);
};

window.addEventListener('scroll', schedule, { passive: true });
window.addEventListener('resize', schedule, { passive: true });
reducedMotion.addEventListener('change', schedule);

apply();
