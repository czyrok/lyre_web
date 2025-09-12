var po = Object.defineProperty, go = Object.defineProperties;
var mo = Object.getOwnPropertyDescriptors;
var An = Object.getOwnPropertySymbols;
var ko = Object.prototype.hasOwnProperty, So = Object.prototype.propertyIsEnumerable;
var vn = (t, e, n) => e in t ? po(t, e, { enumerable: !0, configurable: !0, writable: !0, value: n }) : t[e] = n, W = (t, e) => {
  for (var n in e || (e = {}))
    ko.call(e, n) && vn(t, n, e[n]);
  if (An)
    for (var n of An(e))
      So.call(e, n) && vn(t, n, e[n]);
  return t;
}, q = (t, e) => go(t, mo(e));
var R = (t, e, n) => new Promise((r, s) => {
  var o = (u) => {
    try {
      l(n.next(u));
    } catch (i) {
      s(i);
    }
  }, a = (u) => {
    try {
      l(n.throw(u));
    } catch (i) {
      s(i);
    }
  }, l = (u) => u.done ? r(u.value) : Promise.resolve(u.value).then(o, a);
  l((n = n.apply(t, e)).next());
});
const Ze = Math.min, Vt = Math.max, Ce = Math.round, he = Math.floor, kt = (t) => ({
  x: t,
  y: t
});
function yo(t, e) {
  return typeof t == "function" ? t(e) : t;
}
function bo(t) {
  return W({
    top: 0,
    right: 0,
    bottom: 0,
    left: 0
  }, t);
}
function xo(t) {
  return typeof t != "number" ? bo(t) : {
    top: t,
    right: t,
    bottom: t,
    left: t
  };
}
function we(t) {
  const {
    x: e,
    y: n,
    width: r,
    height: s
  } = t;
  return {
    width: r,
    height: s,
    top: n,
    left: e,
    right: e + r,
    bottom: n + s,
    x: e,
    y: n
  };
}
function Co(t, e) {
  return R(this, null, function* () {
    var n;
    e === void 0 && (e = {});
    const {
      x: r,
      y: s,
      platform: o,
      rects: a,
      elements: l,
      strategy: u
    } = t, {
      boundary: i = "clippingAncestors",
      rootBoundary: c = "viewport",
      elementContext: h = "floating",
      altBoundary: f = !1,
      padding: p = 0
    } = yo(e, t), g = xo(p), S = l[f ? h === "floating" ? "reference" : "floating" : h], b = we(yield o.getClippingRect({
      element: (n = yield o.isElement == null ? void 0 : o.isElement(S)) == null || n ? S : S.contextElement || (yield o.getDocumentElement == null ? void 0 : o.getDocumentElement(l.floating)),
      boundary: i,
      rootBoundary: c,
      strategy: u
    })), C = h === "floating" ? {
      x: r,
      y: s,
      width: a.floating.width,
      height: a.floating.height
    } : a.reference, v = yield o.getOffsetParent == null ? void 0 : o.getOffsetParent(l.floating), M = (yield o.isElement == null ? void 0 : o.isElement(v)) ? (yield o.getScale == null ? void 0 : o.getScale(v)) || {
      x: 1,
      y: 1
    } : {
      x: 1,
      y: 1
    }, k = we(o.convertOffsetParentRelativeRectToViewportRelativeRect ? yield o.convertOffsetParentRelativeRectToViewportRelativeRect({
      elements: l,
      rect: C,
      offsetParent: v,
      strategy: u
    }) : C);
    return {
      top: (b.top - k.top + g.top) / M.y,
      bottom: (k.bottom - b.bottom + g.bottom) / M.y,
      left: (b.left - k.left + g.left) / M.x,
      right: (k.right - b.right + g.right) / M.x
    };
  });
}
function Pe() {
  return typeof window != "undefined";
}
function Zt(t) {
  return Or(t) ? (t.nodeName || "").toLowerCase() : "#document";
}
function st(t) {
  var e;
  return (t == null || (e = t.ownerDocument) == null ? void 0 : e.defaultView) || window;
}
function yt(t) {
  var e;
  return (e = (Or(t) ? t.ownerDocument : t.document) || window.document) == null ? void 0 : e.documentElement;
}
function Or(t) {
  return Pe() ? t instanceof Node || t instanceof st(t).Node : !1;
}
function lt(t) {
  return Pe() ? t instanceof Element || t instanceof st(t).Element : !1;
}
function St(t) {
  return Pe() ? t instanceof HTMLElement || t instanceof st(t).HTMLElement : !1;
}
function En(t) {
  return !Pe() || typeof ShadowRoot == "undefined" ? !1 : t instanceof ShadowRoot || t instanceof st(t).ShadowRoot;
}
function ce(t) {
  const {
    overflow: e,
    overflowX: n,
    overflowY: r,
    display: s
  } = ct(t);
  return /auto|scroll|overlay|hidden|clip/.test(e + r + n) && !["inline", "contents"].includes(s);
}
function wo(t) {
  return ["table", "td", "th"].includes(Zt(t));
}
function Oe(t) {
  return [":popover-open", ":modal"].some((e) => {
    try {
      return t.matches(e);
    } catch (n) {
      return !1;
    }
  });
}
function dn(t) {
  const e = gn(), n = lt(t) ? ct(t) : t;
  return ["transform", "translate", "scale", "rotate", "perspective"].some((r) => n[r] ? n[r] !== "none" : !1) || (n.containerType ? n.containerType !== "normal" : !1) || !e && (n.backdropFilter ? n.backdropFilter !== "none" : !1) || !e && (n.filter ? n.filter !== "none" : !1) || ["transform", "translate", "scale", "rotate", "perspective", "filter"].some((r) => (n.willChange || "").includes(r)) || ["paint", "layout", "strict", "content"].some((r) => (n.contain || "").includes(r));
}
function To(t) {
  let e = Lt(t);
  for (; St(e) && !Kt(e); ) {
    if (dn(e))
      return e;
    if (Oe(e))
      return null;
    e = Lt(e);
  }
  return null;
}
function gn() {
  return typeof CSS == "undefined" || !CSS.supports ? !1 : CSS.supports("-webkit-backdrop-filter", "none");
}
function Kt(t) {
  return ["html", "body", "#document"].includes(Zt(t));
}
function ct(t) {
  return st(t).getComputedStyle(t);
}
function Re(t) {
  return lt(t) ? {
    scrollLeft: t.scrollLeft,
    scrollTop: t.scrollTop
  } : {
    scrollLeft: t.scrollX,
    scrollTop: t.scrollY
  };
}
function Lt(t) {
  if (Zt(t) === "html")
    return t;
  const e = (
    // Step into the shadow DOM of the parent of a slotted node.
    t.assignedSlot || // DOM Element detected.
    t.parentNode || // ShadowRoot detected.
    En(t) && t.host || // Fallback.
    yt(t)
  );
  return En(e) ? e.host : e;
}
function Rr(t) {
  const e = Lt(t);
  return Kt(e) ? t.ownerDocument ? t.ownerDocument.body : t.body : St(e) && ce(e) ? e : Rr(e);
}
function oe(t, e, n) {
  var r;
  e === void 0 && (e = []), n === void 0 && (n = !0);
  const s = Rr(t), o = s === ((r = t.ownerDocument) == null ? void 0 : r.body), a = st(s);
  if (o) {
    const l = tn(a);
    return e.concat(a, a.visualViewport || [], ce(s) ? s : [], l && n ? oe(l) : []);
  }
  return e.concat(s, oe(s, [], n));
}
function tn(t) {
  return t.parent && Object.getPrototypeOf(t.parent) ? t.frameElement : null;
}
function Ir(t) {
  const e = ct(t);
  let n = parseFloat(e.width) || 0, r = parseFloat(e.height) || 0;
  const s = St(t), o = s ? t.offsetWidth : n, a = s ? t.offsetHeight : r, l = Ce(n) !== o || Ce(r) !== a;
  return l && (n = o, r = a), {
    width: n,
    height: r,
    $: l
  };
}
function mn(t) {
  return lt(t) ? t : t.contextElement;
}
function Gt(t) {
  const e = mn(t);
  if (!St(e))
    return kt(1);
  const n = e.getBoundingClientRect(), {
    width: r,
    height: s,
    $: o
  } = Ir(e);
  let a = (o ? Ce(n.width) : n.width) / r, l = (o ? Ce(n.height) : n.height) / s;
  return (!a || !Number.isFinite(a)) && (a = 1), (!l || !Number.isFinite(l)) && (l = 1), {
    x: a,
    y: l
  };
}
const Ao = /* @__PURE__ */ kt(0);
function _r(t) {
  const e = st(t);
  return !gn() || !e.visualViewport ? Ao : {
    x: e.visualViewport.offsetLeft,
    y: e.visualViewport.offsetTop
  };
}
function vo(t, e, n) {
  return e === void 0 && (e = !1), !n || e && n !== st(t) ? !1 : e;
}
function Nt(t, e, n, r) {
  e === void 0 && (e = !1), n === void 0 && (n = !1);
  const s = t.getBoundingClientRect(), o = mn(t);
  let a = kt(1);
  e && (r ? lt(r) && (a = Gt(r)) : a = Gt(t));
  const l = vo(o, n, r) ? _r(o) : kt(0);
  let u = (s.left + l.x) / a.x, i = (s.top + l.y) / a.y, c = s.width / a.x, h = s.height / a.y;
  if (o) {
    const f = st(o), p = r && lt(r) ? st(r) : r;
    let g = f, m = tn(g);
    for (; m && r && p !== g; ) {
      const S = Gt(m), b = m.getBoundingClientRect(), C = ct(m), v = b.left + (m.clientLeft + parseFloat(C.paddingLeft)) * S.x, M = b.top + (m.clientTop + parseFloat(C.paddingTop)) * S.y;
      u *= S.x, i *= S.y, c *= S.x, h *= S.y, u += v, i += M, g = st(m), m = tn(g);
    }
  }
  return we({
    width: c,
    height: h,
    x: u,
    y: i
  });
}
function kn(t, e) {
  const n = Re(t).scrollLeft;
  return e ? e.left + n : Nt(yt(t)).left + n;
}
function Nr(t, e, n) {
  n === void 0 && (n = !1);
  const r = t.getBoundingClientRect(), s = r.left + e.scrollLeft - (n ? 0 : (
    // RTL <body> scrollbar.
    kn(t, r)
  )), o = r.top + e.scrollTop;
  return {
    x: s,
    y: o
  };
}
function Eo(t) {
  let {
    elements: e,
    rect: n,
    offsetParent: r,
    strategy: s
  } = t;
  const o = s === "fixed", a = yt(r), l = e ? Oe(e.floating) : !1;
  if (r === a || l && o)
    return n;
  let u = {
    scrollLeft: 0,
    scrollTop: 0
  }, i = kt(1);
  const c = kt(0), h = St(r);
  if ((h || !h && !o) && ((Zt(r) !== "body" || ce(a)) && (u = Re(r)), St(r))) {
    const p = Nt(r);
    i = Gt(r), c.x = p.x + r.clientLeft, c.y = p.y + r.clientTop;
  }
  const f = a && !h && !o ? Nr(a, u, !0) : kt(0);
  return {
    width: n.width * i.x,
    height: n.height * i.y,
    x: n.x * i.x - u.scrollLeft * i.x + c.x + f.x,
    y: n.y * i.y - u.scrollTop * i.y + c.y + f.y
  };
}
function $o(t) {
  return Array.from(t.getClientRects());
}
function Lo(t) {
  const e = yt(t), n = Re(t), r = t.ownerDocument.body, s = Vt(e.scrollWidth, e.clientWidth, r.scrollWidth, r.clientWidth), o = Vt(e.scrollHeight, e.clientHeight, r.scrollHeight, r.clientHeight);
  let a = -n.scrollLeft + kn(t);
  const l = -n.scrollTop;
  return ct(r).direction === "rtl" && (a += Vt(e.clientWidth, r.clientWidth) - s), {
    width: s,
    height: o,
    x: a,
    y: l
  };
}
function Po(t, e) {
  const n = st(t), r = yt(t), s = n.visualViewport;
  let o = r.clientWidth, a = r.clientHeight, l = 0, u = 0;
  if (s) {
    o = s.width, a = s.height;
    const i = gn();
    (!i || i && e === "fixed") && (l = s.offsetLeft, u = s.offsetTop);
  }
  return {
    width: o,
    height: a,
    x: l,
    y: u
  };
}
function Oo(t, e) {
  const n = Nt(t, !0, e === "fixed"), r = n.top + t.clientTop, s = n.left + t.clientLeft, o = St(t) ? Gt(t) : kt(1), a = t.clientWidth * o.x, l = t.clientHeight * o.y, u = s * o.x, i = r * o.y;
  return {
    width: a,
    height: l,
    x: u,
    y: i
  };
}
function $n(t, e, n) {
  let r;
  if (e === "viewport")
    r = Po(t, n);
  else if (e === "document")
    r = Lo(yt(t));
  else if (lt(e))
    r = Oo(e, n);
  else {
    const s = _r(t);
    r = {
      x: e.x - s.x,
      y: e.y - s.y,
      width: e.width,
      height: e.height
    };
  }
  return we(r);
}
function Dr(t, e) {
  const n = Lt(t);
  return n === e || !lt(n) || Kt(n) ? !1 : ct(n).position === "fixed" || Dr(n, e);
}
function Ro(t, e) {
  const n = e.get(t);
  if (n)
    return n;
  let r = oe(t, [], !1).filter((l) => lt(l) && Zt(l) !== "body"), s = null;
  const o = ct(t).position === "fixed";
  let a = o ? Lt(t) : t;
  for (; lt(a) && !Kt(a); ) {
    const l = ct(a), u = dn(a);
    !u && l.position === "fixed" && (s = null), (o ? !u && !s : !u && l.position === "static" && !!s && ["absolute", "fixed"].includes(s.position) || ce(a) && !u && Dr(t, a)) ? r = r.filter((c) => c !== a) : s = l, a = Lt(a);
  }
  return e.set(t, r), r;
}
function Io(t) {
  let {
    element: e,
    boundary: n,
    rootBoundary: r,
    strategy: s
  } = t;
  const a = [...n === "clippingAncestors" ? Oe(e) ? [] : Ro(e, this._c) : [].concat(n), r], l = a[0], u = a.reduce((i, c) => {
    const h = $n(e, c, s);
    return i.top = Vt(h.top, i.top), i.right = Ze(h.right, i.right), i.bottom = Ze(h.bottom, i.bottom), i.left = Vt(h.left, i.left), i;
  }, $n(e, l, s));
  return {
    width: u.right - u.left,
    height: u.bottom - u.top,
    x: u.left,
    y: u.top
  };
}
function _o(t) {
  const {
    width: e,
    height: n
  } = Ir(t);
  return {
    width: e,
    height: n
  };
}
function No(t, e, n) {
  const r = St(e), s = yt(e), o = n === "fixed", a = Nt(t, !0, o, e);
  let l = {
    scrollLeft: 0,
    scrollTop: 0
  };
  const u = kt(0);
  function i() {
    u.x = kn(s);
  }
  if (r || !r && !o)
    if ((Zt(e) !== "body" || ce(s)) && (l = Re(e)), r) {
      const p = Nt(e, !0, o, e);
      u.x = p.x + e.clientLeft, u.y = p.y + e.clientTop;
    } else s && i();
  o && !r && s && i();
  const c = s && !r && !o ? Nr(s, l) : kt(0), h = a.left + l.scrollLeft - u.x - c.x, f = a.top + l.scrollTop - u.y - c.y;
  return {
    x: h,
    y: f,
    width: a.width,
    height: a.height
  };
}
function _e(t) {
  return ct(t).position === "static";
}
function Ln(t, e) {
  if (!St(t) || ct(t).position === "fixed")
    return null;
  if (e)
    return e(t);
  let n = t.offsetParent;
  return yt(t) === n && (n = n.ownerDocument.body), n;
}
function Fr(t, e) {
  const n = st(t);
  if (Oe(t))
    return n;
  if (!St(t)) {
    let s = Lt(t);
    for (; s && !Kt(s); ) {
      if (lt(s) && !_e(s))
        return s;
      s = Lt(s);
    }
    return n;
  }
  let r = Ln(t, e);
  for (; r && wo(r) && _e(r); )
    r = Ln(r, e);
  return r && Kt(r) && _e(r) && !dn(r) ? n : r || To(t) || n;
}
const Do = function(t) {
  return R(this, null, function* () {
    const e = this.getOffsetParent || Fr, n = this.getDimensions, r = yield n(t.floating);
    return {
      reference: No(t.reference, yield e(t.floating), t.strategy),
      floating: {
        x: 0,
        y: 0,
        width: r.width,
        height: r.height
      }
    };
  });
};
function Fo(t) {
  return ct(t).direction === "rtl";
}
const H = {
  convertOffsetParentRelativeRectToViewportRelativeRect: Eo,
  getDocumentElement: yt,
  getClippingRect: Io,
  getOffsetParent: Fr,
  getElementRects: Do,
  getClientRects: $o,
  getDimensions: _o,
  getScale: Gt,
  isElement: lt,
  isRTL: Fo
};
function Mr(t, e) {
  return t.x === e.x && t.y === e.y && t.width === e.width && t.height === e.height;
}
function Mo(t, e) {
  let n = null, r;
  const s = yt(t);
  function o() {
    var l;
    clearTimeout(r), (l = n) == null || l.disconnect(), n = null;
  }
  function a(l, u) {
    l === void 0 && (l = !1), u === void 0 && (u = 1), o();
    const i = t.getBoundingClientRect(), {
      left: c,
      top: h,
      width: f,
      height: p
    } = i;
    if (l || e(), !f || !p)
      return;
    const g = he(h), m = he(s.clientWidth - (c + f)), S = he(s.clientHeight - (h + p)), b = he(c), v = {
      rootMargin: -g + "px " + -m + "px " + -S + "px " + -b + "px",
      threshold: Vt(0, Ze(1, u)) || 1
    };
    let M = !0;
    function k(O) {
      const w = O[0].intersectionRatio;
      if (w !== u) {
        if (!M)
          return a();
        w ? a(!1, w) : r = setTimeout(() => {
          a(!1, 1e-7);
        }, 1e3);
      }
      w === 1 && !Mr(i, t.getBoundingClientRect()) && a(), M = !1;
    }
    try {
      n = new IntersectionObserver(k, q(W({}, v), {
        // Handle <iframe>s
        root: s.ownerDocument
      }));
    } catch (O) {
      n = new IntersectionObserver(k, v);
    }
    n.observe(t);
  }
  return a(!0), o;
}
function en(t, e, n, r) {
  r === void 0 && (r = {});
  const {
    ancestorScroll: s = !0,
    ancestorResize: o = !0,
    elementResize: a = typeof ResizeObserver == "function",
    layoutShift: l = typeof IntersectionObserver == "function",
    animationFrame: u = !1
  } = r, i = mn(t), c = s || o ? [...i ? oe(i) : [], ...oe(e)] : [];
  c.forEach((b) => {
    s && b.addEventListener("scroll", n, {
      passive: !0
    }), o && b.addEventListener("resize", n);
  });
  const h = i && l ? Mo(i, n) : null;
  let f = -1, p = null;
  a && (p = new ResizeObserver((b) => {
    let [C] = b;
    C && C.target === i && p && (p.unobserve(e), cancelAnimationFrame(f), f = requestAnimationFrame(() => {
      var v;
      (v = p) == null || v.observe(e);
    })), n();
  }), i && !u && p.observe(i), p.observe(e));
  let g, m = u ? Nt(t) : null;
  u && S();
  function S() {
    const b = Nt(t);
    m && !Mr(m, b) && n(), m = b, g = requestAnimationFrame(S);
  }
  return n(), () => {
    var b;
    c.forEach((C) => {
      s && C.removeEventListener("scroll", n), o && C.removeEventListener("resize", n);
    }), h == null || h(), (b = p) == null || b.disconnect(), p = null, u && cancelAnimationFrame(g);
  };
}
const jo = Co, { hasOwnProperty: Sn } = Object.prototype, ee = function() {
};
function Pn(t) {
  return typeof t == "function" ? t : ee;
}
function On(t, e) {
  return function(n, r, s) {
    n.type === e && t.call(this, n, r, s);
  };
}
function Bo(t, e) {
  const n = e.structure, r = [];
  for (const s in n) {
    if (Sn.call(n, s) === !1)
      continue;
    let o = n[s];
    const a = {
      name: s,
      type: !1,
      nullable: !1
    };
    Array.isArray(o) || (o = [o]);
    for (const l of o)
      l === null ? a.nullable = !0 : typeof l == "string" ? a.type = "node" : Array.isArray(l) && (a.type = "list");
    a.type && r.push(a);
  }
  return r.length ? {
    context: e.walkContext,
    fields: r
  } : null;
}
function Uo(t) {
  const e = {};
  for (const n in t.node)
    if (Sn.call(t.node, n)) {
      const r = t.node[n];
      if (!r.structure)
        throw new Error("Missed `structure` field in `" + n + "` node type definition");
      e[n] = Bo(n, r);
    }
  return e;
}
function Rn(t, e) {
  const n = t.fields.slice(), r = t.context, s = typeof r == "string";
  return e && n.reverse(), function(o, a, l, u) {
    let i;
    s && (i = a[r], a[r] = o);
    for (const c of n) {
      const h = o[c.name];
      if (!c.nullable || h) {
        if (c.type === "list") {
          if (e ? h.reduceRight(u, !1) : h.reduce(u, !1))
            return !0;
        } else if (l(h))
          return !0;
      }
    }
    s && (a[r] = i);
  };
}
function In({
  StyleSheet: t,
  Atrule: e,
  Rule: n,
  Block: r,
  DeclarationList: s
}) {
  return {
    Atrule: {
      StyleSheet: t,
      Atrule: e,
      Rule: n,
      Block: r
    },
    Rule: {
      StyleSheet: t,
      Atrule: e,
      Rule: n,
      Block: r
    },
    Declaration: {
      StyleSheet: t,
      Atrule: e,
      Rule: n,
      Block: r,
      DeclarationList: s
    }
  };
}
function Wo(t) {
  const e = Uo(t), n = {}, r = {}, s = Symbol("break-walk"), o = Symbol("skip-node");
  for (const i in e)
    Sn.call(e, i) && e[i] !== null && (n[i] = Rn(e[i], !1), r[i] = Rn(e[i], !0));
  const a = In(n), l = In(r), u = function(i, c) {
    function h(b, C, v) {
      const M = f.call(S, b, C, v);
      return M === s ? !0 : M === o ? !1 : !!(g.hasOwnProperty(b.type) && g[b.type](b, S, h, m) || p.call(S, b, C, v) === s);
    }
    let f = ee, p = ee, g = n, m = (b, C, v, M) => b || h(C, v, M);
    const S = {
      break: s,
      skip: o,
      root: i,
      stylesheet: null,
      atrule: null,
      atrulePrelude: null,
      rule: null,
      selector: null,
      block: null,
      declaration: null,
      function: null
    };
    if (typeof c == "function")
      f = c;
    else if (c && (f = Pn(c.enter), p = Pn(c.leave), c.reverse && (g = r), c.visit)) {
      if (a.hasOwnProperty(c.visit))
        g = c.reverse ? l[c.visit] : a[c.visit];
      else if (!e.hasOwnProperty(c.visit))
        throw new Error("Bad value `" + c.visit + "` for `visit` option (should be: " + Object.keys(e).sort().join(", ") + ")");
      f = On(f, c.visit), p = On(p, c.visit);
    }
    if (f === ee && p === ee)
      throw new Error("Neither `enter` nor `leave` walker handler is set or both aren't a function");
    h(i);
  };
  return u.break = s, u.skip = o, u.find = function(i, c) {
    let h = null;
    return u(i, function(f, p, g) {
      if (c.call(this, f, p, g))
        return h = f, s;
    }), h;
  }, u.findLast = function(i, c) {
    let h = null;
    return u(i, {
      reverse: !0,
      enter(f, p, g) {
        if (c.call(this, f, p, g))
          return h = f, s;
      }
    }), h;
  }, u.findAll = function(i, c) {
    const h = [];
    return u(i, function(f, p, g) {
      c.call(this, f, p, g) && h.push(f);
    }), h;
  }, u;
}
const At = 0, d = 1, T = 2, z = 3, N = 4, wt = 5, zo = 6, Q = 7, ot = 8, P = 9, x = 10, F = 11, $ = 12, U = 13, Ie = 14, et = 15, X = 16, tt = 17, ht = 18, te = 19, ae = 20, I = 21, y = 22, ut = 23, Qt = 24, Y = 25, Ho = 0;
function rt(t) {
  return t >= 48 && t <= 57;
}
function Yt(t) {
  return rt(t) || // 0 .. 9
  t >= 65 && t <= 70 || // A .. F
  t >= 97 && t <= 102;
}
function yn(t) {
  return t >= 65 && t <= 90;
}
function Vo(t) {
  return t >= 97 && t <= 122;
}
function Go(t) {
  return yn(t) || Vo(t);
}
function qo(t) {
  return t >= 128;
}
function Te(t) {
  return Go(t) || qo(t) || t === 95;
}
function jr(t) {
  return Te(t) || rt(t) || t === 45;
}
function Ko(t) {
  return t >= 0 && t <= 8 || t === 11 || t >= 14 && t <= 31 || t === 127;
}
function Ae(t) {
  return t === 10 || t === 13 || t === 12;
}
function Dt(t) {
  return Ae(t) || t === 32 || t === 9;
}
function mt(t, e) {
  return !(t !== 92 || Ae(e) || e === Ho);
}
function Ne(t, e, n) {
  return t === 45 ? Te(e) || e === 45 || mt(e, n) : Te(t) ? !0 : t === 92 ? mt(t, e) : !1;
}
function De(t, e, n) {
  return t === 43 || t === 45 ? rt(e) ? 2 : e === 46 && rt(n) ? 3 : 0 : t === 46 ? rt(e) ? 2 : 0 : rt(t) ? 1 : 0;
}
function Br(t) {
  return t === 65279 || t === 65534 ? 1 : 0;
}
const nn = new Array(128), Qo = 128, ke = 130, Ur = 131, bn = 132, Wr = 133;
for (let t = 0; t < nn.length; t++)
  nn[t] = Dt(t) && ke || rt(t) && Ur || Te(t) && bn || Ko(t) && Wr || t || Qo;
function Fe(t) {
  return t < 128 ? nn[t] : bn;
}
function qt(t, e) {
  return e < t.length ? t.charCodeAt(e) : 0;
}
function rn(t, e, n) {
  return n === 13 && qt(t, e + 1) === 10 ? 2 : 1;
}
function zr(t, e, n) {
  let r = t.charCodeAt(e);
  return yn(r) && (r = r | 32), r === n;
}
function ve(t, e, n, r) {
  if (n - e !== r.length || e < 0 || n > t.length)
    return !1;
  for (let s = e; s < n; s++) {
    const o = r.charCodeAt(s - e);
    let a = t.charCodeAt(s);
    if (yn(a) && (a = a | 32), a !== o)
      return !1;
  }
  return !0;
}
function Yo(t, e) {
  for (; e >= 0 && Dt(t.charCodeAt(e)); e--)
    ;
  return e + 1;
}
function fe(t, e) {
  for (; e < t.length && Dt(t.charCodeAt(e)); e++)
    ;
  return e;
}
function Me(t, e) {
  for (; e < t.length && rt(t.charCodeAt(e)); e++)
    ;
  return e;
}
function Xt(t, e) {
  if (e += 2, Yt(qt(t, e - 1))) {
    for (const r = Math.min(t.length, e + 5); e < r && Yt(qt(t, e)); e++)
      ;
    const n = qt(t, e);
    Dt(n) && (e += rn(t, e, n));
  }
  return e;
}
function pe(t, e) {
  for (; e < t.length; e++) {
    const n = t.charCodeAt(e);
    if (!jr(n)) {
      if (mt(n, qt(t, e + 1))) {
        e = Xt(t, e) - 1;
        continue;
      }
      break;
    }
  }
  return e;
}
function Hr(t, e) {
  let n = t.charCodeAt(e);
  if ((n === 43 || n === 45) && (n = t.charCodeAt(e += 1)), rt(n) && (e = Me(t, e + 1), n = t.charCodeAt(e)), n === 46 && rt(t.charCodeAt(e + 1)) && (e += 2, e = Me(t, e)), zr(
    t,
    e,
    101
    /* e */
  )) {
    let r = 0;
    n = t.charCodeAt(e + 1), (n === 45 || n === 43) && (r = 1, n = t.charCodeAt(e + 2)), rt(n) && (e = Me(t, e + 1 + r + 1));
  }
  return e;
}
function je(t, e) {
  for (; e < t.length; e++) {
    const n = t.charCodeAt(e);
    if (n === 41) {
      e++;
      break;
    }
    mt(n, qt(t, e + 1)) && (e = Xt(t, e));
  }
  return e;
}
function Vr(t) {
  if (t.length === 1 && !Yt(t.charCodeAt(0)))
    return t[0];
  let e = parseInt(t, 16);
  return (e === 0 || // If this number is zero,
  e >= 55296 && e <= 57343 || // or is for a surrogate,
  e > 1114111) && (e = 65533), String.fromCodePoint(e);
}
const Gr = [
  "EOF-token",
  "ident-token",
  "function-token",
  "at-keyword-token",
  "hash-token",
  "string-token",
  "bad-string-token",
  "url-token",
  "bad-url-token",
  "delim-token",
  "number-token",
  "percentage-token",
  "dimension-token",
  "whitespace-token",
  "CDO-token",
  "CDC-token",
  "colon-token",
  "semicolon-token",
  "comma-token",
  "[-token",
  "]-token",
  "(-token",
  ")-token",
  "{-token",
  "}-token",
  "comment-token"
], Xo = 16 * 1024;
function Ee(t = null, e) {
  return t === null || t.length < e ? new Uint32Array(Math.max(e + 1024, Xo)) : t;
}
const _n = 10, Jo = 12, Nn = 13;
function Dn(t) {
  const e = t.source, n = e.length, r = e.length > 0 ? Br(e.charCodeAt(0)) : 0, s = Ee(t.lines, n), o = Ee(t.columns, n);
  let a = t.startLine, l = t.startColumn;
  for (let u = r; u < n; u++) {
    const i = e.charCodeAt(u);
    s[u] = a, o[u] = l++, (i === _n || i === Nn || i === Jo) && (i === Nn && u + 1 < n && e.charCodeAt(u + 1) === _n && (u++, s[u] = a, o[u] = l), a++, l = 1);
  }
  s[n] = a, o[n] = l, t.lines = s, t.columns = o, t.computed = !0;
}
class Zo {
  constructor(e, n, r, s) {
    this.setSource(e, n, r, s), this.lines = null, this.columns = null;
  }
  setSource(e = "", n = 0, r = 1, s = 1) {
    this.source = e, this.startOffset = n, this.startLine = r, this.startColumn = s, this.computed = !1;
  }
  getLocation(e, n) {
    return this.computed || Dn(this), {
      source: n,
      offset: this.startOffset + e,
      line: this.lines[e],
      column: this.columns[e]
    };
  }
  getLocationRange(e, n, r) {
    return this.computed || Dn(this), {
      source: r,
      start: {
        offset: this.startOffset + e,
        line: this.lines[e],
        column: this.columns[e]
      },
      end: {
        offset: this.startOffset + n,
        line: this.lines[n],
        column: this.columns[n]
      }
    };
  }
}
const pt = 16777215, dt = 24, Ft = new Uint8Array(32);
Ft[T] = y;
Ft[I] = y;
Ft[te] = ae;
Ft[ut] = Qt;
function Fn(t) {
  return Ft[t] !== 0;
}
class ta {
  constructor(e, n) {
    this.setSource(e, n);
  }
  reset() {
    this.eof = !1, this.tokenIndex = -1, this.tokenType = 0, this.tokenStart = this.firstCharOffset, this.tokenEnd = this.firstCharOffset;
  }
  setSource(e = "", n = () => {
  }) {
    e = String(e || "");
    const r = e.length, s = Ee(this.offsetAndType, e.length + 1), o = Ee(this.balance, e.length + 1);
    let a = 0, l = -1, u = 0, i = e.length;
    this.offsetAndType = null, this.balance = null, o.fill(0), n(e, (c, h, f) => {
      const p = a++;
      if (s[p] = c << dt | f, l === -1 && (l = h), o[p] = i, c === u) {
        const g = o[i];
        o[i] = p, i = g, u = Ft[s[g] >> dt];
      } else Fn(c) && (i = p, u = Ft[c]);
    }), s[a] = At << dt | r, o[a] = a;
    for (let c = 0; c < a; c++) {
      const h = o[c];
      if (h <= c) {
        const f = o[h];
        f !== c && (o[c] = f);
      } else h > a && (o[c] = a);
    }
    this.source = e, this.firstCharOffset = l === -1 ? 0 : l, this.tokenCount = a, this.offsetAndType = s, this.balance = o, this.reset(), this.next();
  }
  lookupType(e) {
    return e += this.tokenIndex, e < this.tokenCount ? this.offsetAndType[e] >> dt : At;
  }
  lookupTypeNonSC(e) {
    for (let n = this.tokenIndex; n < this.tokenCount; n++) {
      const r = this.offsetAndType[n] >> dt;
      if (r !== U && r !== Y && e-- === 0)
        return r;
    }
    return At;
  }
  lookupOffset(e) {
    return e += this.tokenIndex, e < this.tokenCount ? this.offsetAndType[e - 1] & pt : this.source.length;
  }
  lookupOffsetNonSC(e) {
    for (let n = this.tokenIndex; n < this.tokenCount; n++) {
      const r = this.offsetAndType[n] >> dt;
      if (r !== U && r !== Y && e-- === 0)
        return n - this.tokenIndex;
    }
    return At;
  }
  lookupValue(e, n) {
    return e += this.tokenIndex, e < this.tokenCount ? ve(
      this.source,
      this.offsetAndType[e - 1] & pt,
      this.offsetAndType[e] & pt,
      n
    ) : !1;
  }
  getTokenStart(e) {
    return e === this.tokenIndex ? this.tokenStart : e > 0 ? e < this.tokenCount ? this.offsetAndType[e - 1] & pt : this.offsetAndType[this.tokenCount] & pt : this.firstCharOffset;
  }
  substrToCursor(e) {
    return this.source.substring(e, this.tokenStart);
  }
  isBalanceEdge(e) {
    return this.balance[this.tokenIndex] < e;
  }
  isDelim(e, n) {
    return n ? this.lookupType(n) === P && this.source.charCodeAt(this.lookupOffset(n)) === e : this.tokenType === P && this.source.charCodeAt(this.tokenStart) === e;
  }
  skip(e) {
    let n = this.tokenIndex + e;
    n < this.tokenCount ? (this.tokenIndex = n, this.tokenStart = this.offsetAndType[n - 1] & pt, n = this.offsetAndType[n], this.tokenType = n >> dt, this.tokenEnd = n & pt) : (this.tokenIndex = this.tokenCount, this.next());
  }
  next() {
    let e = this.tokenIndex + 1;
    e < this.tokenCount ? (this.tokenIndex = e, this.tokenStart = this.tokenEnd, e = this.offsetAndType[e], this.tokenType = e >> dt, this.tokenEnd = e & pt) : (this.eof = !0, this.tokenIndex = this.tokenCount, this.tokenType = At, this.tokenStart = this.tokenEnd = this.source.length);
  }
  skipSC() {
    for (; this.tokenType === U || this.tokenType === Y; )
      this.next();
  }
  skipUntilBalanced(e, n) {
    let r = e, s = 0, o = 0;
    t:
      for (; r < this.tokenCount; r++) {
        if (s = this.balance[r], s < e)
          break t;
        switch (o = r > 0 ? this.offsetAndType[r - 1] & pt : this.firstCharOffset, n(this.source.charCodeAt(o))) {
          case 1:
            break t;
          case 2:
            r++;
            break t;
          default:
            Fn(this.offsetAndType[r] >> dt) && (r = s);
        }
      }
    this.skip(r - this.tokenIndex);
  }
  forEachToken(e) {
    for (let n = 0, r = this.firstCharOffset; n < this.tokenCount; n++) {
      const s = r, o = this.offsetAndType[n], a = o & pt, l = o >> dt;
      r = a, e(l, s, a, n);
    }
  }
  dump() {
    const e = new Array(this.tokenCount);
    return this.forEachToken((n, r, s, o) => {
      e[o] = {
        idx: o,
        type: Gr[n],
        chunk: this.source.substring(r, s),
        balance: this.balance[o]
      };
    }), e;
  }
}
function qr(t, e) {
  function n(h) {
    return h < l ? t.charCodeAt(h) : 0;
  }
  function r() {
    if (i = Hr(t, i), Ne(n(i), n(i + 1), n(i + 2))) {
      c = $, i = pe(t, i);
      return;
    }
    if (n(i) === 37) {
      c = F, i++;
      return;
    }
    c = x;
  }
  function s() {
    const h = i;
    if (i = pe(t, i), ve(t, h, i, "url") && n(i) === 40) {
      if (i = fe(t, i + 1), n(i) === 34 || n(i) === 39) {
        c = T, i = h + 4;
        return;
      }
      a();
      return;
    }
    if (n(i) === 40) {
      c = T, i++;
      return;
    }
    c = d;
  }
  function o(h) {
    for (h || (h = n(i++)), c = wt; i < t.length; i++) {
      const f = t.charCodeAt(i);
      switch (Fe(f)) {
        // ending code point
        case h:
          i++;
          return;
        // EOF
        // case EofCategory:
        // This is a parse error. Return the <string-token>.
        // return;
        // newline
        case ke:
          if (Ae(f)) {
            i += rn(t, i, f), c = zo;
            return;
          }
          break;
        // U+005C REVERSE SOLIDUS (\)
        case 92:
          if (i === t.length - 1)
            break;
          const p = n(i + 1);
          Ae(p) ? i += rn(t, i + 1, p) : mt(f, p) && (i = Xt(t, i) - 1);
          break;
      }
    }
  }
  function a() {
    for (c = Q, i = fe(t, i); i < t.length; i++) {
      const h = t.charCodeAt(i);
      switch (Fe(h)) {
        // U+0029 RIGHT PARENTHESIS ())
        case 41:
          i++;
          return;
        // EOF
        // case EofCategory:
        // This is a parse error. Return the <url-token>.
        // return;
        // whitespace
        case ke:
          if (i = fe(t, i), n(i) === 41 || i >= t.length) {
            i < t.length && i++;
            return;
          }
          i = je(t, i), c = ot;
          return;
        // U+0022 QUOTATION MARK (")
        // U+0027 APOSTROPHE (')
        // U+0028 LEFT PARENTHESIS (()
        // non-printable code point
        case 34:
        case 39:
        case 40:
        case Wr:
          i = je(t, i), c = ot;
          return;
        // U+005C REVERSE SOLIDUS (\)
        case 92:
          if (mt(h, n(i + 1))) {
            i = Xt(t, i) - 1;
            break;
          }
          i = je(t, i), c = ot;
          return;
      }
    }
  }
  t = String(t || "");
  const l = t.length;
  let u = Br(n(0)), i = u, c;
  for (; i < l; ) {
    const h = t.charCodeAt(i);
    switch (Fe(h)) {
      // whitespace
      case ke:
        c = U, i = fe(t, i + 1);
        break;
      // U+0022 QUOTATION MARK (")
      case 34:
        o();
        break;
      // U+0023 NUMBER SIGN (#)
      case 35:
        jr(n(i + 1)) || mt(n(i + 1), n(i + 2)) ? (c = N, i = pe(t, i + 1)) : (c = P, i++);
        break;
      // U+0027 APOSTROPHE (')
      case 39:
        o();
        break;
      // U+0028 LEFT PARENTHESIS (()
      case 40:
        c = I, i++;
        break;
      // U+0029 RIGHT PARENTHESIS ())
      case 41:
        c = y, i++;
        break;
      // U+002B PLUS SIGN (+)
      case 43:
        De(h, n(i + 1), n(i + 2)) ? r() : (c = P, i++);
        break;
      // U+002C COMMA (,)
      case 44:
        c = ht, i++;
        break;
      // U+002D HYPHEN-MINUS (-)
      case 45:
        De(h, n(i + 1), n(i + 2)) ? r() : n(i + 1) === 45 && n(i + 2) === 62 ? (c = et, i = i + 3) : Ne(h, n(i + 1), n(i + 2)) ? s() : (c = P, i++);
        break;
      // U+002E FULL STOP (.)
      case 46:
        De(h, n(i + 1), n(i + 2)) ? r() : (c = P, i++);
        break;
      // U+002F SOLIDUS (/)
      case 47:
        n(i + 1) === 42 ? (c = Y, i = t.indexOf("*/", i + 2), i = i === -1 ? t.length : i + 2) : (c = P, i++);
        break;
      // U+003A COLON (:)
      case 58:
        c = X, i++;
        break;
      // U+003B SEMICOLON (;)
      case 59:
        c = tt, i++;
        break;
      // U+003C LESS-THAN SIGN (<)
      case 60:
        n(i + 1) === 33 && n(i + 2) === 45 && n(i + 3) === 45 ? (c = Ie, i = i + 4) : (c = P, i++);
        break;
      // U+0040 COMMERCIAL AT (@)
      case 64:
        Ne(n(i + 1), n(i + 2), n(i + 3)) ? (c = z, i = pe(t, i + 1)) : (c = P, i++);
        break;
      // U+005B LEFT SQUARE BRACKET ([)
      case 91:
        c = te, i++;
        break;
      // U+005C REVERSE SOLIDUS (\)
      case 92:
        mt(h, n(i + 1)) ? s() : (c = P, i++);
        break;
      // U+005D RIGHT SQUARE BRACKET (])
      case 93:
        c = ae, i++;
        break;
      // U+007B LEFT CURLY BRACKET ({)
      case 123:
        c = ut, i++;
        break;
      // U+007D RIGHT CURLY BRACKET (})
      case 125:
        c = Qt, i++;
        break;
      // digit
      case Ur:
        r();
        break;
      // name-start code point
      case bn:
        s();
        break;
      // EOF
      // case EofCategory:
      // Return an <EOF-token>.
      // break;
      // anything else
      default:
        c = P, i++;
    }
    e(c, u, u = i);
  }
}
const gt = 43, nt = 45, Se = 110, Rt = !0, ea = !1;
function ye(t, e) {
  let n = this.tokenStart + t;
  const r = this.charCodeAt(n);
  for ((r === gt || r === nt) && (e && this.error("Number sign is not allowed"), n++); n < this.tokenEnd; n++)
    rt(this.charCodeAt(n)) || this.error("Integer is expected", n);
}
function zt(t) {
  return ye.call(this, 0, t);
}
function Tt(t, e) {
  if (!this.cmpChar(this.tokenStart + t, e)) {
    let n = "";
    switch (e) {
      case Se:
        n = "N is expected";
        break;
      case nt:
        n = "HyphenMinus is expected";
        break;
    }
    this.error(n, this.tokenStart + t);
  }
}
function Be() {
  let t = 0, e = 0, n = this.tokenType;
  for (; n === U || n === Y; )
    n = this.lookupType(++t);
  if (n !== x)
    if (this.isDelim(gt, t) || this.isDelim(nt, t)) {
      e = this.isDelim(gt, t) ? gt : nt;
      do
        n = this.lookupType(++t);
      while (n === U || n === Y);
      n !== x && (this.skip(t), zt.call(this, Rt));
    } else
      return null;
  return t > 0 && this.skip(t), e === 0 && (n = this.charCodeAt(this.tokenStart), n !== gt && n !== nt && this.error("Number sign is expected")), zt.call(this, e !== 0), e === nt ? "-" + this.consume(x) : this.consume(x);
}
const na = "AnPlusB", ra = {
  a: [String, null],
  b: [String, null]
};
function Kr() {
  const t = this.tokenStart;
  let e = null, n = null;
  if (this.tokenType === x)
    zt.call(this, ea), n = this.consume(x);
  else if (this.tokenType === d && this.cmpChar(this.tokenStart, nt))
    switch (e = "-1", Tt.call(this, 1, Se), this.tokenEnd - this.tokenStart) {
      // -n
      // -n <signed-integer>
      // -n ['+' | '-'] <signless-integer>
      case 2:
        this.next(), n = Be.call(this);
        break;
      // -n- <signless-integer>
      case 3:
        Tt.call(this, 2, nt), this.next(), this.skipSC(), zt.call(this, Rt), n = "-" + this.consume(x);
        break;
      // <dashndashdigit-ident>
      default:
        Tt.call(this, 2, nt), ye.call(this, 3, Rt), this.next(), n = this.substrToCursor(t + 2);
    }
  else if (this.tokenType === d || this.isDelim(gt) && this.lookupType(1) === d) {
    let r = 0;
    switch (e = "1", this.isDelim(gt) && (r = 1, this.next()), Tt.call(this, 0, Se), this.tokenEnd - this.tokenStart) {
      // '+'? n
      // '+'? n <signed-integer>
      // '+'? n ['+' | '-'] <signless-integer>
      case 1:
        this.next(), n = Be.call(this);
        break;
      // '+'? n- <signless-integer>
      case 2:
        Tt.call(this, 1, nt), this.next(), this.skipSC(), zt.call(this, Rt), n = "-" + this.consume(x);
        break;
      // '+'? <ndashdigit-ident>
      default:
        Tt.call(this, 1, nt), ye.call(this, 2, Rt), this.next(), n = this.substrToCursor(t + r + 1);
    }
  } else if (this.tokenType === $) {
    const r = this.charCodeAt(this.tokenStart), s = r === gt || r === nt;
    let o = this.tokenStart + s;
    for (; o < this.tokenEnd && rt(this.charCodeAt(o)); o++)
      ;
    o === this.tokenStart + s && this.error("Integer is expected", this.tokenStart + s), Tt.call(this, o - this.tokenStart, Se), e = this.substring(t, o), o + 1 === this.tokenEnd ? (this.next(), n = Be.call(this)) : (Tt.call(this, o - this.tokenStart + 1, nt), o + 2 === this.tokenEnd ? (this.next(), this.skipSC(), zt.call(this, Rt), n = "-" + this.consume(x)) : (ye.call(this, o - this.tokenStart + 2, Rt), this.next(), n = this.substrToCursor(o + 1)));
  } else
    this.error();
  return e !== null && e.charCodeAt(0) === gt && (e = e.substr(1)), n !== null && n.charCodeAt(0) === gt && (n = n.substr(1)), {
    type: "AnPlusB",
    loc: this.getLocation(t, this.tokenStart),
    a: e,
    b: n
  };
}
function Qr(t) {
  if (t.a) {
    const e = t.a === "+1" && "n" || t.a === "1" && "n" || t.a === "-1" && "-n" || t.a + "n";
    if (t.b) {
      const n = t.b[0] === "-" || t.b[0] === "+" ? t.b : "+" + t.b;
      this.tokenize(e + n);
    } else
      this.tokenize(e);
  } else
    this.tokenize(t.b);
}
const sa = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Qr,
  name: na,
  parse: Kr,
  structure: ra
}, Symbol.toStringTag, { value: "Module" }));
function Mn() {
  return this.Raw(this.consumeUntilLeftCurlyBracketOrSemicolon, !0);
}
function ia() {
  for (let t = 1, e; e = this.lookupType(t); t++) {
    if (e === Qt)
      return !0;
    if (e === ut || e === z)
      return !1;
  }
  return !1;
}
const oa = "Atrule", aa = "atrule", la = {
  name: String,
  prelude: ["AtrulePrelude", "Raw", null],
  block: ["Block", null]
};
function Yr(t = !1) {
  const e = this.tokenStart;
  let n, r, s = null, o = null;
  switch (this.eat(z), n = this.substrToCursor(e + 1), r = n.toLowerCase(), this.skipSC(), this.eof === !1 && this.tokenType !== ut && this.tokenType !== tt && (this.parseAtrulePrelude ? s = this.parseWithFallback(this.AtrulePrelude.bind(this, n, t), Mn) : s = Mn.call(this, this.tokenIndex), this.skipSC()), this.tokenType) {
    case tt:
      this.next();
      break;
    case ut:
      hasOwnProperty.call(this.atrule, r) && typeof this.atrule[r].block == "function" ? o = this.atrule[r].block.call(this, t) : o = this.Block(ia.call(this));
      break;
  }
  return {
    type: "Atrule",
    loc: this.getLocation(e, this.tokenStart),
    name: n,
    prelude: s,
    block: o
  };
}
function Xr(t) {
  this.token(z, "@" + t.name), t.prelude !== null && this.node(t.prelude), t.block ? this.node(t.block) : this.token(tt, ";");
}
const ca = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Xr,
  name: oa,
  parse: Yr,
  structure: la,
  walkContext: aa
}, Symbol.toStringTag, { value: "Module" })), ua = "AtrulePrelude", ha = "atrulePrelude", fa = {
  children: [[]]
};
function Jr(t) {
  let e = null;
  return t !== null && (t = t.toLowerCase()), this.skipSC(), hasOwnProperty.call(this.atrule, t) && typeof this.atrule[t].prelude == "function" ? e = this.atrule[t].prelude.call(this) : e = this.readSequence(this.scope.AtrulePrelude), this.skipSC(), this.eof !== !0 && this.tokenType !== ut && this.tokenType !== tt && this.error("Semicolon or block is expected"), {
    type: "AtrulePrelude",
    loc: this.getLocationFromList(e),
    children: e
  };
}
function Zr(t) {
  this.children(t);
}
const pa = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Zr,
  name: ua,
  parse: Jr,
  structure: fa,
  walkContext: ha
}, Symbol.toStringTag, { value: "Module" })), da = 36, ts = 42, be = 61, ga = 94, sn = 124, ma = 126;
function ka() {
  this.eof && this.error("Unexpected end of input");
  const t = this.tokenStart;
  let e = !1;
  return this.isDelim(ts) ? (e = !0, this.next()) : this.isDelim(sn) || this.eat(d), this.isDelim(sn) ? this.charCodeAt(this.tokenStart + 1) !== be ? (this.next(), this.eat(d)) : e && this.error("Identifier is expected", this.tokenEnd) : e && this.error("Vertical line is expected"), {
    type: "Identifier",
    loc: this.getLocation(t, this.tokenStart),
    name: this.substrToCursor(t)
  };
}
function Sa() {
  const t = this.tokenStart, e = this.charCodeAt(t);
  return e !== be && // =
  e !== ma && // ~=
  e !== ga && // ^=
  e !== da && // $=
  e !== ts && // *=
  e !== sn && this.error("Attribute selector (=, ~=, ^=, $=, *=, |=) is expected"), this.next(), e !== be && (this.isDelim(be) || this.error("Equal sign is expected"), this.next()), this.substrToCursor(t);
}
const ya = "AttributeSelector", ba = {
  name: "Identifier",
  matcher: [String, null],
  value: ["String", "Identifier", null],
  flags: [String, null]
};
function es() {
  const t = this.tokenStart;
  let e, n = null, r = null, s = null;
  return this.eat(te), this.skipSC(), e = ka.call(this), this.skipSC(), this.tokenType !== ae && (this.tokenType !== d && (n = Sa.call(this), this.skipSC(), r = this.tokenType === wt ? this.String() : this.Identifier(), this.skipSC()), this.tokenType === d && (s = this.consume(d), this.skipSC())), this.eat(ae), {
    type: "AttributeSelector",
    loc: this.getLocation(t, this.tokenStart),
    name: e,
    matcher: n,
    value: r,
    flags: s
  };
}
function ns(t) {
  this.token(P, "["), this.node(t.name), t.matcher !== null && (this.tokenize(t.matcher), this.node(t.value)), t.flags !== null && this.token(d, t.flags), this.token(P, "]");
}
const xa = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ns,
  name: ya,
  parse: es,
  structure: ba
}, Symbol.toStringTag, { value: "Module" })), Ca = 38;
function rs() {
  return this.Raw(null, !0);
}
function jn() {
  return this.parseWithFallback(this.Rule, rs);
}
function Bn() {
  return this.Raw(this.consumeUntilSemicolonIncluded, !0);
}
function wa() {
  if (this.tokenType === tt)
    return Bn.call(this, this.tokenIndex);
  const t = this.parseWithFallback(this.Declaration, Bn);
  return this.tokenType === tt && this.next(), t;
}
const Ta = "Block", Aa = "block", va = {
  children: [[
    "Atrule",
    "Rule",
    "Declaration"
  ]]
};
function ss(t) {
  const e = t ? wa : jn, n = this.tokenStart;
  let r = this.createList();
  this.eat(ut);
  t:
    for (; !this.eof; )
      switch (this.tokenType) {
        case Qt:
          break t;
        case U:
        case Y:
          this.next();
          break;
        case z:
          r.push(this.parseWithFallback(this.Atrule.bind(this, t), rs));
          break;
        default:
          t && this.isDelim(Ca) ? r.push(jn.call(this)) : r.push(e.call(this));
      }
  return this.eof || this.eat(Qt), {
    type: "Block",
    loc: this.getLocation(n, this.tokenStart),
    children: r
  };
}
function is(t) {
  this.token(ut, "{"), this.children(t, (e) => {
    e.type === "Declaration" && this.token(tt, ";");
  }), this.token(Qt, "}");
}
const Ea = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: is,
  name: Ta,
  parse: ss,
  structure: va,
  walkContext: Aa
}, Symbol.toStringTag, { value: "Module" })), $a = "Brackets", La = {
  children: [[]]
};
function os(t, e) {
  const n = this.tokenStart;
  let r = null;
  return this.eat(te), r = t.call(this, e), this.eof || this.eat(ae), {
    type: "Brackets",
    loc: this.getLocation(n, this.tokenStart),
    children: r
  };
}
function as(t) {
  this.token(P, "["), this.children(t), this.token(P, "]");
}
const Pa = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: as,
  name: $a,
  parse: os,
  structure: La
}, Symbol.toStringTag, { value: "Module" })), Oa = "CDC", Ra = [];
function ls() {
  const t = this.tokenStart;
  return this.eat(et), {
    type: "CDC",
    loc: this.getLocation(t, this.tokenStart)
  };
}
function cs() {
  this.token(et, "-->");
}
const Ia = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: cs,
  name: Oa,
  parse: ls,
  structure: Ra
}, Symbol.toStringTag, { value: "Module" })), _a = "CDO", Na = [];
function us() {
  const t = this.tokenStart;
  return this.eat(Ie), {
    type: "CDO",
    loc: this.getLocation(t, this.tokenStart)
  };
}
function hs() {
  this.token(Ie, "<!--");
}
const Da = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: hs,
  name: _a,
  parse: us,
  structure: Na
}, Symbol.toStringTag, { value: "Module" })), Fa = 46, Ma = "ClassSelector", ja = {
  name: String
};
function fs() {
  return this.eatDelim(Fa), {
    type: "ClassSelector",
    loc: this.getLocation(this.tokenStart - 1, this.tokenEnd),
    name: this.consume(d)
  };
}
function ps(t) {
  this.token(P, "."), this.token(d, t.name);
}
const Ba = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ps,
  name: Ma,
  parse: fs,
  structure: ja
}, Symbol.toStringTag, { value: "Module" })), Ua = 43, Un = 47, Wa = 62, za = 126, Ha = "Combinator", Va = {
  name: String
};
function ds() {
  const t = this.tokenStart;
  let e;
  switch (this.tokenType) {
    case U:
      e = " ";
      break;
    case P:
      switch (this.charCodeAt(this.tokenStart)) {
        case Wa:
        case Ua:
        case za:
          this.next();
          break;
        case Un:
          this.next(), this.eatIdent("deep"), this.eatDelim(Un);
          break;
        default:
          this.error("Combinator is expected");
      }
      e = this.substrToCursor(t);
      break;
  }
  return {
    type: "Combinator",
    loc: this.getLocation(t, this.tokenStart),
    name: e
  };
}
function gs(t) {
  this.tokenize(t.name);
}
const Ga = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: gs,
  name: Ha,
  parse: ds,
  structure: Va
}, Symbol.toStringTag, { value: "Module" })), qa = 42, Ka = 47, Qa = "Comment", Ya = {
  value: String
};
function ms() {
  const t = this.tokenStart;
  let e = this.tokenEnd;
  return this.eat(Y), e - t + 2 >= 2 && this.charCodeAt(e - 2) === qa && this.charCodeAt(e - 1) === Ka && (e -= 2), {
    type: "Comment",
    loc: this.getLocation(t, this.tokenStart),
    value: this.substring(t + 2, e)
  };
}
function ks(t) {
  this.token(Y, "/*" + t.value + "*/");
}
const Xa = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ks,
  name: Qa,
  parse: ms,
  structure: Ya
}, Symbol.toStringTag, { value: "Module" })), Ja = /* @__PURE__ */ new Set([X, y, At]), Za = "Condition", tl = {
  kind: String,
  children: [[
    "Identifier",
    "Feature",
    "FeatureFunction",
    "FeatureRange",
    "SupportsDeclaration"
  ]]
};
function Wn(t) {
  return this.lookupTypeNonSC(1) === d && Ja.has(this.lookupTypeNonSC(2)) ? this.Feature(t) : this.FeatureRange(t);
}
const el = {
  media: Wn,
  container: Wn,
  supports() {
    return this.SupportsDeclaration();
  }
};
function Ss(t = "media") {
  const e = this.createList();
  t: for (; !this.eof; )
    switch (this.tokenType) {
      case Y:
      case U:
        this.next();
        continue;
      case d:
        e.push(this.Identifier());
        break;
      case I: {
        let n = this.parseWithFallback(
          () => el[t].call(this, t),
          () => null
        );
        n || (n = this.parseWithFallback(
          () => {
            this.eat(I);
            const r = this.Condition(t);
            return this.eat(y), r;
          },
          () => this.GeneralEnclosed(t)
        )), e.push(n);
        break;
      }
      case T: {
        let n = this.parseWithFallback(
          () => this.FeatureFunction(t),
          () => null
        );
        n || (n = this.GeneralEnclosed(t)), e.push(n);
        break;
      }
      default:
        break t;
    }
  return e.isEmpty && this.error("Condition is expected"), {
    type: "Condition",
    loc: this.getLocationFromList(e),
    kind: t,
    children: e
  };
}
function ys(t) {
  t.children.forEach((e) => {
    e.type === "Condition" ? (this.token(I, "("), this.node(e), this.token(y, ")")) : this.node(e);
  });
}
const nl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ys,
  name: Za,
  parse: Ss,
  structure: tl
}, Symbol.toStringTag, { value: "Module" })), zn = 45;
function rl(t, e) {
  return e = e || 0, t.length - e >= 2 && t.charCodeAt(e) === zn && t.charCodeAt(e + 1) === zn;
}
const bs = 33, sl = 35, il = 36, ol = 38, al = 42, ll = 43, Hn = 47;
function cl() {
  return this.Raw(this.consumeUntilExclamationMarkOrSemicolon, !0);
}
function ul() {
  return this.Raw(this.consumeUntilExclamationMarkOrSemicolon, !1);
}
function hl() {
  const t = this.tokenIndex, e = this.Value();
  return e.type !== "Raw" && this.eof === !1 && this.tokenType !== tt && this.isDelim(bs) === !1 && this.isBalanceEdge(t) === !1 && this.error(), e;
}
const fl = "Declaration", pl = "declaration", dl = {
  important: [Boolean, String],
  property: String,
  value: ["Value", "Raw"]
};
function xs() {
  const t = this.tokenStart, e = this.tokenIndex, n = gl.call(this), r = rl(n), s = r ? this.parseCustomProperty : this.parseValue, o = r ? ul : cl;
  let a = !1, l;
  this.skipSC(), this.eat(X);
  const u = this.tokenIndex;
  if (r || this.skipSC(), s ? l = this.parseWithFallback(hl, o) : l = o.call(this, this.tokenIndex), r && l.type === "Value" && l.children.isEmpty) {
    for (let i = u - this.tokenIndex; i <= 0; i++)
      if (this.lookupType(i) === U) {
        l.children.appendData({
          type: "WhiteSpace",
          loc: null,
          value: " "
        });
        break;
      }
  }
  return this.isDelim(bs) && (a = ml.call(this), this.skipSC()), this.eof === !1 && this.tokenType !== tt && this.isBalanceEdge(e) === !1 && this.error(), {
    type: "Declaration",
    loc: this.getLocation(t, this.tokenStart),
    important: a,
    property: n,
    value: l
  };
}
function Cs(t) {
  this.token(d, t.property), this.token(X, ":"), this.node(t.value), t.important && (this.token(P, "!"), this.token(d, t.important === !0 ? "important" : t.important));
}
function gl() {
  const t = this.tokenStart;
  if (this.tokenType === P)
    switch (this.charCodeAt(this.tokenStart)) {
      case al:
      case il:
      case ll:
      case sl:
      case ol:
        this.next();
        break;
      // TODO: not sure we should support this hack
      case Hn:
        this.next(), this.isDelim(Hn) && this.next();
        break;
    }
  return this.tokenType === N ? this.eat(N) : this.eat(d), this.substrToCursor(t);
}
function ml() {
  this.eat(P), this.skipSC();
  const t = this.consume(d);
  return t === "important" ? !0 : t;
}
const kl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Cs,
  name: fl,
  parse: xs,
  structure: dl,
  walkContext: pl
}, Symbol.toStringTag, { value: "Module" })), Sl = 38;
function Ue() {
  return this.Raw(this.consumeUntilSemicolonIncluded, !0);
}
const yl = "DeclarationList", bl = {
  children: [[
    "Declaration",
    "Atrule",
    "Rule"
  ]]
};
function ws() {
  const t = this.createList();
  for (; !this.eof; )
    switch (this.tokenType) {
      case U:
      case Y:
      case tt:
        this.next();
        break;
      case z:
        t.push(this.parseWithFallback(this.Atrule.bind(this, !0), Ue));
        break;
      default:
        this.isDelim(Sl) ? t.push(this.parseWithFallback(this.Rule, Ue)) : t.push(this.parseWithFallback(this.Declaration, Ue));
    }
  return {
    type: "DeclarationList",
    loc: this.getLocationFromList(t),
    children: t
  };
}
function Ts(t) {
  this.children(t, (e) => {
    e.type === "Declaration" && this.token(tt, ";");
  });
}
const xl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ts,
  name: yl,
  parse: ws,
  structure: bl
}, Symbol.toStringTag, { value: "Module" })), Cl = "Dimension", wl = {
  value: String,
  unit: String
};
function As() {
  const t = this.tokenStart, e = this.consumeNumber($);
  return {
    type: "Dimension",
    loc: this.getLocation(t, this.tokenStart),
    value: e,
    unit: this.substring(t + e.length, this.tokenStart)
  };
}
function vs(t) {
  this.token($, t.value + t.unit);
}
const Tl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: vs,
  name: Cl,
  parse: As,
  structure: wl
}, Symbol.toStringTag, { value: "Module" })), Al = 47, vl = "Feature", El = {
  kind: String,
  name: String,
  value: ["Identifier", "Number", "Dimension", "Ratio", "Function", null]
};
function Es(t) {
  const e = this.tokenStart;
  let n, r = null;
  if (this.eat(I), this.skipSC(), n = this.consume(d), this.skipSC(), this.tokenType !== y) {
    switch (this.eat(X), this.skipSC(), this.tokenType) {
      case x:
        this.lookupNonWSType(1) === P ? r = this.Ratio() : r = this.Number();
        break;
      case $:
        r = this.Dimension();
        break;
      case d:
        r = this.Identifier();
        break;
      case T:
        r = this.parseWithFallback(
          () => {
            const s = this.Function(this.readSequence, this.scope.Value);
            return this.skipSC(), this.isDelim(Al) && this.error(), s;
          },
          () => this.Ratio()
        );
        break;
      default:
        this.error("Number, dimension, ratio or identifier is expected");
    }
    this.skipSC();
  }
  return this.eof || this.eat(y), {
    type: "Feature",
    loc: this.getLocation(e, this.tokenStart),
    kind: t,
    name: n,
    value: r
  };
}
function $s(t) {
  this.token(I, "("), this.token(d, t.name), t.value !== null && (this.token(X, ":"), this.node(t.value)), this.token(y, ")");
}
const $l = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: $s,
  name: vl,
  parse: Es,
  structure: El
}, Symbol.toStringTag, { value: "Module" })), Ll = "FeatureFunction", Pl = {
  kind: String,
  feature: String,
  value: ["Declaration", "Selector"]
};
function Ol(t, e) {
  const r = (this.features[t] || {})[e];
  return typeof r != "function" && this.error(`Unknown feature ${e}()`), r;
}
function Ls(t = "unknown") {
  const e = this.tokenStart, n = this.consumeFunctionName(), r = Ol.call(this, t, n.toLowerCase());
  this.skipSC();
  const s = this.parseWithFallback(
    () => {
      const o = this.tokenIndex, a = r.call(this);
      return this.eof === !1 && this.isBalanceEdge(o) === !1 && this.error(), a;
    },
    () => this.Raw(null, !1)
  );
  return this.eof || this.eat(y), {
    type: "FeatureFunction",
    loc: this.getLocation(e, this.tokenStart),
    kind: t,
    feature: n,
    value: s
  };
}
function Ps(t) {
  this.token(T, t.feature + "("), this.node(t.value), this.token(y, ")");
}
const Rl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ps,
  name: Ll,
  parse: Ls,
  structure: Pl
}, Symbol.toStringTag, { value: "Module" })), Vn = 47, Il = 60, Gn = 61, _l = 62, Nl = "FeatureRange", Dl = {
  kind: String,
  left: ["Identifier", "Number", "Dimension", "Ratio", "Function"],
  leftComparison: String,
  middle: ["Identifier", "Number", "Dimension", "Ratio", "Function"],
  rightComparison: [String, null],
  right: ["Identifier", "Number", "Dimension", "Ratio", "Function", null]
};
function We() {
  switch (this.skipSC(), this.tokenType) {
    case x:
      return this.isDelim(Vn, this.lookupOffsetNonSC(1)) ? this.Ratio() : this.Number();
    case $:
      return this.Dimension();
    case d:
      return this.Identifier();
    case T:
      return this.parseWithFallback(
        () => {
          const t = this.Function(this.readSequence, this.scope.Value);
          return this.skipSC(), this.isDelim(Vn) && this.error(), t;
        },
        () => this.Ratio()
      );
    default:
      this.error("Number, dimension, ratio or identifier is expected");
  }
}
function qn(t) {
  if (this.skipSC(), this.isDelim(Il) || this.isDelim(_l)) {
    const e = this.source[this.tokenStart];
    return this.next(), this.isDelim(Gn) ? (this.next(), e + "=") : e;
  }
  if (this.isDelim(Gn))
    return "=";
  this.error(`Expected ${t ? '":", ' : ""}"<", ">", "=" or ")"`);
}
function Os(t = "unknown") {
  const e = this.tokenStart;
  this.skipSC(), this.eat(I);
  const n = We.call(this), r = qn.call(this, n.type === "Identifier"), s = We.call(this);
  let o = null, a = null;
  return this.lookupNonWSType(0) !== y && (o = qn.call(this), a = We.call(this)), this.skipSC(), this.eat(y), {
    type: "FeatureRange",
    loc: this.getLocation(e, this.tokenStart),
    kind: t,
    left: n,
    leftComparison: r,
    middle: s,
    rightComparison: o,
    right: a
  };
}
function Rs(t) {
  this.token(I, "("), this.node(t.left), this.tokenize(t.leftComparison), this.node(t.middle), t.right && (this.tokenize(t.rightComparison), this.node(t.right)), this.token(y, ")");
}
const Fl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Rs,
  name: Nl,
  parse: Os,
  structure: Dl
}, Symbol.toStringTag, { value: "Module" })), Ml = "Function", jl = "function", Bl = {
  name: String,
  children: [[]]
};
function Is(t, e) {
  const n = this.tokenStart, r = this.consumeFunctionName(), s = r.toLowerCase();
  let o;
  return o = e.hasOwnProperty(s) ? e[s].call(this, e) : t.call(this, e), this.eof || this.eat(y), {
    type: "Function",
    loc: this.getLocation(n, this.tokenStart),
    name: r,
    children: o
  };
}
function _s(t) {
  this.token(T, t.name + "("), this.children(t), this.token(y, ")");
}
const Ul = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: _s,
  name: Ml,
  parse: Is,
  structure: Bl,
  walkContext: jl
}, Symbol.toStringTag, { value: "Module" })), Wl = "GeneralEnclosed", zl = {
  kind: String,
  function: [String, null],
  children: [[]]
};
function Ns(t) {
  const e = this.tokenStart;
  let n = null;
  this.tokenType === T ? n = this.consumeFunctionName() : this.eat(I);
  const r = this.parseWithFallback(
    () => {
      const s = this.tokenIndex, o = this.readSequence(this.scope.Value);
      return this.eof === !1 && this.isBalanceEdge(s) === !1 && this.error(), o;
    },
    () => this.createSingleNodeList(
      this.Raw(null, !1)
    )
  );
  return this.eof || this.eat(y), {
    type: "GeneralEnclosed",
    loc: this.getLocation(e, this.tokenStart),
    kind: t,
    function: n,
    children: r
  };
}
function Ds(t) {
  t.function ? this.token(T, t.function + "(") : this.token(I, "("), this.children(t), this.token(y, ")");
}
const Hl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ds,
  name: Wl,
  parse: Ns,
  structure: zl
}, Symbol.toStringTag, { value: "Module" })), Vl = "XXX", Gl = "Hash", ql = {
  value: String
};
function Fs() {
  const t = this.tokenStart;
  return this.eat(N), {
    type: "Hash",
    loc: this.getLocation(t, this.tokenStart),
    value: this.substrToCursor(t + 1)
  };
}
function Ms(t) {
  this.token(N, "#" + t.value);
}
const Kl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ms,
  name: Gl,
  parse: Fs,
  structure: ql,
  xxx: Vl
}, Symbol.toStringTag, { value: "Module" })), Ql = "Identifier", Yl = {
  name: String
};
function js() {
  return {
    type: "Identifier",
    loc: this.getLocation(this.tokenStart, this.tokenEnd),
    name: this.consume(d)
  };
}
function Bs(t) {
  this.token(d, t.name);
}
const Xl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Bs,
  name: Ql,
  parse: js,
  structure: Yl
}, Symbol.toStringTag, { value: "Module" })), Jl = "IdSelector", Zl = {
  name: String
};
function Us() {
  const t = this.tokenStart;
  return this.eat(N), {
    type: "IdSelector",
    loc: this.getLocation(t, this.tokenStart),
    name: this.substrToCursor(t + 1)
  };
}
function Ws(t) {
  this.token(P, "#" + t.name);
}
const tc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ws,
  name: Jl,
  parse: Us,
  structure: Zl
}, Symbol.toStringTag, { value: "Module" })), ec = 46, nc = "Layer", rc = {
  name: String
};
function zs() {
  let t = this.tokenStart, e = this.consume(d);
  for (; this.isDelim(ec); )
    this.eat(P), e += "." + this.consume(d);
  return {
    type: "Layer",
    loc: this.getLocation(t, this.tokenStart),
    name: e
  };
}
function Hs(t) {
  this.tokenize(t.name);
}
const sc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Hs,
  name: nc,
  parse: zs,
  structure: rc
}, Symbol.toStringTag, { value: "Module" })), ic = "LayerList", oc = {
  children: [[
    "Layer"
  ]]
};
function Vs() {
  const t = this.createList();
  for (this.skipSC(); !this.eof && (t.push(this.Layer()), this.lookupTypeNonSC(0) === ht); )
    this.skipSC(), this.next(), this.skipSC();
  return {
    type: "LayerList",
    loc: this.getLocationFromList(t),
    children: t
  };
}
function Gs(t) {
  this.children(t, () => this.token(ht, ","));
}
const ac = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Gs,
  name: ic,
  parse: Vs,
  structure: oc
}, Symbol.toStringTag, { value: "Module" })), lc = "MediaQuery", cc = {
  modifier: [String, null],
  mediaType: [String, null],
  condition: ["Condition", null]
};
function qs() {
  const t = this.tokenStart;
  let e = null, n = null, r = null;
  if (this.skipSC(), this.tokenType === d && this.lookupTypeNonSC(1) !== I) {
    const s = this.consume(d), o = s.toLowerCase();
    switch (o === "not" || o === "only" ? (this.skipSC(), e = o, n = this.consume(d)) : n = s, this.lookupTypeNonSC(0)) {
      case d: {
        this.skipSC(), this.eatIdent("and"), r = this.Condition("media");
        break;
      }
      case ut:
      case tt:
      case ht:
      case At:
        break;
      default:
        this.error("Identifier or parenthesis is expected");
    }
  } else
    switch (this.tokenType) {
      case d:
      case I:
      case T: {
        r = this.Condition("media");
        break;
      }
      case ut:
      case tt:
      case At:
        break;
      default:
        this.error("Identifier or parenthesis is expected");
    }
  return {
    type: "MediaQuery",
    loc: this.getLocation(t, this.tokenStart),
    modifier: e,
    mediaType: n,
    condition: r
  };
}
function Ks(t) {
  t.mediaType ? (t.modifier && this.token(d, t.modifier), this.token(d, t.mediaType), t.condition && (this.token(d, "and"), this.node(t.condition))) : t.condition && this.node(t.condition);
}
const uc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ks,
  name: lc,
  parse: qs,
  structure: cc
}, Symbol.toStringTag, { value: "Module" })), hc = "MediaQueryList", fc = {
  children: [[
    "MediaQuery"
  ]]
};
function Qs() {
  const t = this.createList();
  for (this.skipSC(); !this.eof && (t.push(this.MediaQuery()), this.tokenType === ht); )
    this.next();
  return {
    type: "MediaQueryList",
    loc: this.getLocationFromList(t),
    children: t
  };
}
function Ys(t) {
  this.children(t, () => this.token(ht, ","));
}
const pc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ys,
  name: hc,
  parse: Qs,
  structure: fc
}, Symbol.toStringTag, { value: "Module" })), dc = 38, gc = "NestingSelector", mc = {};
function Xs() {
  const t = this.tokenStart;
  return this.eatDelim(dc), {
    type: "NestingSelector",
    loc: this.getLocation(t, this.tokenStart)
  };
}
function Js() {
  this.token(P, "&");
}
const kc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Js,
  name: gc,
  parse: Xs,
  structure: mc
}, Symbol.toStringTag, { value: "Module" })), Sc = "Nth", yc = {
  nth: ["AnPlusB", "Identifier"],
  selector: ["SelectorList", null]
};
function Zs() {
  this.skipSC();
  const t = this.tokenStart;
  let e = t, n = null, r;
  return this.lookupValue(0, "odd") || this.lookupValue(0, "even") ? r = this.Identifier() : r = this.AnPlusB(), e = this.tokenStart, this.skipSC(), this.lookupValue(0, "of") && (this.next(), n = this.SelectorList(), e = this.tokenStart), {
    type: "Nth",
    loc: this.getLocation(t, e),
    nth: r,
    selector: n
  };
}
function ti(t) {
  this.node(t.nth), t.selector !== null && (this.token(d, "of"), this.node(t.selector));
}
const bc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ti,
  name: Sc,
  parse: Zs,
  structure: yc
}, Symbol.toStringTag, { value: "Module" })), xc = "Number", Cc = {
  value: String
};
function ei() {
  return {
    type: "Number",
    loc: this.getLocation(this.tokenStart, this.tokenEnd),
    value: this.consume(x)
  };
}
function ni(t) {
  this.token(x, t.value);
}
const wc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ni,
  name: xc,
  parse: ei,
  structure: Cc
}, Symbol.toStringTag, { value: "Module" })), Tc = "Operator", Ac = {
  value: String
};
function ri() {
  const t = this.tokenStart;
  return this.next(), {
    type: "Operator",
    loc: this.getLocation(t, this.tokenStart),
    value: this.substrToCursor(t)
  };
}
function si(t) {
  this.tokenize(t.value);
}
const vc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: si,
  name: Tc,
  parse: ri,
  structure: Ac
}, Symbol.toStringTag, { value: "Module" })), Ec = "Parentheses", $c = {
  children: [[]]
};
function ii(t, e) {
  const n = this.tokenStart;
  let r = null;
  return this.eat(I), r = t.call(this, e), this.eof || this.eat(y), {
    type: "Parentheses",
    loc: this.getLocation(n, this.tokenStart),
    children: r
  };
}
function oi(t) {
  this.token(I, "("), this.children(t), this.token(y, ")");
}
const Lc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: oi,
  name: Ec,
  parse: ii,
  structure: $c
}, Symbol.toStringTag, { value: "Module" })), Pc = "Percentage", Oc = {
  value: String
};
function ai() {
  return {
    type: "Percentage",
    loc: this.getLocation(this.tokenStart, this.tokenEnd),
    value: this.consumeNumber(F)
  };
}
function li(t) {
  this.token(F, t.value + "%");
}
const Rc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: li,
  name: Pc,
  parse: ai,
  structure: Oc
}, Symbol.toStringTag, { value: "Module" })), Ic = "PseudoClassSelector", _c = "function", Nc = {
  name: String,
  children: [["Raw"], null]
};
function ci() {
  const t = this.tokenStart;
  let e = null, n, r;
  return this.eat(X), this.tokenType === T ? (n = this.consumeFunctionName(), r = n.toLowerCase(), this.lookupNonWSType(0) == y ? e = this.createList() : hasOwnProperty.call(this.pseudo, r) ? (this.skipSC(), e = this.pseudo[r].call(this), this.skipSC()) : (e = this.createList(), e.push(
    this.Raw(null, !1)
  )), this.eat(y)) : n = this.consume(d), {
    type: "PseudoClassSelector",
    loc: this.getLocation(t, this.tokenStart),
    name: n,
    children: e
  };
}
function ui(t) {
  this.token(X, ":"), t.children === null ? this.token(d, t.name) : (this.token(T, t.name + "("), this.children(t), this.token(y, ")"));
}
const Dc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: ui,
  name: Ic,
  parse: ci,
  structure: Nc,
  walkContext: _c
}, Symbol.toStringTag, { value: "Module" })), Fc = "PseudoElementSelector", Mc = "function", jc = {
  name: String,
  children: [["Raw"], null]
};
function hi() {
  const t = this.tokenStart;
  let e = null, n, r;
  return this.eat(X), this.eat(X), this.tokenType === T ? (n = this.consumeFunctionName(), r = n.toLowerCase(), this.lookupNonWSType(0) == y ? e = this.createList() : hasOwnProperty.call(this.pseudo, r) ? (this.skipSC(), e = this.pseudo[r].call(this), this.skipSC()) : (e = this.createList(), e.push(
    this.Raw(null, !1)
  )), this.eat(y)) : n = this.consume(d), {
    type: "PseudoElementSelector",
    loc: this.getLocation(t, this.tokenStart),
    name: n,
    children: e
  };
}
function fi(t) {
  this.token(X, ":"), this.token(X, ":"), t.children === null ? this.token(d, t.name) : (this.token(T, t.name + "("), this.children(t), this.token(y, ")"));
}
const Bc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: fi,
  name: Fc,
  parse: hi,
  structure: jc,
  walkContext: Mc
}, Symbol.toStringTag, { value: "Module" })), Kn = 47;
function Qn() {
  switch (this.skipSC(), this.tokenType) {
    case x:
      return this.Number();
    case T:
      return this.Function(this.readSequence, this.scope.Value);
    default:
      this.error("Number of function is expected");
  }
}
const Uc = "Ratio", Wc = {
  left: ["Number", "Function"],
  right: ["Number", "Function", null]
};
function pi() {
  const t = this.tokenStart, e = Qn.call(this);
  let n = null;
  return this.skipSC(), this.isDelim(Kn) && (this.eatDelim(Kn), n = Qn.call(this)), {
    type: "Ratio",
    loc: this.getLocation(t, this.tokenStart),
    left: e,
    right: n
  };
}
function di(t) {
  this.node(t.left), this.token(P, "/"), t.right ? this.node(t.right) : this.node(x, 1);
}
const zc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: di,
  name: Uc,
  parse: pi,
  structure: Wc
}, Symbol.toStringTag, { value: "Module" }));
function Hc() {
  return this.tokenIndex > 0 && this.lookupType(-1) === U ? this.tokenIndex > 1 ? this.getTokenStart(this.tokenIndex - 1) : this.firstCharOffset : this.tokenStart;
}
const Vc = "Raw", Gc = {
  value: String
};
function gi(t, e) {
  const n = this.getTokenStart(this.tokenIndex);
  let r;
  return this.skipUntilBalanced(this.tokenIndex, t || this.consumeUntilBalanceEnd), e && this.tokenStart > n ? r = Hc.call(this) : r = this.tokenStart, {
    type: "Raw",
    loc: this.getLocation(n, r),
    value: this.substring(n, r)
  };
}
function mi(t) {
  this.tokenize(t.value);
}
const qc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: mi,
  name: Vc,
  parse: gi,
  structure: Gc
}, Symbol.toStringTag, { value: "Module" }));
function Yn() {
  return this.Raw(this.consumeUntilLeftCurlyBracket, !0);
}
function Kc() {
  const t = this.SelectorList();
  return t.type !== "Raw" && this.eof === !1 && this.tokenType !== ut && this.error(), t;
}
const Qc = "Rule", Yc = "rule", Xc = {
  prelude: ["SelectorList", "Raw"],
  block: ["Block"]
};
function ki() {
  const t = this.tokenIndex, e = this.tokenStart;
  let n, r;
  return this.parseRulePrelude ? n = this.parseWithFallback(Kc, Yn) : n = Yn.call(this, t), r = this.Block(!0), {
    type: "Rule",
    loc: this.getLocation(e, this.tokenStart),
    prelude: n,
    block: r
  };
}
function Si(t) {
  this.node(t.prelude), this.node(t.block);
}
const Jc = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Si,
  name: Qc,
  parse: ki,
  structure: Xc,
  walkContext: Yc
}, Symbol.toStringTag, { value: "Module" })), Zc = "Scope", tu = {
  root: ["SelectorList", "Raw", null],
  limit: ["SelectorList", "Raw", null]
};
function yi() {
  let t = null, e = null;
  this.skipSC();
  const n = this.tokenStart;
  return this.tokenType === I && (this.next(), this.skipSC(), t = this.parseWithFallback(
    this.SelectorList,
    () => this.Raw(!1, !0)
  ), this.skipSC(), this.eat(y)), this.lookupNonWSType(0) === d && (this.skipSC(), this.eatIdent("to"), this.skipSC(), this.eat(I), this.skipSC(), e = this.parseWithFallback(
    this.SelectorList,
    () => this.Raw(!1, !0)
  ), this.skipSC(), this.eat(y)), {
    type: "Scope",
    loc: this.getLocation(n, this.tokenStart),
    root: t,
    limit: e
  };
}
function bi(t) {
  t.root && (this.token(I, "("), this.node(t.root), this.token(y, ")")), t.limit && (this.token(d, "to"), this.token(I, "("), this.node(t.limit), this.token(y, ")"));
}
const eu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: bi,
  name: Zc,
  parse: yi,
  structure: tu
}, Symbol.toStringTag, { value: "Module" })), nu = "Selector", ru = {
  children: [[
    "TypeSelector",
    "IdSelector",
    "ClassSelector",
    "AttributeSelector",
    "PseudoClassSelector",
    "PseudoElementSelector",
    "Combinator"
  ]]
};
function xi() {
  const t = this.readSequence(this.scope.Selector);
  return this.getFirstListNode(t) === null && this.error("Selector is expected"), {
    type: "Selector",
    loc: this.getLocationFromList(t),
    children: t
  };
}
function Ci(t) {
  this.children(t);
}
const su = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ci,
  name: nu,
  parse: xi,
  structure: ru
}, Symbol.toStringTag, { value: "Module" })), iu = "SelectorList", ou = "selector", au = {
  children: [[
    "Selector",
    "Raw"
  ]]
};
function wi() {
  const t = this.createList();
  for (; !this.eof; ) {
    if (t.push(this.Selector()), this.tokenType === ht) {
      this.next();
      continue;
    }
    break;
  }
  return {
    type: "SelectorList",
    loc: this.getLocationFromList(t),
    children: t
  };
}
function Ti(t) {
  this.children(t, () => this.token(ht, ","));
}
const lu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ti,
  name: iu,
  parse: wi,
  structure: au,
  walkContext: ou
}, Symbol.toStringTag, { value: "Module" })), on = 92, Ai = 34, cu = 39;
function vi(t) {
  const e = t.length, n = t.charCodeAt(0), r = n === Ai || n === cu ? 1 : 0, s = r === 1 && e > 1 && t.charCodeAt(e - 1) === n ? e - 2 : e - 1;
  let o = "";
  for (let a = r; a <= s; a++) {
    let l = t.charCodeAt(a);
    if (l === on) {
      if (a === s) {
        a !== e - 1 && (o = t.substr(a + 1));
        break;
      }
      if (l = t.charCodeAt(++a), mt(on, l)) {
        const u = a - 1, i = Xt(t, u);
        a = i - 1, o += Vr(t.substring(u + 1, i));
      } else
        l === 13 && t.charCodeAt(a + 1) === 10 && a++;
    } else
      o += t[a];
  }
  return o;
}
function uu(t, e) {
  const n = '"', r = Ai;
  let s = "", o = !1;
  for (let a = 0; a < t.length; a++) {
    const l = t.charCodeAt(a);
    if (l === 0) {
      s += "�";
      continue;
    }
    if (l <= 31 || l === 127) {
      s += "\\" + l.toString(16), o = !0;
      continue;
    }
    l === r || l === on ? (s += "\\" + t.charAt(a), o = !1) : (o && (Yt(l) || Dt(l)) && (s += " "), s += t.charAt(a), o = !1);
  }
  return n + s + n;
}
const hu = "String", fu = {
  value: String
};
function Ei() {
  return {
    type: "String",
    loc: this.getLocation(this.tokenStart, this.tokenEnd),
    value: vi(this.consume(wt))
  };
}
function $i(t) {
  this.token(wt, uu(t.value));
}
const pu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: $i,
  name: hu,
  parse: Ei,
  structure: fu
}, Symbol.toStringTag, { value: "Module" })), du = 33;
function Xn() {
  return this.Raw(null, !1);
}
const gu = "StyleSheet", mu = "stylesheet", ku = {
  children: [[
    "Comment",
    "CDO",
    "CDC",
    "Atrule",
    "Rule",
    "Raw"
  ]]
};
function Li() {
  const t = this.tokenStart, e = this.createList();
  let n;
  for (; !this.eof; ) {
    switch (this.tokenType) {
      case U:
        this.next();
        continue;
      case Y:
        if (this.charCodeAt(this.tokenStart + 2) !== du) {
          this.next();
          continue;
        }
        n = this.Comment();
        break;
      case Ie:
        n = this.CDO();
        break;
      case et:
        n = this.CDC();
        break;
      // CSS Syntax Module Level 3
      // §2.2 Error handling
      // At the "top level" of a stylesheet, an <at-keyword-token> starts an at-rule.
      case z:
        n = this.parseWithFallback(this.Atrule, Xn);
        break;
      // Anything else starts a qualified rule ...
      default:
        n = this.parseWithFallback(this.Rule, Xn);
    }
    e.push(n);
  }
  return {
    type: "StyleSheet",
    loc: this.getLocation(t, this.tokenStart),
    children: e
  };
}
function Pi(t) {
  this.children(t);
}
const Su = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Pi,
  name: gu,
  parse: Li,
  structure: ku,
  walkContext: mu
}, Symbol.toStringTag, { value: "Module" })), yu = "SupportsDeclaration", bu = {
  declaration: "Declaration"
};
function Oi() {
  const t = this.tokenStart;
  this.eat(I), this.skipSC();
  const e = this.Declaration();
  return this.eof || this.eat(y), {
    type: "SupportsDeclaration",
    loc: this.getLocation(t, this.tokenStart),
    declaration: e
  };
}
function Ri(t) {
  this.token(I, "("), this.node(t.declaration), this.token(y, ")");
}
const xu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ri,
  name: yu,
  parse: Oi,
  structure: bu
}, Symbol.toStringTag, { value: "Module" })), Cu = 42, Jn = 124;
function ze() {
  this.tokenType !== d && this.isDelim(Cu) === !1 && this.error("Identifier or asterisk is expected"), this.next();
}
const wu = "TypeSelector", Tu = {
  name: String
};
function Ii() {
  const t = this.tokenStart;
  return this.isDelim(Jn) ? (this.next(), ze.call(this)) : (ze.call(this), this.isDelim(Jn) && (this.next(), ze.call(this))), {
    type: "TypeSelector",
    loc: this.getLocation(t, this.tokenStart),
    name: this.substrToCursor(t)
  };
}
function _i(t) {
  this.tokenize(t.name);
}
const Au = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: _i,
  name: wu,
  parse: Ii,
  structure: Tu
}, Symbol.toStringTag, { value: "Module" })), Ni = 43, Di = 45, an = 63;
function ne(t, e) {
  let n = 0;
  for (let r = this.tokenStart + t; r < this.tokenEnd; r++) {
    const s = this.charCodeAt(r);
    if (s === Di && e && n !== 0)
      return ne.call(this, t + n + 1, !1), -1;
    Yt(s) || this.error(
      e && n !== 0 ? "Hyphen minus" + (n < 6 ? " or hex digit" : "") + " is expected" : n < 6 ? "Hex digit is expected" : "Unexpected input",
      r
    ), ++n > 6 && this.error("Too many hex digits", r);
  }
  return this.next(), n;
}
function de(t) {
  let e = 0;
  for (; this.isDelim(an); )
    ++e > t && this.error("Too many question marks"), this.next();
}
function vu(t) {
  this.charCodeAt(this.tokenStart) !== t && this.error((t === Ni ? "Plus sign" : "Hyphen minus") + " is expected");
}
function Eu() {
  let t = 0;
  switch (this.tokenType) {
    case x:
      if (t = ne.call(this, 1, !0), this.isDelim(an)) {
        de.call(this, 6 - t);
        break;
      }
      if (this.tokenType === $ || this.tokenType === x) {
        vu.call(this, Di), ne.call(this, 1, !1);
        break;
      }
      break;
    case $:
      t = ne.call(this, 1, !0), t > 0 && de.call(this, 6 - t);
      break;
    default:
      if (this.eatDelim(Ni), this.tokenType === d) {
        t = ne.call(this, 0, !0), t > 0 && de.call(this, 6 - t);
        break;
      }
      if (this.isDelim(an)) {
        this.next(), de.call(this, 5);
        break;
      }
      this.error("Hex digit or question mark is expected");
  }
}
const $u = "UnicodeRange", Lu = {
  value: String
};
function Fi() {
  const t = this.tokenStart;
  return this.eatIdent("u"), Eu.call(this), {
    type: "UnicodeRange",
    loc: this.getLocation(t, this.tokenStart),
    value: this.substrToCursor(t)
  };
}
function Mi(t) {
  this.tokenize(t.value);
}
const Pu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Mi,
  name: $u,
  parse: Fi,
  structure: Lu
}, Symbol.toStringTag, { value: "Module" })), Ou = 32, ln = 92, Ru = 34, Iu = 39, _u = 40, ji = 41;
function Nu(t) {
  const e = t.length;
  let n = 4, r = t.charCodeAt(e - 1) === ji ? e - 2 : e - 1, s = "";
  for (; n < r && Dt(t.charCodeAt(n)); )
    n++;
  for (; n < r && Dt(t.charCodeAt(r)); )
    r--;
  for (let o = n; o <= r; o++) {
    let a = t.charCodeAt(o);
    if (a === ln) {
      if (o === r) {
        o !== e - 1 && (s = t.substr(o + 1));
        break;
      }
      if (a = t.charCodeAt(++o), mt(ln, a)) {
        const l = o - 1, u = Xt(t, l);
        o = u - 1, s += Vr(t.substring(l + 1, u));
      } else
        a === 13 && t.charCodeAt(o + 1) === 10 && o++;
    } else
      s += t[o];
  }
  return s;
}
function Du(t) {
  let e = "", n = !1;
  for (let r = 0; r < t.length; r++) {
    const s = t.charCodeAt(r);
    if (s === 0) {
      e += "�";
      continue;
    }
    if (s <= 31 || s === 127) {
      e += "\\" + s.toString(16), n = !0;
      continue;
    }
    s === Ou || s === ln || s === Ru || s === Iu || s === _u || s === ji ? (e += "\\" + t.charAt(r), n = !1) : (n && Yt(s) && (e += " "), e += t.charAt(r), n = !1);
  }
  return "url(" + e + ")";
}
const Fu = "Url", Mu = {
  value: String
};
function Bi() {
  const t = this.tokenStart;
  let e;
  switch (this.tokenType) {
    case Q:
      e = Nu(this.consume(Q));
      break;
    case T:
      this.cmpStr(this.tokenStart, this.tokenEnd, "url(") || this.error("Function name must be `url`"), this.eat(T), this.skipSC(), e = vi(this.consume(wt)), this.skipSC(), this.eof || this.eat(y);
      break;
    default:
      this.error("Url or Function is expected");
  }
  return {
    type: "Url",
    loc: this.getLocation(t, this.tokenStart),
    value: e
  };
}
function Ui(t) {
  this.token(Q, Du(t.value));
}
const ju = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Ui,
  name: Fu,
  parse: Bi,
  structure: Mu
}, Symbol.toStringTag, { value: "Module" })), Bu = "Value", Uu = {
  children: [[]]
};
function Wi() {
  const t = this.tokenStart, e = this.readSequence(this.scope.Value);
  return {
    type: "Value",
    loc: this.getLocation(t, this.tokenStart),
    children: e
  };
}
function zi(t) {
  this.children(t);
}
const Wu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: zi,
  name: Bu,
  parse: Wi,
  structure: Uu
}, Symbol.toStringTag, { value: "Module" })), zu = Object.freeze({
  type: "WhiteSpace",
  loc: null,
  value: " "
}), Hu = "WhiteSpace", Vu = {
  value: String
};
function Hi() {
  return this.eat(U), zu;
}
function Vi(t) {
  this.token(U, t.value);
}
const Gu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  generate: Vi,
  name: Hu,
  parse: Hi,
  structure: Vu
}, Symbol.toStringTag, { value: "Module" })), qu = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  AnPlusB: sa,
  Atrule: ca,
  AtrulePrelude: pa,
  AttributeSelector: xa,
  Block: Ea,
  Brackets: Pa,
  CDC: Ia,
  CDO: Da,
  ClassSelector: Ba,
  Combinator: Ga,
  Comment: Xa,
  Condition: nl,
  Declaration: kl,
  DeclarationList: xl,
  Dimension: Tl,
  Feature: $l,
  FeatureFunction: Rl,
  FeatureRange: Fl,
  Function: Ul,
  GeneralEnclosed: Hl,
  Hash: Kl,
  IdSelector: tc,
  Identifier: Xl,
  Layer: sc,
  LayerList: ac,
  MediaQuery: uc,
  MediaQueryList: pc,
  NestingSelector: kc,
  Nth: bc,
  Number: wc,
  Operator: vc,
  Parentheses: Lc,
  Percentage: Rc,
  PseudoClassSelector: Dc,
  PseudoElementSelector: Bc,
  Ratio: zc,
  Raw: qc,
  Rule: Jc,
  Scope: eu,
  Selector: su,
  SelectorList: lu,
  String: pu,
  StyleSheet: Su,
  SupportsDeclaration: xu,
  TypeSelector: Au,
  UnicodeRange: Pu,
  Url: ju,
  Value: Wu,
  WhiteSpace: Gu
}, Symbol.toStringTag, { value: "Module" })), Ku = {
  node: qu
}, Et = Wo(Ku), xn = [
  "left",
  "right",
  "top",
  "bottom",
  "inset-block-start",
  "inset-block-end",
  "inset-inline-start",
  "inset-inline-end",
  "inset-block",
  "inset-inline",
  "inset"
];
function le(t) {
  return xn.includes(t);
}
const Cn = [
  "margin-block-start",
  "margin-block-end",
  "margin-block",
  "margin-inline-start",
  "margin-inline-end",
  "margin-inline",
  "margin-bottom",
  "margin-left",
  "margin-right",
  "margin-top",
  "margin"
];
function Qu(t) {
  return Cn.includes(t);
}
const wn = [
  "width",
  "height",
  "min-width",
  "min-height",
  "max-width",
  "max-height",
  "block-size",
  "inline-size",
  "min-block-size",
  "min-inline-size",
  "max-block-size",
  "max-inline-size"
];
function Gi(t) {
  return wn.includes(t);
}
const qi = [
  "justify-self",
  "align-self",
  "place-self"
];
function Yu(t) {
  return qi.includes(t);
}
const Ki = [
  ...xn,
  ...Cn,
  ...wn,
  ...qi,
  "position-anchor",
  "position-area"
], Xu = [
  ...wn,
  ...xn,
  ...Cn
];
function Qi(t) {
  return Xu.includes(
    t
  );
}
const Ju = [
  "top",
  "left",
  "right",
  "bottom",
  "start",
  "end",
  "self-start",
  "self-end",
  "center",
  "inside",
  "outside"
];
function Yi(t) {
  return Ju.includes(t);
}
const Zu = [
  "width",
  "height",
  "block",
  "inline",
  "self-block",
  "self-inline"
];
function th(t) {
  return Zu.includes(t);
}
const Zn = /* @__PURE__ */ new Set(["Atrule", "Selector", "Declaration"]);
function eh(t) {
  const e = new SourceMapGenerator(), n = {
    line: 1,
    column: 0
  }, r = {
    line: 0,
    // should be zero to add first mapping
    column: 0
  }, s = {
    line: 1,
    column: 0
  }, o = {
    generated: s
  };
  let a = 1, l = 0, u = !1;
  const i = t.node;
  t.node = function(f) {
    if (f.loc && f.loc.start && Zn.has(f.type)) {
      const p = f.loc.start.line, g = f.loc.start.column - 1;
      (r.line !== p || r.column !== g) && (r.line = p, r.column = g, n.line = a, n.column = l, u && (u = !1, (n.line !== s.line || n.column !== s.column) && e.addMapping(o)), u = !0, e.addMapping({
        source: f.loc.source,
        original: r,
        generated: n
      }));
    }
    i.call(this, f), u && Zn.has(f.type) && (s.line = a, s.column = l);
  };
  const c = t.emit;
  t.emit = function(f, p, g) {
    for (let m = 0; m < f.length; m++)
      f.charCodeAt(m) === 10 ? (a++, l = 0) : l++;
    c(f, p, g);
  };
  const h = t.result;
  return t.result = function() {
    return u && e.addMapping(o), {
      css: h(),
      map: e
    };
  }, t;
}
const nh = 43, rh = 45, He = (t, e) => {
  if (t === P && (t = e), typeof t == "string") {
    const n = t.charCodeAt(0);
    return n > 127 ? 32768 : n << 8;
  }
  return t;
}, Xi = [
  [d, d],
  [d, T],
  [d, Q],
  [d, ot],
  [d, "-"],
  [d, x],
  [d, F],
  [d, $],
  [d, et],
  [d, I],
  [z, d],
  [z, T],
  [z, Q],
  [z, ot],
  [z, "-"],
  [z, x],
  [z, F],
  [z, $],
  [z, et],
  [N, d],
  [N, T],
  [N, Q],
  [N, ot],
  [N, "-"],
  [N, x],
  [N, F],
  [N, $],
  [N, et],
  [$, d],
  [$, T],
  [$, Q],
  [$, ot],
  [$, "-"],
  [$, x],
  [$, F],
  [$, $],
  [$, et],
  ["#", d],
  ["#", T],
  ["#", Q],
  ["#", ot],
  ["#", "-"],
  ["#", x],
  ["#", F],
  ["#", $],
  ["#", et],
  // https://github.com/w3c/csswg-drafts/pull/6874
  ["-", d],
  ["-", T],
  ["-", Q],
  ["-", ot],
  ["-", "-"],
  ["-", x],
  ["-", F],
  ["-", $],
  ["-", et],
  // https://github.com/w3c/csswg-drafts/pull/6874
  [x, d],
  [x, T],
  [x, Q],
  [x, ot],
  [x, x],
  [x, F],
  [x, $],
  [x, "%"],
  [x, et],
  // https://github.com/w3c/csswg-drafts/pull/6874
  ["@", d],
  ["@", T],
  ["@", Q],
  ["@", ot],
  ["@", "-"],
  ["@", et],
  // https://github.com/w3c/csswg-drafts/pull/6874
  [".", x],
  [".", F],
  [".", $],
  ["+", x],
  ["+", F],
  ["+", $],
  ["/", "*"]
], sh = Xi.concat([
  [d, N],
  [$, N],
  [N, N],
  [z, I],
  [z, wt],
  [z, X],
  [F, F],
  [F, $],
  [F, T],
  [F, "-"],
  [y, d],
  [y, T],
  [y, F],
  [y, $],
  [y, N],
  [y, "-"]
]);
function Ji(t) {
  const e = new Set(
    t.map(([n, r]) => He(n) << 16 | He(r))
  );
  return function(n, r, s) {
    const o = He(r, s), a = s.charCodeAt(0);
    return (a === rh && r !== d && r !== T && r !== et || a === nh ? e.has(n << 16 | a << 8) : e.has(n << 16 | o)) && this.emit(" ", U, !0), o;
  };
}
const ih = Ji(Xi), Zi = Ji(sh), tr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  safe: Zi,
  spec: ih
}, Symbol.toStringTag, { value: "Module" })), oh = 92;
function ah(t, e) {
  if (typeof e == "function") {
    let n = null;
    t.children.forEach((r) => {
      n !== null && e.call(this, n), this.node(r), n = r;
    });
    return;
  }
  t.children.forEach(this.node, this);
}
function lh(t) {
  qr(t, (e, n, r) => {
    this.token(e, t.slice(n, r));
  });
}
function ch(t) {
  const e = /* @__PURE__ */ new Map();
  for (let [n, r] of Object.entries(t.node))
    typeof (r.generate || r) == "function" && e.set(n, r.generate || r);
  return function(n, r) {
    let s = "", o = 0, a = {
      node(u) {
        if (e.has(u.type))
          e.get(u.type).call(l, u);
        else
          throw new Error("Unknown node type: " + u.type);
      },
      tokenBefore: Zi,
      token(u, i) {
        o = this.tokenBefore(o, u, i), this.emit(i, u, !1), u === P && i.charCodeAt(0) === oh && this.emit(`
`, U, !0);
      },
      emit(u) {
        s += u;
      },
      result() {
        return s;
      }
    };
    r && (typeof r.decorator == "function" && (a = r.decorator(a)), r.sourceMap && (a = eh(a)), r.mode in tr && (a.tokenBefore = tr[r.mode]));
    const l = {
      node: (u) => a.node(u),
      children: ah,
      token: (u, i) => a.token(u, i),
      tokenize: lh
    };
    return a.node(n), a.result();
  };
}
const uh = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  AnPlusB: Qr,
  Atrule: Xr,
  AtrulePrelude: Zr,
  AttributeSelector: ns,
  Block: is,
  Brackets: as,
  CDC: cs,
  CDO: hs,
  ClassSelector: ps,
  Combinator: gs,
  Comment: ks,
  Condition: ys,
  Declaration: Cs,
  DeclarationList: Ts,
  Dimension: vs,
  Feature: $s,
  FeatureFunction: Ps,
  FeatureRange: Rs,
  Function: _s,
  GeneralEnclosed: Ds,
  Hash: Ms,
  IdSelector: Ws,
  Identifier: Bs,
  Layer: Hs,
  LayerList: Gs,
  MediaQuery: Ks,
  MediaQueryList: Ys,
  NestingSelector: Js,
  Nth: ti,
  Number: ni,
  Operator: si,
  Parentheses: oi,
  Percentage: li,
  PseudoClassSelector: ui,
  PseudoElementSelector: fi,
  Ratio: di,
  Raw: mi,
  Rule: Si,
  Scope: bi,
  Selector: Ci,
  SelectorList: Ti,
  String: $i,
  StyleSheet: Pi,
  SupportsDeclaration: Ri,
  TypeSelector: _i,
  UnicodeRange: Mi,
  Url: Ui,
  Value: zi,
  WhiteSpace: Vi
}, Symbol.toStringTag, { value: "Module" })), hh = {
  node: uh
}, fh = ch(hh);
let Wt = null;
class K {
  static createItem(e) {
    return {
      prev: null,
      next: null,
      data: e
    };
  }
  constructor() {
    this.head = null, this.tail = null, this.cursor = null;
  }
  createItem(e) {
    return K.createItem(e);
  }
  // cursor helpers
  allocateCursor(e, n) {
    let r;
    return Wt !== null ? (r = Wt, Wt = Wt.cursor, r.prev = e, r.next = n, r.cursor = this.cursor) : r = {
      prev: e,
      next: n,
      cursor: this.cursor
    }, this.cursor = r, r;
  }
  releaseCursor() {
    const { cursor: e } = this;
    this.cursor = e.cursor, e.prev = null, e.next = null, e.cursor = Wt, Wt = e;
  }
  updateCursors(e, n, r, s) {
    let { cursor: o } = this;
    for (; o !== null; )
      o.prev === e && (o.prev = n), o.next === r && (o.next = s), o = o.cursor;
  }
  *[Symbol.iterator]() {
    for (let e = this.head; e !== null; e = e.next)
      yield e.data;
  }
  // getters
  get size() {
    let e = 0;
    for (let n = this.head; n !== null; n = n.next)
      e++;
    return e;
  }
  get isEmpty() {
    return this.head === null;
  }
  get first() {
    return this.head && this.head.data;
  }
  get last() {
    return this.tail && this.tail.data;
  }
  // convertors
  fromArray(e) {
    let n = null;
    this.head = null;
    for (let r of e) {
      const s = K.createItem(r);
      n !== null ? n.next = s : this.head = s, s.prev = n, n = s;
    }
    return this.tail = n, this;
  }
  toArray() {
    return [...this];
  }
  toJSON() {
    return [...this];
  }
  // array-like methods
  forEach(e, n = this) {
    const r = this.allocateCursor(null, this.head);
    for (; r.next !== null; ) {
      const s = r.next;
      r.next = s.next, e.call(n, s.data, s, this);
    }
    this.releaseCursor();
  }
  forEachRight(e, n = this) {
    const r = this.allocateCursor(this.tail, null);
    for (; r.prev !== null; ) {
      const s = r.prev;
      r.prev = s.prev, e.call(n, s.data, s, this);
    }
    this.releaseCursor();
  }
  reduce(e, n, r = this) {
    let s = this.allocateCursor(null, this.head), o = n, a;
    for (; s.next !== null; )
      a = s.next, s.next = a.next, o = e.call(r, o, a.data, a, this);
    return this.releaseCursor(), o;
  }
  reduceRight(e, n, r = this) {
    let s = this.allocateCursor(this.tail, null), o = n, a;
    for (; s.prev !== null; )
      a = s.prev, s.prev = a.prev, o = e.call(r, o, a.data, a, this);
    return this.releaseCursor(), o;
  }
  some(e, n = this) {
    for (let r = this.head; r !== null; r = r.next)
      if (e.call(n, r.data, r, this))
        return !0;
    return !1;
  }
  map(e, n = this) {
    const r = new K();
    for (let s = this.head; s !== null; s = s.next)
      r.appendData(e.call(n, s.data, s, this));
    return r;
  }
  filter(e, n = this) {
    const r = new K();
    for (let s = this.head; s !== null; s = s.next)
      e.call(n, s.data, s, this) && r.appendData(s.data);
    return r;
  }
  nextUntil(e, n, r = this) {
    if (e === null)
      return;
    const s = this.allocateCursor(null, e);
    for (; s.next !== null; ) {
      const o = s.next;
      if (s.next = o.next, n.call(r, o.data, o, this))
        break;
    }
    this.releaseCursor();
  }
  prevUntil(e, n, r = this) {
    if (e === null)
      return;
    const s = this.allocateCursor(e, null);
    for (; s.prev !== null; ) {
      const o = s.prev;
      if (s.prev = o.prev, n.call(r, o.data, o, this))
        break;
    }
    this.releaseCursor();
  }
  // mutation
  clear() {
    this.head = null, this.tail = null;
  }
  copy() {
    const e = new K();
    for (let n of this)
      e.appendData(n);
    return e;
  }
  prepend(e) {
    return this.updateCursors(null, e, this.head, e), this.head !== null ? (this.head.prev = e, e.next = this.head) : this.tail = e, this.head = e, this;
  }
  prependData(e) {
    return this.prepend(K.createItem(e));
  }
  append(e) {
    return this.insert(e);
  }
  appendData(e) {
    return this.insert(K.createItem(e));
  }
  insert(e, n = null) {
    if (n !== null)
      if (this.updateCursors(n.prev, e, n, e), n.prev === null) {
        if (this.head !== n)
          throw new Error("before doesn't belong to list");
        this.head = e, n.prev = e, e.next = n, this.updateCursors(null, e);
      } else
        n.prev.next = e, e.prev = n.prev, n.prev = e, e.next = n;
    else
      this.updateCursors(this.tail, e, null, e), this.tail !== null ? (this.tail.next = e, e.prev = this.tail) : this.head = e, this.tail = e;
    return this;
  }
  insertData(e, n) {
    return this.insert(K.createItem(e), n);
  }
  remove(e) {
    if (this.updateCursors(e, e.prev, e, e.next), e.prev !== null)
      e.prev.next = e.next;
    else {
      if (this.head !== e)
        throw new Error("item doesn't belong to list");
      this.head = e.next;
    }
    if (e.next !== null)
      e.next.prev = e.prev;
    else {
      if (this.tail !== e)
        throw new Error("item doesn't belong to list");
      this.tail = e.prev;
    }
    return e.prev = null, e.next = null, e;
  }
  push(e) {
    this.insert(K.createItem(e));
  }
  pop() {
    return this.tail !== null ? this.remove(this.tail) : null;
  }
  unshift(e) {
    this.prepend(K.createItem(e));
  }
  shift() {
    return this.head !== null ? this.remove(this.head) : null;
  }
  prependList(e) {
    return this.insertList(e, this.head);
  }
  appendList(e) {
    return this.insertList(e);
  }
  insertList(e, n) {
    return e.head === null ? this : (n != null ? (this.updateCursors(n.prev, e.tail, n, e.head), n.prev !== null ? (n.prev.next = e.head, e.head.prev = n.prev) : this.head = e.head, n.prev = e.tail, e.tail.next = n) : (this.updateCursors(this.tail, e.tail, null, e.head), this.tail !== null ? (this.tail.next = e.head, e.head.prev = this.tail) : this.head = e.head, this.tail = e.tail), e.head = null, e.tail = null, this);
  }
  replace(e, n) {
    "head" in n ? this.insertList(n, e) : this.insert(n, e), this.remove(e);
  }
}
function ph(t, e) {
  const n = Object.create(SyntaxError.prototype), r = new Error();
  return Object.assign(n, {
    name: t,
    message: e,
    get stack() {
      return (r.stack || "").replace(/^(.+\n){1,3}/, `${t}: ${e}
`);
    }
  });
}
const Ve = 100, er = 60, nr = "    ";
function rr({ source: t, line: e, column: n, baseLine: r, baseColumn: s }, o) {
  function a(g, m) {
    return i.slice(g, m).map(
      (S, b) => String(g + b + 1).padStart(f) + " |" + S
    ).join(`
`);
  }
  const l = `
`.repeat(Math.max(r - 1, 0)), u = " ".repeat(Math.max(s - 1, 0)), i = (l + u + t).split(/\r\n?|\n|\f/), c = Math.max(1, e - o) - 1, h = Math.min(e + o, i.length + 1), f = Math.max(4, String(h).length) + 1;
  let p = 0;
  n += (nr.length - 1) * (i[e - 1].substr(0, n - 1).match(/\t/g) || []).length, n > Ve && (p = n - er + 3, n = er - 2);
  for (let g = c; g <= h; g++)
    g >= 0 && g < i.length && (i[g] = i[g].replace(/\t/g, nr), i[g] = (p > 0 && i[g].length > p ? "…" : "") + i[g].substr(p, Ve - 2) + (i[g].length > p + Ve - 1 ? "…" : ""));
  return [
    a(c, e),
    new Array(n + f + 2).join("-") + "^",
    a(e, h)
  ].filter(Boolean).join(`
`).replace(/^(\s+\d+\s+\|\n)+/, "").replace(/\n(\s+\d+\s+\|)+$/, "");
}
function sr(t, e, n, r, s, o = 1, a = 1) {
  return Object.assign(ph("SyntaxError", t), {
    source: e,
    offset: n,
    line: r,
    column: s,
    sourceFragment(u) {
      return rr({ source: e, line: r, column: s, baseLine: o, baseColumn: a }, isNaN(u) ? 0 : u);
    },
    get formattedMessage() {
      return `Parse error: ${t}
` + rr({ source: e, line: r, column: s, baseLine: o, baseColumn: a }, 2);
    }
  });
}
function dh(t) {
  const e = this.createList();
  let n = !1;
  const r = {
    recognizer: t
  };
  for (; !this.eof; ) {
    switch (this.tokenType) {
      case Y:
        this.next();
        continue;
      case U:
        n = !0, this.next();
        continue;
    }
    let s = t.getNode.call(this, r);
    if (s === void 0)
      break;
    n && (t.onWhiteSpace && t.onWhiteSpace.call(this, s, e, r), n = !1), e.push(s);
  }
  return n && t.onWhiteSpace && t.onWhiteSpace.call(this, null, e, r), e;
}
const ir = () => {
}, gh = 33, mh = 35, Ge = 59, or = 123, ar = 0;
function kh(t) {
  return function() {
    return this[t]();
  };
}
function qe(t) {
  const e = /* @__PURE__ */ Object.create(null);
  for (const n of Object.keys(t)) {
    const r = t[n], s = r.parse || r;
    s && (e[n] = s);
  }
  return e;
}
function Sh(t) {
  const e = {
    context: /* @__PURE__ */ Object.create(null),
    features: Object.assign(/* @__PURE__ */ Object.create(null), t.features),
    scope: Object.assign(/* @__PURE__ */ Object.create(null), t.scope),
    atrule: qe(t.atrule),
    pseudo: qe(t.pseudo),
    node: qe(t.node)
  };
  for (const [n, r] of Object.entries(t.parseContext))
    switch (typeof r) {
      case "function":
        e.context[n] = r;
        break;
      case "string":
        e.context[n] = kh(r);
        break;
    }
  return W(W({
    config: e
  }, e), e.node);
}
function yh(t) {
  let e = "", n = "<unknown>", r = !1, s = ir, o = !1;
  const a = new Zo(), l = Object.assign(new ta(), Sh(t || {}), {
    parseAtrulePrelude: !0,
    parseRulePrelude: !0,
    parseValue: !0,
    parseCustomProperty: !1,
    readSequence: dh,
    consumeUntilBalanceEnd: () => 0,
    consumeUntilLeftCurlyBracket(i) {
      return i === or ? 1 : 0;
    },
    consumeUntilLeftCurlyBracketOrSemicolon(i) {
      return i === or || i === Ge ? 1 : 0;
    },
    consumeUntilExclamationMarkOrSemicolon(i) {
      return i === gh || i === Ge ? 1 : 0;
    },
    consumeUntilSemicolonIncluded(i) {
      return i === Ge ? 2 : 0;
    },
    createList() {
      return new K();
    },
    createSingleNodeList(i) {
      return new K().appendData(i);
    },
    getFirstListNode(i) {
      return i && i.first;
    },
    getLastListNode(i) {
      return i && i.last;
    },
    parseWithFallback(i, c) {
      const h = this.tokenIndex;
      try {
        return i.call(this);
      } catch (f) {
        if (o)
          throw f;
        this.skip(h - this.tokenIndex);
        const p = c.call(this);
        return o = !0, s(f, p), o = !1, p;
      }
    },
    lookupNonWSType(i) {
      let c;
      do
        if (c = this.lookupType(i++), c !== U && c !== Y)
          return c;
      while (c !== ar);
      return ar;
    },
    charCodeAt(i) {
      return i >= 0 && i < e.length ? e.charCodeAt(i) : 0;
    },
    substring(i, c) {
      return e.substring(i, c);
    },
    substrToCursor(i) {
      return this.source.substring(i, this.tokenStart);
    },
    cmpChar(i, c) {
      return zr(e, i, c);
    },
    cmpStr(i, c, h) {
      return ve(e, i, c, h);
    },
    consume(i) {
      const c = this.tokenStart;
      return this.eat(i), this.substrToCursor(c);
    },
    consumeFunctionName() {
      const i = e.substring(this.tokenStart, this.tokenEnd - 1);
      return this.eat(T), i;
    },
    consumeNumber(i) {
      const c = e.substring(this.tokenStart, Hr(e, this.tokenStart));
      return this.eat(i), c;
    },
    eat(i) {
      if (this.tokenType !== i) {
        const c = Gr[i].slice(0, -6).replace(/-/g, " ").replace(/^./, (p) => p.toUpperCase());
        let h = `${/[[\](){}]/.test(c) ? `"${c}"` : c} is expected`, f = this.tokenStart;
        switch (i) {
          case d:
            this.tokenType === T || this.tokenType === Q ? (f = this.tokenEnd - 1, h = "Identifier is expected but function found") : h = "Identifier is expected";
            break;
          case N:
            this.isDelim(mh) && (this.next(), f++, h = "Name is expected");
            break;
          case F:
            this.tokenType === x && (f = this.tokenEnd, h = "Percent sign is expected");
            break;
        }
        this.error(h, f);
      }
      this.next();
    },
    eatIdent(i) {
      (this.tokenType !== d || this.lookupValue(0, i) === !1) && this.error(`Identifier "${i}" is expected`), this.next();
    },
    eatDelim(i) {
      this.isDelim(i) || this.error(`Delim "${String.fromCharCode(i)}" is expected`), this.next();
    },
    getLocation(i, c) {
      return r ? a.getLocationRange(
        i,
        c,
        n
      ) : null;
    },
    getLocationFromList(i) {
      if (r) {
        const c = this.getFirstListNode(i), h = this.getLastListNode(i);
        return a.getLocationRange(
          c !== null ? c.loc.start.offset - a.startOffset : this.tokenStart,
          h !== null ? h.loc.end.offset - a.startOffset : this.tokenStart,
          n
        );
      }
      return null;
    },
    error(i, c) {
      const h = typeof c != "undefined" && c < e.length ? a.getLocation(c) : this.eof ? a.getLocation(Yo(e, e.length - 1)) : a.getLocation(this.tokenStart);
      throw new sr(
        i || "Unexpected input",
        e,
        h.offset,
        h.line,
        h.column,
        a.startLine,
        a.startColumn
      );
    }
  });
  return Object.assign(function(i, c) {
    e = i, c = c || {}, l.setSource(e, qr), a.setSource(
      e,
      c.offset,
      c.line,
      c.column
    ), n = c.filename || "<unknown>", r = !!c.positions, s = typeof c.onParseError == "function" ? c.onParseError : ir, o = !1, l.parseAtrulePrelude = "parseAtrulePrelude" in c ? !!c.parseAtrulePrelude : !0, l.parseRulePrelude = "parseRulePrelude" in c ? !!c.parseRulePrelude : !0, l.parseValue = "parseValue" in c ? !!c.parseValue : !0, l.parseCustomProperty = "parseCustomProperty" in c ? !!c.parseCustomProperty : !1;
    const { context: h = "default", onComment: f } = c;
    if (!(h in l.context))
      throw new Error("Unknown context `" + h + "`");
    typeof f == "function" && l.forEachToken((g, m, S) => {
      if (g === Y) {
        const b = l.getLocation(m, S), C = ve(e, S - 2, S, "*/") ? e.slice(m + 2, S - 2) : e.slice(m + 2, S);
        f(C, b);
      }
    });
    const p = l.context[h].call(l, c);
    return l.eof || l.error(), p;
  }, {
    SyntaxError: sr,
    config: l.config
  });
}
const bh = 35, xh = 42, lr = 43, Ch = 45, wh = 47, Th = 117;
function to(t) {
  switch (this.tokenType) {
    case N:
      return this.Hash();
    case ht:
      return this.Operator();
    case I:
      return this.Parentheses(this.readSequence, t.recognizer);
    case te:
      return this.Brackets(this.readSequence, t.recognizer);
    case wt:
      return this.String();
    case $:
      return this.Dimension();
    case F:
      return this.Percentage();
    case x:
      return this.Number();
    case T:
      return this.cmpStr(this.tokenStart, this.tokenEnd, "url(") ? this.Url() : this.Function(this.readSequence, t.recognizer);
    case Q:
      return this.Url();
    case d:
      return this.cmpChar(this.tokenStart, Th) && this.cmpChar(this.tokenStart + 1, lr) ? this.UnicodeRange() : this.Identifier();
    case P: {
      const e = this.charCodeAt(this.tokenStart);
      if (e === wh || e === xh || e === lr || e === Ch)
        return this.Operator();
      e === bh && this.error("Hex or identifier is expected", this.tokenStart + 1);
      break;
    }
  }
}
const Ah = {
  getNode: to
}, vh = 35, Eh = 38, $h = 42, Lh = 43, Ph = 47, cr = 46, Oh = 62, Rh = 124, Ih = 126;
function _h(t, e) {
  e.last !== null && e.last.type !== "Combinator" && t !== null && t.type !== "Combinator" && e.push({
    // FIXME: this.Combinator() should be used instead
    type: "Combinator",
    loc: null,
    name: " "
  });
}
function Nh() {
  switch (this.tokenType) {
    case te:
      return this.AttributeSelector();
    case N:
      return this.IdSelector();
    case X:
      return this.lookupType(1) === X ? this.PseudoElementSelector() : this.PseudoClassSelector();
    case d:
      return this.TypeSelector();
    case x:
    case F:
      return this.Percentage();
    case $:
      this.charCodeAt(this.tokenStart) === cr && this.error("Identifier is expected", this.tokenStart + 1);
      break;
    case P: {
      switch (this.charCodeAt(this.tokenStart)) {
        case Lh:
        case Oh:
        case Ih:
        case Ph:
          return this.Combinator();
        case cr:
          return this.ClassSelector();
        case $h:
        case Rh:
          return this.TypeSelector();
        case vh:
          return this.IdSelector();
        case Eh:
          return this.NestingSelector();
      }
      break;
    }
  }
}
const Dh = {
  onWhiteSpace: _h,
  getNode: Nh
};
function Fh() {
  return this.createSingleNodeList(
    this.Raw(null, !1)
  );
}
function Mh() {
  const t = this.createList();
  if (this.skipSC(), t.push(this.Identifier()), this.skipSC(), this.tokenType === ht) {
    t.push(this.Operator());
    const e = this.tokenIndex, n = this.parseCustomProperty ? this.Value(null) : this.Raw(this.consumeUntilExclamationMarkOrSemicolon, !1);
    if (n.type === "Value" && n.children.isEmpty) {
      for (let r = e - this.tokenIndex; r <= 0; r++)
        if (this.lookupType(r) === U) {
          n.children.appendData({
            type: "WhiteSpace",
            loc: null,
            value: " "
          });
          break;
        }
    }
    t.push(n);
  }
  return t;
}
function ur(t) {
  return t !== null && t.type === "Operator" && (t.value[t.value.length - 1] === "-" || t.value[t.value.length - 1] === "+");
}
const jh = {
  getNode: to,
  onWhiteSpace(t, e) {
    ur(t) && (t.value = " " + t.value), ur(e.last) && (e.last.value += " ");
  },
  expression: Fh,
  var: Mh
}, Bh = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  AtrulePrelude: Ah,
  Selector: Dh,
  Value: jh
}, Symbol.toStringTag, { value: "Module" })), Uh = /* @__PURE__ */ new Set(["none", "and", "not", "or"]), Wh = {
  parse: {
    prelude() {
      const t = this.createList();
      if (this.tokenType === d) {
        const e = this.substring(this.tokenStart, this.tokenEnd);
        Uh.has(e.toLowerCase()) || t.push(this.Identifier());
      }
      return t.push(this.Condition("container")), t;
    },
    block(t = !1) {
      return this.Block(t);
    }
  }
}, zh = {
  parse: {
    prelude: null,
    block() {
      return this.Block(!0);
    }
  }
};
function Ke(t, e) {
  return this.parseWithFallback(
    () => {
      try {
        return t.call(this);
      } finally {
        this.skipSC(), this.lookupNonWSType(0) !== y && this.error();
      }
    },
    e || (() => this.Raw(null, !0))
  );
}
const hr = {
  layer() {
    this.skipSC();
    const t = this.createList(), e = Ke.call(this, this.Layer);
    return (e.type !== "Raw" || e.value !== "") && t.push(e), t;
  },
  supports() {
    this.skipSC();
    const t = this.createList(), e = Ke.call(
      this,
      this.Declaration,
      () => Ke.call(this, () => this.Condition("supports"))
    );
    return (e.type !== "Raw" || e.value !== "") && t.push(e), t;
  }
}, Hh = {
  parse: {
    prelude() {
      const t = this.createList();
      switch (this.tokenType) {
        case wt:
          t.push(this.String());
          break;
        case Q:
        case T:
          t.push(this.Url());
          break;
        default:
          this.error("String or url() is expected");
      }
      return this.skipSC(), this.tokenType === d && this.cmpStr(this.tokenStart, this.tokenEnd, "layer") ? t.push(this.Identifier()) : this.tokenType === T && this.cmpStr(this.tokenStart, this.tokenEnd, "layer(") && t.push(this.Function(null, hr)), this.skipSC(), this.tokenType === T && this.cmpStr(this.tokenStart, this.tokenEnd, "supports(") && t.push(this.Function(null, hr)), (this.lookupNonWSType(0) === d || this.lookupNonWSType(0) === I) && t.push(this.MediaQueryList()), t;
    },
    block: null
  }
}, Vh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.LayerList()
      );
    },
    block() {
      return this.Block(!1);
    }
  }
}, Gh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.MediaQueryList()
      );
    },
    block(t = !1) {
      return this.Block(t);
    }
  }
}, qh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.SelectorList()
      );
    },
    block() {
      return this.Block(!0);
    }
  }
}, Kh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.SelectorList()
      );
    },
    block() {
      return this.Block(!0);
    }
  }
}, Qh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.Scope()
      );
    },
    block(t = !1) {
      return this.Block(t);
    }
  }
}, Yh = {
  parse: {
    prelude: null,
    block(t = !1) {
      return this.Block(t);
    }
  }
}, Xh = {
  parse: {
    prelude() {
      return this.createSingleNodeList(
        this.Condition("supports")
      );
    },
    block(t = !1) {
      return this.Block(t);
    }
  }
}, Jh = {
  container: Wh,
  "font-face": zh,
  import: Hh,
  layer: Vh,
  media: Gh,
  nest: qh,
  page: Kh,
  scope: Qh,
  "starting-style": Yh,
  supports: Xh
};
function Zh() {
  const t = this.createList();
  this.skipSC();
  t: for (; !this.eof; ) {
    switch (this.tokenType) {
      case d:
        t.push(this.Identifier());
        break;
      case wt:
        t.push(this.String());
        break;
      case ht:
        t.push(this.Operator());
        break;
      case y:
        break t;
      default:
        this.error("Identifier, string or comma is expected");
    }
    this.skipSC();
  }
  return t;
}
const Ot = {
  parse() {
    return this.createSingleNodeList(
      this.SelectorList()
    );
  }
}, Qe = {
  parse() {
    return this.createSingleNodeList(
      this.Selector()
    );
  }
}, tf = {
  parse() {
    return this.createSingleNodeList(
      this.Identifier()
    );
  }
}, ef = {
  parse: Zh
}, ge = {
  parse() {
    return this.createSingleNodeList(
      this.Nth()
    );
  }
}, nf = {
  dir: tf,
  has: Ot,
  lang: ef,
  matches: Ot,
  is: Ot,
  "-moz-any": Ot,
  "-webkit-any": Ot,
  where: Ot,
  not: Ot,
  "nth-child": ge,
  "nth-last-child": ge,
  "nth-last-of-type": ge,
  "nth-of-type": ge,
  slotted: Qe,
  host: Qe,
  "host-context": Qe
}, rf = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  AnPlusB: Kr,
  Atrule: Yr,
  AtrulePrelude: Jr,
  AttributeSelector: es,
  Block: ss,
  Brackets: os,
  CDC: ls,
  CDO: us,
  ClassSelector: fs,
  Combinator: ds,
  Comment: ms,
  Condition: Ss,
  Declaration: xs,
  DeclarationList: ws,
  Dimension: As,
  Feature: Es,
  FeatureFunction: Ls,
  FeatureRange: Os,
  Function: Is,
  GeneralEnclosed: Ns,
  Hash: Fs,
  IdSelector: Us,
  Identifier: js,
  Layer: zs,
  LayerList: Vs,
  MediaQuery: qs,
  MediaQueryList: Qs,
  NestingSelector: Xs,
  Nth: Zs,
  Number: ei,
  Operator: ri,
  Parentheses: ii,
  Percentage: ai,
  PseudoClassSelector: ci,
  PseudoElementSelector: hi,
  Ratio: pi,
  Raw: gi,
  Rule: ki,
  Scope: yi,
  Selector: xi,
  SelectorList: wi,
  String: Ei,
  StyleSheet: Li,
  SupportsDeclaration: Oi,
  TypeSelector: Ii,
  UnicodeRange: Fi,
  Url: Bi,
  Value: Wi,
  WhiteSpace: Hi
}, Symbol.toStringTag, { value: "Module" })), sf = {
  parseContext: {
    default: "StyleSheet",
    stylesheet: "StyleSheet",
    atrule: "Atrule",
    atrulePrelude(t) {
      return this.AtrulePrelude(t.atrule ? String(t.atrule) : null);
    },
    mediaQueryList: "MediaQueryList",
    mediaQuery: "MediaQuery",
    condition(t) {
      return this.Condition(t.kind);
    },
    rule: "Rule",
    selectorList: "SelectorList",
    selector: "Selector",
    block() {
      return this.Block(!0);
    },
    declarationList: "DeclarationList",
    declaration: "Declaration",
    value: "Value"
  },
  features: {
    supports: {
      selector() {
        return this.Selector();
      }
    },
    container: {
      style() {
        return this.Declaration();
      }
    }
  },
  scope: Bh,
  atrule: Jh,
  pseudo: nf,
  node: rf
}, of = yh(sf);
function $e(t) {
  const e = {};
  for (const n of Object.keys(t)) {
    let r = t[n];
    r && (Array.isArray(r) || r instanceof K ? r = r.map($e) : r.constructor === Object && (r = $e(r))), e[n] = r;
  }
  return e;
}
let af = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict", at = (t = 21) => {
  let e = "", n = t | 0;
  for (; n--; )
    e += af[Math.random() * 64 | 0];
  return e;
};
const eo = at(), re = /* @__PURE__ */ new Set();
function Le(t) {
  return !!(t && t.type === "Function" && t.name === "anchor");
}
function $t(t, e = !1) {
  return of(t, {
    parseAtrulePrelude: !1,
    parseCustomProperty: !0,
    onParseError: (n) => {
      e && re.add(n);
    }
  });
}
function Z(t) {
  return fh(t, {
    // Default `safe` adds extra (potentially breaking) spaces for compatibility
    // with old browsers.
    mode: "spec"
  });
}
function lf(t) {
  return t.type === "Declaration";
}
function cf(t) {
  return t.toArray().reduce(
    (e, n) => n.type === "Operator" && n.value === "," ? (e.push([]), e) : (n.type === "Identifier" && e[e.length - 1].push(n), e),
    [[]]
  );
}
function cn(t) {
  return t ? t.children.map((e) => {
    var s;
    let n;
    ((s = e.children.last) == null ? void 0 : s.type) === "PseudoElementSelector" && (e = $e(e), n = Z(e.children.last), e.children.pop());
    const r = Z(e);
    return {
      selector: r + (n != null ? n : ""),
      elementPart: r,
      pseudoElementPart: n
    };
  }).toArray() : [];
}
function uf() {
  re.size > 0 && (console.group(
    `The CSS anchor positioning polyfill was not applied due to ${re.size === 1 ? "a CSS parse error" : "CSS parse errors"}.`
  ), re.forEach((t) => {
    console.warn(t.formattedMessage);
  }), console.groupEnd());
}
function hf() {
  re.clear();
}
const un = [
  ...Ki,
  "anchor-scope",
  "anchor-name"
].reduce(
  (t, e) => (t[e] = `--${e}-${eo}`, t),
  {}
);
function ff(t, e) {
  return lf(t) && un[t.property] && e ? (e.children.appendData(q(W({}, t), {
    property: un[t.property]
  })), { updated: !0 }) : {};
}
function pf(t) {
  for (const e of t) {
    let n = !1;
    const r = $t(e.css, !0);
    Et(r, {
      visit: "Declaration",
      enter(s) {
        var l;
        const o = (l = this.rule) == null ? void 0 : l.block, { updated: a } = ff(s, o);
        a && (n = !0);
      }
    }), n && (e.css = Z(r), e.changed = !0);
  }
  return t.some((e) => e.changed === !0);
}
var no = /* @__PURE__ */ ((t) => (t.All = "all", t.None = "none", t))(no || {});
function it(t, e) {
  var r;
  return e = (r = un[e]) != null ? r : e, (t instanceof HTMLElement ? getComputedStyle(t) : t.computedStyle).getPropertyValue(e).trim();
}
function Jt(t, e, n) {
  return it(t, e) === n;
}
function df(t, { selector: e, pseudoElementPart: n }) {
  const r = getComputedStyle(t, n), s = document.createElement("div"), o = document.createElement("style");
  s.id = `fake-pseudo-element-${at()}`;
  for (const l of Array.from(r)) {
    const u = r.getPropertyValue(l);
    s.style.setProperty(l, u);
  }
  o.textContent += `#${s.id}${n} { content: ${r.content}; }`, o.textContent += `${e} { display: none !important; }`, document.head.append(o);
  const a = n === "::before" ? "afterbegin" : "beforeend";
  return t.insertAdjacentElement(a, s), { fakePseudoElement: s, sheet: o, computedStyle: r };
}
function gf(t) {
  let e = t;
  for (; e; ) {
    if (Jt(e, "overflow", "scroll"))
      return e;
    e = e.parentElement;
  }
  return e;
}
function mf(t) {
  let e = gf(t);
  return e === document.documentElement && (e = null), e != null ? e : { scrollTop: 0, scrollLeft: 0 };
}
function kf(t) {
  const { elementPart: e, pseudoElementPart: n } = t, r = [];
  if (n && !(n === "::before" || n === "::after")) return r;
  const a = Array.from(
    document.querySelectorAll(e)
  );
  if (!n)
    return r.push(...a), r;
  for (const l of a) {
    const { fakePseudoElement: u, sheet: i, computedStyle: c } = df(
      l,
      t
    ), h = u.getBoundingClientRect(), { scrollY: f, scrollX: p } = globalThis, g = mf(l);
    r.push({
      fakePseudoElement: u,
      computedStyle: c,
      removeFakePseudoElement() {
        u.remove(), i.remove();
      },
      // For https://floating-ui.com/docs/autoupdate#ancestorscroll to work on
      // `VirtualElement`s.
      contextElement: l,
      // https://floating-ui.com/docs/virtual-elements
      getBoundingClientRect() {
        const { scrollY: m, scrollX: S } = globalThis, { scrollTop: b, scrollLeft: C } = g;
        return DOMRect.fromRect({
          y: h.y + (f - m) + (g.scrollTop - b),
          x: h.x + (p - S) + (g.scrollLeft - C),
          width: h.width,
          height: h.height
        });
      }
    });
  }
  return r;
}
function Sf(t, e) {
  const n = it(t, "anchor-name");
  return e ? n.split(",").map((r) => r.trim()).includes(e) : !n;
}
function yf(t, e) {
  const n = it(t, "anchor-scope");
  return n === e || n === "all";
}
const Tn = (t) => R(null, null, function* () {
  var n, r, s;
  let e = yield (n = H.getOffsetParent) == null ? void 0 : n.call(H, t);
  return (yield (r = H.isElement) == null ? void 0 : r.call(H, e)) || (e = (yield (s = H.getDocumentElement) == null ? void 0 : s.call(H, t)) || window.document.documentElement), e;
}), fr = "InvalidMimeType";
function bf(t) {
  return !!((t.type === "text/css" || t.rel === "stylesheet") && t.href);
}
function xf(t) {
  const e = new URL(t.href, document.baseURI);
  if (bf(t) && e.origin === location.origin)
    return e;
}
function Cf(t) {
  return R(this, null, function* () {
    return (yield Promise.all(
      t.map((n) => R(null, null, function* () {
        var r;
        if (!n.url)
          return n;
        if ((r = n.el) != null && r.disabled)
          return null;
        try {
          const s = yield fetch(n.url.toString()), o = s.headers.get("content-type");
          if (!(o != null && o.startsWith("text/css"))) {
            const l = new Error(
              `Error loading ${n.url}: expected content-type "text/css", got "${o}".`
            );
            throw l.name = fr, l;
          }
          const a = yield s.text();
          return q(W({}, n), { css: a });
        } catch (s) {
          if (s instanceof Error && s.name === fr)
            return console.warn(s), null;
          throw s;
        }
      }))
    )).filter((n) => n !== null);
  });
}
const pr = '[style*="anchor"]', dr = '[style*="position-area"]';
function wf(t) {
  const e = t ? t.filter(
    (r) => r instanceof HTMLElement && (r.matches(pr) || r.matches(dr))
  ) : Array.from(
    document.querySelectorAll(
      [
        pr,
        dr
      ].join(",")
    )
  ), n = [];
  return e.filter((r) => r instanceof HTMLElement).forEach((r) => {
    const s = at(12), o = "data-has-inline-styles";
    r.setAttribute(o, s);
    const a = r.getAttribute("style"), l = `[${o}="${s}"] { ${a} }`;
    n.push({ el: r, css: l });
  }), n;
}
function Tf(t, e) {
  return R(this, null, function* () {
    const n = t != null ? t : Array.from(document.querySelectorAll("link, style")), r = [];
    n.filter((a) => a instanceof HTMLElement).forEach((a) => {
      if (a.tagName.toLowerCase() === "link") {
        const l = xf(a);
        l && r.push({ el: a, url: l });
      }
      a.tagName.toLowerCase() === "style" && r.push({ el: a, css: a.innerHTML });
    });
    const s = e ? t != null ? t : [] : void 0, o = wf(s);
    return yield Cf([...r, ...o]);
  });
}
const Af = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict";
let ro = (t = 21) => {
  let e = "", n = crypto.getRandomValues(new Uint8Array(t |= 0));
  for (; t--; )
    e += Af[n[t] & 63];
  return e;
};
const so = "--pa-cascade-property", io = "data-anchor-position-wrapper", oo = "data-pa-wrapper-for-", gr = "POLYFILL-POSITION-AREA", vf = [
  "left",
  "center",
  "right",
  "span-left",
  "span-right",
  "x-start",
  "x-end",
  "span-x-start",
  "span-x-end",
  "x-self-start",
  "x-self-end",
  "span-x-self-start",
  "span-x-self-end",
  "span-all",
  "top",
  "bottom",
  "span-top",
  "span-bottom",
  "y-start",
  "y-end",
  "span-y-start",
  "span-y-end",
  "y-self-start",
  "y-self-end",
  "span-y-self-start",
  "span-y-self-end",
  "block-start",
  "block-end",
  "span-block-start",
  "span-block-end",
  "inline-start",
  "inline-end",
  "span-inline-start",
  "span-inline-end",
  "self-block-start",
  "self-block-end",
  "span-self-block-start",
  "span-self-block-end",
  "self-inline-start",
  "self-inline-end",
  "span-self-inline-start",
  "span-self-inline-end",
  "start",
  "end",
  "span-start",
  "span-end",
  "self-start",
  "self-end",
  "span-self-start",
  "span-self-end"
];
function ao(t) {
  return vf.includes(t);
}
const mr = {
  left: [
    0,
    1,
    "Irrelevant"
    /* Irrelevant */
  ],
  center: [
    1,
    2,
    "Irrelevant"
    /* Irrelevant */
  ],
  right: [
    2,
    3,
    "Irrelevant"
    /* Irrelevant */
  ],
  "span-left": [
    0,
    2,
    "Irrelevant"
    /* Irrelevant */
  ],
  "span-right": [
    1,
    3,
    "Irrelevant"
    /* Irrelevant */
  ],
  "x-start": [
    0,
    1,
    "Physical"
    /* Physical */
  ],
  "x-end": [
    2,
    3,
    "Physical"
    /* Physical */
  ],
  "span-x-start": [
    0,
    2,
    "Physical"
    /* Physical */
  ],
  "span-x-end": [
    1,
    3,
    "Physical"
    /* Physical */
  ],
  "x-self-start": [
    0,
    1,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "x-self-end": [
    2,
    3,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "span-x-self-start": [
    0,
    2,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "span-x-self-end": [
    1,
    3,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "span-all": [
    0,
    3,
    "Irrelevant"
    /* Irrelevant */
  ],
  top: [
    0,
    1,
    "Irrelevant"
    /* Irrelevant */
  ],
  bottom: [
    2,
    3,
    "Irrelevant"
    /* Irrelevant */
  ],
  "span-top": [
    0,
    2,
    "Irrelevant"
    /* Irrelevant */
  ],
  "span-bottom": [
    1,
    3,
    "Irrelevant"
    /* Irrelevant */
  ],
  "y-start": [
    0,
    1,
    "Physical"
    /* Physical */
  ],
  "y-end": [
    2,
    3,
    "Physical"
    /* Physical */
  ],
  "span-y-start": [
    0,
    2,
    "Physical"
    /* Physical */
  ],
  "span-y-end": [
    1,
    3,
    "Physical"
    /* Physical */
  ],
  "y-self-start": [
    0,
    1,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "y-self-end": [
    2,
    3,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "span-y-self-start": [
    0,
    2,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "span-y-self-end": [
    1,
    3,
    "PhysicalSelf"
    /* PhysicalSelf */
  ],
  "block-start": [
    0,
    1,
    "Logical"
    /* Logical */
  ],
  "block-end": [
    2,
    3,
    "Logical"
    /* Logical */
  ],
  "span-block-start": [
    0,
    2,
    "Logical"
    /* Logical */
  ],
  "span-block-end": [
    1,
    3,
    "Logical"
    /* Logical */
  ],
  "inline-start": [
    0,
    1,
    "Logical"
    /* Logical */
  ],
  "inline-end": [
    2,
    3,
    "Logical"
    /* Logical */
  ],
  "span-inline-start": [
    0,
    2,
    "Logical"
    /* Logical */
  ],
  "span-inline-end": [
    1,
    3,
    "Logical"
    /* Logical */
  ],
  "self-block-start": [
    0,
    1,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "self-block-end": [
    2,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-block-start": [
    0,
    2,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-block-end": [
    1,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "self-inline-start": [
    0,
    1,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "self-inline-end": [
    2,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-inline-start": [
    0,
    2,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-inline-end": [
    1,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  start: [
    0,
    1,
    "Logical"
    /* Logical */
  ],
  end: [
    2,
    3,
    "Logical"
    /* Logical */
  ],
  "span-start": [
    0,
    2,
    "Logical"
    /* Logical */
  ],
  "span-end": [
    1,
    3,
    "Logical"
    /* Logical */
  ],
  "self-start": [
    0,
    1,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "self-end": [
    2,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-start": [
    0,
    2,
    "LogicalSelf"
    /* LogicalSelf */
  ],
  "span-self-end": [
    1,
    3,
    "LogicalSelf"
    /* LogicalSelf */
  ]
}, Ef = [
  "left",
  "center",
  "right",
  "span-left",
  "span-right",
  "x-start",
  "x-end",
  "span-x-start",
  "span-x-end",
  "x-self-start",
  "x-self-end",
  "span-x-self-start",
  "span-x-self-end",
  "span-all"
], $f = [
  "top",
  "center",
  "bottom",
  "span-top",
  "span-bottom",
  "y-start",
  "y-end",
  "span-y-start",
  "span-y-end",
  "y-self-start",
  "y-self-end",
  "span-y-self-start",
  "span-y-self-end",
  "span-all"
], Lf = [
  "block-start",
  "center",
  "block-end",
  "span-block-start",
  "span-block-end",
  "span-all"
], Pf = [
  "inline-start",
  "center",
  "inline-end",
  "span-inline-start",
  "span-inline-end",
  "span-all"
], Of = [
  "self-block-start",
  "center",
  "self-block-end",
  "span-self-block-start",
  "span-self-block-end",
  "span-all"
], Rf = [
  "self-inline-start",
  "center",
  "self-inline-end",
  "span-self-inline-start",
  "span-self-inline-end",
  "span-all"
], kr = [
  "start",
  "center",
  "end",
  "span-start",
  "span-end",
  "span-all"
], Sr = [
  "self-start",
  "center",
  "self-end",
  "span-self-start",
  "span-self-end",
  "span-all"
], If = ["block", "top", "bottom", "y"], _f = ["inline", "left", "right", "x"];
function hn(t) {
  const e = t.split("-");
  for (const n of e) {
    if (If.includes(n)) return "block";
    if (_f.includes(n)) return "inline";
  }
  return "ambiguous";
}
function Nf(t, e) {
  return e[0].includes(t[0]) && e[1].includes(t[1]) || e[0].includes(t[1]) && e[1].includes(t[0]);
}
const Df = [
  [Ef, $f],
  [Lf, Pf],
  [Of, Rf],
  [kr, kr],
  [Sr, Sr]
];
function Ff(t) {
  for (const e of Df)
    if (Nf(t, e)) return !0;
  return !1;
}
const yr = (t) => {
  const e = getComputedStyle(t);
  return {
    writingMode: e.writingMode,
    direction: e.direction
  };
}, Mf = (t, e) => R(null, null, function* () {
  const n = yield Tn(t);
  switch (e) {
    case "Logical":
    case "Physical":
      return yr(n);
    case "LogicalSelf":
    case "PhysicalSelf":
      return yr(t);
    default:
      return null;
  }
}), Ye = (t) => t.reverse().map((e) => 3 - e), lo = (t, e) => t === "Irrelevant" ? e : t, jf = (r, s) => R(null, [r, s], function* ({
  block: t,
  inline: e
}, n) {
  const o = lo(t[2], e[2]), a = yield Mf(n, o), l = {
    block: [t[0], t[1]],
    inline: [e[0], e[1]]
  };
  if (a) {
    if (a.direction === "rtl" && (l.inline = Ye(l.inline)), a.writingMode.startsWith("vertical")) {
      const u = l.block;
      l.block = l.inline, l.inline = u;
    }
    if (a.writingMode.startsWith("sideways")) {
      const u = l.block;
      l.block = l.inline, l.inline = u, a.writingMode.endsWith("lr") && (l.block = Ye(l.block));
    }
    a.writingMode.endsWith("rl") && (l.inline = Ye(l.inline));
  }
  return l;
}), Bf = ({
  block: t,
  inline: e
}) => {
  const n = [0, "top", "bottom", 0], r = [0, "left", "right", 0];
  return {
    block: [n[t[0]], n[t[1]]],
    inline: [r[e[0]], r[e[1]]]
  };
};
function br([t, e]) {
  return t === 0 && e === 3 ? "center" : t === 0 ? "end" : e === 3 ? "start" : "center";
}
function Uf(t) {
  return t.type === "Declaration" && t.property === "position-area";
}
function Wf(t) {
  const e = t.value.children.toArray().map(({ name: n }) => n);
  return e.length === 1 && (hn(e[0]) === "ambiguous" ? e.push(e[0]) : e.push("span-all")), e;
}
function zf(t) {
  if (!Uf(t)) return;
  const e = Wf(t);
  if (!Ff(e)) return;
  const n = {};
  switch (hn(e[0])) {
    case "block":
      n.block = e[0], n.inline = e[1];
      break;
    case "inline":
      n.inline = e[0], n.block = e[1];
      break;
    case "ambiguous":
      hn(e[1]) == "block" ? (n.block = e[1], n.inline = e[0]) : (n.inline = e[1], n.block = e[0]);
      break;
  }
  const r = {
    block: mr[n.block],
    inline: mr[n.inline]
  }, s = `--pa-declaration-${ro(12)}`;
  return {
    values: n,
    grid: r,
    selectorUUID: s
  };
}
function Hf(t, e) {
  [
    // Insets are applied to a wrapping element
    "justify-self",
    "align-self"
  ].forEach((n) => {
    e.children.appendData({
      type: "Declaration",
      property: n,
      value: { type: "Raw", value: `var(--pa-value-${n})` },
      important: !1
    });
  }), e.children.appendData({
    type: "Declaration",
    property: so,
    value: { type: "Raw", value: t.selectorUUID },
    important: !1
  });
}
function Vf(t, e) {
  var r, s;
  let n;
  if (((r = t.parentElement) == null ? void 0 : r.tagName) === gr)
    n = t.parentElement;
  else {
    n = document.createElement(gr), n.style.display = "grid", n.style.position = "absolute";
    const o = getComputedStyle(t).pointerEvents;
    n.style.pointerEvents = "none", t.style.pointerEvents = o, ["top", "left", "right", "bottom"].forEach((a) => {
      n.style.setProperty(a, `var(--pa-value-${a})`);
    }), (s = t.parentElement) == null || s.insertBefore(n, t), n.appendChild(t);
  }
  return n.setAttribute(
    `${oo}${e}`,
    ""
  ), n;
}
function Gf(t, e, n) {
  return R(this, null, function* () {
    const r = `--pa-target-${ro(12)}`, s = yield jf(
      e.grid,
      t
    ), o = Bf(s), a = lo(
      e.grid.block[2],
      e.grid.inline[2]
    ), l = [
      "LogicalSelf",
      "PhysicalSelf"
      /* PhysicalSelf */
    ].includes(a) ? s : e.grid, u = {
      block: br([l.block[0], l.block[1]]),
      inline: br([
        l.inline[0],
        l.inline[1]
      ])
    };
    return {
      insets: o,
      alignments: u,
      targetUUID: r,
      targetEl: t,
      anchorEl: n,
      wrapperEl: Vf(t, r),
      values: e.values,
      grid: e.grid,
      selectorUUID: e.selectorUUID
    };
  });
}
function qf(t, e) {
  return `
    [${io}="${e}"][${oo}${t}] {
      --pa-value-top: var(${t}-top);
      --pa-value-left: var(${t}-left);
      --pa-value-right: var(${t}-right);
      --pa-value-bottom: var(${t}-bottom);
      --pa-value-justify-self: var(${t}-justify-self);
      --pa-value-align-self: var(${t}-align-self);
    }
  `.replaceAll(`
`, "");
}
const Kf = [
  "normal",
  "most-width",
  "most-height",
  "most-block-size",
  "most-inline-size"
], Qf = [
  "flip-block",
  "flip-inline",
  "flip-start"
];
function Yf(t) {
  return t.type === "Declaration";
}
function Xf(t) {
  return t.type === "Declaration" && t.property === "position-try-fallbacks";
}
function Jf(t) {
  return t.type === "Declaration" && t.property === "position-try-order";
}
function Zf(t) {
  return t.type === "Declaration" && t.property === "position-try";
}
function tp(t) {
  return t.type === "Atrule" && t.name === "position-try";
}
function ep(t) {
  return Qf.includes(t);
}
function np(t) {
  return Kf.includes(t);
}
function rp(t, e) {
  const n = document.querySelector(t);
  if (n) {
    let r = ip(n);
    return e.forEach((s) => {
      r = co(r, s);
    }), r;
  }
}
function sp(t, e) {
  let n = t.declarations;
  return e.forEach((r) => {
    n = co(n, r);
  }), n;
}
function ip(t) {
  const e = {};
  return Ki.forEach((n) => {
    const r = it(t, `--${n}-${eo}`);
    r && (e[n] = r);
  }), e;
}
const op = {
  "flip-block": {
    top: "bottom",
    bottom: "top",
    "inset-block-start": "inset-block-end",
    "inset-block-end": "inset-block-start",
    "margin-top": "margin-bottom",
    "margin-bottom": "margin-top"
  },
  "flip-inline": {
    left: "right",
    right: "left",
    "inset-inline-start": "inset-inline-end",
    "inset-inline-end": "inset-inline-start",
    "margin-left": "margin-right",
    "margin-right": "margin-left"
  },
  "flip-start": {
    left: "top",
    right: "bottom",
    top: "left",
    bottom: "right",
    "inset-block-start": "inset-block-end",
    "inset-block-end": "inset-block-start",
    "inset-inline-start": "inset-inline-end",
    "inset-inline-end": "inset-inline-start",
    "inset-block": "inset-inline",
    "inset-inline": "inset-block"
  }
}, ap = {
  "flip-block": {
    top: "bottom",
    bottom: "top",
    start: "end",
    end: "start",
    "self-end": "self-start",
    "self-start": "self-end"
  },
  "flip-inline": {
    left: "right",
    right: "left",
    start: "end",
    end: "start",
    "self-end": "self-start",
    "self-start": "self-end"
  },
  "flip-start": {
    top: "left",
    left: "top",
    right: "bottom",
    bottom: "right"
  }
}, lp = {
  "flip-block": {
    top: "bottom",
    bottom: "top",
    start: "end",
    end: "start"
  },
  "flip-inline": {
    left: "right",
    right: "left",
    start: "end",
    end: "start"
  },
  "flip-start": {
    // TODO: Requires fuller logic
  }
};
function cp(t, e) {
  return op[e][t] || t;
}
function up(t, e) {
  return ap[e][t] || t;
}
function hp(t, e) {
  if (e === "flip-start")
    return t;
  {
    const n = lp[e];
    return t.split("-").map((r) => n[r] || r).join("-");
  }
}
function fp(t, e, n) {
  if (t === "margin") {
    const [r, s, o, a] = e.children.toArray();
    n === "flip-block" ? a ? e.children.fromArray([o, s, r, a]) : o && e.children.fromArray([o, s, r]) : n === "flip-inline" && a && e.children.fromArray([r, a, o, s]);
  } else if (t === "margin-block") {
    const [r, s] = e.children.toArray();
    n === "flip-block" && s && e.children.fromArray([s, r]);
  } else if (t === "margin-inline") {
    const [r, s] = e.children.toArray();
    n === "flip-inline" && s && e.children.fromArray([s, r]);
  }
}
const pp = (t, e) => {
  var s;
  return ((s = $t(`#id{${t}: ${e};}`).children.first) == null ? void 0 : s.block.children.first).value;
};
function co(t, e) {
  const n = {};
  return Object.entries(t).forEach(([r, s]) => {
    var u;
    const o = r, a = pp(o, s), l = cp(o, e);
    l !== o && ((u = n[o]) != null || (n[o] = "revert")), Et(a, {
      visit: "Function",
      enter(i) {
        Le(i) && i.children.forEach((c) => {
          se(c) && Yi(c.name) && (c.name = up(c.name, e));
        });
      }
    }), o === "position-area" && a.children.forEach((i) => {
      se(i) && ao(i.name) && (i.name = hp(i.name, e));
    }), o.startsWith("margin") && fp(o, a, e), n[l] = Z(a);
  }), n;
}
function uo(t) {
  const e = cf(t), n = [];
  return e.forEach((r) => {
    const s = {
      atRules: [],
      tactics: [],
      positionAreas: []
    };
    r.forEach((o) => {
      ep(o.name) ? s.tactics.push(o.name) : o.name.startsWith("--") ? s.atRules.push(o.name) : ao(o.name) && s.positionAreas.push(o.name);
    }), s.positionAreas.length ? n.push({
      positionArea: s.positionAreas[0],
      type: "position-area"
    }) : s.atRules.length && s.tactics.length ? n.push({
      tactics: s.tactics,
      atRule: s.atRules[0],
      type: "at-rule-with-try-tactic"
    }) : s.atRules.length ? n.push({
      atRule: s.atRules[0],
      type: "at-rule"
    }) : s.tactics.length && n.push({
      tactics: s.tactics,
      type: "try-tactic"
    });
  }), n;
}
function dp(t) {
  return Xf(t) && t.value.children.first ? uo(t.value.children) : [];
}
function gp(t) {
  if (Zf(t) && t.value.children.first) {
    const e = $e(t);
    let n;
    const r = e.value.children.first.name;
    r && np(r) && (n = r, e.value.children.shift());
    const s = uo(e.value.children);
    return { order: n, options: s };
  }
  return {};
}
function mp(t) {
  return Jf(t) && t.value.children.first ? {
    order: t.value.children.first.name
  } : {};
}
function kp(t) {
  const { order: e, options: n } = gp(t);
  if (e || n)
    return { order: e, options: n };
  const { order: r } = mp(t), s = dp(t);
  return r || s ? { order: r, options: s } : {};
}
function Sp(t) {
  return le(t.property) || Qu(t.property) || Gi(t.property) || Yu(t.property) || ["position-anchor", "position-area"].includes(t.property);
}
function yp(t) {
  var e, n;
  if (tp(t) && ((e = t.prelude) != null && e.value) && ((n = t.block) != null && n.children)) {
    const r = t.prelude.value, s = t.block.children.filter(
      (a) => Yf(a) && Sp(a)
    ), o = {
      uuid: `${r}-try-${at(12)}`,
      declarations: Object.fromEntries(
        s.map((a) => [a.property, Z(a.value)])
      )
    };
    return { name: r, tryBlock: o };
  }
  return {};
}
function bp(t) {
  const e = {}, n = {}, r = {};
  for (const s of t) {
    const o = $t(s.css);
    Et(o, {
      visit: "Atrule",
      enter(a) {
        const { name: l, tryBlock: u } = yp(a);
        l && u && (e[l] = u);
      }
    });
  }
  for (const s of t) {
    let o = !1;
    const a = /* @__PURE__ */ new Set(), l = $t(s.css);
    Et(l, {
      visit: "Declaration",
      enter(u) {
        var g;
        const i = (g = this.rule) == null ? void 0 : g.prelude, c = cn(i);
        if (!c.length) return;
        const { order: h, options: f } = kp(u), p = {};
        h && (p.order = h), c.forEach(({ selector: m }) => {
          var S, b;
          f == null || f.forEach((C) => {
            var M, k, O;
            let v;
            if (C.type === "at-rule")
              v = C.atRule;
            else if (C.type === "try-tactic") {
              v = `${m}-${C.tactics.join("-")}`;
              const w = rp(
                m,
                C.tactics
              );
              w && (e[v] = {
                uuid: `${m}-${C.tactics.join("-")}-try-${at(12)}`,
                declarations: w
              });
            } else if (C.type === "at-rule-with-try-tactic") {
              v = `${m}-${C.atRule}-${C.tactics.join("-")}`;
              const w = e[C.atRule], E = sp(
                w,
                C.tactics
              );
              E && (e[v] = {
                uuid: `${m}-${C.atRule}-${C.tactics.join("-")}-try-${at(12)}`,
                declarations: E
              });
            }
            if (v && e[v]) {
              const w = `[data-anchor-polyfill="${e[v].uuid}"]`;
              (M = n[w]) != null || (n[w] = []), n[w].push(m), a.has(v) || ((k = p.fallbacks) != null || (p.fallbacks = []), p.fallbacks.push(e[v]), a.add(v), (O = this.stylesheet) == null || O.children.prependData({
                type: "Rule",
                prelude: {
                  type: "Raw",
                  value: w
                },
                block: {
                  type: "Block",
                  children: new K().fromArray(
                    Object.entries(e[v].declarations).map(
                      ([E, _]) => ({
                        type: "Declaration",
                        important: !0,
                        property: E,
                        value: {
                          type: "Raw",
                          value: _
                        }
                      })
                    )
                  )
                }
              }), o = !0);
            }
          }), Object.keys(p).length > 0 && (r[m] ? (p.order && (r[m].order = p.order), p.fallbacks && ((b = (S = r[m]).fallbacks) != null || (S.fallbacks = []), r[m].fallbacks.push(
            ...p.fallbacks
          ))) : r[m] = p);
        });
      }
    }), o && (s.css = Z(l), s.changed = !0);
  }
  return { fallbackTargets: n, validPositions: r };
}
function xp(t, e) {
  return !t || t === e ? !1 : ho(t) ? t.document.contains(e) : t.contains(e);
}
function ho(t) {
  return !!(t && t === t.window);
}
function Cp(t) {
  return Jt(t, "position", "fixed");
}
function fn(t) {
  return !!(t && (Cp(t) || Jt(t, "position", "absolute")));
}
function xr(t, e) {
  return t.compareDocumentPosition(e) & Node.DOCUMENT_POSITION_FOLLOWING;
}
function wp(t) {
  return R(this, null, function* () {
    return yield H.getOffsetParent(t);
  });
}
function Xe(t) {
  return R(this, null, function* () {
    if (!["absolute", "fixed"].includes(it(t, "position")))
      return yield wp(t);
    let e = t.parentElement;
    for (; e; ) {
      if (!Jt(e, "position", "static") && Jt(e, "display", "block"))
        return e;
      e = e.parentElement;
    }
    return window;
  });
}
function Tp(t, e, n, r) {
  return R(this, null, function* () {
    const s = yield Xe(t), o = yield Xe(n);
    if (!(xp(o, t) || ho(o)) || s === o && !(!fn(t) || xr(t, n)))
      return !1;
    if (s !== o) {
      let a;
      const l = [];
      for (a = s; a && a !== o && a !== window; )
        l.push(a), a = yield Xe(a);
      const u = l[l.length - 1];
      if (u instanceof HTMLElement && !(!fn(u) || xr(u, n)))
        return !1;
    }
    {
      let a = t.parentElement;
      for (; a; ) {
        if (Jt(a, "content-visibility", "hidden"))
          return !1;
        a = a.parentElement;
      }
    }
    return !(e && r && Cr(t, e, r) !== Cr(n, e, r));
  });
}
function Cr(t, e, n) {
  for (; !(t.matches(n) && yf(t, e)); ) {
    if (!t.parentElement)
      return null;
    t = t.parentElement;
  }
  return t;
}
function Ap(t, e, n, r) {
  return R(this, null, function* () {
    if (!(t instanceof HTMLElement && n.length && fn(t)))
      return null;
    const s = n.flatMap(kf).filter((a) => Sf(a, e)), o = r.map((a) => a.selector).join(",") || null;
    for (let a = s.length - 1; a >= 0; a--) {
      const l = s[a], u = "fakePseudoElement" in l;
      if (yield Tp(
        u ? l.fakePseudoElement : l,
        e,
        t,
        o
      ))
        return u && l.removeFakePseudoElement(), l;
    }
    return null;
  });
}
function vp(t) {
  return t.type === "Declaration" && t.property === "anchor-name";
}
function Ep(t) {
  return t.type === "Declaration" && t.property === "anchor-scope";
}
function pn(t) {
  return !!(t && t.type === "Function" && t.name === "anchor-size");
}
function xe(t) {
  return !!(t && t.type === "Function" && t.name === "var");
}
function se(t) {
  return !!(t.type === "Identifier" && t.name);
}
function $p(t) {
  return !!(t.type === "Percentage" && t.value);
}
function wr(t, e) {
  let n, r, s, o = "", a = !1, l;
  const u = [];
  t.children.toArray().forEach((f) => {
    if (a) {
      o = `${o}${Z(f)}`;
      return;
    }
    if (f.type === "Operator" && f.value === ",") {
      a = !0;
      return;
    }
    u.push(f);
  });
  let [i, c] = u;
  if (c || (c = i, i = void 0), i && (se(i) && i.name.startsWith("--") ? n = i.name : xe(i) && i.children.first && (l = i.children.first.name)), c)
    if (Le(t)) {
      if (se(c) && Yi(c.name))
        r = c.name;
      else if ($p(c)) {
        const f = Number(c.value);
        r = Number.isNaN(f) ? void 0 : f;
      }
    } else pn(t) && se(c) && th(c.name) && (s = c.name);
  const h = `--anchor-${at(12)}`;
  return Object.assign(t, {
    type: "Raw",
    value: `var(${h})`,
    children: null
  }), Reflect.deleteProperty(t, "name"), {
    anchorName: n,
    anchorSide: r,
    anchorSize: s,
    fallbackValue: o || "0px",
    customPropName: l,
    uuid: h
  };
}
function Tr(t) {
  return t.value.children.map(({ name: e }) => e);
}
let Ht = {}, vt = {}, _t = {}, ie = {}, It = {};
function Lp() {
  Ht = {}, vt = {}, _t = {}, ie = {}, It = {};
}
function Pp(t, e) {
  var n;
  if ((Le(t) || pn(t)) && e) {
    if (e.property.startsWith("--")) {
      const r = Z(e.value), s = wr(t);
      return ie[s.uuid] = r, _t[e.property] = [
        ...(n = _t[e.property]) != null ? n : [],
        s
      ], { changed: !0 };
    }
    if (Le(t) && le(e.property) || pn(t) && Qi(e.property)) {
      const r = wr(t);
      return { prop: e.property, data: r, changed: !0 };
    }
  }
  return {};
}
function Ar(t, e) {
  return R(this, null, function* () {
    let n = e == null ? void 0 : e.anchorName;
    const r = e == null ? void 0 : e.customPropName;
    if (t && !n) {
      const l = it(
        t,
        "position-anchor"
      );
      l ? n = l : r && (n = it(t, r));
    }
    const s = n ? Ht[n] || [] : [], o = n ? vt[no.All] || [] : [], a = n ? vt[n] || [] : [];
    return yield Ap(
      t,
      n || null,
      s,
      [...o, ...a]
    );
  });
}
function Op(t) {
  return R(this, null, function* () {
    var c, h, f, p, g, m, S, b, C, v, M;
    const e = {}, n = {};
    Lp();
    const { fallbackTargets: r, validPositions: s } = bp(t);
    for (const k of t) {
      let O = !1;
      const w = $t(k.css);
      Et(w, function(E) {
        var J, bt, Pt, xt, ft, Ct;
        const _ = (J = this.rule) == null ? void 0 : J.prelude, L = cn(_);
        if (vp(E) && L.length)
          for (const B of Tr(E))
            (bt = Ht[B]) != null || (Ht[B] = []), Ht[B].push(...L);
        if (Ep(E) && L.length)
          for (const B of Tr(E))
            (Pt = vt[B]) != null || (vt[B] = []), vt[B].push(...L);
        const {
          prop: A,
          data: j,
          changed: D
        } = Pp(E, this.declaration);
        if (A && j && L.length)
          for (const { selector: B } of L)
            e[B] = q(W({}, e[B]), {
              [A]: [...(ft = (xt = e[B]) == null ? void 0 : xt[A]) != null ? ft : [], j]
            });
        let V;
        if (this.block && (V = zf(E), V)) {
          Hf(
            V,
            this.block
          );
          for (const { selector: B } of L)
            n[B] = [
              ...(Ct = n[B]) != null ? Ct : [],
              V
            ];
        }
        (D || V) && (O = !0);
      }), O && (k.css = Z(w), k.changed = !0);
    }
    const o = new Set(Object.keys(_t)), a = {}, l = (k) => {
      var E, _, L, A, j;
      const O = [], w = new Set((_ = (E = a[k]) == null ? void 0 : E.names) != null ? _ : []);
      for (; w.size > 0; )
        for (const D of w)
          O.push(...(L = _t[D]) != null ? L : []), w.delete(D), (j = (A = a[D]) == null ? void 0 : A.names) != null && j.length && a[D].names.forEach((V) => w.add(V));
      return O;
    };
    for (; o.size > 0; ) {
      const k = [];
      for (const O of t) {
        let w = !1;
        const E = $t(O.css);
        Et(E, {
          visit: "Function",
          enter(_) {
            var D, V;
            const L = (D = this.rule) == null ? void 0 : D.prelude, A = this.declaration, j = A == null ? void 0 : A.property;
            if ((L == null ? void 0 : L.children.isEmpty) === !1 && xe(_) && A && j && _.children.first && o.has(_.children.first.name) && // For now, we only want assignments to other CSS custom properties
            j.startsWith("--")) {
              const J = _.children.first, bt = (V = _t[J.name]) != null ? V : [], Pt = l(J.name);
              if (!(bt.length || Pt.length))
                return;
              const xt = `${J.name}-anchor-${at(12)}`, ft = Z(A.value);
              ie[xt] = ft, a[j] || (a[j] = { names: [], uuids: [] });
              const Ct = a[j];
              Ct.names.includes(J.name) || Ct.names.push(J.name), Ct.uuids.push(xt), k.push(j), J.name = xt, w = !0;
            }
          }
        }), w && (O.css = Z(E), O.changed = !0);
      }
      o.clear(), k.forEach((O) => o.add(O));
    }
    for (const k of t) {
      let O = !1;
      const w = $t(k.css);
      Et(w, {
        visit: "Function",
        enter(E) {
          var j, D, V, J, bt, Pt, xt;
          const _ = (j = this.rule) == null ? void 0 : j.prelude, L = this.declaration, A = L == null ? void 0 : L.property;
          if ((_ == null ? void 0 : _.children.isEmpty) === !1 && xe(E) && L && A && E.children.first && // Now we only want assignments to inset/sizing properties
          (le(A) || Gi(A))) {
            const ft = E.children.first, Ct = (D = _t[ft.name]) != null ? D : [], B = l(ft.name);
            if (!(Ct.length || B.length))
              return;
            const ue = `${A}-${at(12)}`;
            if (B.length) {
              const Mt = /* @__PURE__ */ new Set([ft.name]);
              for (; Mt.size > 0; )
                for (const jt of Mt) {
                  const G = a[jt];
                  if ((V = G == null ? void 0 : G.names) != null && V.length && ((J = G == null ? void 0 : G.uuids) != null && J.length))
                    for (const Bt of G.names)
                      for (const Ut of G.uuids)
                        It[Ut] = q(W({}, It[Ut]), {
                          // - `key` (`propUuid`) is the property-specific
                          //   uuid to append to the new custom property name
                          // - `value` is the new property-specific custom
                          //   property value to use
                          [ue]: `${Bt}-${ue}`
                        });
                  Mt.delete(jt), (bt = G == null ? void 0 : G.names) != null && bt.length && G.names.forEach((Bt) => Mt.add(Bt));
                }
            }
            const fo = cn(_);
            for (const Mt of [...Ct, ...B]) {
              const jt = W({}, Mt), G = `--anchor-${at(12)}-${A}`, Bt = jt.uuid;
              jt.uuid = G;
              for (const { selector: Ut } of fo)
                e[Ut] = q(W({}, e[Ut]), {
                  [A]: [...(xt = (Pt = e[Ut]) == null ? void 0 : Pt[A]) != null ? xt : [], jt]
                });
              It[Bt] = q(W({}, It[Bt]), {
                // - `key` (`propUuid`) is the property-specific
                //   uuid to append to the new custom property name
                // - `value` is the new property-specific custom
                //   property value to use
                [ue]: G
              });
            }
            ft.name = `${ft.name}-${ue}`, O = !0;
          }
        }
      }), O && (k.css = Z(w), k.changed = !0);
    }
    if (Object.keys(It).length > 0)
      for (const k of t) {
        let O = !1;
        const w = $t(k.css);
        Et(w, {
          visit: "Function",
          enter(E) {
            var _, L, A, j;
            if (xe(E) && ((L = (_ = E.children.first) == null ? void 0 : _.name) != null && L.startsWith("--")) && ((j = (A = this.declaration) == null ? void 0 : A.property) != null && j.startsWith("--")) && this.block) {
              const D = E.children.first, V = It[D.name];
              if (V)
                for (const [J, bt] of Object.entries(V))
                  this.block.children.appendData({
                    type: "Declaration",
                    important: !1,
                    property: `${this.declaration.property}-${J}`,
                    value: {
                      type: "Raw",
                      value: Z(this.declaration.value).replace(
                        `var(${D.name})`,
                        `var(${bt})`
                      )
                    }
                  }), O = !0;
              ie[D.name] && (this.declaration.value = {
                type: "Raw",
                value: ie[D.name]
              }, O = !0);
            }
          }
        }), O && (k.css = Z(w), k.changed = !0);
      }
    const u = /* @__PURE__ */ new Map();
    for (const [k, O] of Object.entries(e)) {
      let w;
      k.startsWith("[data-anchor-polyfill=") && ((c = r[k]) != null && c.length) ? w = document.querySelectorAll(r[k].join(",")) : w = document.querySelectorAll(k);
      for (const [E, _] of Object.entries(O))
        for (const L of _)
          for (const A of w) {
            const j = yield Ar(A, L), D = `--anchor-${at(12)}`;
            u.set(A, q(W({}, (h = u.get(A)) != null ? h : {}), {
              [L.uuid]: D
            })), A.setAttribute(
              "style",
              `${L.uuid}: var(${D}); ${(f = A.getAttribute("style")) != null ? f : ""}`
            ), s[k] = q(W({}, s[k]), {
              declarations: q(W({}, (p = s[k]) == null ? void 0 : p.declarations), {
                [E]: [
                  ...(S = (m = (g = s[k]) == null ? void 0 : g.declarations) == null ? void 0 : m[E]) != null ? S : [],
                  q(W({}, L), { anchorEl: j, targetEl: A, uuid: D })
                ]
              })
            });
          }
    }
    const i = {
      el: document.createElement("link"),
      changed: !1,
      created: !0,
      css: ""
    };
    t.push(i);
    for (const [k, O] of Object.entries(n)) {
      const w = document.querySelectorAll(k);
      for (const E of w) {
        const _ = yield Ar(E);
        for (const L of O) {
          const A = yield Gf(
            E,
            L,
            _
          );
          i.css += qf(
            A.targetUUID,
            L.selectorUUID
          ), i.changed = !0, s[k] = q(W({}, s[k]), {
            declarations: q(W({}, (b = s[k]) == null ? void 0 : b.declarations), {
              "position-area": [
                ...(M = (v = (C = s[k]) == null ? void 0 : C.declarations) == null ? void 0 : v["position-area"]) != null ? M : [],
                A
              ]
            })
          });
        }
      }
    }
    return { rules: s, inlineStyles: u, anchorScopes: vt };
  });
}
const Rp = [
  "as",
  "blocking",
  "crossorigin",
  // 'disabled' is not relevant for style elements, but this exclusion is
  // theoretical, as a <link rel=stylesheet disabled> will not be loaded, and
  // will not reach this part of the polyfill. See #246.
  "disabled",
  "fetchpriority",
  "href",
  "hreflang",
  "integrity",
  "referrerpolicy",
  "rel",
  "type"
];
function vr(t, e, n = !1) {
  const r = [];
  for (const { el: s, css: o, changed: a, created: l = !1 } of t) {
    const u = { el: s, css: o, changed: !1 };
    if (a) {
      if (s.tagName.toLowerCase() === "style")
        s.innerHTML = o;
      else if (s instanceof HTMLLinkElement) {
        const i = document.createElement("style");
        i.textContent = o;
        for (const c of s.getAttributeNames())
          if (!c.startsWith("on") && !Rp.includes(c)) {
            const h = s.getAttribute(c);
            h !== null && i.setAttribute(c, h);
          }
        s.hasAttribute("href") && i.setAttribute("data-original-href", s.getAttribute("href"));
        
        i.setAttribute("data-generated-by-polyfill", "true");
        document.head.insertAdjacentElement("beforeend", i);
        
        u.el = i;
      } else if (s.hasAttribute("data-has-inline-styles")) {
        const i = s.getAttribute("data-has-inline-styles");
        if (i) {
          const c = `[data-has-inline-styles="${i}"]{`;
          let f = o.slice(c.length, 0 - "}".length);
          const p = e == null ? void 0 : e.get(s);
          if (p)
            for (const [g, m] of Object.entries(p))
              f = `${g}: var(${m}); ${f}`;
          s.setAttribute("style", f);
        }
      }
    }
    n && s.hasAttribute("data-has-inline-styles") && s.removeAttribute("data-has-inline-styles"), r.push(u);
  }
  return r;
}
const Ip = q(W({}, H), { _c: /* @__PURE__ */ new Map() }), _p = (t, e) => {
  let n;
  switch (t) {
    case "start":
    case "self-start":
      n = 0;
      break;
    case "end":
    case "self-end":
      n = 100;
      break;
    default:
      typeof t == "number" && !Number.isNaN(t) && (n = t);
  }
  if (n !== void 0)
    return e ? 100 - n : n;
}, Np = (t, e) => {
  let n;
  switch (t) {
    case "block":
    case "self-block":
      n = e ? "width" : "height";
      break;
    case "inline":
    case "self-inline":
      n = e ? "height" : "width";
      break;
  }
  return n;
}, Er = (t) => {
  switch (t) {
    case "top":
    case "bottom":
      return "y";
    case "left":
    case "right":
      return "x";
  }
  return null;
}, Dp = (t) => {
  switch (t) {
    case "x":
      return "width";
    case "y":
      return "height";
  }
  return null;
}, $r = (t) => it(t, "display") === "inline", Lr = (t, e) => (e === "x" ? ["border-left-width", "border-right-width"] : ["border-top-width", "border-bottom-width"]).reduce(
  (r, s) => r + parseInt(it(t, s), 10),
  0
) || 0, me = (t, e) => parseInt(it(t, `margin-${e}`), 10) || 0, Fp = (t) => ({
  top: me(t, "top"),
  right: me(t, "right"),
  bottom: me(t, "bottom"),
  left: me(t, "left")
}), Je = (a) => R(null, [a], function* ({
  targetEl: t,
  targetProperty: e,
  anchorRect: n,
  anchorSide: r,
  anchorSize: s,
  fallback: o = null
}) {
  var l;
  if (!((s || r !== void 0) && t && n))
    return o;
  if (s) {
    if (!Qi(e))
      return o;
    let u;
    switch (s) {
      case "width":
      case "height":
        u = s;
        break;
      default: {
        let i = !1;
        const c = it(t, "writing-mode");
        i = c.startsWith("vertical-") || c.startsWith("sideways-"), u = Np(s, i);
      }
    }
    return u ? `${n[u]}px` : o;
  }
  if (r !== void 0) {
    let u, i;
    const c = Er(e);
    if (!(le(e) && c && (!le(r) || c === Er(r))))
      return o;
    const h = ["top", "left"];
    switch (r) {
      case "left":
      case "top":
        u = 0;
        break;
      case "right":
      case "bottom":
        u = 100;
        break;
      case "center":
        u = 50;
        break;
      case "inside":
        u = h.includes(e) ? 0 : 100;
        break;
      case "outside":
        u = h.includes(e) ? 100 : 0;
        break;
      default:
        if (t) {
          const g = (yield (l = H.isRTL) == null ? void 0 : l.call(H, t)) || !1;
          u = _p(r, g);
        }
    }
    const f = typeof u == "number" && !Number.isNaN(u), p = Dp(c);
    if (f && p) {
      (e === "bottom" || e === "right") && (i = yield Tn(t));
      let g = n[c] + n[p] * (u / 100);
      switch (e) {
        case "bottom": {
          if (!i)
            break;
          let m = i.clientHeight;
          if (m === 0 && $r(i)) {
            const S = Lr(i, c);
            m = i.offsetHeight - S;
          }
          g = m - g;
          break;
        }
        case "right": {
          if (!i)
            break;
          let m = i.clientWidth;
          if (m === 0 && $r(i)) {
            const S = Lr(i, c);
            m = i.offsetWidth - S;
          }
          g = m - g;
          break;
        }
      }
      return `${g}px`;
    }
  }
  return o;
}), Mp = (t) => "wrapperEl" in t, jp = (t) => "uuid" in t;
function Bp(t, e = !1) {
  return R(this, null, function* () {
    const n = document.documentElement;
    for (const [r, s] of Object.entries(t))
      for (const o of s) {
        const a = o.anchorEl, l = o.targetEl;
        if (a && l)
          if (Mp(o)) {
            const u = o.wrapperEl, i = (c, h, f) => R(null, null, function* () {
              return c === 0 ? "0px" : yield Je({
                targetEl: u,
                targetProperty: h,
                anchorRect: f,
                anchorSide: c
              });
            });
            const clean = en(
              a,
              u,
              () => R(null, null, function* () {
                const c = it(
                  l,
                  so
                );
                u.setAttribute(io, c);
                const h = yield H.getElementRects({
                  reference: a,
                  floating: u,
                  strategy: "absolute"
                }), f = o.insets, p = yield i(
                  f.block[0],
                  "top",
                  h.reference
                ), g = yield i(
                  f.block[1],
                  "bottom",
                  h.reference
                ), m = yield i(
                  f.inline[0],
                  "left",
                  h.reference
                ), S = yield i(
                  f.inline[1],
                  "right",
                  h.reference
                );
                n.style.setProperty(
                  `${o.targetUUID}-top`,
                  p || null
                ), n.style.setProperty(
                  `${o.targetUUID}-left`,
                  m || null
                ), n.style.setProperty(
                  `${o.targetUUID}-right`,
                  S || null
                ), n.style.setProperty(
                  `${o.targetUUID}-bottom`,
                  g || null
                ), n.style.setProperty(
                  `${o.targetUUID}-justify-self`,
                  o.alignments.inline
                ), n.style.setProperty(
                  `${o.targetUUID}-align-self`,
                  o.alignments.block
                );
              }),
              { animationFrame: e }
            );
            window.addEventListener(
              'css-anchor-positioning-clean',
              () => {
                clean();
              },
              { once: true }
            );
          } else {
            const clean = en(
              a,
              l,
              () => R(null, null, function* () {
                const u = yield H.getElementRects({
                  reference: a,
                  floating: l,
                  strategy: "absolute"
                }), i = yield Je({
                  targetEl: l,
                  targetProperty: r,
                  anchorRect: u.reference,
                  anchorSide: o.anchorSide,
                  anchorSize: o.anchorSize,
                  fallback: o.fallbackValue
                });
                n.style.setProperty(o.uuid, i);
              }),
              { animationFrame: e }
            );
            window.addEventListener(
              'css-anchor-positioning-clean',
              () => {
                clean();
              },
              { once: true }
            );
          }
        else if (jp(o)) {
          const u = yield Je({
            targetProperty: r,
            anchorSide: o.anchorSide,
            anchorSize: o.anchorSize,
            fallback: o.fallbackValue
          });
          n.style.setProperty(o.uuid, u);
        }
      }
  });
}
function Pr(t, e) {
  return R(this, null, function* () {
    const n = yield H.getElementRects({
      reference: t,
      floating: t,
      strategy: "absolute"
    });
    return yield jo(
      {
        x: t.offsetLeft,
        y: t.offsetTop,
        platform: Ip,
        rects: n,
        elements: {
          floating: t,
          reference: e
        },
        strategy: "absolute"
      },
      {
        padding: Fp(t)
      }
    );
  });
}
function Up(t, e, n = !1) {
  return R(this, null, function* () {
    if (!e.length)
      return;
    const r = document.querySelectorAll(t);
    for (const s of r) {
      let o = !1;
      const a = yield Tn(s);
      const clean = en(
        // We're just checking whether the target element overflows, so we don't
        // care about the position of the anchor element in this case. Passing in
        // an empty object instead of a reference element avoids unnecessarily
        // watching for irrelevant changes.
        {},
        s,
        () => R(null, null, function* () {
          if (o)
            return;
          o = !0, s.removeAttribute("data-anchor-polyfill");
          const l = yield Pr(s, a);
          if (Object.values(l).every((u) => u <= 0)) {
            s.removeAttribute("data-anchor-polyfill-last-successful"), o = !1;
            return;
          }
          for (const [u, { uuid: i }] of e.entries()) {
            s.setAttribute("data-anchor-polyfill", i);
            const c = yield Pr(s, a);
            if (Object.values(c).every((h) => h <= 0)) {
              s.setAttribute("data-anchor-polyfill-last-successful", i), o = !1;
              break;
            }
            if (u === e.length - 1) {
              const h = s.getAttribute(
                "data-anchor-polyfill-last-successful"
              );
              h ? s.setAttribute("data-anchor-polyfill", h) : s.removeAttribute("data-anchor-polyfill"), o = !1;
              break;
            }
          }
        }),
        { animationFrame: n, layoutShift: !1 }
      );
      window.addEventListener(
        'css-anchor-positioning-clean',
        () => {
          clean();
        },
        { once: true }
      );
    }
  });
}
function Wp(t, e = !1) {
  return R(this, null, function* () {
    var n, r;
    for (const s of Object.values(t))
      yield Bp((n = s.declarations) != null ? n : {}, e);
    for (const [s, o] of Object.entries(t))
      yield Up(
        s,
        (r = o.fallbacks) != null ? r : [],
        e
      );
  });
}
function zp(t = {}) {
  const e = typeof t == "boolean" ? { useAnimationFrame: t } : t, n = e.useAnimationFrame === void 0 ? !!window.UPDATE_ANCHOR_ON_ANIMATION_FRAME : e.useAnimationFrame;
  return Array.isArray(e.elements) || (e.elements = void 0), Object.assign(e, { useAnimationFrame: n });
}
function Vp(t) {
  return R(this, null, function* () {
    const e = zp(
      t != null ? t : window.ANCHOR_POSITIONING_POLYFILL_OPTIONS
    );
    let n = yield Tf(e.elements, e.excludeInlineStyles), r = {}, s;
    hf();
    try {
      pf(n) && (n = vr(n));
      const a = yield Op(n);
      r = a.rules, s = a.inlineStyles;
    } catch (o) {
      throw uf(), o;
    }
    return Object.values(r).length && (vr(n, s, !0), yield Wp(r, e.useAnimationFrame)), r;
  });
}
export {
  Vp as default
};