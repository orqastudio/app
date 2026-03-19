var ys = Object.defineProperty;
var Gn = (e) => {
  throw TypeError(e);
};
var ws = (e, t, r) => t in e ? ys(e, t, { enumerable: !0, configurable: !0, writable: !0, value: r }) : e[t] = r;
var xt = (e, t, r) => ws(e, typeof t != "symbol" ? t + "" : t, r), sn = (e, t, r) => t.has(e) || Gn("Cannot " + r);
var _ = (e, t, r) => (sn(e, t, "read from private field"), r ? r.call(e) : t.get(e)), ie = (e, t, r) => t.has(e) ? Gn("Cannot add the same private member more than once") : t instanceof WeakSet ? t.add(e) : t.set(e, r), Se = (e, t, r, n) => (sn(e, t, "write to private field"), n ? n.call(e, r) : t.set(e, r), r), We = (e, t, r) => (sn(e, t, "access private method"), r);
import { untrack as ks, unmount as Cs } from "svelte";
import { SmallBadge as lo, SelectMenu as co, Badge as uo, ScrollArea as fo, Icon as jr, EmptyState as fn, LoadingSpinner as Es, ErrorDisplay as Ss } from "@orqastudio/svelte-components/pure";
import { getStores as Ts } from "@orqastudio/sdk";
import { StatusIndicator as vo } from "@orqastudio/svelte-components/connected";
const As = "5";
var so;
typeof window < "u" && ((so = window.__svelte ?? (window.__svelte = {})).v ?? (so.v = /* @__PURE__ */ new Set())).add(As);
const Ms = 1, zs = 2, po = 4, Ps = 8, Rs = 16, Ds = 1, Ns = 2, xe = Symbol(), mo = "http://www.w3.org/1999/xhtml", Ls = !1;
var go = Array.isArray, Is = Array.prototype.indexOf, ar = Array.prototype.includes, Sn = Array.from, Os = Object.defineProperty, an = Object.getOwnPropertyDescriptor, Vs = Object.getOwnPropertyDescriptors, Fs = Object.prototype, Gs = Array.prototype, ho = Object.getPrototypeOf;
const js = () => {
};
function Bs(e) {
  for (var t = 0; t < e.length; t++)
    e[t]();
}
function bo() {
  var e, t, r = new Promise((n, o) => {
    e = n, t = o;
  });
  return { promise: r, resolve: e, reject: t };
}
const be = 2, yr = 4, en = 8, _o = 1 << 24, Mt = 16, at = 32, lr = 64, Us = 128, qe = 512, he = 1024, ke = 2048, lt = 4096, Ge = 8192, tt = 16384, vr = 32768, jn = 1 << 25, cr = 65536, Bn = 1 << 17, Hs = 1 << 18, tn = 1 << 19, Ws = 1 << 20, it = 1 << 25, Ft = 65536, vn = 1 << 21, Tn = 1 << 22, Et = 1 << 23, ln = Symbol("$state"), Ks = Symbol(""), dt = new class extends Error {
  constructor() {
    super(...arguments);
    xt(this, "name", "StaleReactionError");
    xt(this, "message", "The reaction that called `getAbortSignal()` was re-run or destroyed");
  }
}();
function qs() {
  throw new Error("https://svelte.dev/e/async_derived_orphan");
}
function Ys(e, t, r) {
  throw new Error("https://svelte.dev/e/each_key_duplicate");
}
function Xs() {
  throw new Error("https://svelte.dev/e/effect_update_depth_exceeded");
}
function Zs() {
  throw new Error("https://svelte.dev/e/state_descriptors_fixed");
}
function Js() {
  throw new Error("https://svelte.dev/e/state_prototype_fixed");
}
function Qs() {
  throw new Error("https://svelte.dev/e/state_unsafe_mutation");
}
function xo(e) {
  return e === this.v;
}
function $s(e, t) {
  return e != e ? t == t : e !== t || e !== null && typeof e == "object" || typeof e == "function";
}
function yo(e) {
  return !$s(e, this.v);
}
let rn = !1, ei = !1;
function ti() {
  rn = !0;
}
let ze = null;
function Br(e) {
  ze = e;
}
function Bt(e, t = !1, r) {
  ze = {
    p: ze,
    i: !1,
    c: null,
    e: null,
    s: e,
    x: null,
    r: (
      /** @type {Effect} */
      ae
    ),
    l: rn && !t ? { s: null, u: null, $: [] } : null
  };
}
function Ut(e) {
  var t = (
    /** @type {ComponentContext} */
    ze
  ), r = t.e;
  if (r !== null) {
    t.e = null;
    for (var n of r)
      wi(n);
  }
  return t.i = !0, ze = t.p, /** @type {T} */
  {};
}
function Rr() {
  return !rn || ze !== null && ze.l === null;
}
let Kt = [];
function ri() {
  var e = Kt;
  Kt = [], Bs(e);
}
function wr(e) {
  if (Kt.length === 0) {
    var t = Kt;
    queueMicrotask(() => {
      t === Kt && ri();
    });
  }
  Kt.push(e);
}
function ni(e) {
  var t = ae;
  if (t === null)
    return ee.f |= Et, e;
  if ((t.f & vr) === 0 && (t.f & yr) === 0)
    throw e;
  Ur(e, t);
}
function Ur(e, t) {
  for (; t !== null; ) {
    if ((t.f & Us) !== 0) {
      if ((t.f & vr) === 0)
        throw e;
      try {
        t.b.error(e);
        return;
      } catch (r) {
        e = r;
      }
    }
    t = t.parent;
  }
  throw e;
}
const oi = -7169;
function pe(e, t) {
  e.f = e.f & oi | t;
}
function An(e) {
  (e.f & qe) !== 0 || e.deps === null ? pe(e, he) : pe(e, lt);
}
function wo(e) {
  if (e !== null)
    for (const t of e)
      (t.f & be) === 0 || (t.f & Ft) === 0 || (t.f ^= Ft, wo(
        /** @type {Derived} */
        t.deps
      ));
}
function si(e, t, r) {
  (e.f & ke) !== 0 ? t.add(e) : (e.f & lt) !== 0 && r.add(e), wo(e.deps), pe(e, he);
}
const Pt = /* @__PURE__ */ new Set();
let oe = null, ye = null, pn = null, cn = !1, qt = null, Fr = null;
var Un = 0;
let ii = 1;
var $t, er, tr, rr, Tr, Ve, Lt, vt, pt, nr, Ce, mn, gn, hn, bn, ko;
const Zr = class Zr {
  constructor() {
    ie(this, Ce);
    // for debugging. TODO remove once async is stable
    xt(this, "id", ii++);
    /**
     * The current values of any sources that are updated in this batch
     * They keys of this map are identical to `this.#previous`
     * @type {Map<Source, any>}
     */
    xt(this, "current", /* @__PURE__ */ new Map());
    /**
     * The values of any sources that are updated in this batch _before_ those updates took place.
     * They keys of this map are identical to `this.#current`
     * @type {Map<Source, any>}
     */
    xt(this, "previous", /* @__PURE__ */ new Map());
    /**
     * When the batch is committed (and the DOM is updated), we need to remove old branches
     * and append new ones by calling the functions added inside (if/each/key/etc) blocks
     * @type {Set<(batch: Batch) => void>}
     */
    ie(this, $t, /* @__PURE__ */ new Set());
    /**
     * If a fork is discarded, we need to destroy any effects that are no longer needed
     * @type {Set<(batch: Batch) => void>}
     */
    ie(this, er, /* @__PURE__ */ new Set());
    /**
     * The number of async effects that are currently in flight
     */
    ie(this, tr, 0);
    /**
     * The number of async effects that are currently in flight, _not_ inside a pending boundary
     */
    ie(this, rr, 0);
    /**
     * A deferred that resolves when the batch is committed, used with `settled()`
     * TODO replace with Promise.withResolvers once supported widely enough
     * @type {{ promise: Promise<void>, resolve: (value?: any) => void, reject: (reason: unknown) => void } | null}
     */
    ie(this, Tr, null);
    /**
     * The root effects that need to be flushed
     * @type {Effect[]}
     */
    ie(this, Ve, []);
    /**
     * Deferred effects (which run after async work has completed) that are DIRTY
     * @type {Set<Effect>}
     */
    ie(this, Lt, /* @__PURE__ */ new Set());
    /**
     * Deferred effects that are MAYBE_DIRTY
     * @type {Set<Effect>}
     */
    ie(this, vt, /* @__PURE__ */ new Set());
    /**
     * A map of branches that still exist, but will be destroyed when this batch
     * is committed — we skip over these during `process`.
     * The value contains child effects that were dirty/maybe_dirty before being reset,
     * so they can be rescheduled if the branch survives.
     * @type {Map<Effect, { d: Effect[], m: Effect[] }>}
     */
    ie(this, pt, /* @__PURE__ */ new Map());
    xt(this, "is_fork", !1);
    ie(this, nr, !1);
  }
  /**
   * Add an effect to the #skipped_branches map and reset its children
   * @param {Effect} effect
   */
  skip_effect(t) {
    _(this, pt).has(t) || _(this, pt).set(t, { d: [], m: [] });
  }
  /**
   * Remove an effect from the #skipped_branches map and reschedule
   * any tracked dirty/maybe_dirty child effects
   * @param {Effect} effect
   */
  unskip_effect(t) {
    var r = _(this, pt).get(t);
    if (r) {
      _(this, pt).delete(t);
      for (var n of r.d)
        pe(n, ke), this.schedule(n);
      for (n of r.m)
        pe(n, lt), this.schedule(n);
    }
  }
  /**
   * Associate a change to a given source with the current
   * batch, noting its previous and current values
   * @param {Source} source
   * @param {any} old_value
   */
  capture(t, r) {
    r !== xe && !this.previous.has(t) && this.previous.set(t, r), (t.f & Et) === 0 && (this.current.set(t, t.v), ye == null || ye.set(t, t.v));
  }
  activate() {
    oe = this;
  }
  deactivate() {
    oe = null, ye = null;
  }
  flush() {
    try {
      cn = !0, oe = this, We(this, Ce, gn).call(this);
    } finally {
      Un = 0, pn = null, qt = null, Fr = null, cn = !1, oe = null, ye = null, St.clear();
    }
  }
  discard() {
    for (const t of _(this, er)) t(this);
    _(this, er).clear(), Pt.delete(this);
  }
  /**
   *
   * @param {boolean} blocking
   */
  increment(t) {
    Se(this, tr, _(this, tr) + 1), t && Se(this, rr, _(this, rr) + 1);
  }
  /**
   * @param {boolean} blocking
   * @param {boolean} skip - whether to skip updates (because this is triggered by a stale reaction)
   */
  decrement(t, r) {
    Se(this, tr, _(this, tr) - 1), t && Se(this, rr, _(this, rr) - 1), !(_(this, nr) || r) && (Se(this, nr, !0), wr(() => {
      Se(this, nr, !1), this.flush();
    }));
  }
  /**
   * @param {Set<Effect>} dirty_effects
   * @param {Set<Effect>} maybe_dirty_effects
   */
  transfer_effects(t, r) {
    for (const n of t)
      _(this, Lt).add(n);
    for (const n of r)
      _(this, vt).add(n);
    t.clear(), r.clear();
  }
  /** @param {(batch: Batch) => void} fn */
  oncommit(t) {
    _(this, $t).add(t);
  }
  /** @param {(batch: Batch) => void} fn */
  ondiscard(t) {
    _(this, er).add(t);
  }
  settled() {
    return (_(this, Tr) ?? Se(this, Tr, bo())).promise;
  }
  static ensure() {
    if (oe === null) {
      const t = oe = new Zr();
      cn || (Pt.add(oe), wr(() => {
        oe === t && t.flush();
      }));
    }
    return oe;
  }
  apply() {
    {
      ye = null;
      return;
    }
  }
  /**
   *
   * @param {Effect} effect
   */
  schedule(t) {
    var o;
    if (pn = t, (o = t.b) != null && o.is_pending && (t.f & (yr | en | _o)) !== 0 && (t.f & vr) === 0) {
      t.b.defer_effect(t);
      return;
    }
    for (var r = t; r.parent !== null; ) {
      r = r.parent;
      var n = r.f;
      if (qt !== null && r === ae && (ee === null || (ee.f & be) === 0))
        return;
      if ((n & (lr | at)) !== 0) {
        if ((n & he) === 0)
          return;
        r.f ^= he;
      }
    }
    _(this, Ve).push(r);
  }
};
$t = new WeakMap(), er = new WeakMap(), tr = new WeakMap(), rr = new WeakMap(), Tr = new WeakMap(), Ve = new WeakMap(), Lt = new WeakMap(), vt = new WeakMap(), pt = new WeakMap(), nr = new WeakMap(), Ce = new WeakSet(), mn = function() {
  return this.is_fork || _(this, rr) > 0;
}, gn = function() {
  var u, l;
  if (Un++ > 1e3 && (Pt.delete(this), ai()), !We(this, Ce, mn).call(this)) {
    for (const c of _(this, Lt))
      _(this, vt).delete(c), pe(c, ke), this.schedule(c);
    for (const c of _(this, vt))
      pe(c, lt), this.schedule(c);
  }
  const t = _(this, Ve);
  Se(this, Ve, []), this.apply();
  var r = qt = [], n = [], o = Fr = [];
  for (const c of t)
    try {
      We(this, Ce, hn).call(this, c, r, n);
    } catch (d) {
      throw To(c), d;
    }
  if (oe = null, o.length > 0) {
    var s = Zr.ensure();
    for (const c of o)
      s.schedule(c);
  }
  if (qt = null, Fr = null, We(this, Ce, mn).call(this)) {
    We(this, Ce, bn).call(this, n), We(this, Ce, bn).call(this, r);
    for (const [c, d] of _(this, pt))
      So(c, d);
  } else {
    _(this, tr) === 0 && Pt.delete(this), _(this, Lt).clear(), _(this, vt).clear();
    for (const c of _(this, $t)) c(this);
    _(this, $t).clear(), Hn(n), Hn(r), (u = _(this, Tr)) == null || u.resolve();
  }
  var a = (
    /** @type {Batch | null} */
    /** @type {unknown} */
    oe
  );
  if (_(this, Ve).length > 0) {
    const c = a ?? (a = this);
    _(c, Ve).push(..._(this, Ve).filter((d) => !_(c, Ve).includes(d)));
  }
  a !== null && (Pt.add(a), We(l = a, Ce, gn).call(l)), Pt.has(this) || We(this, Ce, ko).call(this);
}, /**
 * Traverse the effect tree, executing effects or stashing
 * them for later execution as appropriate
 * @param {Effect} root
 * @param {Effect[]} effects
 * @param {Effect[]} render_effects
 */
hn = function(t, r, n) {
  t.f ^= he;
  for (var o = t.first; o !== null; ) {
    var s = o.f, a = (s & (at | lr)) !== 0, u = a && (s & he) !== 0, l = u || (s & Ge) !== 0 || _(this, pt).has(o);
    if (!l && o.fn !== null) {
      a ? o.f ^= he : (s & yr) !== 0 ? r.push(o) : Nr(o) && ((s & Mt) !== 0 && _(this, vt).add(o), dr(o));
      var c = o.first;
      if (c !== null) {
        o = c;
        continue;
      }
    }
    for (; o !== null; ) {
      var d = o.next;
      if (d !== null) {
        o = d;
        break;
      }
      o = o.parent;
    }
  }
}, /**
 * @param {Effect[]} effects
 */
bn = function(t) {
  for (var r = 0; r < t.length; r += 1)
    si(t[r], _(this, Lt), _(this, vt));
}, ko = function() {
  var l;
  for (const c of Pt) {
    var t = c.id < this.id, r = [];
    for (const [d, b] of this.current) {
      if (c.current.has(d))
        if (t && b !== c.current.get(d))
          c.current.set(d, b);
        else
          continue;
      r.push(d);
    }
    var n = [...c.current.keys()].filter((d) => !this.current.has(d));
    if (n.length === 0)
      t && c.discard();
    else if (r.length > 0) {
      c.activate();
      var o = /* @__PURE__ */ new Set(), s = /* @__PURE__ */ new Map();
      for (var a of r)
        Co(a, n, o, s);
      if (_(c, Ve).length > 0) {
        c.apply();
        for (var u of _(c, Ve))
          We(l = c, Ce, hn).call(l, u, [], []);
        Se(c, Ve, []);
      }
      c.deactivate();
    }
  }
};
let kr = Zr;
function ai() {
  try {
    Xs();
  } catch (e) {
    Ur(e, pn);
  }
}
let Je = null;
function Hn(e) {
  var t = e.length;
  if (t !== 0) {
    for (var r = 0; r < t; ) {
      var n = e[r++];
      if ((n.f & (tt | Ge)) === 0 && Nr(n) && (Je = /* @__PURE__ */ new Set(), dr(n), n.deps === null && n.first === null && n.nodes === null && n.teardown === null && n.ac === null && Vo(n), (Je == null ? void 0 : Je.size) > 0)) {
        St.clear();
        for (const o of Je) {
          if ((o.f & (tt | Ge)) !== 0) continue;
          const s = [o];
          let a = o.parent;
          for (; a !== null; )
            Je.has(a) && (Je.delete(a), s.push(a)), a = a.parent;
          for (let u = s.length - 1; u >= 0; u--) {
            const l = s[u];
            (l.f & (tt | Ge)) === 0 && dr(l);
          }
        }
        Je.clear();
      }
    }
    Je = null;
  }
}
function Co(e, t, r, n) {
  if (!r.has(e) && (r.add(e), e.reactions !== null))
    for (const o of e.reactions) {
      const s = o.f;
      (s & be) !== 0 ? Co(
        /** @type {Derived} */
        o,
        t,
        r,
        n
      ) : (s & (Tn | Mt)) !== 0 && (s & ke) === 0 && Eo(o, t, n) && (pe(o, ke), Mn(
        /** @type {Effect} */
        o
      ));
    }
}
function Eo(e, t, r) {
  const n = r.get(e);
  if (n !== void 0) return n;
  if (e.deps !== null)
    for (const o of e.deps) {
      if (ar.call(t, o))
        return !0;
      if ((o.f & be) !== 0 && Eo(
        /** @type {Derived} */
        o,
        t,
        r
      ))
        return r.set(
          /** @type {Derived} */
          o,
          !0
        ), !0;
    }
  return r.set(e, !1), !1;
}
function Mn(e) {
  oe.schedule(e);
}
function So(e, t) {
  if (!((e.f & at) !== 0 && (e.f & he) !== 0)) {
    (e.f & ke) !== 0 ? t.d.push(e) : (e.f & lt) !== 0 && t.m.push(e), pe(e, he);
    for (var r = e.first; r !== null; )
      So(r, t), r = r.next;
  }
}
function To(e) {
  pe(e, he);
  for (var t = e.first; t !== null; )
    To(t), t = t.next;
}
function Ao(e) {
  let t = 0, r = Gt(0), n;
  return () => {
    Dn() && (i(r), Ci(() => (t === 0 && (n = Mi(() => e(() => Tt(r)))), t += 1, () => {
      wr(() => {
        t -= 1, t === 0 && (n == null || n(), n = void 0, Tt(r));
      });
    })));
  };
}
function li(e, t, r, n) {
  const o = Rr() ? zn : zo;
  var s = e.filter((f) => !f.settled);
  if (r.length === 0 && s.length === 0) {
    n(t.map(o));
    return;
  }
  var a = (
    /** @type {Effect} */
    ae
  ), u = ci(), l = s.length === 1 ? s[0].promise : s.length > 1 ? Promise.all(s.map((f) => f.promise)) : null;
  function c(f) {
    u();
    try {
      n(f);
    } catch (y) {
      (a.f & tt) === 0 && Ur(y, a);
    }
    Hr();
  }
  if (r.length === 0) {
    l.then(() => c(t.map(o)));
    return;
  }
  var d = Mo();
  function b() {
    Promise.all(r.map((f) => /* @__PURE__ */ ui(f))).then((f) => c([...t.map(o), ...f])).catch((f) => Ur(f, a)).finally(() => d());
  }
  l ? l.then(() => {
    u(), b(), Hr();
  }) : b();
}
function ci() {
  var e = (
    /** @type {Effect} */
    ae
  ), t = ee, r = ze, n = (
    /** @type {Batch} */
    oe
  );
  return function(s = !0) {
    At(e), ct(t), Br(r), s && (e.f & tt) === 0 && (n == null || n.activate(), n == null || n.apply());
  };
}
function Hr(e = !0) {
  At(null), ct(null), Br(null), e && (oe == null || oe.deactivate());
}
function Mo() {
  var e = (
    /** @type {Boundary} */
    /** @type {Effect} */
    ae.b
  ), t = (
    /** @type {Batch} */
    oe
  ), r = e.is_rendered();
  return e.update_pending_count(1, t), t.increment(r), (n = !1) => {
    e.update_pending_count(-1, t), t.decrement(r, n);
  };
}
// @__NO_SIDE_EFFECTS__
function zn(e) {
  var t = be | ke, r = ee !== null && (ee.f & be) !== 0 ? (
    /** @type {Derived} */
    ee
  ) : null;
  return ae !== null && (ae.f |= tn), {
    ctx: ze,
    deps: null,
    effects: null,
    equals: xo,
    f: t,
    fn: e,
    reactions: null,
    rv: 0,
    v: (
      /** @type {V} */
      xe
    ),
    wv: 0,
    parent: r ?? ae,
    ac: null
  };
}
// @__NO_SIDE_EFFECTS__
function ui(e, t, r) {
  let n = (
    /** @type {Effect | null} */
    ae
  );
  n === null && qs();
  var o = (
    /** @type {Promise<V>} */
    /** @type {unknown} */
    void 0
  ), s = Gt(
    /** @type {V} */
    xe
  ), a = !ee, u = /* @__PURE__ */ new Map();
  return ki(() => {
    var y;
    var l = (
      /** @type {Effect} */
      ae
    ), c = bo();
    o = c.promise;
    try {
      Promise.resolve(e()).then(c.resolve, c.reject).finally(Hr);
    } catch (x) {
      c.reject(x), Hr();
    }
    var d = (
      /** @type {Batch} */
      oe
    );
    if (a) {
      if ((l.f & vr) !== 0)
        var b = Mo();
      if (
        /** @type {Boundary} */
        n.b.is_rendered()
      )
        (y = u.get(d)) == null || y.reject(dt), u.delete(d);
      else {
        for (const x of u.values())
          x.reject(dt);
        u.clear();
      }
      u.set(d, c);
    }
    const f = (x, w = void 0) => {
      if (b) {
        var v = w === dt;
        b(v);
      }
      if (!(w === dt || (l.f & tt) !== 0)) {
        if (d.activate(), w)
          s.f |= Et, Cr(s, w);
        else {
          (s.f & Et) !== 0 && (s.f ^= Et), Cr(s, x);
          for (const [P, V] of u) {
            if (u.delete(P), P === d) break;
            V.reject(dt);
          }
        }
        d.deactivate();
      }
    };
    c.promise.then(f, (x) => f(null, x || "unknown"));
  }), Io(() => {
    for (const l of u.values())
      l.reject(dt);
  }), new Promise((l) => {
    function c(d) {
      function b() {
        d === o ? l(s) : c(o);
      }
      d.then(b, b);
    }
    c(o);
  });
}
// @__NO_SIDE_EFFECTS__
function I(e) {
  const t = /* @__PURE__ */ zn(e);
  return Bo(t), t;
}
// @__NO_SIDE_EFFECTS__
function zo(e) {
  const t = /* @__PURE__ */ zn(e);
  return t.equals = yo, t;
}
function di(e) {
  var t = e.effects;
  if (t !== null) {
    e.effects = null;
    for (var r = 0; r < t.length; r += 1)
      ht(
        /** @type {Effect} */
        t[r]
      );
  }
}
function fi(e) {
  for (var t = e.parent; t !== null; ) {
    if ((t.f & be) === 0)
      return (t.f & tt) === 0 ? (
        /** @type {Effect} */
        t
      ) : null;
    t = t.parent;
  }
  return null;
}
function Pn(e) {
  var t, r = ae;
  At(fi(e));
  try {
    e.f &= ~Ft, di(e), t = Ko(e);
  } finally {
    At(r);
  }
  return t;
}
function Po(e) {
  var t = e.v, r = Pn(e);
  if (!e.equals(r) && (e.wv = Ho(), (!(oe != null && oe.is_fork) || e.deps === null) && (e.v = r, oe == null || oe.capture(e, t), e.deps === null))) {
    pe(e, he);
    return;
  }
  ur || (ye !== null ? (Dn() || oe != null && oe.is_fork) && ye.set(e, r) : An(e));
}
function vi(e) {
  var t, r;
  if (e.effects !== null)
    for (const n of e.effects)
      (n.teardown || n.ac) && ((t = n.teardown) == null || t.call(n), (r = n.ac) == null || r.abort(dt), n.teardown = js, n.ac = null, Sr(n, 0), Ln(n));
}
function Ro(e) {
  if (e.effects !== null)
    for (const t of e.effects)
      t.teardown && dr(t);
}
let _n = /* @__PURE__ */ new Set();
const St = /* @__PURE__ */ new Map();
let Do = !1;
function Gt(e, t) {
  var r = {
    f: 0,
    // TODO ideally we could skip this altogether, but it causes type errors
    v: e,
    reactions: null,
    equals: xo,
    rv: 0,
    wv: 0
  };
  return r;
}
// @__NO_SIDE_EFFECTS__
function ve(e, t) {
  const r = Gt(e);
  return Bo(r), r;
}
// @__NO_SIDE_EFFECTS__
function pi(e, t = !1, r = !0) {
  var o;
  const n = Gt(e);
  return t || (n.equals = yo), rn && r && ze !== null && ze.l !== null && ((o = ze.l).s ?? (o.s = [])).push(n), n;
}
function re(e, t, r = !1) {
  ee !== null && // since we are untracking the function inside `$inspect.with` we need to add this check
  // to ensure we error if state is set inside an inspect effect
  (!et || (ee.f & Bn) !== 0) && Rr() && (ee.f & (be | Mt | Tn | Bn)) !== 0 && (Ye === null || !ar.call(Ye, e)) && Qs();
  let n = r ? Yt(t) : t;
  return Cr(e, n, Fr);
}
function Cr(e, t, r = null) {
  if (!e.equals(t)) {
    var n = e.v;
    ur ? St.set(e, t) : St.set(e, n), e.v = t;
    var o = kr.ensure();
    if (o.capture(e, n), (e.f & be) !== 0) {
      const s = (
        /** @type {Derived} */
        e
      );
      (e.f & ke) !== 0 && Pn(s), ye === null && An(s);
    }
    e.wv = Ho(), No(e, ke, r), Rr() && ae !== null && (ae.f & he) !== 0 && (ae.f & (at | lr)) === 0 && (Ke === null ? Ti([e]) : Ke.push(e)), !o.is_fork && _n.size > 0 && !Do && mi();
  }
  return t;
}
function mi() {
  Do = !1;
  for (const e of _n)
    (e.f & he) !== 0 && pe(e, lt), Nr(e) && dr(e);
  _n.clear();
}
function Tt(e) {
  re(e, e.v + 1);
}
function No(e, t, r) {
  var n = e.reactions;
  if (n !== null)
    for (var o = Rr(), s = n.length, a = 0; a < s; a++) {
      var u = n[a], l = u.f;
      if (!(!o && u === ae)) {
        var c = (l & ke) === 0;
        if (c && pe(u, t), (l & be) !== 0) {
          var d = (
            /** @type {Derived} */
            u
          );
          ye == null || ye.delete(d), (l & Ft) === 0 && (l & qe && (u.f |= Ft), No(d, lt, r));
        } else if (c) {
          var b = (
            /** @type {Effect} */
            u
          );
          (l & Mt) !== 0 && Je !== null && Je.add(b), r !== null ? r.push(b) : Mn(b);
        }
      }
    }
}
function Yt(e) {
  if (typeof e != "object" || e === null || ln in e)
    return e;
  const t = ho(e);
  if (t !== Fs && t !== Gs)
    return e;
  var r = /* @__PURE__ */ new Map(), n = go(e), o = /* @__PURE__ */ ve(0), s = bt, a = (u) => {
    if (bt === s)
      return u();
    var l = ee, c = bt;
    ct(null), Kn(s);
    var d = u();
    return ct(l), Kn(c), d;
  };
  return n && r.set("length", /* @__PURE__ */ ve(
    /** @type {any[]} */
    e.length
  )), new Proxy(
    /** @type {any} */
    e,
    {
      defineProperty(u, l, c) {
        (!("value" in c) || c.configurable === !1 || c.enumerable === !1 || c.writable === !1) && Zs();
        var d = r.get(l);
        return d === void 0 ? a(() => {
          var b = /* @__PURE__ */ ve(c.value);
          return r.set(l, b), b;
        }) : re(d, c.value, !0), !0;
      },
      deleteProperty(u, l) {
        var c = r.get(l);
        if (c === void 0) {
          if (l in u) {
            const d = a(() => /* @__PURE__ */ ve(xe));
            r.set(l, d), Tt(o);
          }
        } else
          re(c, xe), Tt(o);
        return !0;
      },
      get(u, l, c) {
        var y;
        if (l === ln)
          return e;
        var d = r.get(l), b = l in u;
        if (d === void 0 && (!b || (y = an(u, l)) != null && y.writable) && (d = a(() => {
          var x = Yt(b ? u[l] : xe), w = /* @__PURE__ */ ve(x);
          return w;
        }), r.set(l, d)), d !== void 0) {
          var f = i(d);
          return f === xe ? void 0 : f;
        }
        return Reflect.get(u, l, c);
      },
      getOwnPropertyDescriptor(u, l) {
        var c = Reflect.getOwnPropertyDescriptor(u, l);
        if (c && "value" in c) {
          var d = r.get(l);
          d && (c.value = i(d));
        } else if (c === void 0) {
          var b = r.get(l), f = b == null ? void 0 : b.v;
          if (b !== void 0 && f !== xe)
            return {
              enumerable: !0,
              configurable: !0,
              value: f,
              writable: !0
            };
        }
        return c;
      },
      has(u, l) {
        var f;
        if (l === ln)
          return !0;
        var c = r.get(l), d = c !== void 0 && c.v !== xe || Reflect.has(u, l);
        if (c !== void 0 || ae !== null && (!d || (f = an(u, l)) != null && f.writable)) {
          c === void 0 && (c = a(() => {
            var y = d ? Yt(u[l]) : xe, x = /* @__PURE__ */ ve(y);
            return x;
          }), r.set(l, c));
          var b = i(c);
          if (b === xe)
            return !1;
        }
        return d;
      },
      set(u, l, c, d) {
        var O;
        var b = r.get(l), f = l in u;
        if (n && l === "length")
          for (var y = c; y < /** @type {Source<number>} */
          b.v; y += 1) {
            var x = r.get(y + "");
            x !== void 0 ? re(x, xe) : y in u && (x = a(() => /* @__PURE__ */ ve(xe)), r.set(y + "", x));
          }
        if (b === void 0)
          (!f || (O = an(u, l)) != null && O.writable) && (b = a(() => /* @__PURE__ */ ve(void 0)), re(b, Yt(c)), r.set(l, b));
        else {
          f = b.v !== xe;
          var w = a(() => Yt(c));
          re(b, w);
        }
        var v = Reflect.getOwnPropertyDescriptor(u, l);
        if (v != null && v.set && v.set.call(d, c), !f) {
          if (n && typeof l == "string") {
            var P = (
              /** @type {Source<number>} */
              r.get("length")
            ), V = Number(l);
            Number.isInteger(V) && V >= P.v && re(P, V + 1);
          }
          Tt(o);
        }
        return !0;
      },
      ownKeys(u) {
        i(o);
        var l = Reflect.ownKeys(u).filter((b) => {
          var f = r.get(b);
          return f === void 0 || f.v !== xe;
        });
        for (var [c, d] of r)
          d.v !== xe && !(c in u) && l.push(c);
        return l;
      },
      setPrototypeOf() {
        Js();
      }
    }
  );
}
var gi, hi, bi;
function Ot(e = "") {
  return document.createTextNode(e);
}
// @__NO_SIDE_EFFECTS__
function Wr(e) {
  return (
    /** @type {TemplateNode | null} */
    hi.call(e)
  );
}
// @__NO_SIDE_EFFECTS__
function Dr(e) {
  return (
    /** @type {TemplateNode | null} */
    bi.call(e)
  );
}
function h(e, t) {
  return /* @__PURE__ */ Wr(e);
}
function gt(e, t = !1) {
  {
    var r = /* @__PURE__ */ Wr(e);
    return r instanceof Comment && r.data === "" ? /* @__PURE__ */ Dr(r) : r;
  }
}
function W(e, t = 1, r = !1) {
  let n = e;
  for (; t--; )
    n = /** @type {TemplateNode} */
    /* @__PURE__ */ Dr(n);
  return n;
}
function _i(e) {
  e.textContent = "";
}
function Lo() {
  return !1;
}
function xi(e, t, r) {
  return (
    /** @type {T extends keyof HTMLElementTagNameMap ? HTMLElementTagNameMap[T] : Element} */
    document.createElementNS(mo, e, void 0)
  );
}
function Rn(e) {
  var t = ee, r = ae;
  ct(null), At(null);
  try {
    return e();
  } finally {
    ct(t), At(r);
  }
}
function yi(e, t) {
  var r = t.last;
  r === null ? t.last = t.first = e : (r.next = e, e.prev = r, t.last = e);
}
function Ht(e, t) {
  var r = ae;
  r !== null && (r.f & Ge) !== 0 && (e |= Ge);
  var n = {
    ctx: ze,
    deps: null,
    nodes: null,
    f: e | ke | qe,
    first: null,
    fn: t,
    last: null,
    next: null,
    parent: r,
    b: r && r.b,
    prev: null,
    teardown: null,
    wv: 0,
    ac: null
  }, o = n;
  if ((e & yr) !== 0)
    qt !== null ? qt.push(n) : kr.ensure().schedule(n);
  else if (t !== null) {
    try {
      dr(n);
    } catch (a) {
      throw ht(n), a;
    }
    o.deps === null && o.teardown === null && o.nodes === null && o.first === o.last && // either `null`, or a singular child
    (o.f & tn) === 0 && (o = o.first, (e & Mt) !== 0 && (e & cr) !== 0 && o !== null && (o.f |= cr));
  }
  if (o !== null && (o.parent = r, r !== null && yi(o, r), ee !== null && (ee.f & be) !== 0 && (e & lr) === 0)) {
    var s = (
      /** @type {Derived} */
      ee
    );
    (s.effects ?? (s.effects = [])).push(o);
  }
  return n;
}
function Dn() {
  return ee !== null && !et;
}
function Io(e) {
  const t = Ht(en, null);
  return pe(t, he), t.teardown = e, t;
}
function wi(e) {
  return Ht(yr | Ws, e);
}
function ki(e) {
  return Ht(Tn | tn, e);
}
function Ci(e, t = 0) {
  return Ht(en | t, e);
}
function X(e, t = [], r = [], n = []) {
  li(n, t, r, (o) => {
    Ht(en, () => e(...o.map(i)));
  });
}
function Nn(e, t = 0) {
  var r = Ht(Mt | t, e);
  return r;
}
function Er(e) {
  return Ht(at | tn, e);
}
function Oo(e) {
  var t = e.teardown;
  if (t !== null) {
    const r = ur, n = ee;
    Wn(!0), ct(null);
    try {
      t.call(null);
    } finally {
      Wn(r), ct(n);
    }
  }
}
function Ln(e, t = !1) {
  var r = e.first;
  for (e.first = e.last = null; r !== null; ) {
    const o = r.ac;
    o !== null && Rn(() => {
      o.abort(dt);
    });
    var n = r.next;
    (r.f & lr) !== 0 ? r.parent = null : ht(r, t), r = n;
  }
}
function Ei(e) {
  for (var t = e.first; t !== null; ) {
    var r = t.next;
    (t.f & at) === 0 && ht(t), t = r;
  }
}
function ht(e, t = !0) {
  var r = !1;
  (t || (e.f & Hs) !== 0) && e.nodes !== null && e.nodes.end !== null && (Si(
    e.nodes.start,
    /** @type {TemplateNode} */
    e.nodes.end
  ), r = !0), pe(e, jn), Ln(e, t && !r), Sr(e, 0);
  var n = e.nodes && e.nodes.t;
  if (n !== null)
    for (const s of n)
      s.stop();
  Oo(e), e.f ^= jn, e.f |= tt;
  var o = e.parent;
  o !== null && o.first !== null && Vo(e), e.next = e.prev = e.teardown = e.ctx = e.deps = e.fn = e.nodes = e.ac = null;
}
function Si(e, t) {
  for (; e !== null; ) {
    var r = e === t ? null : /* @__PURE__ */ Dr(e);
    e.remove(), e = r;
  }
}
function Vo(e) {
  var t = e.parent, r = e.prev, n = e.next;
  r !== null && (r.next = n), n !== null && (n.prev = r), t !== null && (t.first === e && (t.first = n), t.last === e && (t.last = r));
}
function In(e, t, r = !0) {
  var n = [];
  Fo(e, n, !0);
  var o = () => {
    r && ht(e), t && t();
  }, s = n.length;
  if (s > 0) {
    var a = () => --s || o();
    for (var u of n)
      u.out(a);
  } else
    o();
}
function Fo(e, t, r) {
  if ((e.f & Ge) === 0) {
    e.f ^= Ge;
    var n = e.nodes && e.nodes.t;
    if (n !== null)
      for (const u of n)
        (u.is_global || r) && t.push(u);
    for (var o = e.first; o !== null; ) {
      var s = o.next, a = (o.f & cr) !== 0 || // If this is a branch effect without a block effect parent,
      // it means the parent block effect was pruned. In that case,
      // transparency information was transferred to the branch effect.
      (o.f & at) !== 0 && (e.f & Mt) !== 0;
      Fo(o, t, a ? r : !1), o = s;
    }
  }
}
function On(e) {
  Go(e, !0);
}
function Go(e, t) {
  if ((e.f & Ge) !== 0) {
    e.f ^= Ge, (e.f & he) === 0 && (pe(e, ke), kr.ensure().schedule(e));
    for (var r = e.first; r !== null; ) {
      var n = r.next, o = (r.f & cr) !== 0 || (r.f & at) !== 0;
      Go(r, o ? t : !1), r = n;
    }
    var s = e.nodes && e.nodes.t;
    if (s !== null)
      for (const a of s)
        (a.is_global || t) && a.in();
  }
}
function jo(e, t) {
  if (e.nodes)
    for (var r = e.nodes.start, n = e.nodes.end; r !== null; ) {
      var o = r === n ? null : /* @__PURE__ */ Dr(r);
      t.append(r), r = o;
    }
}
let Gr = !1, ur = !1;
function Wn(e) {
  ur = e;
}
let ee = null, et = !1;
function ct(e) {
  ee = e;
}
let ae = null;
function At(e) {
  ae = e;
}
let Ye = null;
function Bo(e) {
  ee !== null && (Ye === null ? Ye = [e] : Ye.push(e));
}
let Me = null, Oe = 0, Ke = null;
function Ti(e) {
  Ke = e;
}
let Uo = 1, Dt = 0, bt = Dt;
function Kn(e) {
  bt = e;
}
function Ho() {
  return ++Uo;
}
function Nr(e) {
  var t = e.f;
  if ((t & ke) !== 0)
    return !0;
  if (t & be && (e.f &= ~Ft), (t & lt) !== 0) {
    for (var r = (
      /** @type {Value[]} */
      e.deps
    ), n = r.length, o = 0; o < n; o++) {
      var s = r[o];
      if (Nr(
        /** @type {Derived} */
        s
      ) && Po(
        /** @type {Derived} */
        s
      ), s.wv > e.wv)
        return !0;
    }
    (t & qe) !== 0 && // During time traveling we don't want to reset the status so that
    // traversal of the graph in the other batches still happens
    ye === null && pe(e, he);
  }
  return !1;
}
function Wo(e, t, r = !0) {
  var n = e.reactions;
  if (n !== null && !(Ye !== null && ar.call(Ye, e)))
    for (var o = 0; o < n.length; o++) {
      var s = n[o];
      (s.f & be) !== 0 ? Wo(
        /** @type {Derived} */
        s,
        t,
        !1
      ) : t === s && (r ? pe(s, ke) : (s.f & he) !== 0 && pe(s, lt), Mn(
        /** @type {Effect} */
        s
      ));
    }
}
function Ko(e) {
  var w;
  var t = Me, r = Oe, n = Ke, o = ee, s = Ye, a = ze, u = et, l = bt, c = e.f;
  Me = /** @type {null | Value[]} */
  null, Oe = 0, Ke = null, ee = (c & (at | lr)) === 0 ? e : null, Ye = null, Br(e.ctx), et = !1, bt = ++Dt, e.ac !== null && (Rn(() => {
    e.ac.abort(dt);
  }), e.ac = null);
  try {
    e.f |= vn;
    var d = (
      /** @type {Function} */
      e.fn
    ), b = d();
    e.f |= vr;
    var f = e.deps, y = oe == null ? void 0 : oe.is_fork;
    if (Me !== null) {
      var x;
      if (y || Sr(e, Oe), f !== null && Oe > 0)
        for (f.length = Oe + Me.length, x = 0; x < Me.length; x++)
          f[Oe + x] = Me[x];
      else
        e.deps = f = Me;
      if (Dn() && (e.f & qe) !== 0)
        for (x = Oe; x < f.length; x++)
          ((w = f[x]).reactions ?? (w.reactions = [])).push(e);
    } else !y && f !== null && Oe < f.length && (Sr(e, Oe), f.length = Oe);
    if (Rr() && Ke !== null && !et && f !== null && (e.f & (be | lt | ke)) === 0)
      for (x = 0; x < /** @type {Source[]} */
      Ke.length; x++)
        Wo(
          Ke[x],
          /** @type {Effect} */
          e
        );
    if (o !== null && o !== e) {
      if (Dt++, o.deps !== null)
        for (let v = 0; v < r; v += 1)
          o.deps[v].rv = Dt;
      if (t !== null)
        for (const v of t)
          v.rv = Dt;
      Ke !== null && (n === null ? n = Ke : n.push(.../** @type {Source[]} */
      Ke));
    }
    return (e.f & Et) !== 0 && (e.f ^= Et), b;
  } catch (v) {
    return ni(v);
  } finally {
    e.f ^= vn, Me = t, Oe = r, Ke = n, ee = o, Ye = s, Br(a), et = u, bt = l;
  }
}
function Ai(e, t) {
  let r = t.reactions;
  if (r !== null) {
    var n = Is.call(r, e);
    if (n !== -1) {
      var o = r.length - 1;
      o === 0 ? r = t.reactions = null : (r[n] = r[o], r.pop());
    }
  }
  if (r === null && (t.f & be) !== 0 && // Destroying a child effect while updating a parent effect can cause a dependency to appear
  // to be unused, when in fact it is used by the currently-updating parent. Checking `new_deps`
  // allows us to skip the expensive work of disconnecting and immediately reconnecting it
  (Me === null || !ar.call(Me, t))) {
    var s = (
      /** @type {Derived} */
      t
    );
    (s.f & qe) !== 0 && (s.f ^= qe, s.f &= ~Ft), An(s), vi(s), Sr(s, 0);
  }
}
function Sr(e, t) {
  var r = e.deps;
  if (r !== null)
    for (var n = t; n < r.length; n++)
      Ai(e, r[n]);
}
function dr(e) {
  var t = e.f;
  if ((t & tt) === 0) {
    pe(e, he);
    var r = ae, n = Gr;
    ae = e, Gr = !0;
    try {
      (t & (Mt | _o)) !== 0 ? Ei(e) : Ln(e), Oo(e);
      var o = Ko(e);
      e.teardown = typeof o == "function" ? o : null, e.wv = Uo;
      var s;
      Ls && ei && (e.f & ke) !== 0 && e.deps;
    } finally {
      Gr = n, ae = r;
    }
  }
}
function i(e) {
  var t = e.f, r = (t & be) !== 0;
  if (ee !== null && !et) {
    var n = ae !== null && (ae.f & tt) !== 0;
    if (!n && (Ye === null || !ar.call(Ye, e))) {
      var o = ee.deps;
      if ((ee.f & vn) !== 0)
        e.rv < Dt && (e.rv = Dt, Me === null && o !== null && o[Oe] === e ? Oe++ : Me === null ? Me = [e] : Me.push(e));
      else {
        (ee.deps ?? (ee.deps = [])).push(e);
        var s = e.reactions;
        s === null ? e.reactions = [ee] : ar.call(s, ee) || s.push(ee);
      }
    }
  }
  if (ur && St.has(e))
    return St.get(e);
  if (r) {
    var a = (
      /** @type {Derived} */
      e
    );
    if (ur) {
      var u = a.v;
      return ((a.f & he) === 0 && a.reactions !== null || Yo(a)) && (u = Pn(a)), St.set(a, u), u;
    }
    var l = (a.f & qe) === 0 && !et && ee !== null && (Gr || (ee.f & qe) !== 0), c = (a.f & vr) === 0;
    Nr(a) && (l && (a.f |= qe), Po(a)), l && !c && (Ro(a), qo(a));
  }
  if (ye != null && ye.has(e))
    return ye.get(e);
  if ((e.f & Et) !== 0)
    throw e.v;
  return e.v;
}
function qo(e) {
  if (e.f |= qe, e.deps !== null)
    for (const t of e.deps)
      (t.reactions ?? (t.reactions = [])).push(e), (t.f & be) !== 0 && (t.f & qe) === 0 && (Ro(
        /** @type {Derived} */
        t
      ), qo(
        /** @type {Derived} */
        t
      ));
}
function Yo(e) {
  if (e.v === xe) return !0;
  if (e.deps === null) return !1;
  for (const t of e.deps)
    if (St.has(t) || (t.f & be) !== 0 && Yo(
      /** @type {Derived} */
      t
    ))
      return !0;
  return !1;
}
function Mi(e) {
  var t = et;
  try {
    return et = !0, e();
  } finally {
    et = t;
  }
}
const Nt = Symbol("events"), zi = /* @__PURE__ */ new Set(), Pi = /* @__PURE__ */ new Set();
function Xo(e, t, r, n = {}) {
  function o(s) {
    if (n.capture || Ri.call(t, s), !s.cancelBubble)
      return Rn(() => r == null ? void 0 : r.call(this, s));
  }
  return e.startsWith("pointer") || e.startsWith("touch") || e === "wheel" ? wr(() => {
    t.addEventListener(e, o, n);
  }) : t.addEventListener(e, o, n), o;
}
function Kr(e, t, r, n = {}) {
  var o = Xo(t, e, r, n);
  return () => {
    e.removeEventListener(t, o, n);
  };
}
function Te(e, t, r, n, o) {
  var s = { capture: n, passive: o }, a = Xo(e, t, r, s);
  (t === document.body || // @ts-ignore
  t === window || // @ts-ignore
  t === document || // Firefox has quirky behavior, it can happen that we still get "canplay" events when the element is already removed
  t instanceof HTMLMediaElement) && Io(() => {
    t.removeEventListener(e, a, s);
  });
}
function _t(e, t, r) {
  (t[Nt] ?? (t[Nt] = {}))[e] = r;
}
function Xe(e) {
  for (var t = 0; t < e.length; t++)
    zi.add(e[t]);
  for (var r of Pi)
    r(e);
}
let qn = null;
function Ri(e) {
  var v, P;
  var t = this, r = (
    /** @type {Node} */
    t.ownerDocument
  ), n = e.type, o = ((v = e.composedPath) == null ? void 0 : v.call(e)) || [], s = (
    /** @type {null | Element} */
    o[0] || e.target
  );
  qn = e;
  var a = 0, u = qn === e && e[Nt];
  if (u) {
    var l = o.indexOf(u);
    if (l !== -1 && (t === document || t === /** @type {any} */
    window)) {
      e[Nt] = t;
      return;
    }
    var c = o.indexOf(t);
    if (c === -1)
      return;
    l <= c && (a = l);
  }
  if (s = /** @type {Element} */
  o[a] || e.target, s !== t) {
    Os(e, "currentTarget", {
      configurable: !0,
      get() {
        return s || r;
      }
    });
    var d = ee, b = ae;
    ct(null), At(null);
    try {
      for (var f, y = []; s !== null; ) {
        var x = s.assignedSlot || s.parentNode || /** @type {any} */
        s.host || null;
        try {
          var w = (P = s[Nt]) == null ? void 0 : P[n];
          w != null && (!/** @type {any} */
          s.disabled || // DOM could've been updated already by the time this is reached, so we check this as well
          // -> the target could not have been disabled because it emits the event in the first place
          e.target === s) && w.call(s, e);
        } catch (V) {
          f ? y.push(V) : f = V;
        }
        if (e.cancelBubble || x === t || x === null)
          break;
        s = x;
      }
      if (f) {
        for (let V of y)
          queueMicrotask(() => {
            throw V;
          });
        throw f;
      }
    } finally {
      e[Nt] = t, delete e.currentTarget, ct(d), At(b);
    }
  }
}
var io;
const un = (
  // We gotta write it like this because after downleveling the pure comment may end up in the wrong location
  ((io = globalThis == null ? void 0 : globalThis.window) == null ? void 0 : io.trustedTypes) && /* @__PURE__ */ globalThis.window.trustedTypes.createPolicy("svelte-trusted-html", {
    /** @param {string} html */
    createHTML: (e) => e
  })
);
function Di(e) {
  return (
    /** @type {string} */
    (un == null ? void 0 : un.createHTML(e)) ?? e
  );
}
function Ni(e) {
  var t = xi("template");
  return t.innerHTML = Di(e.replaceAll("<!>", "<!---->")), t.content;
}
function qr(e, t) {
  var r = (
    /** @type {Effect} */
    ae
  );
  r.nodes === null && (r.nodes = { start: e, end: t, a: null, t: null });
}
// @__NO_SIDE_EFFECTS__
function G(e, t) {
  var r = (t & Ds) !== 0, n = (t & Ns) !== 0, o, s = !e.startsWith("<!>");
  return () => {
    o === void 0 && (o = Ni(s ? e : "<!>" + e), r || (o = /** @type {TemplateNode} */
    /* @__PURE__ */ Wr(o)));
    var a = (
      /** @type {TemplateNode} */
      n || gi ? document.importNode(o, !0) : o.cloneNode(!0)
    );
    if (r) {
      var u = (
        /** @type {TemplateNode} */
        /* @__PURE__ */ Wr(a)
      ), l = (
        /** @type {TemplateNode} */
        a.lastChild
      );
      qr(u, l);
    } else
      qr(a, a);
    return a;
  };
}
function nn(e = "") {
  {
    var t = Ot(e + "");
    return qr(t, t), t;
  }
}
function Vt() {
  var e = document.createDocumentFragment(), t = document.createComment(""), r = Ot();
  return e.append(t, r), qr(t, r), e;
}
function D(e, t) {
  e !== null && e.before(
    /** @type {Node} */
    t
  );
}
function J(e, t) {
  var r = t == null ? "" : typeof t == "object" ? `${t}` : t;
  r !== (e.__t ?? (e.__t = e.nodeValue)) && (e.__t = r, e.nodeValue = `${r}`);
}
var Qe, st, Fe, It, Ar, Mr, Jr;
class Zo {
  /**
   * @param {TemplateNode} anchor
   * @param {boolean} transition
   */
  constructor(t, r = !0) {
    /** @type {TemplateNode} */
    xt(this, "anchor");
    /** @type {Map<Batch, Key>} */
    ie(this, Qe, /* @__PURE__ */ new Map());
    /**
     * Map of keys to effects that are currently rendered in the DOM.
     * These effects are visible and actively part of the document tree.
     * Example:
     * ```
     * {#if condition}
     * 	foo
     * {:else}
     * 	bar
     * {/if}
     * ```
     * Can result in the entries `true->Effect` and `false->Effect`
     * @type {Map<Key, Effect>}
     */
    ie(this, st, /* @__PURE__ */ new Map());
    /**
     * Similar to #onscreen with respect to the keys, but contains branches that are not yet
     * in the DOM, because their insertion is deferred.
     * @type {Map<Key, Branch>}
     */
    ie(this, Fe, /* @__PURE__ */ new Map());
    /**
     * Keys of effects that are currently outroing
     * @type {Set<Key>}
     */
    ie(this, It, /* @__PURE__ */ new Set());
    /**
     * Whether to pause (i.e. outro) on change, or destroy immediately.
     * This is necessary for `<svelte:element>`
     */
    ie(this, Ar, !0);
    /**
     * @param {Batch} batch
     */
    ie(this, Mr, (t) => {
      if (_(this, Qe).has(t)) {
        var r = (
          /** @type {Key} */
          _(this, Qe).get(t)
        ), n = _(this, st).get(r);
        if (n)
          On(n), _(this, It).delete(r);
        else {
          var o = _(this, Fe).get(r);
          o && (_(this, st).set(r, o.effect), _(this, Fe).delete(r), o.fragment.lastChild.remove(), this.anchor.before(o.fragment), n = o.effect);
        }
        for (const [s, a] of _(this, Qe)) {
          if (_(this, Qe).delete(s), s === t)
            break;
          const u = _(this, Fe).get(a);
          u && (ht(u.effect), _(this, Fe).delete(a));
        }
        for (const [s, a] of _(this, st)) {
          if (s === r || _(this, It).has(s)) continue;
          const u = () => {
            if (Array.from(_(this, Qe).values()).includes(s)) {
              var c = document.createDocumentFragment();
              jo(a, c), c.append(Ot()), _(this, Fe).set(s, { effect: a, fragment: c });
            } else
              ht(a);
            _(this, It).delete(s), _(this, st).delete(s);
          };
          _(this, Ar) || !n ? (_(this, It).add(s), In(a, u, !1)) : u();
        }
      }
    });
    /**
     * @param {Batch} batch
     */
    ie(this, Jr, (t) => {
      _(this, Qe).delete(t);
      const r = Array.from(_(this, Qe).values());
      for (const [n, o] of _(this, Fe))
        r.includes(n) || (ht(o.effect), _(this, Fe).delete(n));
    });
    this.anchor = t, Se(this, Ar, r);
  }
  /**
   *
   * @param {any} key
   * @param {null | ((target: TemplateNode) => void)} fn
   */
  ensure(t, r) {
    var n = (
      /** @type {Batch} */
      oe
    ), o = Lo();
    if (r && !_(this, st).has(t) && !_(this, Fe).has(t))
      if (o) {
        var s = document.createDocumentFragment(), a = Ot();
        s.append(a), _(this, Fe).set(t, {
          effect: Er(() => r(a)),
          fragment: s
        });
      } else
        _(this, st).set(
          t,
          Er(() => r(this.anchor))
        );
    if (_(this, Qe).set(n, t), o) {
      for (const [u, l] of _(this, st))
        u === t ? n.unskip_effect(l) : n.skip_effect(l);
      for (const [u, l] of _(this, Fe))
        u === t ? n.unskip_effect(l.effect) : n.skip_effect(l.effect);
      n.oncommit(_(this, Mr)), n.ondiscard(_(this, Jr));
    } else
      _(this, Mr).call(this, n);
  }
}
Qe = new WeakMap(), st = new WeakMap(), Fe = new WeakMap(), It = new WeakMap(), Ar = new WeakMap(), Mr = new WeakMap(), Jr = new WeakMap();
function Li(e, t, ...r) {
  var n = new Zo(e);
  Nn(() => {
    const o = t() ?? null;
    n.ensure(o, o && ((s) => o(s, ...r)));
  }, cr);
}
function ne(e, t, r = !1) {
  var n = new Zo(e), o = r ? cr : 0;
  function s(a, u) {
    n.ensure(a, u);
  }
  Nn(() => {
    var a = !1;
    t((u, l = 0) => {
      a = !0, s(l, u);
    }), a || s(-1, null);
  }, o);
}
function Ii(e, t) {
  return t;
}
function Oi(e, t, r) {
  for (var n = [], o = t.length, s, a = t.length, u = 0; u < o; u++) {
    let b = t[u];
    In(
      b,
      () => {
        if (s) {
          if (s.pending.delete(b), s.done.add(b), s.pending.size === 0) {
            var f = (
              /** @type {Set<EachOutroGroup>} */
              e.outrogroups
            );
            xn(e, Sn(s.done)), f.delete(s), f.size === 0 && (e.outrogroups = null);
          }
        } else
          a -= 1;
      },
      !1
    );
  }
  if (a === 0) {
    var l = n.length === 0 && r !== null;
    if (l) {
      var c = (
        /** @type {Element} */
        r
      ), d = (
        /** @type {Element} */
        c.parentNode
      );
      _i(d), d.append(c), e.items.clear();
    }
    xn(e, t, !l);
  } else
    s = {
      pending: new Set(t),
      done: /* @__PURE__ */ new Set()
    }, (e.outrogroups ?? (e.outrogroups = /* @__PURE__ */ new Set())).add(s);
}
function xn(e, t, r = !0) {
  var n;
  if (e.pending.size > 0) {
    n = /* @__PURE__ */ new Set();
    for (const a of e.pending.values())
      for (const u of a)
        n.add(
          /** @type {EachItem} */
          e.items.get(u).e
        );
  }
  for (var o = 0; o < t.length; o++) {
    var s = t[o];
    if (n != null && n.has(s)) {
      s.f |= it;
      const a = document.createDocumentFragment();
      jo(s, a);
    } else
      ht(t[o], r);
  }
}
var Yn;
function jt(e, t, r, n, o, s = null) {
  var a = e, u = /* @__PURE__ */ new Map(), l = (t & po) !== 0;
  if (l) {
    var c = (
      /** @type {Element} */
      e
    );
    a = c.appendChild(Ot());
  }
  var d = null, b = /* @__PURE__ */ zo(() => {
    var O = r();
    return go(O) ? O : O == null ? [] : Sn(O);
  }), f, y = /* @__PURE__ */ new Map(), x = !0;
  function w(O) {
    (V.effect.f & tt) === 0 && (V.pending.delete(O), V.fallback = d, Vi(V, f, a, t, n), d !== null && (f.length === 0 ? (d.f & it) === 0 ? On(d) : (d.f ^= it, _r(d, null, a)) : In(d, () => {
      d = null;
    })));
  }
  function v(O) {
    V.pending.delete(O);
  }
  var P = Nn(() => {
    f = /** @type {V[]} */
    i(b);
    for (var O = f.length, Y = /* @__PURE__ */ new Set(), C = (
      /** @type {Batch} */
      oe
    ), B = Lo(), H = 0; H < O; H += 1) {
      var m = f[H], k = n(m, H), N = x ? null : u.get(k);
      N ? (N.v && Cr(N.v, m), N.i && Cr(N.i, H), B && C.unskip_effect(N.e)) : (N = Fi(
        u,
        x ? a : Yn ?? (Yn = Ot()),
        m,
        k,
        H,
        o,
        t,
        r
      ), x || (N.e.f |= it), u.set(k, N)), Y.add(k);
    }
    if (O === 0 && s && !d && (x ? d = Er(() => s(a)) : (d = Er(() => s(Yn ?? (Yn = Ot()))), d.f |= it)), O > Y.size && Ys(), !x)
      if (y.set(C, Y), B) {
        for (const [M, q] of u)
          Y.has(M) || C.skip_effect(q.e);
        C.oncommit(w), C.ondiscard(v);
      } else
        w(C);
    i(b);
  }), V = { effect: P, items: u, pending: y, outrogroups: null, fallback: d };
  x = !1;
}
function hr(e) {
  for (; e !== null && (e.f & at) === 0; )
    e = e.next;
  return e;
}
function Vi(e, t, r, n, o) {
  var N, M, q, S, g, A, T, F, K;
  var s = (n & Ps) !== 0, a = t.length, u = e.items, l = hr(e.effect.first), c, d = null, b, f = [], y = [], x, w, v, P;
  if (s)
    for (P = 0; P < a; P += 1)
      x = t[P], w = o(x, P), v = /** @type {EachItem} */
      u.get(w).e, (v.f & it) === 0 && ((M = (N = v.nodes) == null ? void 0 : N.a) == null || M.measure(), (b ?? (b = /* @__PURE__ */ new Set())).add(v));
  for (P = 0; P < a; P += 1) {
    if (x = t[P], w = o(x, P), v = /** @type {EachItem} */
    u.get(w).e, e.outrogroups !== null)
      for (const U of e.outrogroups)
        U.pending.delete(v), U.done.delete(v);
    if ((v.f & Ge) !== 0 && (On(v), s && ((S = (q = v.nodes) == null ? void 0 : q.a) == null || S.unfix(), (b ?? (b = /* @__PURE__ */ new Set())).delete(v))), (v.f & it) !== 0)
      if (v.f ^= it, v === l)
        _r(v, null, r);
      else {
        var V = d ? d.next : l;
        v === e.effect.last && (e.effect.last = v.prev), v.prev && (v.prev.next = v.next), v.next && (v.next.prev = v.prev), yt(e, d, v), yt(e, v, V), _r(v, V, r), d = v, f = [], y = [], l = hr(d.next);
        continue;
      }
    if (v !== l) {
      if (c !== void 0 && c.has(v)) {
        if (f.length < y.length) {
          var O = y[0], Y;
          d = O.prev;
          var C = f[0], B = f[f.length - 1];
          for (Y = 0; Y < f.length; Y += 1)
            _r(f[Y], O, r);
          for (Y = 0; Y < y.length; Y += 1)
            c.delete(y[Y]);
          yt(e, C.prev, B.next), yt(e, d, C), yt(e, B, O), l = O, d = B, P -= 1, f = [], y = [];
        } else
          c.delete(v), _r(v, l, r), yt(e, v.prev, v.next), yt(e, v, d === null ? e.effect.first : d.next), yt(e, d, v), d = v;
        continue;
      }
      for (f = [], y = []; l !== null && l !== v; )
        (c ?? (c = /* @__PURE__ */ new Set())).add(l), y.push(l), l = hr(l.next);
      if (l === null)
        continue;
    }
    (v.f & it) === 0 && f.push(v), d = v, l = hr(v.next);
  }
  if (e.outrogroups !== null) {
    for (const U of e.outrogroups)
      U.pending.size === 0 && (xn(e, Sn(U.done)), (g = e.outrogroups) == null || g.delete(U));
    e.outrogroups.size === 0 && (e.outrogroups = null);
  }
  if (l !== null || c !== void 0) {
    var H = [];
    if (c !== void 0)
      for (v of c)
        (v.f & Ge) === 0 && H.push(v);
    for (; l !== null; )
      (l.f & Ge) === 0 && l !== e.fallback && H.push(l), l = hr(l.next);
    var m = H.length;
    if (m > 0) {
      var k = (n & po) !== 0 && a === 0 ? r : null;
      if (s) {
        for (P = 0; P < m; P += 1)
          (T = (A = H[P].nodes) == null ? void 0 : A.a) == null || T.measure();
        for (P = 0; P < m; P += 1)
          (K = (F = H[P].nodes) == null ? void 0 : F.a) == null || K.fix();
      }
      Oi(e, H, k);
    }
  }
  s && wr(() => {
    var U, E;
    if (b !== void 0)
      for (v of b)
        (E = (U = v.nodes) == null ? void 0 : U.a) == null || E.apply();
  });
}
function Fi(e, t, r, n, o, s, a, u) {
  var l = (a & Ms) !== 0 ? (a & Rs) === 0 ? /* @__PURE__ */ pi(r, !1, !1) : Gt(r) : null, c = (a & zs) !== 0 ? Gt(o) : null;
  return {
    v: l,
    i: c,
    e: Er(() => (s(t, l ?? r, c ?? o, u), () => {
      e.delete(n);
    }))
  };
}
function _r(e, t, r) {
  if (e.nodes)
    for (var n = e.nodes.start, o = e.nodes.end, s = t && (t.f & it) === 0 ? (
      /** @type {EffectNodes} */
      t.nodes.start
    ) : r; n !== null; ) {
      var a = (
        /** @type {TemplateNode} */
        /* @__PURE__ */ Dr(n)
      );
      if (s.before(n), n === o)
        return;
      n = a;
    }
}
function yt(e, t, r) {
  t === null ? e.effect.first = r : t.next = r, r === null ? e.effect.last = t : r.prev = t;
}
function Jo(e) {
  var t, r, n = "";
  if (typeof e == "string" || typeof e == "number") n += e;
  else if (typeof e == "object") if (Array.isArray(e)) {
    var o = e.length;
    for (t = 0; t < o; t++) e[t] && (r = Jo(e[t])) && (n && (n += " "), n += r);
  } else for (r in e) e[r] && (n && (n += " "), n += r);
  return n;
}
function Gi() {
  for (var e, t, r = 0, n = "", o = arguments.length; r < o; r++) (e = arguments[r]) && (t = Jo(e)) && (n && (n += " "), n += t);
  return n;
}
function Xt(e) {
  return typeof e == "object" ? Gi(e) : e ?? "";
}
function ji(e, t, r) {
  var n = e == null ? "" : "" + e;
  return n === "" ? null : n;
}
function Bi(e, t) {
  return e == null ? null : String(e);
}
function Zt(e, t, r, n, o, s) {
  var a = e.__className;
  if (a !== r || a === void 0) {
    var u = ji(r);
    u == null ? e.removeAttribute("class") : e.className = u, e.__className = r;
  }
  return s;
}
function Qo(e, t, r, n) {
  var o = e.__style;
  if (o !== t) {
    var s = Bi(t);
    s == null ? e.removeAttribute("style") : e.style.cssText = s, e.__style = t;
  }
  return n;
}
const Ui = Symbol("is custom element"), Hi = Symbol("is html");
function mt(e, t, r, n) {
  var o = Wi(e);
  o[t] !== (o[t] = r) && (t === "loading" && (e[Ks] = r), r == null ? e.removeAttribute(t) : typeof r != "string" && Ki(e).includes(t) ? e[t] = r : e.setAttribute(t, r));
}
function Wi(e) {
  return (
    /** @type {Record<string | symbol, unknown>} **/
    // @ts-expect-error
    e.__attributes ?? (e.__attributes = {
      [Ui]: e.nodeName.includes("-"),
      [Hi]: e.namespaceURI === mo
    })
  );
}
var Xn = /* @__PURE__ */ new Map();
function Ki(e) {
  var t = e.getAttribute("is") || e.nodeName, r = Xn.get(t);
  if (r) return r;
  Xn.set(t, r = []);
  for (var n, o = e, s = Element.prototype; s !== o; ) {
    n = Vs(o);
    for (var a in n)
      n[a].set && r.push(a);
    o = ho(o);
  }
  return r;
}
function Jt(e, t, r, n) {
  var o = (
    /** @type {V} */
    n
  ), s = !0, a = () => (s && (s = !1, o = /** @type {V} */
  n), o), u;
  u = /** @type {V} */
  e[t], u === void 0 && n !== void 0 && (u = a());
  var l;
  return l = () => {
    var c = (
      /** @type {V} */
      e[t]
    );
    return c === void 0 ? a() : (s = !0, c);
  }, l;
}
var qi = ["forEach", "isDisjointFrom", "isSubsetOf", "isSupersetOf"], Yi = ["difference", "intersection", "symmetricDifference", "union"], Zn = !1, or, $e, Ct, Qr, fr, $o, es;
const $r = class $r extends Set {
  /**
   * @param {Iterable<T> | null | undefined} [value]
   */
  constructor(r) {
    super();
    ie(this, fr);
    /** @type {Map<T, Source<boolean>>} */
    ie(this, or, /* @__PURE__ */ new Map());
    ie(this, $e, /* @__PURE__ */ ve(0));
    ie(this, Ct, /* @__PURE__ */ ve(0));
    ie(this, Qr, bt || -1);
    if (r) {
      for (var n of r)
        super.add(n);
      _(this, Ct).v = super.size;
    }
    Zn || We(this, fr, es).call(this);
  }
  /** @param {T} value */
  has(r) {
    var n = super.has(r), o = _(this, or), s = o.get(r);
    if (s === void 0) {
      if (!n)
        return i(_(this, $e)), !1;
      s = We(this, fr, $o).call(this, !0), o.set(r, s);
    }
    return i(s), n;
  }
  /** @param {T} value */
  add(r) {
    return super.has(r) || (super.add(r), re(_(this, Ct), super.size), Tt(_(this, $e))), this;
  }
  /** @param {T} value */
  delete(r) {
    var n = super.delete(r), o = _(this, or), s = o.get(r);
    return s !== void 0 && (o.delete(r), re(s, !1)), n && (re(_(this, Ct), super.size), Tt(_(this, $e))), n;
  }
  clear() {
    if (super.size !== 0) {
      super.clear();
      var r = _(this, or);
      for (var n of r.values())
        re(n, !1);
      r.clear(), re(_(this, Ct), 0), Tt(_(this, $e));
    }
  }
  keys() {
    return this.values();
  }
  values() {
    return i(_(this, $e)), super.values();
  }
  entries() {
    return i(_(this, $e)), super.entries();
  }
  [Symbol.iterator]() {
    return this.keys();
  }
  get size() {
    return i(_(this, Ct));
  }
};
or = new WeakMap(), $e = new WeakMap(), Ct = new WeakMap(), Qr = new WeakMap(), fr = new WeakSet(), /**
 * If the source is being created inside the same reaction as the SvelteSet instance,
 * we use `state` so that it will not be a dependency of the reaction. Otherwise we
 * use `source` so it will be.
 *
 * @template T
 * @param {T} value
 * @returns {Source<T>}
 */
$o = function(r) {
  return bt === _(this, Qr) ? /* @__PURE__ */ ve(r) : Gt(r);
}, // We init as part of the first instance so that we can treeshake this class
es = function() {
  Zn = !0;
  var r = $r.prototype, n = Set.prototype;
  for (const o of qi)
    r[o] = function(...s) {
      return i(_(this, $e)), n[o].apply(this, s);
    };
  for (const o of Yi)
    r[o] = function(...s) {
      i(_(this, $e));
      var a = (
        /** @type {Set<T>} */
        n[o].apply(this, s)
      );
      return new $r(a);
    };
};
let yn = $r;
var Xi = /* @__PURE__ */ G('<p class="line-clamp-2 text-xs text-muted-foreground"> </p>'), Zi = /* @__PURE__ */ G('<div class="mt-3"><div class="mb-1 flex items-center justify-between"><span class="text-[10px] text-muted-foreground uppercase tracking-wide">Progress</span> <span class="text-[10px] tabular-nums text-muted-foreground"> </span></div> <div class="h-1.5 rounded-full bg-muted"><div class="h-1.5 rounded-full bg-emerald-500 transition-all duration-300"></div></div></div>'), Ji = /* @__PURE__ */ G('<p class="mt-3 text-[10px] text-muted-foreground"> </p>'), Qi = /* @__PURE__ */ G('<div class="flex items-center gap-1.5 text-xs"><span class="block h-1.5 w-1.5 shrink-0 rounded-full bg-blue-500"></span> <span class="truncate text-muted-foreground"> </span></div>'), $i = /* @__PURE__ */ G('<span class="text-[10px] text-muted-foreground/60"> </span>'), ea = /* @__PURE__ */ G('<div class="mt-3 border-t border-border/50 pt-2"><p class="mb-1 text-[10px] uppercase tracking-wide text-muted-foreground">Now</p> <div class="flex flex-col gap-1"><!> <!></div></div>'), ta = /* @__PURE__ */ G('<div class="flex items-center gap-1.5 text-xs"><!> <span class="truncate text-muted-foreground"> </span></div>'), ra = /* @__PURE__ */ G('<span class="text-[10px] text-muted-foreground/60"> </span>'), na = /* @__PURE__ */ G('<div class="mt-2 border-t border-border/50 pt-2"><p class="mb-1 text-[10px] uppercase tracking-wide text-muted-foreground">Critical</p> <div class="flex flex-col gap-1"><!> <!></div></div>'), oa = /* @__PURE__ */ G('<button class="group w-full rounded-xl border border-border bg-card p-4 text-left transition-all hover:border-border/80 hover:bg-accent/40 hover:shadow-sm"><div class="flex items-start justify-between gap-3"><div class="flex min-w-0 flex-col gap-1"><span class="truncate text-sm font-semibold leading-tight"> </span> <!></div> <div class="shrink-0"><!></div></div> <!> <!> <!></button>');
function sa(e, t) {
  Bt(t, !0);
  let r = Jt(t, "epicLabel", 3, "Epic");
  const n = /* @__PURE__ */ I(() => `${r().toLowerCase()}s`), o = /* @__PURE__ */ I(() => t.epicCount > 0 ? t.doneEpicCount / t.epicCount * 100 : 0);
  var s = oa(), a = h(s), u = h(a), l = h(u), c = h(l), d = W(l, 2);
  {
    var b = (C) => {
      var B = Xi(), H = h(B);
      X(() => J(H, t.milestone.description)), D(C, B);
    };
    ne(d, (C) => {
      t.milestone.description && C(b);
    });
  }
  var f = W(u, 2), y = h(f);
  {
    let C = /* @__PURE__ */ I(() => t.milestone.status ?? "planning");
    vo(y, {
      get status() {
        return i(C);
      },
      mode: "badge"
    });
  }
  var x = W(a, 2);
  {
    var w = (C) => {
      var B = Zi(), H = h(B), m = W(h(H), 2), k = h(m), N = W(H, 2), M = h(N);
      X(() => {
        J(k, `${t.doneEpicCount ?? ""}/${t.epicCount ?? ""} ${i(n) ?? ""}`), Qo(M, `width: ${i(o) ?? ""}%`);
      }), D(C, B);
    }, v = (C) => {
      var B = Ji(), H = h(B);
      X(() => J(H, `No ${i(n) ?? ""} yet`)), D(C, B);
    };
    ne(x, (C) => {
      t.epicCount > 0 ? C(w) : C(v, -1);
    });
  }
  var P = W(x, 2);
  {
    var V = (C) => {
      var B = ea(), H = W(h(B), 2), m = h(H);
      jt(m, 17, () => t.inProgressEpics.slice(0, 2), (M) => M.id, (M, q) => {
        var S = Qi(), g = W(h(S), 2), A = h(g);
        X(() => J(A, i(q).title)), D(M, S);
      });
      var k = W(m, 2);
      {
        var N = (M) => {
          var q = $i(), S = h(q);
          X(() => J(S, `+${t.inProgressEpics.length - 2} more`)), D(M, q);
        };
        ne(k, (M) => {
          t.inProgressEpics.length > 2 && M(N);
        });
      }
      D(C, B);
    };
    ne(P, (C) => {
      t.inProgressEpics.length > 0 && C(V);
    });
  }
  var O = W(P, 2);
  {
    var Y = (C) => {
      var B = na(), H = W(h(B), 2), m = h(H);
      jt(m, 17, () => t.criticalEpics.slice(0, 2), (M) => M.id, (M, q) => {
        var S = ta(), g = h(S);
        lo(g, {
          variant: "destructive",
          children: (F, K) => {
            var U = nn("P1");
            D(F, U);
          },
          $$slots: { default: !0 }
        });
        var A = W(g, 2), T = h(A);
        X(() => J(T, i(q).title)), D(M, S);
      });
      var k = W(m, 2);
      {
        var N = (M) => {
          var q = ra(), S = h(q);
          X(() => J(S, `+${t.criticalEpics.length - 2} more`)), D(M, q);
        };
        ne(k, (M) => {
          t.criticalEpics.length > 2 && M(N);
        });
      }
      D(C, B);
    };
    ne(O, (C) => {
      t.criticalEpics.length > 0 && C(Y);
    });
  }
  X(() => J(c, t.milestone.title)), _t("click", s, function(...C) {
    var B;
    (B = t.onClick) == null || B.apply(this, C);
  }), D(e, s), Ut();
}
Xe(["click"]);
function ts(e) {
  var t, r, n = "";
  if (typeof e == "string" || typeof e == "number") n += e;
  else if (typeof e == "object") if (Array.isArray(e)) {
    var o = e.length;
    for (t = 0; t < o; t++) e[t] && (r = ts(e[t])) && (n && (n += " "), n += r);
  } else for (r in e) e[r] && (n && (n += " "), n += r);
  return n;
}
function ia() {
  for (var e, t, r = 0, n = "", o = arguments.length; r < o; r++) (e = arguments[r]) && (t = ts(e)) && (n && (n += " "), n += t);
  return n;
}
const aa = (e, t) => {
  const r = new Array(e.length + t.length);
  for (let n = 0; n < e.length; n++)
    r[n] = e[n];
  for (let n = 0; n < t.length; n++)
    r[e.length + n] = t[n];
  return r;
}, la = (e, t) => ({
  classGroupId: e,
  validator: t
}), rs = (e = /* @__PURE__ */ new Map(), t = null, r) => ({
  nextPart: e,
  validators: t,
  classGroupId: r
}), Yr = "-", Jn = [], ca = "arbitrary..", ua = (e) => {
  const t = fa(e), {
    conflictingClassGroups: r,
    conflictingClassGroupModifiers: n
  } = e;
  return {
    getClassGroupId: (a) => {
      if (a.startsWith("[") && a.endsWith("]"))
        return da(a);
      const u = a.split(Yr), l = u[0] === "" && u.length > 1 ? 1 : 0;
      return ns(u, l, t);
    },
    getConflictingClassGroupIds: (a, u) => {
      if (u) {
        const l = n[a], c = r[a];
        return l ? c ? aa(c, l) : l : c || Jn;
      }
      return r[a] || Jn;
    }
  };
}, ns = (e, t, r) => {
  if (e.length - t === 0)
    return r.classGroupId;
  const o = e[t], s = r.nextPart.get(o);
  if (s) {
    const c = ns(e, t + 1, s);
    if (c) return c;
  }
  const a = r.validators;
  if (a === null)
    return;
  const u = t === 0 ? e.join(Yr) : e.slice(t).join(Yr), l = a.length;
  for (let c = 0; c < l; c++) {
    const d = a[c];
    if (d.validator(u))
      return d.classGroupId;
  }
}, da = (e) => e.slice(1, -1).indexOf(":") === -1 ? void 0 : (() => {
  const t = e.slice(1, -1), r = t.indexOf(":"), n = t.slice(0, r);
  return n ? ca + n : void 0;
})(), fa = (e) => {
  const {
    theme: t,
    classGroups: r
  } = e;
  return va(r, t);
}, va = (e, t) => {
  const r = rs();
  for (const n in e) {
    const o = e[n];
    Vn(o, r, n, t);
  }
  return r;
}, Vn = (e, t, r, n) => {
  const o = e.length;
  for (let s = 0; s < o; s++) {
    const a = e[s];
    pa(a, t, r, n);
  }
}, pa = (e, t, r, n) => {
  if (typeof e == "string") {
    ma(e, t, r);
    return;
  }
  if (typeof e == "function") {
    ga(e, t, r, n);
    return;
  }
  ha(e, t, r, n);
}, ma = (e, t, r) => {
  const n = e === "" ? t : os(t, e);
  n.classGroupId = r;
}, ga = (e, t, r, n) => {
  if (ba(e)) {
    Vn(e(n), t, r, n);
    return;
  }
  t.validators === null && (t.validators = []), t.validators.push(la(r, e));
}, ha = (e, t, r, n) => {
  const o = Object.entries(e), s = o.length;
  for (let a = 0; a < s; a++) {
    const [u, l] = o[a];
    Vn(l, os(t, u), r, n);
  }
}, os = (e, t) => {
  let r = e;
  const n = t.split(Yr), o = n.length;
  for (let s = 0; s < o; s++) {
    const a = n[s];
    let u = r.nextPart.get(a);
    u || (u = rs(), r.nextPart.set(a, u)), r = u;
  }
  return r;
}, ba = (e) => "isThemeGetter" in e && e.isThemeGetter === !0, _a = (e) => {
  if (e < 1)
    return {
      get: () => {
      },
      set: () => {
      }
    };
  let t = 0, r = /* @__PURE__ */ Object.create(null), n = /* @__PURE__ */ Object.create(null);
  const o = (s, a) => {
    r[s] = a, t++, t > e && (t = 0, n = r, r = /* @__PURE__ */ Object.create(null));
  };
  return {
    get(s) {
      let a = r[s];
      if (a !== void 0)
        return a;
      if ((a = n[s]) !== void 0)
        return o(s, a), a;
    },
    set(s, a) {
      s in r ? r[s] = a : o(s, a);
    }
  };
}, wn = "!", Qn = ":", xa = [], $n = (e, t, r, n, o) => ({
  modifiers: e,
  hasImportantModifier: t,
  baseClassName: r,
  maybePostfixModifierPosition: n,
  isExternal: o
}), ya = (e) => {
  const {
    prefix: t,
    experimentalParseClassName: r
  } = e;
  let n = (o) => {
    const s = [];
    let a = 0, u = 0, l = 0, c;
    const d = o.length;
    for (let w = 0; w < d; w++) {
      const v = o[w];
      if (a === 0 && u === 0) {
        if (v === Qn) {
          s.push(o.slice(l, w)), l = w + 1;
          continue;
        }
        if (v === "/") {
          c = w;
          continue;
        }
      }
      v === "[" ? a++ : v === "]" ? a-- : v === "(" ? u++ : v === ")" && u--;
    }
    const b = s.length === 0 ? o : o.slice(l);
    let f = b, y = !1;
    b.endsWith(wn) ? (f = b.slice(0, -1), y = !0) : (
      /**
       * In Tailwind CSS v3 the important modifier was at the start of the base class name. This is still supported for legacy reasons.
       * @see https://github.com/dcastil/tailwind-merge/issues/513#issuecomment-2614029864
       */
      b.startsWith(wn) && (f = b.slice(1), y = !0)
    );
    const x = c && c > l ? c - l : void 0;
    return $n(s, y, f, x);
  };
  if (t) {
    const o = t + Qn, s = n;
    n = (a) => a.startsWith(o) ? s(a.slice(o.length)) : $n(xa, !1, a, void 0, !0);
  }
  if (r) {
    const o = n;
    n = (s) => r({
      className: s,
      parseClassName: o
    });
  }
  return n;
}, wa = (e) => {
  const t = /* @__PURE__ */ new Map();
  return e.orderSensitiveModifiers.forEach((r, n) => {
    t.set(r, 1e6 + n);
  }), (r) => {
    const n = [];
    let o = [];
    for (let s = 0; s < r.length; s++) {
      const a = r[s], u = a[0] === "[", l = t.has(a);
      u || l ? (o.length > 0 && (o.sort(), n.push(...o), o = []), n.push(a)) : o.push(a);
    }
    return o.length > 0 && (o.sort(), n.push(...o)), n;
  };
}, ka = (e) => ({
  cache: _a(e.cacheSize),
  parseClassName: ya(e),
  sortModifiers: wa(e),
  ...ua(e)
}), Ca = /\s+/, Ea = (e, t) => {
  const {
    parseClassName: r,
    getClassGroupId: n,
    getConflictingClassGroupIds: o,
    sortModifiers: s
  } = t, a = [], u = e.trim().split(Ca);
  let l = "";
  for (let c = u.length - 1; c >= 0; c -= 1) {
    const d = u[c], {
      isExternal: b,
      modifiers: f,
      hasImportantModifier: y,
      baseClassName: x,
      maybePostfixModifierPosition: w
    } = r(d);
    if (b) {
      l = d + (l.length > 0 ? " " + l : l);
      continue;
    }
    let v = !!w, P = n(v ? x.substring(0, w) : x);
    if (!P) {
      if (!v) {
        l = d + (l.length > 0 ? " " + l : l);
        continue;
      }
      if (P = n(x), !P) {
        l = d + (l.length > 0 ? " " + l : l);
        continue;
      }
      v = !1;
    }
    const V = f.length === 0 ? "" : f.length === 1 ? f[0] : s(f).join(":"), O = y ? V + wn : V, Y = O + P;
    if (a.indexOf(Y) > -1)
      continue;
    a.push(Y);
    const C = o(P, v);
    for (let B = 0; B < C.length; ++B) {
      const H = C[B];
      a.push(O + H);
    }
    l = d + (l.length > 0 ? " " + l : l);
  }
  return l;
}, Sa = (...e) => {
  let t = 0, r, n, o = "";
  for (; t < e.length; )
    (r = e[t++]) && (n = ss(r)) && (o && (o += " "), o += n);
  return o;
}, ss = (e) => {
  if (typeof e == "string")
    return e;
  let t, r = "";
  for (let n = 0; n < e.length; n++)
    e[n] && (t = ss(e[n])) && (r && (r += " "), r += t);
  return r;
}, kn = (e, ...t) => {
  let r, n, o, s;
  const a = (l) => {
    const c = t.reduce((d, b) => b(d), e());
    return r = ka(c), n = r.cache.get, o = r.cache.set, s = u, u(l);
  }, u = (l) => {
    const c = n(l);
    if (c)
      return c;
    const d = Ea(l, r);
    return o(l, d), d;
  };
  return s = a, (...l) => s(Sa(...l));
}, Ta = [], ge = (e) => {
  const t = (r) => r[e] || Ta;
  return t.isThemeGetter = !0, t;
}, is = /^\[(?:(\w[\w-]*):)?(.+)\]$/i, as = /^\((?:(\w[\w-]*):)?(.+)\)$/i, Aa = /^\d+(?:\.\d+)?\/\d+(?:\.\d+)?$/, Ma = /^(\d+(\.\d+)?)?(xs|sm|md|lg|xl)$/, za = /\d+(%|px|r?em|[sdl]?v([hwib]|min|max)|pt|pc|in|cm|mm|cap|ch|ex|r?lh|cq(w|h|i|b|min|max))|\b(calc|min|max|clamp)\(.+\)|^0$/, Pa = /^(rgba?|hsla?|hwb|(ok)?(lab|lch)|color-mix)\(.+\)$/, Ra = /^(inset_)?-?((\d+)?\.?(\d+)[a-z]+|0)_-?((\d+)?\.?(\d+)[a-z]+|0)/, Da = /^(url|image|image-set|cross-fade|element|(repeating-)?(linear|radial|conic)-gradient)\(.+\)$/, wt = (e) => Aa.test(e), Z = (e) => !!e && !Number.isNaN(Number(e)), kt = (e) => !!e && Number.isInteger(Number(e)), dn = (e) => e.endsWith("%") && Z(e.slice(0, -1)), ut = (e) => Ma.test(e), ls = () => !0, Na = (e) => (
  // `colorFunctionRegex` check is necessary because color functions can have percentages in them which which would be incorrectly classified as lengths.
  // For example, `hsl(0 0% 0%)` would be classified as a length without this check.
  // I could also use lookbehind assertion in `lengthUnitRegex` but that isn't supported widely enough.
  za.test(e) && !Pa.test(e)
), Fn = () => !1, La = (e) => Ra.test(e), Ia = (e) => Da.test(e), Oa = (e) => !R(e) && !L(e), Va = (e) => zt(e, ds, Fn), R = (e) => is.test(e), Rt = (e) => zt(e, fs, Na), eo = (e) => zt(e, Ka, Z), Fa = (e) => zt(e, ps, ls), Ga = (e) => zt(e, vs, Fn), to = (e) => zt(e, cs, Fn), ja = (e) => zt(e, us, Ia), Lr = (e) => zt(e, ms, La), L = (e) => as.test(e), br = (e) => Wt(e, fs), Ba = (e) => Wt(e, vs), ro = (e) => Wt(e, cs), Ua = (e) => Wt(e, ds), Ha = (e) => Wt(e, us), Ir = (e) => Wt(e, ms, !0), Wa = (e) => Wt(e, ps, !0), zt = (e, t, r) => {
  const n = is.exec(e);
  return n ? n[1] ? t(n[1]) : r(n[2]) : !1;
}, Wt = (e, t, r = !1) => {
  const n = as.exec(e);
  return n ? n[1] ? t(n[1]) : r : !1;
}, cs = (e) => e === "position" || e === "percentage", us = (e) => e === "image" || e === "url", ds = (e) => e === "length" || e === "size" || e === "bg-size", fs = (e) => e === "length", Ka = (e) => e === "number", vs = (e) => e === "family-name", ps = (e) => e === "number" || e === "weight", ms = (e) => e === "shadow", Cn = () => {
  const e = ge("color"), t = ge("font"), r = ge("text"), n = ge("font-weight"), o = ge("tracking"), s = ge("leading"), a = ge("breakpoint"), u = ge("container"), l = ge("spacing"), c = ge("radius"), d = ge("shadow"), b = ge("inset-shadow"), f = ge("text-shadow"), y = ge("drop-shadow"), x = ge("blur"), w = ge("perspective"), v = ge("aspect"), P = ge("ease"), V = ge("animate"), O = () => ["auto", "avoid", "all", "avoid-page", "page", "left", "right", "column"], Y = () => [
    "center",
    "top",
    "bottom",
    "left",
    "right",
    "top-left",
    // Deprecated since Tailwind CSS v4.1.0, see https://github.com/tailwindlabs/tailwindcss/pull/17378
    "left-top",
    "top-right",
    // Deprecated since Tailwind CSS v4.1.0, see https://github.com/tailwindlabs/tailwindcss/pull/17378
    "right-top",
    "bottom-right",
    // Deprecated since Tailwind CSS v4.1.0, see https://github.com/tailwindlabs/tailwindcss/pull/17378
    "right-bottom",
    "bottom-left",
    // Deprecated since Tailwind CSS v4.1.0, see https://github.com/tailwindlabs/tailwindcss/pull/17378
    "left-bottom"
  ], C = () => [...Y(), L, R], B = () => ["auto", "hidden", "clip", "visible", "scroll"], H = () => ["auto", "contain", "none"], m = () => [L, R, l], k = () => [wt, "full", "auto", ...m()], N = () => [kt, "none", "subgrid", L, R], M = () => ["auto", {
    span: ["full", kt, L, R]
  }, kt, L, R], q = () => [kt, "auto", L, R], S = () => ["auto", "min", "max", "fr", L, R], g = () => ["start", "end", "center", "between", "around", "evenly", "stretch", "baseline", "center-safe", "end-safe"], A = () => ["start", "end", "center", "stretch", "center-safe", "end-safe"], T = () => ["auto", ...m()], F = () => [wt, "auto", "full", "dvw", "dvh", "lvw", "lvh", "svw", "svh", "min", "max", "fit", ...m()], K = () => [wt, "screen", "full", "dvw", "lvw", "svw", "min", "max", "fit", ...m()], U = () => [wt, "screen", "full", "lh", "dvh", "lvh", "svh", "min", "max", "fit", ...m()], E = () => [e, L, R], me = () => [...Y(), ro, to, {
    position: [L, R]
  }], le = () => ["no-repeat", {
    repeat: ["", "x", "y", "space", "round"]
  }], te = () => ["auto", "cover", "contain", Ua, Va, {
    size: [L, R]
  }], de = () => [dn, br, Rt], $ = () => [
    // Deprecated since Tailwind CSS v4.0.0
    "",
    "none",
    "full",
    c,
    L,
    R
  ], se = () => ["", Z, br, Rt], Pe = () => ["solid", "dashed", "dotted", "double"], rt = () => ["normal", "multiply", "screen", "overlay", "darken", "lighten", "color-dodge", "color-burn", "hard-light", "soft-light", "difference", "exclusion", "hue", "saturation", "color", "luminosity"], Q = () => [Z, dn, ro, to], we = () => [
    // Deprecated since Tailwind CSS v4.0.0
    "",
    "none",
    x,
    L,
    R
  ], Ee = () => ["none", Z, L, R], nt = () => ["none", Z, L, R], Ze = () => [Z, L, R], Re = () => [wt, "full", ...m()];
  return {
    cacheSize: 500,
    theme: {
      animate: ["spin", "ping", "pulse", "bounce"],
      aspect: ["video"],
      blur: [ut],
      breakpoint: [ut],
      color: [ls],
      container: [ut],
      "drop-shadow": [ut],
      ease: ["in", "out", "in-out"],
      font: [Oa],
      "font-weight": ["thin", "extralight", "light", "normal", "medium", "semibold", "bold", "extrabold", "black"],
      "inset-shadow": [ut],
      leading: ["none", "tight", "snug", "normal", "relaxed", "loose"],
      perspective: ["dramatic", "near", "normal", "midrange", "distant", "none"],
      radius: [ut],
      shadow: [ut],
      spacing: ["px", Z],
      text: [ut],
      "text-shadow": [ut],
      tracking: ["tighter", "tight", "normal", "wide", "wider", "widest"]
    },
    classGroups: {
      // --------------
      // --- Layout ---
      // --------------
      /**
       * Aspect Ratio
       * @see https://tailwindcss.com/docs/aspect-ratio
       */
      aspect: [{
        aspect: ["auto", "square", wt, R, L, v]
      }],
      /**
       * Container
       * @see https://tailwindcss.com/docs/container
       * @deprecated since Tailwind CSS v4.0.0
       */
      container: ["container"],
      /**
       * Columns
       * @see https://tailwindcss.com/docs/columns
       */
      columns: [{
        columns: [Z, R, L, u]
      }],
      /**
       * Break After
       * @see https://tailwindcss.com/docs/break-after
       */
      "break-after": [{
        "break-after": O()
      }],
      /**
       * Break Before
       * @see https://tailwindcss.com/docs/break-before
       */
      "break-before": [{
        "break-before": O()
      }],
      /**
       * Break Inside
       * @see https://tailwindcss.com/docs/break-inside
       */
      "break-inside": [{
        "break-inside": ["auto", "avoid", "avoid-page", "avoid-column"]
      }],
      /**
       * Box Decoration Break
       * @see https://tailwindcss.com/docs/box-decoration-break
       */
      "box-decoration": [{
        "box-decoration": ["slice", "clone"]
      }],
      /**
       * Box Sizing
       * @see https://tailwindcss.com/docs/box-sizing
       */
      box: [{
        box: ["border", "content"]
      }],
      /**
       * Display
       * @see https://tailwindcss.com/docs/display
       */
      display: ["block", "inline-block", "inline", "flex", "inline-flex", "table", "inline-table", "table-caption", "table-cell", "table-column", "table-column-group", "table-footer-group", "table-header-group", "table-row-group", "table-row", "flow-root", "grid", "inline-grid", "contents", "list-item", "hidden"],
      /**
       * Screen Reader Only
       * @see https://tailwindcss.com/docs/display#screen-reader-only
       */
      sr: ["sr-only", "not-sr-only"],
      /**
       * Floats
       * @see https://tailwindcss.com/docs/float
       */
      float: [{
        float: ["right", "left", "none", "start", "end"]
      }],
      /**
       * Clear
       * @see https://tailwindcss.com/docs/clear
       */
      clear: [{
        clear: ["left", "right", "both", "none", "start", "end"]
      }],
      /**
       * Isolation
       * @see https://tailwindcss.com/docs/isolation
       */
      isolation: ["isolate", "isolation-auto"],
      /**
       * Object Fit
       * @see https://tailwindcss.com/docs/object-fit
       */
      "object-fit": [{
        object: ["contain", "cover", "fill", "none", "scale-down"]
      }],
      /**
       * Object Position
       * @see https://tailwindcss.com/docs/object-position
       */
      "object-position": [{
        object: C()
      }],
      /**
       * Overflow
       * @see https://tailwindcss.com/docs/overflow
       */
      overflow: [{
        overflow: B()
      }],
      /**
       * Overflow X
       * @see https://tailwindcss.com/docs/overflow
       */
      "overflow-x": [{
        "overflow-x": B()
      }],
      /**
       * Overflow Y
       * @see https://tailwindcss.com/docs/overflow
       */
      "overflow-y": [{
        "overflow-y": B()
      }],
      /**
       * Overscroll Behavior
       * @see https://tailwindcss.com/docs/overscroll-behavior
       */
      overscroll: [{
        overscroll: H()
      }],
      /**
       * Overscroll Behavior X
       * @see https://tailwindcss.com/docs/overscroll-behavior
       */
      "overscroll-x": [{
        "overscroll-x": H()
      }],
      /**
       * Overscroll Behavior Y
       * @see https://tailwindcss.com/docs/overscroll-behavior
       */
      "overscroll-y": [{
        "overscroll-y": H()
      }],
      /**
       * Position
       * @see https://tailwindcss.com/docs/position
       */
      position: ["static", "fixed", "absolute", "relative", "sticky"],
      /**
       * Inset
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      inset: [{
        inset: k()
      }],
      /**
       * Inset Inline
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      "inset-x": [{
        "inset-x": k()
      }],
      /**
       * Inset Block
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      "inset-y": [{
        "inset-y": k()
      }],
      /**
       * Inset Inline Start
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       * @todo class group will be renamed to `inset-s` in next major release
       */
      start: [{
        "inset-s": k(),
        /**
         * @deprecated since Tailwind CSS v4.2.0 in favor of `inset-s-*` utilities.
         * @see https://github.com/tailwindlabs/tailwindcss/pull/19613
         */
        start: k()
      }],
      /**
       * Inset Inline End
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       * @todo class group will be renamed to `inset-e` in next major release
       */
      end: [{
        "inset-e": k(),
        /**
         * @deprecated since Tailwind CSS v4.2.0 in favor of `inset-e-*` utilities.
         * @see https://github.com/tailwindlabs/tailwindcss/pull/19613
         */
        end: k()
      }],
      /**
       * Inset Block Start
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      "inset-bs": [{
        "inset-bs": k()
      }],
      /**
       * Inset Block End
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      "inset-be": [{
        "inset-be": k()
      }],
      /**
       * Top
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      top: [{
        top: k()
      }],
      /**
       * Right
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      right: [{
        right: k()
      }],
      /**
       * Bottom
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      bottom: [{
        bottom: k()
      }],
      /**
       * Left
       * @see https://tailwindcss.com/docs/top-right-bottom-left
       */
      left: [{
        left: k()
      }],
      /**
       * Visibility
       * @see https://tailwindcss.com/docs/visibility
       */
      visibility: ["visible", "invisible", "collapse"],
      /**
       * Z-Index
       * @see https://tailwindcss.com/docs/z-index
       */
      z: [{
        z: [kt, "auto", L, R]
      }],
      // ------------------------
      // --- Flexbox and Grid ---
      // ------------------------
      /**
       * Flex Basis
       * @see https://tailwindcss.com/docs/flex-basis
       */
      basis: [{
        basis: [wt, "full", "auto", u, ...m()]
      }],
      /**
       * Flex Direction
       * @see https://tailwindcss.com/docs/flex-direction
       */
      "flex-direction": [{
        flex: ["row", "row-reverse", "col", "col-reverse"]
      }],
      /**
       * Flex Wrap
       * @see https://tailwindcss.com/docs/flex-wrap
       */
      "flex-wrap": [{
        flex: ["nowrap", "wrap", "wrap-reverse"]
      }],
      /**
       * Flex
       * @see https://tailwindcss.com/docs/flex
       */
      flex: [{
        flex: [Z, wt, "auto", "initial", "none", R]
      }],
      /**
       * Flex Grow
       * @see https://tailwindcss.com/docs/flex-grow
       */
      grow: [{
        grow: ["", Z, L, R]
      }],
      /**
       * Flex Shrink
       * @see https://tailwindcss.com/docs/flex-shrink
       */
      shrink: [{
        shrink: ["", Z, L, R]
      }],
      /**
       * Order
       * @see https://tailwindcss.com/docs/order
       */
      order: [{
        order: [kt, "first", "last", "none", L, R]
      }],
      /**
       * Grid Template Columns
       * @see https://tailwindcss.com/docs/grid-template-columns
       */
      "grid-cols": [{
        "grid-cols": N()
      }],
      /**
       * Grid Column Start / End
       * @see https://tailwindcss.com/docs/grid-column
       */
      "col-start-end": [{
        col: M()
      }],
      /**
       * Grid Column Start
       * @see https://tailwindcss.com/docs/grid-column
       */
      "col-start": [{
        "col-start": q()
      }],
      /**
       * Grid Column End
       * @see https://tailwindcss.com/docs/grid-column
       */
      "col-end": [{
        "col-end": q()
      }],
      /**
       * Grid Template Rows
       * @see https://tailwindcss.com/docs/grid-template-rows
       */
      "grid-rows": [{
        "grid-rows": N()
      }],
      /**
       * Grid Row Start / End
       * @see https://tailwindcss.com/docs/grid-row
       */
      "row-start-end": [{
        row: M()
      }],
      /**
       * Grid Row Start
       * @see https://tailwindcss.com/docs/grid-row
       */
      "row-start": [{
        "row-start": q()
      }],
      /**
       * Grid Row End
       * @see https://tailwindcss.com/docs/grid-row
       */
      "row-end": [{
        "row-end": q()
      }],
      /**
       * Grid Auto Flow
       * @see https://tailwindcss.com/docs/grid-auto-flow
       */
      "grid-flow": [{
        "grid-flow": ["row", "col", "dense", "row-dense", "col-dense"]
      }],
      /**
       * Grid Auto Columns
       * @see https://tailwindcss.com/docs/grid-auto-columns
       */
      "auto-cols": [{
        "auto-cols": S()
      }],
      /**
       * Grid Auto Rows
       * @see https://tailwindcss.com/docs/grid-auto-rows
       */
      "auto-rows": [{
        "auto-rows": S()
      }],
      /**
       * Gap
       * @see https://tailwindcss.com/docs/gap
       */
      gap: [{
        gap: m()
      }],
      /**
       * Gap X
       * @see https://tailwindcss.com/docs/gap
       */
      "gap-x": [{
        "gap-x": m()
      }],
      /**
       * Gap Y
       * @see https://tailwindcss.com/docs/gap
       */
      "gap-y": [{
        "gap-y": m()
      }],
      /**
       * Justify Content
       * @see https://tailwindcss.com/docs/justify-content
       */
      "justify-content": [{
        justify: [...g(), "normal"]
      }],
      /**
       * Justify Items
       * @see https://tailwindcss.com/docs/justify-items
       */
      "justify-items": [{
        "justify-items": [...A(), "normal"]
      }],
      /**
       * Justify Self
       * @see https://tailwindcss.com/docs/justify-self
       */
      "justify-self": [{
        "justify-self": ["auto", ...A()]
      }],
      /**
       * Align Content
       * @see https://tailwindcss.com/docs/align-content
       */
      "align-content": [{
        content: ["normal", ...g()]
      }],
      /**
       * Align Items
       * @see https://tailwindcss.com/docs/align-items
       */
      "align-items": [{
        items: [...A(), {
          baseline: ["", "last"]
        }]
      }],
      /**
       * Align Self
       * @see https://tailwindcss.com/docs/align-self
       */
      "align-self": [{
        self: ["auto", ...A(), {
          baseline: ["", "last"]
        }]
      }],
      /**
       * Place Content
       * @see https://tailwindcss.com/docs/place-content
       */
      "place-content": [{
        "place-content": g()
      }],
      /**
       * Place Items
       * @see https://tailwindcss.com/docs/place-items
       */
      "place-items": [{
        "place-items": [...A(), "baseline"]
      }],
      /**
       * Place Self
       * @see https://tailwindcss.com/docs/place-self
       */
      "place-self": [{
        "place-self": ["auto", ...A()]
      }],
      // Spacing
      /**
       * Padding
       * @see https://tailwindcss.com/docs/padding
       */
      p: [{
        p: m()
      }],
      /**
       * Padding Inline
       * @see https://tailwindcss.com/docs/padding
       */
      px: [{
        px: m()
      }],
      /**
       * Padding Block
       * @see https://tailwindcss.com/docs/padding
       */
      py: [{
        py: m()
      }],
      /**
       * Padding Inline Start
       * @see https://tailwindcss.com/docs/padding
       */
      ps: [{
        ps: m()
      }],
      /**
       * Padding Inline End
       * @see https://tailwindcss.com/docs/padding
       */
      pe: [{
        pe: m()
      }],
      /**
       * Padding Block Start
       * @see https://tailwindcss.com/docs/padding
       */
      pbs: [{
        pbs: m()
      }],
      /**
       * Padding Block End
       * @see https://tailwindcss.com/docs/padding
       */
      pbe: [{
        pbe: m()
      }],
      /**
       * Padding Top
       * @see https://tailwindcss.com/docs/padding
       */
      pt: [{
        pt: m()
      }],
      /**
       * Padding Right
       * @see https://tailwindcss.com/docs/padding
       */
      pr: [{
        pr: m()
      }],
      /**
       * Padding Bottom
       * @see https://tailwindcss.com/docs/padding
       */
      pb: [{
        pb: m()
      }],
      /**
       * Padding Left
       * @see https://tailwindcss.com/docs/padding
       */
      pl: [{
        pl: m()
      }],
      /**
       * Margin
       * @see https://tailwindcss.com/docs/margin
       */
      m: [{
        m: T()
      }],
      /**
       * Margin Inline
       * @see https://tailwindcss.com/docs/margin
       */
      mx: [{
        mx: T()
      }],
      /**
       * Margin Block
       * @see https://tailwindcss.com/docs/margin
       */
      my: [{
        my: T()
      }],
      /**
       * Margin Inline Start
       * @see https://tailwindcss.com/docs/margin
       */
      ms: [{
        ms: T()
      }],
      /**
       * Margin Inline End
       * @see https://tailwindcss.com/docs/margin
       */
      me: [{
        me: T()
      }],
      /**
       * Margin Block Start
       * @see https://tailwindcss.com/docs/margin
       */
      mbs: [{
        mbs: T()
      }],
      /**
       * Margin Block End
       * @see https://tailwindcss.com/docs/margin
       */
      mbe: [{
        mbe: T()
      }],
      /**
       * Margin Top
       * @see https://tailwindcss.com/docs/margin
       */
      mt: [{
        mt: T()
      }],
      /**
       * Margin Right
       * @see https://tailwindcss.com/docs/margin
       */
      mr: [{
        mr: T()
      }],
      /**
       * Margin Bottom
       * @see https://tailwindcss.com/docs/margin
       */
      mb: [{
        mb: T()
      }],
      /**
       * Margin Left
       * @see https://tailwindcss.com/docs/margin
       */
      ml: [{
        ml: T()
      }],
      /**
       * Space Between X
       * @see https://tailwindcss.com/docs/margin#adding-space-between-children
       */
      "space-x": [{
        "space-x": m()
      }],
      /**
       * Space Between X Reverse
       * @see https://tailwindcss.com/docs/margin#adding-space-between-children
       */
      "space-x-reverse": ["space-x-reverse"],
      /**
       * Space Between Y
       * @see https://tailwindcss.com/docs/margin#adding-space-between-children
       */
      "space-y": [{
        "space-y": m()
      }],
      /**
       * Space Between Y Reverse
       * @see https://tailwindcss.com/docs/margin#adding-space-between-children
       */
      "space-y-reverse": ["space-y-reverse"],
      // --------------
      // --- Sizing ---
      // --------------
      /**
       * Size
       * @see https://tailwindcss.com/docs/width#setting-both-width-and-height
       */
      size: [{
        size: F()
      }],
      /**
       * Inline Size
       * @see https://tailwindcss.com/docs/width
       */
      "inline-size": [{
        inline: ["auto", ...K()]
      }],
      /**
       * Min-Inline Size
       * @see https://tailwindcss.com/docs/min-width
       */
      "min-inline-size": [{
        "min-inline": ["auto", ...K()]
      }],
      /**
       * Max-Inline Size
       * @see https://tailwindcss.com/docs/max-width
       */
      "max-inline-size": [{
        "max-inline": ["none", ...K()]
      }],
      /**
       * Block Size
       * @see https://tailwindcss.com/docs/height
       */
      "block-size": [{
        block: ["auto", ...U()]
      }],
      /**
       * Min-Block Size
       * @see https://tailwindcss.com/docs/min-height
       */
      "min-block-size": [{
        "min-block": ["auto", ...U()]
      }],
      /**
       * Max-Block Size
       * @see https://tailwindcss.com/docs/max-height
       */
      "max-block-size": [{
        "max-block": ["none", ...U()]
      }],
      /**
       * Width
       * @see https://tailwindcss.com/docs/width
       */
      w: [{
        w: [u, "screen", ...F()]
      }],
      /**
       * Min-Width
       * @see https://tailwindcss.com/docs/min-width
       */
      "min-w": [{
        "min-w": [
          u,
          "screen",
          /** Deprecated. @see https://github.com/tailwindlabs/tailwindcss.com/issues/2027#issuecomment-2620152757 */
          "none",
          ...F()
        ]
      }],
      /**
       * Max-Width
       * @see https://tailwindcss.com/docs/max-width
       */
      "max-w": [{
        "max-w": [
          u,
          "screen",
          "none",
          /** Deprecated since Tailwind CSS v4.0.0. @see https://github.com/tailwindlabs/tailwindcss.com/issues/2027#issuecomment-2620152757 */
          "prose",
          /** Deprecated since Tailwind CSS v4.0.0. @see https://github.com/tailwindlabs/tailwindcss.com/issues/2027#issuecomment-2620152757 */
          {
            screen: [a]
          },
          ...F()
        ]
      }],
      /**
       * Height
       * @see https://tailwindcss.com/docs/height
       */
      h: [{
        h: ["screen", "lh", ...F()]
      }],
      /**
       * Min-Height
       * @see https://tailwindcss.com/docs/min-height
       */
      "min-h": [{
        "min-h": ["screen", "lh", "none", ...F()]
      }],
      /**
       * Max-Height
       * @see https://tailwindcss.com/docs/max-height
       */
      "max-h": [{
        "max-h": ["screen", "lh", ...F()]
      }],
      // ------------------
      // --- Typography ---
      // ------------------
      /**
       * Font Size
       * @see https://tailwindcss.com/docs/font-size
       */
      "font-size": [{
        text: ["base", r, br, Rt]
      }],
      /**
       * Font Smoothing
       * @see https://tailwindcss.com/docs/font-smoothing
       */
      "font-smoothing": ["antialiased", "subpixel-antialiased"],
      /**
       * Font Style
       * @see https://tailwindcss.com/docs/font-style
       */
      "font-style": ["italic", "not-italic"],
      /**
       * Font Weight
       * @see https://tailwindcss.com/docs/font-weight
       */
      "font-weight": [{
        font: [n, Wa, Fa]
      }],
      /**
       * Font Stretch
       * @see https://tailwindcss.com/docs/font-stretch
       */
      "font-stretch": [{
        "font-stretch": ["ultra-condensed", "extra-condensed", "condensed", "semi-condensed", "normal", "semi-expanded", "expanded", "extra-expanded", "ultra-expanded", dn, R]
      }],
      /**
       * Font Family
       * @see https://tailwindcss.com/docs/font-family
       */
      "font-family": [{
        font: [Ba, Ga, t]
      }],
      /**
       * Font Feature Settings
       * @see https://tailwindcss.com/docs/font-feature-settings
       */
      "font-features": [{
        "font-features": [R]
      }],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-normal": ["normal-nums"],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-ordinal": ["ordinal"],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-slashed-zero": ["slashed-zero"],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-figure": ["lining-nums", "oldstyle-nums"],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-spacing": ["proportional-nums", "tabular-nums"],
      /**
       * Font Variant Numeric
       * @see https://tailwindcss.com/docs/font-variant-numeric
       */
      "fvn-fraction": ["diagonal-fractions", "stacked-fractions"],
      /**
       * Letter Spacing
       * @see https://tailwindcss.com/docs/letter-spacing
       */
      tracking: [{
        tracking: [o, L, R]
      }],
      /**
       * Line Clamp
       * @see https://tailwindcss.com/docs/line-clamp
       */
      "line-clamp": [{
        "line-clamp": [Z, "none", L, eo]
      }],
      /**
       * Line Height
       * @see https://tailwindcss.com/docs/line-height
       */
      leading: [{
        leading: [
          /** Deprecated since Tailwind CSS v4.0.0. @see https://github.com/tailwindlabs/tailwindcss.com/issues/2027#issuecomment-2620152757 */
          s,
          ...m()
        ]
      }],
      /**
       * List Style Image
       * @see https://tailwindcss.com/docs/list-style-image
       */
      "list-image": [{
        "list-image": ["none", L, R]
      }],
      /**
       * List Style Position
       * @see https://tailwindcss.com/docs/list-style-position
       */
      "list-style-position": [{
        list: ["inside", "outside"]
      }],
      /**
       * List Style Type
       * @see https://tailwindcss.com/docs/list-style-type
       */
      "list-style-type": [{
        list: ["disc", "decimal", "none", L, R]
      }],
      /**
       * Text Alignment
       * @see https://tailwindcss.com/docs/text-align
       */
      "text-alignment": [{
        text: ["left", "center", "right", "justify", "start", "end"]
      }],
      /**
       * Placeholder Color
       * @deprecated since Tailwind CSS v3.0.0
       * @see https://v3.tailwindcss.com/docs/placeholder-color
       */
      "placeholder-color": [{
        placeholder: E()
      }],
      /**
       * Text Color
       * @see https://tailwindcss.com/docs/text-color
       */
      "text-color": [{
        text: E()
      }],
      /**
       * Text Decoration
       * @see https://tailwindcss.com/docs/text-decoration
       */
      "text-decoration": ["underline", "overline", "line-through", "no-underline"],
      /**
       * Text Decoration Style
       * @see https://tailwindcss.com/docs/text-decoration-style
       */
      "text-decoration-style": [{
        decoration: [...Pe(), "wavy"]
      }],
      /**
       * Text Decoration Thickness
       * @see https://tailwindcss.com/docs/text-decoration-thickness
       */
      "text-decoration-thickness": [{
        decoration: [Z, "from-font", "auto", L, Rt]
      }],
      /**
       * Text Decoration Color
       * @see https://tailwindcss.com/docs/text-decoration-color
       */
      "text-decoration-color": [{
        decoration: E()
      }],
      /**
       * Text Underline Offset
       * @see https://tailwindcss.com/docs/text-underline-offset
       */
      "underline-offset": [{
        "underline-offset": [Z, "auto", L, R]
      }],
      /**
       * Text Transform
       * @see https://tailwindcss.com/docs/text-transform
       */
      "text-transform": ["uppercase", "lowercase", "capitalize", "normal-case"],
      /**
       * Text Overflow
       * @see https://tailwindcss.com/docs/text-overflow
       */
      "text-overflow": ["truncate", "text-ellipsis", "text-clip"],
      /**
       * Text Wrap
       * @see https://tailwindcss.com/docs/text-wrap
       */
      "text-wrap": [{
        text: ["wrap", "nowrap", "balance", "pretty"]
      }],
      /**
       * Text Indent
       * @see https://tailwindcss.com/docs/text-indent
       */
      indent: [{
        indent: m()
      }],
      /**
       * Vertical Alignment
       * @see https://tailwindcss.com/docs/vertical-align
       */
      "vertical-align": [{
        align: ["baseline", "top", "middle", "bottom", "text-top", "text-bottom", "sub", "super", L, R]
      }],
      /**
       * Whitespace
       * @see https://tailwindcss.com/docs/whitespace
       */
      whitespace: [{
        whitespace: ["normal", "nowrap", "pre", "pre-line", "pre-wrap", "break-spaces"]
      }],
      /**
       * Word Break
       * @see https://tailwindcss.com/docs/word-break
       */
      break: [{
        break: ["normal", "words", "all", "keep"]
      }],
      /**
       * Overflow Wrap
       * @see https://tailwindcss.com/docs/overflow-wrap
       */
      wrap: [{
        wrap: ["break-word", "anywhere", "normal"]
      }],
      /**
       * Hyphens
       * @see https://tailwindcss.com/docs/hyphens
       */
      hyphens: [{
        hyphens: ["none", "manual", "auto"]
      }],
      /**
       * Content
       * @see https://tailwindcss.com/docs/content
       */
      content: [{
        content: ["none", L, R]
      }],
      // -------------------
      // --- Backgrounds ---
      // -------------------
      /**
       * Background Attachment
       * @see https://tailwindcss.com/docs/background-attachment
       */
      "bg-attachment": [{
        bg: ["fixed", "local", "scroll"]
      }],
      /**
       * Background Clip
       * @see https://tailwindcss.com/docs/background-clip
       */
      "bg-clip": [{
        "bg-clip": ["border", "padding", "content", "text"]
      }],
      /**
       * Background Origin
       * @see https://tailwindcss.com/docs/background-origin
       */
      "bg-origin": [{
        "bg-origin": ["border", "padding", "content"]
      }],
      /**
       * Background Position
       * @see https://tailwindcss.com/docs/background-position
       */
      "bg-position": [{
        bg: me()
      }],
      /**
       * Background Repeat
       * @see https://tailwindcss.com/docs/background-repeat
       */
      "bg-repeat": [{
        bg: le()
      }],
      /**
       * Background Size
       * @see https://tailwindcss.com/docs/background-size
       */
      "bg-size": [{
        bg: te()
      }],
      /**
       * Background Image
       * @see https://tailwindcss.com/docs/background-image
       */
      "bg-image": [{
        bg: ["none", {
          linear: [{
            to: ["t", "tr", "r", "br", "b", "bl", "l", "tl"]
          }, kt, L, R],
          radial: ["", L, R],
          conic: [kt, L, R]
        }, Ha, ja]
      }],
      /**
       * Background Color
       * @see https://tailwindcss.com/docs/background-color
       */
      "bg-color": [{
        bg: E()
      }],
      /**
       * Gradient Color Stops From Position
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-from-pos": [{
        from: de()
      }],
      /**
       * Gradient Color Stops Via Position
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-via-pos": [{
        via: de()
      }],
      /**
       * Gradient Color Stops To Position
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-to-pos": [{
        to: de()
      }],
      /**
       * Gradient Color Stops From
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-from": [{
        from: E()
      }],
      /**
       * Gradient Color Stops Via
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-via": [{
        via: E()
      }],
      /**
       * Gradient Color Stops To
       * @see https://tailwindcss.com/docs/gradient-color-stops
       */
      "gradient-to": [{
        to: E()
      }],
      // ---------------
      // --- Borders ---
      // ---------------
      /**
       * Border Radius
       * @see https://tailwindcss.com/docs/border-radius
       */
      rounded: [{
        rounded: $()
      }],
      /**
       * Border Radius Start
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-s": [{
        "rounded-s": $()
      }],
      /**
       * Border Radius End
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-e": [{
        "rounded-e": $()
      }],
      /**
       * Border Radius Top
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-t": [{
        "rounded-t": $()
      }],
      /**
       * Border Radius Right
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-r": [{
        "rounded-r": $()
      }],
      /**
       * Border Radius Bottom
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-b": [{
        "rounded-b": $()
      }],
      /**
       * Border Radius Left
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-l": [{
        "rounded-l": $()
      }],
      /**
       * Border Radius Start Start
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-ss": [{
        "rounded-ss": $()
      }],
      /**
       * Border Radius Start End
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-se": [{
        "rounded-se": $()
      }],
      /**
       * Border Radius End End
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-ee": [{
        "rounded-ee": $()
      }],
      /**
       * Border Radius End Start
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-es": [{
        "rounded-es": $()
      }],
      /**
       * Border Radius Top Left
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-tl": [{
        "rounded-tl": $()
      }],
      /**
       * Border Radius Top Right
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-tr": [{
        "rounded-tr": $()
      }],
      /**
       * Border Radius Bottom Right
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-br": [{
        "rounded-br": $()
      }],
      /**
       * Border Radius Bottom Left
       * @see https://tailwindcss.com/docs/border-radius
       */
      "rounded-bl": [{
        "rounded-bl": $()
      }],
      /**
       * Border Width
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w": [{
        border: se()
      }],
      /**
       * Border Width Inline
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-x": [{
        "border-x": se()
      }],
      /**
       * Border Width Block
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-y": [{
        "border-y": se()
      }],
      /**
       * Border Width Inline Start
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-s": [{
        "border-s": se()
      }],
      /**
       * Border Width Inline End
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-e": [{
        "border-e": se()
      }],
      /**
       * Border Width Block Start
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-bs": [{
        "border-bs": se()
      }],
      /**
       * Border Width Block End
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-be": [{
        "border-be": se()
      }],
      /**
       * Border Width Top
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-t": [{
        "border-t": se()
      }],
      /**
       * Border Width Right
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-r": [{
        "border-r": se()
      }],
      /**
       * Border Width Bottom
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-b": [{
        "border-b": se()
      }],
      /**
       * Border Width Left
       * @see https://tailwindcss.com/docs/border-width
       */
      "border-w-l": [{
        "border-l": se()
      }],
      /**
       * Divide Width X
       * @see https://tailwindcss.com/docs/border-width#between-children
       */
      "divide-x": [{
        "divide-x": se()
      }],
      /**
       * Divide Width X Reverse
       * @see https://tailwindcss.com/docs/border-width#between-children
       */
      "divide-x-reverse": ["divide-x-reverse"],
      /**
       * Divide Width Y
       * @see https://tailwindcss.com/docs/border-width#between-children
       */
      "divide-y": [{
        "divide-y": se()
      }],
      /**
       * Divide Width Y Reverse
       * @see https://tailwindcss.com/docs/border-width#between-children
       */
      "divide-y-reverse": ["divide-y-reverse"],
      /**
       * Border Style
       * @see https://tailwindcss.com/docs/border-style
       */
      "border-style": [{
        border: [...Pe(), "hidden", "none"]
      }],
      /**
       * Divide Style
       * @see https://tailwindcss.com/docs/border-style#setting-the-divider-style
       */
      "divide-style": [{
        divide: [...Pe(), "hidden", "none"]
      }],
      /**
       * Border Color
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color": [{
        border: E()
      }],
      /**
       * Border Color Inline
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-x": [{
        "border-x": E()
      }],
      /**
       * Border Color Block
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-y": [{
        "border-y": E()
      }],
      /**
       * Border Color Inline Start
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-s": [{
        "border-s": E()
      }],
      /**
       * Border Color Inline End
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-e": [{
        "border-e": E()
      }],
      /**
       * Border Color Block Start
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-bs": [{
        "border-bs": E()
      }],
      /**
       * Border Color Block End
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-be": [{
        "border-be": E()
      }],
      /**
       * Border Color Top
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-t": [{
        "border-t": E()
      }],
      /**
       * Border Color Right
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-r": [{
        "border-r": E()
      }],
      /**
       * Border Color Bottom
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-b": [{
        "border-b": E()
      }],
      /**
       * Border Color Left
       * @see https://tailwindcss.com/docs/border-color
       */
      "border-color-l": [{
        "border-l": E()
      }],
      /**
       * Divide Color
       * @see https://tailwindcss.com/docs/divide-color
       */
      "divide-color": [{
        divide: E()
      }],
      /**
       * Outline Style
       * @see https://tailwindcss.com/docs/outline-style
       */
      "outline-style": [{
        outline: [...Pe(), "none", "hidden"]
      }],
      /**
       * Outline Offset
       * @see https://tailwindcss.com/docs/outline-offset
       */
      "outline-offset": [{
        "outline-offset": [Z, L, R]
      }],
      /**
       * Outline Width
       * @see https://tailwindcss.com/docs/outline-width
       */
      "outline-w": [{
        outline: ["", Z, br, Rt]
      }],
      /**
       * Outline Color
       * @see https://tailwindcss.com/docs/outline-color
       */
      "outline-color": [{
        outline: E()
      }],
      // ---------------
      // --- Effects ---
      // ---------------
      /**
       * Box Shadow
       * @see https://tailwindcss.com/docs/box-shadow
       */
      shadow: [{
        shadow: [
          // Deprecated since Tailwind CSS v4.0.0
          "",
          "none",
          d,
          Ir,
          Lr
        ]
      }],
      /**
       * Box Shadow Color
       * @see https://tailwindcss.com/docs/box-shadow#setting-the-shadow-color
       */
      "shadow-color": [{
        shadow: E()
      }],
      /**
       * Inset Box Shadow
       * @see https://tailwindcss.com/docs/box-shadow#adding-an-inset-shadow
       */
      "inset-shadow": [{
        "inset-shadow": ["none", b, Ir, Lr]
      }],
      /**
       * Inset Box Shadow Color
       * @see https://tailwindcss.com/docs/box-shadow#setting-the-inset-shadow-color
       */
      "inset-shadow-color": [{
        "inset-shadow": E()
      }],
      /**
       * Ring Width
       * @see https://tailwindcss.com/docs/box-shadow#adding-a-ring
       */
      "ring-w": [{
        ring: se()
      }],
      /**
       * Ring Width Inset
       * @see https://v3.tailwindcss.com/docs/ring-width#inset-rings
       * @deprecated since Tailwind CSS v4.0.0
       * @see https://github.com/tailwindlabs/tailwindcss/blob/v4.0.0/packages/tailwindcss/src/utilities.ts#L4158
       */
      "ring-w-inset": ["ring-inset"],
      /**
       * Ring Color
       * @see https://tailwindcss.com/docs/box-shadow#setting-the-ring-color
       */
      "ring-color": [{
        ring: E()
      }],
      /**
       * Ring Offset Width
       * @see https://v3.tailwindcss.com/docs/ring-offset-width
       * @deprecated since Tailwind CSS v4.0.0
       * @see https://github.com/tailwindlabs/tailwindcss/blob/v4.0.0/packages/tailwindcss/src/utilities.ts#L4158
       */
      "ring-offset-w": [{
        "ring-offset": [Z, Rt]
      }],
      /**
       * Ring Offset Color
       * @see https://v3.tailwindcss.com/docs/ring-offset-color
       * @deprecated since Tailwind CSS v4.0.0
       * @see https://github.com/tailwindlabs/tailwindcss/blob/v4.0.0/packages/tailwindcss/src/utilities.ts#L4158
       */
      "ring-offset-color": [{
        "ring-offset": E()
      }],
      /**
       * Inset Ring Width
       * @see https://tailwindcss.com/docs/box-shadow#adding-an-inset-ring
       */
      "inset-ring-w": [{
        "inset-ring": se()
      }],
      /**
       * Inset Ring Color
       * @see https://tailwindcss.com/docs/box-shadow#setting-the-inset-ring-color
       */
      "inset-ring-color": [{
        "inset-ring": E()
      }],
      /**
       * Text Shadow
       * @see https://tailwindcss.com/docs/text-shadow
       */
      "text-shadow": [{
        "text-shadow": ["none", f, Ir, Lr]
      }],
      /**
       * Text Shadow Color
       * @see https://tailwindcss.com/docs/text-shadow#setting-the-shadow-color
       */
      "text-shadow-color": [{
        "text-shadow": E()
      }],
      /**
       * Opacity
       * @see https://tailwindcss.com/docs/opacity
       */
      opacity: [{
        opacity: [Z, L, R]
      }],
      /**
       * Mix Blend Mode
       * @see https://tailwindcss.com/docs/mix-blend-mode
       */
      "mix-blend": [{
        "mix-blend": [...rt(), "plus-darker", "plus-lighter"]
      }],
      /**
       * Background Blend Mode
       * @see https://tailwindcss.com/docs/background-blend-mode
       */
      "bg-blend": [{
        "bg-blend": rt()
      }],
      /**
       * Mask Clip
       * @see https://tailwindcss.com/docs/mask-clip
       */
      "mask-clip": [{
        "mask-clip": ["border", "padding", "content", "fill", "stroke", "view"]
      }, "mask-no-clip"],
      /**
       * Mask Composite
       * @see https://tailwindcss.com/docs/mask-composite
       */
      "mask-composite": [{
        mask: ["add", "subtract", "intersect", "exclude"]
      }],
      /**
       * Mask Image
       * @see https://tailwindcss.com/docs/mask-image
       */
      "mask-image-linear-pos": [{
        "mask-linear": [Z]
      }],
      "mask-image-linear-from-pos": [{
        "mask-linear-from": Q()
      }],
      "mask-image-linear-to-pos": [{
        "mask-linear-to": Q()
      }],
      "mask-image-linear-from-color": [{
        "mask-linear-from": E()
      }],
      "mask-image-linear-to-color": [{
        "mask-linear-to": E()
      }],
      "mask-image-t-from-pos": [{
        "mask-t-from": Q()
      }],
      "mask-image-t-to-pos": [{
        "mask-t-to": Q()
      }],
      "mask-image-t-from-color": [{
        "mask-t-from": E()
      }],
      "mask-image-t-to-color": [{
        "mask-t-to": E()
      }],
      "mask-image-r-from-pos": [{
        "mask-r-from": Q()
      }],
      "mask-image-r-to-pos": [{
        "mask-r-to": Q()
      }],
      "mask-image-r-from-color": [{
        "mask-r-from": E()
      }],
      "mask-image-r-to-color": [{
        "mask-r-to": E()
      }],
      "mask-image-b-from-pos": [{
        "mask-b-from": Q()
      }],
      "mask-image-b-to-pos": [{
        "mask-b-to": Q()
      }],
      "mask-image-b-from-color": [{
        "mask-b-from": E()
      }],
      "mask-image-b-to-color": [{
        "mask-b-to": E()
      }],
      "mask-image-l-from-pos": [{
        "mask-l-from": Q()
      }],
      "mask-image-l-to-pos": [{
        "mask-l-to": Q()
      }],
      "mask-image-l-from-color": [{
        "mask-l-from": E()
      }],
      "mask-image-l-to-color": [{
        "mask-l-to": E()
      }],
      "mask-image-x-from-pos": [{
        "mask-x-from": Q()
      }],
      "mask-image-x-to-pos": [{
        "mask-x-to": Q()
      }],
      "mask-image-x-from-color": [{
        "mask-x-from": E()
      }],
      "mask-image-x-to-color": [{
        "mask-x-to": E()
      }],
      "mask-image-y-from-pos": [{
        "mask-y-from": Q()
      }],
      "mask-image-y-to-pos": [{
        "mask-y-to": Q()
      }],
      "mask-image-y-from-color": [{
        "mask-y-from": E()
      }],
      "mask-image-y-to-color": [{
        "mask-y-to": E()
      }],
      "mask-image-radial": [{
        "mask-radial": [L, R]
      }],
      "mask-image-radial-from-pos": [{
        "mask-radial-from": Q()
      }],
      "mask-image-radial-to-pos": [{
        "mask-radial-to": Q()
      }],
      "mask-image-radial-from-color": [{
        "mask-radial-from": E()
      }],
      "mask-image-radial-to-color": [{
        "mask-radial-to": E()
      }],
      "mask-image-radial-shape": [{
        "mask-radial": ["circle", "ellipse"]
      }],
      "mask-image-radial-size": [{
        "mask-radial": [{
          closest: ["side", "corner"],
          farthest: ["side", "corner"]
        }]
      }],
      "mask-image-radial-pos": [{
        "mask-radial-at": Y()
      }],
      "mask-image-conic-pos": [{
        "mask-conic": [Z]
      }],
      "mask-image-conic-from-pos": [{
        "mask-conic-from": Q()
      }],
      "mask-image-conic-to-pos": [{
        "mask-conic-to": Q()
      }],
      "mask-image-conic-from-color": [{
        "mask-conic-from": E()
      }],
      "mask-image-conic-to-color": [{
        "mask-conic-to": E()
      }],
      /**
       * Mask Mode
       * @see https://tailwindcss.com/docs/mask-mode
       */
      "mask-mode": [{
        mask: ["alpha", "luminance", "match"]
      }],
      /**
       * Mask Origin
       * @see https://tailwindcss.com/docs/mask-origin
       */
      "mask-origin": [{
        "mask-origin": ["border", "padding", "content", "fill", "stroke", "view"]
      }],
      /**
       * Mask Position
       * @see https://tailwindcss.com/docs/mask-position
       */
      "mask-position": [{
        mask: me()
      }],
      /**
       * Mask Repeat
       * @see https://tailwindcss.com/docs/mask-repeat
       */
      "mask-repeat": [{
        mask: le()
      }],
      /**
       * Mask Size
       * @see https://tailwindcss.com/docs/mask-size
       */
      "mask-size": [{
        mask: te()
      }],
      /**
       * Mask Type
       * @see https://tailwindcss.com/docs/mask-type
       */
      "mask-type": [{
        "mask-type": ["alpha", "luminance"]
      }],
      /**
       * Mask Image
       * @see https://tailwindcss.com/docs/mask-image
       */
      "mask-image": [{
        mask: ["none", L, R]
      }],
      // ---------------
      // --- Filters ---
      // ---------------
      /**
       * Filter
       * @see https://tailwindcss.com/docs/filter
       */
      filter: [{
        filter: [
          // Deprecated since Tailwind CSS v3.0.0
          "",
          "none",
          L,
          R
        ]
      }],
      /**
       * Blur
       * @see https://tailwindcss.com/docs/blur
       */
      blur: [{
        blur: we()
      }],
      /**
       * Brightness
       * @see https://tailwindcss.com/docs/brightness
       */
      brightness: [{
        brightness: [Z, L, R]
      }],
      /**
       * Contrast
       * @see https://tailwindcss.com/docs/contrast
       */
      contrast: [{
        contrast: [Z, L, R]
      }],
      /**
       * Drop Shadow
       * @see https://tailwindcss.com/docs/drop-shadow
       */
      "drop-shadow": [{
        "drop-shadow": [
          // Deprecated since Tailwind CSS v4.0.0
          "",
          "none",
          y,
          Ir,
          Lr
        ]
      }],
      /**
       * Drop Shadow Color
       * @see https://tailwindcss.com/docs/filter-drop-shadow#setting-the-shadow-color
       */
      "drop-shadow-color": [{
        "drop-shadow": E()
      }],
      /**
       * Grayscale
       * @see https://tailwindcss.com/docs/grayscale
       */
      grayscale: [{
        grayscale: ["", Z, L, R]
      }],
      /**
       * Hue Rotate
       * @see https://tailwindcss.com/docs/hue-rotate
       */
      "hue-rotate": [{
        "hue-rotate": [Z, L, R]
      }],
      /**
       * Invert
       * @see https://tailwindcss.com/docs/invert
       */
      invert: [{
        invert: ["", Z, L, R]
      }],
      /**
       * Saturate
       * @see https://tailwindcss.com/docs/saturate
       */
      saturate: [{
        saturate: [Z, L, R]
      }],
      /**
       * Sepia
       * @see https://tailwindcss.com/docs/sepia
       */
      sepia: [{
        sepia: ["", Z, L, R]
      }],
      /**
       * Backdrop Filter
       * @see https://tailwindcss.com/docs/backdrop-filter
       */
      "backdrop-filter": [{
        "backdrop-filter": [
          // Deprecated since Tailwind CSS v3.0.0
          "",
          "none",
          L,
          R
        ]
      }],
      /**
       * Backdrop Blur
       * @see https://tailwindcss.com/docs/backdrop-blur
       */
      "backdrop-blur": [{
        "backdrop-blur": we()
      }],
      /**
       * Backdrop Brightness
       * @see https://tailwindcss.com/docs/backdrop-brightness
       */
      "backdrop-brightness": [{
        "backdrop-brightness": [Z, L, R]
      }],
      /**
       * Backdrop Contrast
       * @see https://tailwindcss.com/docs/backdrop-contrast
       */
      "backdrop-contrast": [{
        "backdrop-contrast": [Z, L, R]
      }],
      /**
       * Backdrop Grayscale
       * @see https://tailwindcss.com/docs/backdrop-grayscale
       */
      "backdrop-grayscale": [{
        "backdrop-grayscale": ["", Z, L, R]
      }],
      /**
       * Backdrop Hue Rotate
       * @see https://tailwindcss.com/docs/backdrop-hue-rotate
       */
      "backdrop-hue-rotate": [{
        "backdrop-hue-rotate": [Z, L, R]
      }],
      /**
       * Backdrop Invert
       * @see https://tailwindcss.com/docs/backdrop-invert
       */
      "backdrop-invert": [{
        "backdrop-invert": ["", Z, L, R]
      }],
      /**
       * Backdrop Opacity
       * @see https://tailwindcss.com/docs/backdrop-opacity
       */
      "backdrop-opacity": [{
        "backdrop-opacity": [Z, L, R]
      }],
      /**
       * Backdrop Saturate
       * @see https://tailwindcss.com/docs/backdrop-saturate
       */
      "backdrop-saturate": [{
        "backdrop-saturate": [Z, L, R]
      }],
      /**
       * Backdrop Sepia
       * @see https://tailwindcss.com/docs/backdrop-sepia
       */
      "backdrop-sepia": [{
        "backdrop-sepia": ["", Z, L, R]
      }],
      // --------------
      // --- Tables ---
      // --------------
      /**
       * Border Collapse
       * @see https://tailwindcss.com/docs/border-collapse
       */
      "border-collapse": [{
        border: ["collapse", "separate"]
      }],
      /**
       * Border Spacing
       * @see https://tailwindcss.com/docs/border-spacing
       */
      "border-spacing": [{
        "border-spacing": m()
      }],
      /**
       * Border Spacing X
       * @see https://tailwindcss.com/docs/border-spacing
       */
      "border-spacing-x": [{
        "border-spacing-x": m()
      }],
      /**
       * Border Spacing Y
       * @see https://tailwindcss.com/docs/border-spacing
       */
      "border-spacing-y": [{
        "border-spacing-y": m()
      }],
      /**
       * Table Layout
       * @see https://tailwindcss.com/docs/table-layout
       */
      "table-layout": [{
        table: ["auto", "fixed"]
      }],
      /**
       * Caption Side
       * @see https://tailwindcss.com/docs/caption-side
       */
      caption: [{
        caption: ["top", "bottom"]
      }],
      // ---------------------------------
      // --- Transitions and Animation ---
      // ---------------------------------
      /**
       * Transition Property
       * @see https://tailwindcss.com/docs/transition-property
       */
      transition: [{
        transition: ["", "all", "colors", "opacity", "shadow", "transform", "none", L, R]
      }],
      /**
       * Transition Behavior
       * @see https://tailwindcss.com/docs/transition-behavior
       */
      "transition-behavior": [{
        transition: ["normal", "discrete"]
      }],
      /**
       * Transition Duration
       * @see https://tailwindcss.com/docs/transition-duration
       */
      duration: [{
        duration: [Z, "initial", L, R]
      }],
      /**
       * Transition Timing Function
       * @see https://tailwindcss.com/docs/transition-timing-function
       */
      ease: [{
        ease: ["linear", "initial", P, L, R]
      }],
      /**
       * Transition Delay
       * @see https://tailwindcss.com/docs/transition-delay
       */
      delay: [{
        delay: [Z, L, R]
      }],
      /**
       * Animation
       * @see https://tailwindcss.com/docs/animation
       */
      animate: [{
        animate: ["none", V, L, R]
      }],
      // ------------------
      // --- Transforms ---
      // ------------------
      /**
       * Backface Visibility
       * @see https://tailwindcss.com/docs/backface-visibility
       */
      backface: [{
        backface: ["hidden", "visible"]
      }],
      /**
       * Perspective
       * @see https://tailwindcss.com/docs/perspective
       */
      perspective: [{
        perspective: [w, L, R]
      }],
      /**
       * Perspective Origin
       * @see https://tailwindcss.com/docs/perspective-origin
       */
      "perspective-origin": [{
        "perspective-origin": C()
      }],
      /**
       * Rotate
       * @see https://tailwindcss.com/docs/rotate
       */
      rotate: [{
        rotate: Ee()
      }],
      /**
       * Rotate X
       * @see https://tailwindcss.com/docs/rotate
       */
      "rotate-x": [{
        "rotate-x": Ee()
      }],
      /**
       * Rotate Y
       * @see https://tailwindcss.com/docs/rotate
       */
      "rotate-y": [{
        "rotate-y": Ee()
      }],
      /**
       * Rotate Z
       * @see https://tailwindcss.com/docs/rotate
       */
      "rotate-z": [{
        "rotate-z": Ee()
      }],
      /**
       * Scale
       * @see https://tailwindcss.com/docs/scale
       */
      scale: [{
        scale: nt()
      }],
      /**
       * Scale X
       * @see https://tailwindcss.com/docs/scale
       */
      "scale-x": [{
        "scale-x": nt()
      }],
      /**
       * Scale Y
       * @see https://tailwindcss.com/docs/scale
       */
      "scale-y": [{
        "scale-y": nt()
      }],
      /**
       * Scale Z
       * @see https://tailwindcss.com/docs/scale
       */
      "scale-z": [{
        "scale-z": nt()
      }],
      /**
       * Scale 3D
       * @see https://tailwindcss.com/docs/scale
       */
      "scale-3d": ["scale-3d"],
      /**
       * Skew
       * @see https://tailwindcss.com/docs/skew
       */
      skew: [{
        skew: Ze()
      }],
      /**
       * Skew X
       * @see https://tailwindcss.com/docs/skew
       */
      "skew-x": [{
        "skew-x": Ze()
      }],
      /**
       * Skew Y
       * @see https://tailwindcss.com/docs/skew
       */
      "skew-y": [{
        "skew-y": Ze()
      }],
      /**
       * Transform
       * @see https://tailwindcss.com/docs/transform
       */
      transform: [{
        transform: [L, R, "", "none", "gpu", "cpu"]
      }],
      /**
       * Transform Origin
       * @see https://tailwindcss.com/docs/transform-origin
       */
      "transform-origin": [{
        origin: C()
      }],
      /**
       * Transform Style
       * @see https://tailwindcss.com/docs/transform-style
       */
      "transform-style": [{
        transform: ["3d", "flat"]
      }],
      /**
       * Translate
       * @see https://tailwindcss.com/docs/translate
       */
      translate: [{
        translate: Re()
      }],
      /**
       * Translate X
       * @see https://tailwindcss.com/docs/translate
       */
      "translate-x": [{
        "translate-x": Re()
      }],
      /**
       * Translate Y
       * @see https://tailwindcss.com/docs/translate
       */
      "translate-y": [{
        "translate-y": Re()
      }],
      /**
       * Translate Z
       * @see https://tailwindcss.com/docs/translate
       */
      "translate-z": [{
        "translate-z": Re()
      }],
      /**
       * Translate None
       * @see https://tailwindcss.com/docs/translate
       */
      "translate-none": ["translate-none"],
      // ---------------------
      // --- Interactivity ---
      // ---------------------
      /**
       * Accent Color
       * @see https://tailwindcss.com/docs/accent-color
       */
      accent: [{
        accent: E()
      }],
      /**
       * Appearance
       * @see https://tailwindcss.com/docs/appearance
       */
      appearance: [{
        appearance: ["none", "auto"]
      }],
      /**
       * Caret Color
       * @see https://tailwindcss.com/docs/just-in-time-mode#caret-color-utilities
       */
      "caret-color": [{
        caret: E()
      }],
      /**
       * Color Scheme
       * @see https://tailwindcss.com/docs/color-scheme
       */
      "color-scheme": [{
        scheme: ["normal", "dark", "light", "light-dark", "only-dark", "only-light"]
      }],
      /**
       * Cursor
       * @see https://tailwindcss.com/docs/cursor
       */
      cursor: [{
        cursor: ["auto", "default", "pointer", "wait", "text", "move", "help", "not-allowed", "none", "context-menu", "progress", "cell", "crosshair", "vertical-text", "alias", "copy", "no-drop", "grab", "grabbing", "all-scroll", "col-resize", "row-resize", "n-resize", "e-resize", "s-resize", "w-resize", "ne-resize", "nw-resize", "se-resize", "sw-resize", "ew-resize", "ns-resize", "nesw-resize", "nwse-resize", "zoom-in", "zoom-out", L, R]
      }],
      /**
       * Field Sizing
       * @see https://tailwindcss.com/docs/field-sizing
       */
      "field-sizing": [{
        "field-sizing": ["fixed", "content"]
      }],
      /**
       * Pointer Events
       * @see https://tailwindcss.com/docs/pointer-events
       */
      "pointer-events": [{
        "pointer-events": ["auto", "none"]
      }],
      /**
       * Resize
       * @see https://tailwindcss.com/docs/resize
       */
      resize: [{
        resize: ["none", "", "y", "x"]
      }],
      /**
       * Scroll Behavior
       * @see https://tailwindcss.com/docs/scroll-behavior
       */
      "scroll-behavior": [{
        scroll: ["auto", "smooth"]
      }],
      /**
       * Scroll Margin
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-m": [{
        "scroll-m": m()
      }],
      /**
       * Scroll Margin Inline
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mx": [{
        "scroll-mx": m()
      }],
      /**
       * Scroll Margin Block
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-my": [{
        "scroll-my": m()
      }],
      /**
       * Scroll Margin Inline Start
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-ms": [{
        "scroll-ms": m()
      }],
      /**
       * Scroll Margin Inline End
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-me": [{
        "scroll-me": m()
      }],
      /**
       * Scroll Margin Block Start
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mbs": [{
        "scroll-mbs": m()
      }],
      /**
       * Scroll Margin Block End
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mbe": [{
        "scroll-mbe": m()
      }],
      /**
       * Scroll Margin Top
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mt": [{
        "scroll-mt": m()
      }],
      /**
       * Scroll Margin Right
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mr": [{
        "scroll-mr": m()
      }],
      /**
       * Scroll Margin Bottom
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-mb": [{
        "scroll-mb": m()
      }],
      /**
       * Scroll Margin Left
       * @see https://tailwindcss.com/docs/scroll-margin
       */
      "scroll-ml": [{
        "scroll-ml": m()
      }],
      /**
       * Scroll Padding
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-p": [{
        "scroll-p": m()
      }],
      /**
       * Scroll Padding Inline
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-px": [{
        "scroll-px": m()
      }],
      /**
       * Scroll Padding Block
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-py": [{
        "scroll-py": m()
      }],
      /**
       * Scroll Padding Inline Start
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-ps": [{
        "scroll-ps": m()
      }],
      /**
       * Scroll Padding Inline End
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pe": [{
        "scroll-pe": m()
      }],
      /**
       * Scroll Padding Block Start
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pbs": [{
        "scroll-pbs": m()
      }],
      /**
       * Scroll Padding Block End
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pbe": [{
        "scroll-pbe": m()
      }],
      /**
       * Scroll Padding Top
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pt": [{
        "scroll-pt": m()
      }],
      /**
       * Scroll Padding Right
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pr": [{
        "scroll-pr": m()
      }],
      /**
       * Scroll Padding Bottom
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pb": [{
        "scroll-pb": m()
      }],
      /**
       * Scroll Padding Left
       * @see https://tailwindcss.com/docs/scroll-padding
       */
      "scroll-pl": [{
        "scroll-pl": m()
      }],
      /**
       * Scroll Snap Align
       * @see https://tailwindcss.com/docs/scroll-snap-align
       */
      "snap-align": [{
        snap: ["start", "end", "center", "align-none"]
      }],
      /**
       * Scroll Snap Stop
       * @see https://tailwindcss.com/docs/scroll-snap-stop
       */
      "snap-stop": [{
        snap: ["normal", "always"]
      }],
      /**
       * Scroll Snap Type
       * @see https://tailwindcss.com/docs/scroll-snap-type
       */
      "snap-type": [{
        snap: ["none", "x", "y", "both"]
      }],
      /**
       * Scroll Snap Type Strictness
       * @see https://tailwindcss.com/docs/scroll-snap-type
       */
      "snap-strictness": [{
        snap: ["mandatory", "proximity"]
      }],
      /**
       * Touch Action
       * @see https://tailwindcss.com/docs/touch-action
       */
      touch: [{
        touch: ["auto", "none", "manipulation"]
      }],
      /**
       * Touch Action X
       * @see https://tailwindcss.com/docs/touch-action
       */
      "touch-x": [{
        "touch-pan": ["x", "left", "right"]
      }],
      /**
       * Touch Action Y
       * @see https://tailwindcss.com/docs/touch-action
       */
      "touch-y": [{
        "touch-pan": ["y", "up", "down"]
      }],
      /**
       * Touch Action Pinch Zoom
       * @see https://tailwindcss.com/docs/touch-action
       */
      "touch-pz": ["touch-pinch-zoom"],
      /**
       * User Select
       * @see https://tailwindcss.com/docs/user-select
       */
      select: [{
        select: ["none", "text", "all", "auto"]
      }],
      /**
       * Will Change
       * @see https://tailwindcss.com/docs/will-change
       */
      "will-change": [{
        "will-change": ["auto", "scroll", "contents", "transform", L, R]
      }],
      // -----------
      // --- SVG ---
      // -----------
      /**
       * Fill
       * @see https://tailwindcss.com/docs/fill
       */
      fill: [{
        fill: ["none", ...E()]
      }],
      /**
       * Stroke Width
       * @see https://tailwindcss.com/docs/stroke-width
       */
      "stroke-w": [{
        stroke: [Z, br, Rt, eo]
      }],
      /**
       * Stroke
       * @see https://tailwindcss.com/docs/stroke
       */
      stroke: [{
        stroke: ["none", ...E()]
      }],
      // ---------------------
      // --- Accessibility ---
      // ---------------------
      /**
       * Forced Color Adjust
       * @see https://tailwindcss.com/docs/forced-color-adjust
       */
      "forced-color-adjust": [{
        "forced-color-adjust": ["auto", "none"]
      }]
    },
    conflictingClassGroups: {
      overflow: ["overflow-x", "overflow-y"],
      overscroll: ["overscroll-x", "overscroll-y"],
      inset: ["inset-x", "inset-y", "inset-bs", "inset-be", "start", "end", "top", "right", "bottom", "left"],
      "inset-x": ["right", "left"],
      "inset-y": ["top", "bottom"],
      flex: ["basis", "grow", "shrink"],
      gap: ["gap-x", "gap-y"],
      p: ["px", "py", "ps", "pe", "pbs", "pbe", "pt", "pr", "pb", "pl"],
      px: ["pr", "pl"],
      py: ["pt", "pb"],
      m: ["mx", "my", "ms", "me", "mbs", "mbe", "mt", "mr", "mb", "ml"],
      mx: ["mr", "ml"],
      my: ["mt", "mb"],
      size: ["w", "h"],
      "font-size": ["leading"],
      "fvn-normal": ["fvn-ordinal", "fvn-slashed-zero", "fvn-figure", "fvn-spacing", "fvn-fraction"],
      "fvn-ordinal": ["fvn-normal"],
      "fvn-slashed-zero": ["fvn-normal"],
      "fvn-figure": ["fvn-normal"],
      "fvn-spacing": ["fvn-normal"],
      "fvn-fraction": ["fvn-normal"],
      "line-clamp": ["display", "overflow"],
      rounded: ["rounded-s", "rounded-e", "rounded-t", "rounded-r", "rounded-b", "rounded-l", "rounded-ss", "rounded-se", "rounded-ee", "rounded-es", "rounded-tl", "rounded-tr", "rounded-br", "rounded-bl"],
      "rounded-s": ["rounded-ss", "rounded-es"],
      "rounded-e": ["rounded-se", "rounded-ee"],
      "rounded-t": ["rounded-tl", "rounded-tr"],
      "rounded-r": ["rounded-tr", "rounded-br"],
      "rounded-b": ["rounded-br", "rounded-bl"],
      "rounded-l": ["rounded-tl", "rounded-bl"],
      "border-spacing": ["border-spacing-x", "border-spacing-y"],
      "border-w": ["border-w-x", "border-w-y", "border-w-s", "border-w-e", "border-w-bs", "border-w-be", "border-w-t", "border-w-r", "border-w-b", "border-w-l"],
      "border-w-x": ["border-w-r", "border-w-l"],
      "border-w-y": ["border-w-t", "border-w-b"],
      "border-color": ["border-color-x", "border-color-y", "border-color-s", "border-color-e", "border-color-bs", "border-color-be", "border-color-t", "border-color-r", "border-color-b", "border-color-l"],
      "border-color-x": ["border-color-r", "border-color-l"],
      "border-color-y": ["border-color-t", "border-color-b"],
      translate: ["translate-x", "translate-y", "translate-none"],
      "translate-none": ["translate", "translate-x", "translate-y", "translate-z"],
      "scroll-m": ["scroll-mx", "scroll-my", "scroll-ms", "scroll-me", "scroll-mbs", "scroll-mbe", "scroll-mt", "scroll-mr", "scroll-mb", "scroll-ml"],
      "scroll-mx": ["scroll-mr", "scroll-ml"],
      "scroll-my": ["scroll-mt", "scroll-mb"],
      "scroll-p": ["scroll-px", "scroll-py", "scroll-ps", "scroll-pe", "scroll-pbs", "scroll-pbe", "scroll-pt", "scroll-pr", "scroll-pb", "scroll-pl"],
      "scroll-px": ["scroll-pr", "scroll-pl"],
      "scroll-py": ["scroll-pt", "scroll-pb"],
      touch: ["touch-x", "touch-y", "touch-pz"],
      "touch-x": ["touch"],
      "touch-y": ["touch"],
      "touch-pz": ["touch"]
    },
    conflictingClassGroupModifiers: {
      "font-size": ["leading"]
    },
    orderSensitiveModifiers: ["*", "**", "after", "backdrop", "before", "details-content", "file", "first-letter", "first-line", "marker", "placeholder", "selection"]
  };
}, qa = (e, {
  cacheSize: t,
  prefix: r,
  experimentalParseClassName: n,
  extend: o = {},
  override: s = {}
}) => (xr(e, "cacheSize", t), xr(e, "prefix", r), xr(e, "experimentalParseClassName", n), Or(e.theme, s.theme), Or(e.classGroups, s.classGroups), Or(e.conflictingClassGroups, s.conflictingClassGroups), Or(e.conflictingClassGroupModifiers, s.conflictingClassGroupModifiers), xr(e, "orderSensitiveModifiers", s.orderSensitiveModifiers), Vr(e.theme, o.theme), Vr(e.classGroups, o.classGroups), Vr(e.conflictingClassGroups, o.conflictingClassGroups), Vr(e.conflictingClassGroupModifiers, o.conflictingClassGroupModifiers), gs(e, o, "orderSensitiveModifiers"), e), xr = (e, t, r) => {
  r !== void 0 && (e[t] = r);
}, Or = (e, t) => {
  if (t)
    for (const r in t)
      xr(e, r, t[r]);
}, Vr = (e, t) => {
  if (t)
    for (const r in t)
      gs(e, t, r);
}, gs = (e, t, r) => {
  const n = t[r];
  n !== void 0 && (e[r] = e[r] ? e[r].concat(n) : n);
}, Ya = (e, ...t) => typeof e == "function" ? kn(Cn, e, ...t) : kn(() => qa(Cn(), e), ...t), hs = /* @__PURE__ */ kn(Cn);
function Qt(...e) {
  return hs(ia(e));
}
var Xa = /\s+/g, Za = (e) => typeof e != "string" || !e ? e : e.replace(Xa, " ").trim(), Xr = (...e) => {
  const t = [], r = (n) => {
    if (!n && n !== 0 && n !== 0n) return;
    if (Array.isArray(n)) {
      for (let s = 0, a = n.length; s < a; s++) r(n[s]);
      return;
    }
    const o = typeof n;
    if (o === "string" || o === "number" || o === "bigint") {
      if (o === "number" && n !== n) return;
      t.push(String(n));
    } else if (o === "object") {
      const s = Object.keys(n);
      for (let a = 0, u = s.length; a < u; a++) {
        const l = s[a];
        n[l] && t.push(l);
      }
    }
  };
  for (let n = 0, o = e.length; n < o; n++) {
    const s = e[n];
    s != null && r(s);
  }
  return t.length > 0 ? Za(t.join(" ")) : void 0;
}, no = (e) => e === !1 ? "false" : e === !0 ? "true" : e === 0 ? "0" : e, Ae = (e) => {
  if (!e || typeof e != "object") return !0;
  for (const t in e) return !1;
  return !0;
}, Ja = (e, t) => {
  if (e === t) return !0;
  if (!e || !t) return !1;
  const r = Object.keys(e), n = Object.keys(t);
  if (r.length !== n.length) return !1;
  for (let o = 0; o < r.length; o++) {
    const s = r[o];
    if (!n.includes(s) || e[s] !== t[s]) return !1;
  }
  return !0;
}, Qa = (e, t) => {
  for (const r in t)
    if (Object.prototype.hasOwnProperty.call(t, r)) {
      const n = t[r];
      r in e ? e[r] = Xr(e[r], n) : e[r] = n;
    }
  return e;
}, bs = (e, t) => {
  for (let r = 0; r < e.length; r++) {
    const n = e[r];
    Array.isArray(n) ? bs(n, t) : n && t.push(n);
  }
}, _s = (...e) => {
  const t = [];
  bs(e, t);
  const r = [];
  for (let n = 0; n < t.length; n++)
    t[n] && r.push(t[n]);
  return r;
}, En = (e, t) => {
  const r = {};
  for (const n in e) {
    const o = e[n];
    if (n in t) {
      const s = t[n];
      Array.isArray(o) || Array.isArray(s) ? r[n] = _s(s, o) : typeof o == "object" && typeof s == "object" && o && s ? r[n] = En(o, s) : r[n] = s + " " + o;
    } else
      r[n] = o;
  }
  for (const n in t)
    n in e || (r[n] = t[n]);
  return r;
}, $a = {
  twMerge: !0,
  twMergeConfig: {}
};
function el() {
  let e = null, t = {}, r = !1;
  return {
    get cachedTwMerge() {
      return e;
    },
    set cachedTwMerge(n) {
      e = n;
    },
    get cachedTwMergeConfig() {
      return t;
    },
    set cachedTwMergeConfig(n) {
      t = n;
    },
    get didTwMergeConfigChange() {
      return r;
    },
    set didTwMergeConfigChange(n) {
      r = n;
    },
    reset() {
      e = null, t = {}, r = !1;
    }
  };
}
var ft = el(), tl = (e) => {
  const t = (n, o) => {
    const {
      extend: s = null,
      slots: a = {},
      variants: u = {},
      compoundVariants: l = [],
      compoundSlots: c = [],
      defaultVariants: d = {}
    } = n, b = { ...$a, ...o }, f = s != null && s.base ? Xr(s.base, n == null ? void 0 : n.base) : n == null ? void 0 : n.base, y = s != null && s.variants && !Ae(s.variants) ? En(u, s.variants) : u, x = s != null && s.defaultVariants && !Ae(s.defaultVariants) ? { ...s.defaultVariants, ...d } : d;
    !Ae(b.twMergeConfig) && !Ja(b.twMergeConfig, ft.cachedTwMergeConfig) && (ft.didTwMergeConfigChange = !0, ft.cachedTwMergeConfig = b.twMergeConfig);
    const w = Ae(s == null ? void 0 : s.slots), v = Ae(a) ? {} : {
      // add "base" to the slots object
      base: Xr(n == null ? void 0 : n.base, w && (s == null ? void 0 : s.base)),
      ...a
    }, P = w ? v : Qa(
      { ...s == null ? void 0 : s.slots },
      Ae(v) ? { base: n == null ? void 0 : n.base } : v
    ), V = Ae(s == null ? void 0 : s.compoundVariants) ? l : _s(s == null ? void 0 : s.compoundVariants, l), O = (C) => {
      if (Ae(y) && Ae(a) && w)
        return e(f, C == null ? void 0 : C.class, C == null ? void 0 : C.className)(b);
      if (V && !Array.isArray(V))
        throw new TypeError(
          `The "compoundVariants" prop must be an array. Received: ${typeof V}`
        );
      if (c && !Array.isArray(c))
        throw new TypeError(
          `The "compoundSlots" prop must be an array. Received: ${typeof c}`
        );
      const B = (g, A = y, T = null, F = null) => {
        const K = A[g];
        if (!K || Ae(K))
          return null;
        const U = (F == null ? void 0 : F[g]) ?? (C == null ? void 0 : C[g]);
        if (U === null) return null;
        const E = no(U);
        if (typeof E == "object")
          return null;
        const me = x == null ? void 0 : x[g], le = E ?? no(me);
        return K[le || "false"];
      }, H = () => {
        if (!y) return null;
        const g = Object.keys(y), A = [];
        for (let T = 0; T < g.length; T++) {
          const F = B(g[T], y);
          F && A.push(F);
        }
        return A;
      }, m = (g, A) => {
        if (!y || typeof y != "object") return null;
        const T = [];
        for (const F in y) {
          const K = B(F, y, g, A), U = g === "base" && typeof K == "string" ? K : K && K[g];
          U && T.push(U);
        }
        return T;
      }, k = {};
      for (const g in C) {
        const A = C[g];
        A !== void 0 && (k[g] = A);
      }
      const N = (g, A) => {
        var F;
        const T = typeof (C == null ? void 0 : C[g]) == "object" ? {
          [g]: (F = C[g]) == null ? void 0 : F.initial
        } : {};
        return {
          ...x,
          ...k,
          ...T,
          ...A
        };
      }, M = (g = [], A) => {
        const T = [], F = g.length;
        for (let K = 0; K < F; K++) {
          const { class: U, className: E, ...me } = g[K];
          let le = !0;
          const te = N(null, A);
          for (const de in me) {
            const $ = me[de], se = te[de];
            if (Array.isArray($)) {
              if (!$.includes(se)) {
                le = !1;
                break;
              }
            } else {
              if (($ == null || $ === !1) && (se == null || se === !1))
                continue;
              if (se !== $) {
                le = !1;
                break;
              }
            }
          }
          le && (U && T.push(U), E && T.push(E));
        }
        return T;
      }, q = (g) => {
        const A = M(V, g);
        if (!Array.isArray(A)) return A;
        const T = {}, F = e;
        for (let K = 0; K < A.length; K++) {
          const U = A[K];
          if (typeof U == "string")
            T.base = F(T.base, U)(b);
          else if (typeof U == "object")
            for (const E in U)
              T[E] = F(T[E], U[E])(b);
        }
        return T;
      }, S = (g) => {
        if (c.length < 1) return null;
        const A = {}, T = N(null, g);
        for (let F = 0; F < c.length; F++) {
          const {
            slots: K = [],
            class: U,
            className: E,
            ...me
          } = c[F];
          if (!Ae(me)) {
            let le = !0;
            for (const te in me) {
              const de = T[te], $ = me[te];
              if (de === void 0 || (Array.isArray($) ? !$.includes(de) : $ !== de)) {
                le = !1;
                break;
              }
            }
            if (!le) continue;
          }
          for (let le = 0; le < K.length; le++) {
            const te = K[le];
            A[te] || (A[te] = []), A[te].push([U, E]);
          }
        }
        return A;
      };
      if (!Ae(a) || !w) {
        const g = {};
        if (typeof P == "object" && !Ae(P)) {
          const A = e;
          for (const T in P)
            g[T] = (F) => {
              const K = q(F), U = S(F);
              return A(
                P[T],
                m(T, F),
                K ? K[T] : void 0,
                U ? U[T] : void 0,
                F == null ? void 0 : F.class,
                F == null ? void 0 : F.className
              )(b);
            };
        }
        return g;
      }
      return e(
        f,
        H(),
        M(V),
        C == null ? void 0 : C.class,
        C == null ? void 0 : C.className
      )(b);
    }, Y = () => {
      if (!(!y || typeof y != "object"))
        return Object.keys(y);
    };
    return O.variantKeys = Y(), O.extend = s, O.base = f, O.slots = P, O.variants = y, O.defaultVariants = x, O.compoundSlots = c, O.compoundVariants = V, O;
  };
  return {
    tv: t,
    createTV: (n) => (o, s) => t(o, s ? En(n, s) : n)
  };
}, rl = (e) => Ae(e) ? hs : Ya({
  ...e,
  extend: {
    theme: e.theme,
    classGroups: e.classGroups,
    conflictingClassGroupModifiers: e.conflictingClassGroupModifiers,
    conflictingClassGroups: e.conflictingClassGroups,
    ...e.extend
  }
}), nl = (e, t) => {
  const r = Xr(e);
  return !r || !((t == null ? void 0 : t.twMerge) ?? !0) ? r : ((!ft.cachedTwMerge || ft.didTwMergeConfigChange) && (ft.didTwMergeConfigChange = !1, ft.cachedTwMerge = rl(ft.cachedTwMergeConfig)), ft.cachedTwMerge(r) || void 0);
}, ol = (...e) => (t) => nl(e, t), { tv: xs } = tl(ol);
xs({
  base: "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  variants: {
    variant: {
      default: "bg-primary text-primary-foreground hover:bg-primary/90 shadow-xs",
      destructive: "bg-destructive hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white shadow-xs",
      outline: "bg-background hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border shadow-xs",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-xs",
      ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
      link: "text-primary underline-offset-4 hover:underline"
    },
    size: {
      default: "h-9 px-4 py-2 has-[>svg]:px-3",
      sm: "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
      lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
      icon: "size-9",
      "icon-sm": "size-8",
      "icon-lg": "size-10"
    }
  },
  defaultVariants: { variant: "default", size: "default" }
});
xs({
  base: "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex w-fit shrink-0 items-center justify-center gap-2 overflow-hidden rounded border px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[3px] [&>svg]:pointer-events-none [&>svg]:size-3",
  variants: {
    variant: {
      default: "bg-primary text-primary-foreground [a&]:hover:bg-primary/90 border-transparent",
      secondary: "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90 border-border",
      destructive: "bg-destructive [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/70 border-transparent text-white",
      outline: "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
      warning: "bg-warning/15 text-warning border-warning/30 [a&]:hover:bg-warning/25"
    }
  },
  defaultVariants: { variant: "default" }
});
ti();
function sl(e, t) {
  const r = RegExp(e, "g");
  return (n) => {
    if (typeof n != "string")
      throw new TypeError(`expected an argument of type string, but got ${typeof n}`);
    return n.match(r) ? n.replace(r, t) : n;
  };
}
const il = sl(/[A-Z]/, (e) => `-${e.toLowerCase()}`);
function al(e) {
  if (!e || typeof e != "object" || Array.isArray(e))
    throw new TypeError(`expected an argument of type object, but got ${typeof e}`);
  return Object.keys(e).map((t) => `${il(t)}: ${e[t]};`).join(`
`);
}
function ll(e = {}) {
  return al(e).replace(`
`, " ");
}
const cl = {
  position: "absolute",
  width: "1px",
  height: "1px",
  padding: "0",
  margin: "-1px",
  overflow: "hidden",
  clip: "rect(0, 0, 0, 0)",
  whiteSpace: "nowrap",
  borderWidth: "0",
  transform: "translateX(-100%)"
};
ll(cl);
const ul = [
  "onabort",
  "onanimationcancel",
  "onanimationend",
  "onanimationiteration",
  "onanimationstart",
  "onauxclick",
  "onbeforeinput",
  "onbeforetoggle",
  "onblur",
  "oncancel",
  "oncanplay",
  "oncanplaythrough",
  "onchange",
  "onclick",
  "onclose",
  "oncompositionend",
  "oncompositionstart",
  "oncompositionupdate",
  "oncontextlost",
  "oncontextmenu",
  "oncontextrestored",
  "oncopy",
  "oncuechange",
  "oncut",
  "ondblclick",
  "ondrag",
  "ondragend",
  "ondragenter",
  "ondragleave",
  "ondragover",
  "ondragstart",
  "ondrop",
  "ondurationchange",
  "onemptied",
  "onended",
  "onerror",
  "onfocus",
  "onfocusin",
  "onfocusout",
  "onformdata",
  "ongotpointercapture",
  "oninput",
  "oninvalid",
  "onkeydown",
  "onkeypress",
  "onkeyup",
  "onload",
  "onloadeddata",
  "onloadedmetadata",
  "onloadstart",
  "onlostpointercapture",
  "onmousedown",
  "onmouseenter",
  "onmouseleave",
  "onmousemove",
  "onmouseout",
  "onmouseover",
  "onmouseup",
  "onpaste",
  "onpause",
  "onplay",
  "onplaying",
  "onpointercancel",
  "onpointerdown",
  "onpointerenter",
  "onpointerleave",
  "onpointermove",
  "onpointerout",
  "onpointerover",
  "onpointerup",
  "onprogress",
  "onratechange",
  "onreset",
  "onresize",
  "onscroll",
  "onscrollend",
  "onsecuritypolicyviolation",
  "onseeked",
  "onseeking",
  "onselect",
  "onselectionchange",
  "onselectstart",
  "onslotchange",
  "onstalled",
  "onsubmit",
  "onsuspend",
  "ontimeupdate",
  "ontoggle",
  "ontouchcancel",
  "ontouchend",
  "ontouchmove",
  "ontouchstart",
  "ontransitioncancel",
  "ontransitionend",
  "ontransitionrun",
  "ontransitionstart",
  "onvolumechange",
  "onwaiting",
  "onwebkitanimationend",
  "onwebkitanimationiteration",
  "onwebkitanimationstart",
  "onwebkittransitionend",
  "onwheel"
];
new Set(ul);
const dl = typeof window < "u" ? window : void 0;
function fl(e) {
  let t = e.activeElement;
  for (; t != null && t.shadowRoot; ) {
    const r = t.shadowRoot.activeElement;
    if (r === t)
      break;
    t = r;
  }
  return t;
}
var sr, zr, ao;
let vl = (ao = class {
  constructor(t = {}) {
    ie(this, sr);
    ie(this, zr);
    const { window: r = dl, document: n = r == null ? void 0 : r.document } = t;
    r !== void 0 && (Se(this, sr, n), Se(this, zr, Ao((o) => {
      const s = Kr(r, "focusin", o), a = Kr(r, "focusout", o);
      return () => {
        s(), a();
      };
    })));
  }
  get current() {
    var t;
    return (t = _(this, zr)) == null || t.call(this), _(this, sr) ? fl(_(this, sr)) : null;
  }
}, sr = new WeakMap(), zr = new WeakMap(), ao);
new vl();
const pl = typeof window < "u" ? window : void 0;
function ml(e) {
  let t = e.activeElement;
  for (; t != null && t.shadowRoot; ) {
    const r = t.shadowRoot.activeElement;
    if (r === t)
      break;
    t = r;
  }
  return t;
}
var ir, Pr;
class gl {
  constructor(t = {}) {
    ie(this, ir);
    ie(this, Pr);
    const { window: r = pl, document: n = r == null ? void 0 : r.document } = t;
    r !== void 0 && (Se(this, ir, n), Se(this, Pr, Ao((o) => {
      const s = Kr(r, "focusin", o), a = Kr(r, "focusout", o);
      return () => {
        s(), a();
      };
    })));
  }
  get current() {
    var t;
    return (t = _(this, Pr)) == null || t.call(this), _(this, ir) ? ml(_(this, ir)) : null;
  }
}
ir = new WeakMap(), Pr = new WeakMap();
new gl();
Xe(["click"]);
Xe(["click"]);
Xe(["click"]);
Xe(["click"]);
Xe(["click"]);
Xe(["click"]);
Xe(["click"]);
var hl = /* @__PURE__ */ G('<span class="flex h-5 w-5 items-center justify-center rounded-full bg-muted text-[10px] font-semibold tabular-nums text-muted-foreground"> </span>'), bl = /* @__PURE__ */ G('<div role="button" tabindex="0"><div class="flex flex-1 flex-col items-center justify-center gap-2 py-4"><span class="text-xs font-medium text-muted-foreground select-none capitalize" style="writing-mode: vertical-rl; transform: rotate(180deg);"> </span> <!></div></div>'), _l = /* @__PURE__ */ G('<span class="text-[10px] tabular-nums text-muted-foreground"> </span>'), xl = /* @__PURE__ */ G('<button class="rounded p-0.5 text-muted-foreground hover:text-foreground transition-colors text-xs"><span class="text-xs">&rarr;</span></button>'), yl = /* @__PURE__ */ G('<p class="mt-1 text-xs text-muted-foreground"> </p>'), wl = /* @__PURE__ */ G('<div class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-border p-6 text-center text-xs text-muted-foreground"> </div>'), kl = /* @__PURE__ */ G('<div role="listitem"><!></div>'), Cl = /* @__PURE__ */ G('<div class="flex flex-col gap-3 p-3" role="list"><!></div>'), El = /* @__PURE__ */ G('<div role="region"><div class="border-b border-border px-4 py-3"><div class="flex items-center justify-between"><div class="flex items-center gap-2"><!> <!></div> <!></div> <!></div> <!></div>'), Sl = /* @__PURE__ */ G('<div class="flex h-full flex-col gap-3"><div class="flex items-center justify-between"><span class="text-xs text-muted-foreground"> </span> <!></div> <div class="min-h-0 flex-1"><div class="flex h-full gap-4 pb-4"></div></div></div>');
function Tl(e, t) {
  Bt(t, !0);
  let r = Jt(t, "epicParentRel", 3, "delivers"), n = Jt(t, "epicLabel", 3, "Epic"), o = Jt(t, "rootLabel", 3, "Milestone");
  const s = new yn(["done"]);
  function a(S) {
    s.has(S) ? s.delete(S) : s.add(S);
  }
  let u = /* @__PURE__ */ ve(null), l = /* @__PURE__ */ ve(null);
  function c(S, g) {
    var A;
    re(u, g.id, !0), (A = S.dataTransfer) == null || A.setData("text/plain", g.id);
  }
  function d(S, g) {
    S.preventDefault(), re(l, g, !0);
  }
  function b(S) {
    const g = S.relatedTarget;
    g && S.currentTarget.contains(g) || re(l, null);
  }
  function f(S, g) {
    var K, U;
    S.preventDefault(), S.stopPropagation(), re(l, null);
    const A = ((K = S.dataTransfer) == null ? void 0 : K.getData("text/plain")) ?? i(u);
    if (!A) return;
    let T;
    for (const E of t.columns)
      if (T = E.milestones.find((me) => me.id === A), T) break;
    !T || (T.frontmatter.horizon ?? y(T)) === g || ((U = t.onHorizonChange) == null || U.call(t, T, g), re(u, null));
  }
  function y(S) {
    const g = S.status ?? "captured";
    return g === "active" ? "now" : g === "completed" || g === "surpassed" ? "done" : g === "captured" ? "later" : "next";
  }
  function x(S) {
    return t.epics.filter((g) => g.references_out.some((A) => A.relationship_type === r() && A.target_id === S));
  }
  const w = [
    { value: "horizon", label: "Group by Horizon" },
    { value: "status", label: "Group by Status" },
    { value: "priority", label: "Group by Priority" }
  ];
  let v = /* @__PURE__ */ ve("horizon");
  const P = /* @__PURE__ */ I(() => {
    var S;
    return ((S = w.find((g) => g.value === i(v))) == null ? void 0 : S.label) ?? "Group by Horizon";
  }), V = [
    { key: "captured", label: "Captured", isDone: !1 },
    { key: "exploring", label: "Exploring", isDone: !1 },
    { key: "ready", label: "Ready", isDone: !1 },
    { key: "active", label: "Active", isDone: !1 },
    { key: "review", label: "Review", isDone: !1 },
    { key: "completed", label: "Completed", isDone: !0 }
  ], O = [
    { key: "P1", label: "P1 — Critical", isDone: !1 },
    { key: "P2", label: "P2 — High", isDone: !1 },
    { key: "P3", label: "P3 — Normal", isDone: !1 },
    { key: "none", label: "Unranked", isDone: !1 }
  ], Y = /* @__PURE__ */ I(() => t.columns.flatMap((S) => S.milestones)), C = /* @__PURE__ */ I(() => i(v) === "status" ? V.map((S) => ({
    ...S,
    description: "",
    milestones: i(Y).filter((g) => (g.status ?? "planning").toLowerCase() === S.key)
  })) : i(v) === "priority" ? O.map((S) => ({
    ...S,
    description: "",
    milestones: i(Y).filter((g) => (g.frontmatter.priority ?? "none") === S.key)
  })) : t.columns);
  var B = Sl(), H = h(B), m = h(H), k = h(m), N = W(m, 2);
  co(N, {
    get items() {
      return w;
    },
    get selected() {
      return i(v);
    },
    onSelect: (S) => {
      re(v, S, !0);
    },
    get triggerLabel() {
      return i(P);
    },
    triggerSize: "sm"
  });
  var M = W(H, 2), q = h(M);
  jt(q, 21, () => i(C), (S) => S.key, (S, g) => {
    const A = /* @__PURE__ */ I(() => i(g).isDone === !0 && s.has(i(g).key)), T = /* @__PURE__ */ I(() => i(l) === i(g).key), F = /* @__PURE__ */ I(() => i(Y).length);
    var K = Vt(), U = gt(K);
    {
      var E = (le) => {
        var te = bl(), de = h(te), $ = h(de), se = h($), Pe = W($, 2);
        {
          var rt = (Q) => {
            var we = hl(), Ee = h(we);
            X(() => J(Ee, i(g).milestones.length)), D(Q, we);
          };
          ne(Pe, (Q) => {
            i(g).milestones.length > 0 && Q(rt);
          });
        }
        X(
          (Q) => {
            Zt(te, 1, Q), mt(te, "aria-label", `Expand ${i(g).label ?? ""} column`), J(se, i(g).label);
          },
          [
            () => Xt(Qt("flex w-10 shrink-0 cursor-pointer flex-col items-center rounded-xl border border-dashed border-border bg-muted/30 transition-colors hover:bg-muted/50", i(T) && "border-primary bg-primary/10"))
          ]
        ), _t("click", te, () => a(i(g).key)), Te("dragover", te, (Q) => d(Q, i(g).key)), Te("dragleave", te, b), Te("drop", te, (Q) => f(Q, i(g).key)), _t("keydown", te, (Q) => Q.key === "Enter" && a(i(g).key)), D(le, te);
      }, me = (le) => {
        var te = El(), de = h(te), $ = h(de), se = h($), Pe = h(se);
        uo(Pe, {
          variant: "outline",
          class: "text-xs font-semibold capitalize",
          children: (p, z) => {
            var j = nn();
            X(() => J(j, i(g).label)), D(p, j);
          },
          $$slots: { default: !0 }
        });
        var rt = W(Pe, 2);
        {
          var Q = (p) => {
            var z = _l(), j = h(z);
            X(() => J(j, `${i(g).milestones.length ?? ""}/${i(F) ?? ""} Done`)), D(p, z);
          };
          ne(rt, (p) => {
            i(g).isDone && i(F) > 0 && p(Q);
          });
        }
        var we = W(se, 2);
        {
          var Ee = (p) => {
            var z = xl();
            X(() => mt(z, "aria-label", `Collapse ${i(g).label ?? ""}`)), _t("click", z, () => a(i(g).key)), D(p, z);
          };
          ne(we, (p) => {
            i(g).isDone && p(Ee);
          });
        }
        var nt = W($, 2);
        {
          var Ze = (p) => {
            var z = yl(), j = h(z);
            X(() => J(j, i(g).description)), D(p, z);
          };
          ne(nt, (p) => {
            i(g).description && p(Ze);
          });
        }
        var Re = W(de, 2);
        fo(Re, {
          class: "min-h-0 flex-1",
          orientation: "vertical",
          children: (p, z) => {
            var j = Cl(), ue = h(j);
            {
              var fe = (Ne) => {
                var je = wl(), ot = h(je);
                X((Be) => J(ot, `No ${Be ?? ""}s`), [() => o().toLowerCase()]), D(Ne, je);
              }, De = (Ne) => {
                var je = Vt(), ot = gt(je);
                jt(ot, 17, () => i(g).milestones, (Be) => Be.id, (Be, Ue) => {
                  const He = /* @__PURE__ */ I(() => x(i(Ue).id)), pr = /* @__PURE__ */ I(() => i(He).filter((_e) => _e.status === "completed").length), mr = /* @__PURE__ */ I(() => i(He).filter((_e) => _e.status === "active")), Le = /* @__PURE__ */ I(() => i(He).filter((_e) => _e.priority === "P1" && _e.status !== "completed"));
                  var ce = kl(), Ie = h(ce);
                  sa(Ie, {
                    get milestone() {
                      return i(Ue);
                    },
                    get epicCount() {
                      return i(He).length;
                    },
                    get doneEpicCount() {
                      return i(pr);
                    },
                    get inProgressEpics() {
                      return i(mr);
                    },
                    get criticalEpics() {
                      return i(Le);
                    },
                    get epicLabel() {
                      return n();
                    },
                    onClick: () => t.onMilestoneClick(i(Ue))
                  }), X(
                    (_e) => {
                      mt(ce, "draggable", t.onHorizonChange !== void 0 && i(v) === "horizon"), Zt(ce, 1, _e);
                    },
                    [
                      () => Xt(Qt(t.onHorizonChange && i(v) === "horizon" && "cursor-grab active:cursor-grabbing"))
                    ]
                  ), Te("dragstart", ce, (_e) => c(_e, i(Ue))), D(Be, ce);
                }), D(Ne, je);
              };
              ne(ue, (Ne) => {
                i(g).milestones.length === 0 ? Ne(fe) : Ne(De, -1);
              });
            }
            D(p, j);
          },
          $$slots: { default: !0 }
        }), X(
          (p) => {
            Zt(te, 1, p), mt(te, "aria-label", `${i(g).label ?? ""} horizon column`);
          },
          [
            () => Xt(Qt("flex min-w-[12rem] flex-1 flex-col rounded-xl border border-border bg-muted/5 transition-colors", i(T) && "border-primary bg-primary/5"))
          ]
        ), Te("dragover", te, (p) => d(p, i(g).key)), Te("dragleave", te, b), Te("drop", te, (p) => f(p, i(g).key)), D(le, te);
      };
      ne(U, (le) => {
        i(A) ? le(E) : le(me, -1);
      });
    }
    D(S, K);
  }), X((S) => J(k, `${S ?? ""}/${i(Y).length ?? ""} Done`), [
    () => i(Y).filter((S) => S.status === "complete").length
  ]), D(e, B), Ut();
}
Xe(["click", "keydown"]);
var Al = /* @__PURE__ */ G("<span> </span>"), Ml = /* @__PURE__ */ G('<div role="button" tabindex="0"><div class="flex flex-1 flex-col items-center justify-center gap-2 py-4"><span class="text-xs font-medium text-muted-foreground select-none" style="writing-mode: vertical-rl; transform: rotate(180deg);"> </span> <!></div></div>'), zl = /* @__PURE__ */ G('<span class="text-[10px] tabular-nums text-muted-foreground"> </span>'), Pl = /* @__PURE__ */ G('<button class="rounded p-0.5 text-muted-foreground hover:text-foreground transition-colors"><!></button>'), Rl = /* @__PURE__ */ G('<div class="flex flex-col gap-2 p-2" role="list"><!></div>'), Dl = /* @__PURE__ */ G('<div role="region"><div class="flex items-center justify-between border-b border-border px-3 py-2"><div class="flex items-center gap-2"><!> <!></div> <!></div> <!></div>');
function Nl(e, t) {
  Bt(t, !0);
  let r = Jt(t, "collapsed", 3, !0), n = Jt(t, "isDone", 3, !1), o = /* @__PURE__ */ ve(Yt(ks(() => r()))), s = /* @__PURE__ */ ve(!1);
  function a() {
    re(o, !i(o));
  }
  function u(x) {
    var w;
    x.preventDefault(), re(s, !0), (w = t.onDragOver) == null || w.call(t, x);
  }
  function l(x) {
    const w = x.relatedTarget;
    w && x.currentTarget.contains(w) || re(s, !1);
  }
  function c(x) {
    var w;
    x.stopPropagation(), re(s, !1), (w = t.onDrop) == null || w.call(t, x);
  }
  var d = Vt(), b = gt(d);
  {
    var f = (x) => {
      var w = Ml(), v = h(w), P = h(v), V = h(P), O = W(P, 2);
      {
        var Y = (C) => {
          var B = Al(), H = h(B);
          X(
            (m) => {
              Zt(B, 1, m), J(H, t.count);
            },
            [
              () => Xt(Qt("flex h-5 w-5 items-center justify-center rounded-full text-[10px] font-semibold tabular-nums", n() ? "bg-emerald-500/20 text-emerald-700 dark:text-emerald-400" : "bg-muted text-muted-foreground"))
            ]
          ), D(C, B);
        };
        ne(O, (C) => {
          t.count > 0 && C(Y);
        });
      }
      X(
        (C) => {
          Zt(w, 1, C), mt(w, "aria-label", `Expand ${t.title ?? ""} column`), J(V, t.title);
        },
        [
          () => Xt(Qt("flex w-10 shrink-0 flex-col items-center rounded-lg border border-dashed border-border bg-muted/30 transition-colors cursor-pointer hover:bg-muted/50", i(s) && "border-primary bg-primary/10"))
        ]
      ), _t("click", w, a), Te("dragover", w, u), Te("dragleave", w, l), Te("drop", w, c), _t("keydown", w, (C) => C.key === "Enter" && a()), D(x, w);
    }, y = (x) => {
      var w = Dl(), v = h(w), P = h(v), V = h(P);
      uo(V, {
        variant: "outline",
        class: "text-xs font-semibold capitalize",
        children: (m, k) => {
          var N = nn();
          X(() => J(N, t.title)), D(m, N);
        },
        $$slots: { default: !0 }
      });
      var O = W(V, 2);
      {
        var Y = (m) => {
          var k = zl(), N = h(k);
          X(() => J(N, `${t.doneCount ?? ""}/${t.totalCount ?? ""} Done`)), D(m, k);
        };
        ne(O, (m) => {
          t.doneCount !== void 0 && t.totalCount !== void 0 && m(Y);
        });
      }
      var C = W(P, 2);
      {
        var B = (m) => {
          var k = Pl(), N = h(k);
          jr(N, { name: "chevron-right", size: "sm" }), X(() => mt(k, "aria-label", `Collapse ${t.title ?? ""} column`)), _t("click", k, a), D(m, k);
        };
        ne(C, (m) => {
          n() && m(B);
        });
      }
      var H = W(v, 2);
      fo(H, {
        class: "min-h-0 flex-1",
        orientation: "vertical",
        children: (m, k) => {
          var N = Rl(), M = h(N);
          Li(M, () => t.children), D(m, N);
        },
        $$slots: { default: !0 }
      }), X(
        (m) => {
          Zt(w, 1, m), mt(w, "aria-label", `${t.title ?? ""} column`);
        },
        [
          () => Xt(Qt("flex min-w-56 flex-1 flex-col rounded-lg border border-border bg-muted/10 transition-colors", i(s) && "border-primary bg-primary/5"))
        ]
      ), Te("dragover", w, u), Te("dragleave", w, l), Te("drop", w, c), D(x, w);
    };
    ne(b, (x) => {
      i(o) ? x(f) : x(y, -1);
    });
  }
  D(e, d), Ut();
}
Xe(["click", "keydown"]);
var Ll = /* @__PURE__ */ G('<p class="mt-1.5 line-clamp-2 text-xs text-muted-foreground"> </p>'), Il = /* @__PURE__ */ G('<div class="mt-2 flex items-center gap-2"><div class="h-1.5 flex-1 rounded-full bg-muted"><div class="h-1.5 rounded-full bg-emerald-500 transition-all"></div></div> <span class="shrink-0 text-[10px] tabular-nums text-muted-foreground"> </span></div>'), Ol = /* @__PURE__ */ G('<span class="rounded bg-primary/10 px-1 py-0.5 text-[9px] font-medium text-primary"> </span>'), Vl = /* @__PURE__ */ G('<div class="flex items-start justify-between gap-2"><div class="flex min-w-0 items-center gap-2"><!> <span class="truncate text-sm font-medium"> </span></div> <!></div> <!> <!> <div class="mt-1.5 flex items-center gap-1.5"><!> <span class="text-[10px] font-mono text-muted-foreground/60"> </span></div>', 1), Fl = /* @__PURE__ */ G('<button class="w-full rounded-lg border border-border bg-card p-3 text-left transition-colors hover:bg-accent/50 hover:border-border/80"><!></button>'), Gl = /* @__PURE__ */ G('<div class="w-full rounded-lg border border-border bg-card p-3 cursor-grab active:cursor-grabbing" role="listitem"><!></div>');
function jl(e, t) {
  Bt(t, !0);
  const r = (c) => {
    var d = Vl(), b = gt(d), f = h(b), y = h(f);
    {
      let M = /* @__PURE__ */ I(() => t.node.status ?? "captured");
      vo(y, {
        get status() {
          return i(M);
        },
        mode: "dot"
      });
    }
    var x = W(y, 2), w = h(x), v = W(f, 2);
    {
      var P = (M) => {
        {
          let q = /* @__PURE__ */ I(() => n(t.node.priority));
          lo(M, {
            get variant() {
              return i(q);
            },
            children: (S, g) => {
              var A = nn();
              X(() => J(A, t.node.priority)), D(S, A);
            },
            $$slots: { default: !0 }
          });
        }
      };
      ne(v, (M) => {
        t.node.priority && M(P);
      });
    }
    var V = W(b, 2);
    {
      var O = (M) => {
        var q = Ll(), S = h(q);
        X(() => J(S, t.node.description)), D(M, q);
      };
      ne(V, (M) => {
        t.node.description && M(O);
      });
    }
    var Y = W(V, 2);
    {
      var C = (M) => {
        var q = Il(), S = h(q), g = h(S), A = W(S, 2), T = h(A);
        X(() => {
          Qo(g, `width: ${i(o) ?? ""}%`), J(T, `${t.taskCount.done ?? ""}/${t.taskCount.total ?? ""}`);
        }), D(M, q);
      };
      ne(Y, (M) => {
        t.taskCount && t.taskCount.total > 0 && M(C);
      });
    }
    var B = W(Y, 2), H = h(B);
    {
      var m = (M) => {
        var q = Ol(), S = h(q);
        X(() => J(S, t.node.project)), D(M, q);
      };
      ne(H, (M) => {
        t.node.project && M(m);
      });
    }
    var k = W(H, 2), N = h(k);
    X(() => {
      J(w, t.node.title), J(N, t.node.id);
    }), D(c, d);
  };
  function n(c) {
    return c === "P1" ? "destructive" : c === "P2" ? "default" : "secondary";
  }
  const o = /* @__PURE__ */ I(() => t.taskCount && t.taskCount.total > 0 ? t.taskCount.done / t.taskCount.total * 100 : 0);
  var s = Vt(), a = gt(s);
  {
    var u = (c) => {
      var d = Fl(), b = h(d);
      r(b), X(() => mt(d, "draggable", t.onDragStart !== void 0)), Te("dragstart", d, function(...f) {
        var y;
        (y = t.onDragStart) == null || y.apply(this, f);
      }), _t("click", d, function(...f) {
        var y;
        (y = t.onClick) == null || y.apply(this, f);
      }), D(c, d);
    }, l = (c) => {
      var d = Gl(), b = h(d);
      r(b), X(() => mt(d, "draggable", t.onDragStart !== void 0)), Te("dragstart", d, function(...f) {
        var y;
        (y = t.onDragStart) == null || y.apply(this, f);
      }), D(c, d);
    };
    ne(a, (c) => {
      t.onClick ? c(u) : c(l, -1);
    });
  }
  D(e, s), Ut();
}
Xe(["click"]);
var Bl = /* @__PURE__ */ G('<div class="flex flex-1 items-center justify-center"><!></div>'), Ul = /* @__PURE__ */ G('<div class="flex flex-1 items-center justify-center"><!></div>'), Hl = /* @__PURE__ */ G('<div class="rounded border border-dashed border-border p-3 text-center text-xs text-muted-foreground">No items</div>'), Wl = /* @__PURE__ */ G('<div class="min-h-0 flex-1"><div class="flex h-full gap-3 pb-2"><!></div></div>'), Kl = /* @__PURE__ */ G('<div class="flex h-full flex-col gap-3"><div class="flex items-center justify-between"><span class="text-xs text-muted-foreground"> </span> <!></div> <!></div>');
function oo(e, t) {
  Bt(t, !0);
  const r = [
    { value: "status", label: "Group by Status" },
    { value: "priority", label: "Group by Priority" }
  ];
  let n = /* @__PURE__ */ ve("status");
  const o = /* @__PURE__ */ I(() => {
    var k;
    return ((k = r.find((N) => N.value === i(n))) == null ? void 0 : k.label) ?? "Group by Status";
  });
  let s = /* @__PURE__ */ ve(null), a = /* @__PURE__ */ ve(!1);
  function u(k) {
    return i(n) === "priority" ? t.nodes.filter((N) => (N.priority ?? "none") === k) : t.nodes.filter((N) => (N.status ?? "").toLowerCase() === k.toLowerCase());
  }
  function l(k, N) {
    var M;
    re(s, N.id, !0), (M = k.dataTransfer) == null || M.setData("text/plain", N.id);
  }
  function c(k, N) {
    var g, A;
    k.preventDefault();
    const M = ((g = k.dataTransfer) == null ? void 0 : g.getData("text/plain")) ?? i(s);
    if (!M) return;
    const q = t.nodes.find((T) => T.id === M);
    !q || (i(n) === "priority" ? q.priority ?? "" : q.status ?? "") === N || ((A = t.onFieldChange) == null || A.call(t, q, N), re(s, null));
  }
  const d = [
    { key: "P1", label: "P1 — Critical" },
    { key: "P2", label: "P2 — High" },
    { key: "P3", label: "P3 — Normal" },
    { key: "none", label: "Unranked", isDone: !0 }
  ], b = /* @__PURE__ */ I(() => i(n) === "priority" ? d : t.columns), f = /* @__PURE__ */ I(() => t.nodes.length), y = /* @__PURE__ */ I(() => t.nodes.filter((k) => i(
    n
    // priority mode has no "done" semantics
  ) === "priority" ? !1 : !i(b).filter((M) => M.isDone).map((M) => M.key.toLowerCase()).includes((k.status ?? "").toLowerCase())).length), x = /* @__PURE__ */ I(() => i(f) - i(y)), w = /* @__PURE__ */ I(() => i(f) > 0 && i(y) === 0 && i(n) === "status" && !i(a));
  function v(k) {
    return k.isDone ? i(y) > 0 : !1;
  }
  var P = Kl(), V = h(P), O = h(V), Y = h(O), C = W(O, 2);
  co(C, {
    get items() {
      return r;
    },
    get selected() {
      return i(n);
    },
    onSelect: (k) => {
      re(n, k, !0), re(a, !1);
    },
    get triggerLabel() {
      return i(o);
    },
    triggerSize: "sm"
  });
  var B = W(V, 2);
  {
    var H = (k) => {
      var N = Bl(), M = h(N);
      fn(M, {
        icon: "circle-check-big",
        title: "All completed",
        description: "Every item at this level is done.",
        action: {
          label: "View board",
          onclick: () => {
            re(a, !0);
          }
        }
      }), D(k, N);
    }, m = (k) => {
      var N = Wl(), M = h(N), q = h(M);
      {
        var S = (A) => {
          var T = Ul(), F = h(T);
          fn(F, {
            icon: "layers",
            title: "No items",
            description: "Nothing to show here yet."
          }), D(A, T);
        }, g = (A) => {
          var T = Vt(), F = gt(T);
          jt(F, 17, () => i(b), (K) => K.key, (K, U) => {
            const E = /* @__PURE__ */ I(() => u(i(U).key));
            {
              let me = /* @__PURE__ */ I(() => i(U).isDone && i(f) > 0 ? i(E).length : void 0), le = /* @__PURE__ */ I(() => i(U).isDone && i(f) > 0 ? i(f) : void 0), te = /* @__PURE__ */ I(() => v(i(U)));
              Nl(K, {
                get title() {
                  return i(U).label;
                },
                get count() {
                  return i(E).length;
                },
                get doneCount() {
                  return i(me);
                },
                get totalCount() {
                  return i(le);
                },
                get collapsed() {
                  return i(te);
                },
                get isDone() {
                  return i(U).isDone;
                },
                onDrop: (de) => c(de, i(U).key),
                children: (de, $) => {
                  var se = Vt(), Pe = gt(se);
                  {
                    var rt = (we) => {
                      var Ee = Hl();
                      D(we, Ee);
                    }, Q = (we) => {
                      var Ee = Vt(), nt = gt(Ee);
                      jt(nt, 17, () => i(E), (Ze) => Ze.id, (Ze, Re) => {
                        {
                          let p = /* @__PURE__ */ I(() => {
                            var ue;
                            return (ue = t.getTaskCount) == null ? void 0 : ue.call(t, i(Re).id);
                          }), z = /* @__PURE__ */ I(() => t.onCardClick ? () => t.onCardClick(i(Re)) : void 0), j = /* @__PURE__ */ I(() => t.onFieldChange ? (ue) => l(ue, i(Re)) : void 0);
                          jl(Ze, {
                            get node() {
                              return i(Re);
                            },
                            get taskCount() {
                              return i(p);
                            },
                            get onClick() {
                              return i(z);
                            },
                            get onDragStart() {
                              return i(j);
                            }
                          });
                        }
                      }), D(we, Ee);
                    };
                    ne(Pe, (we) => {
                      i(E).length === 0 ? we(rt) : we(Q, -1);
                    });
                  }
                  D(de, se);
                },
                $$slots: { default: !0 }
              });
            }
          }), D(A, T);
        };
        ne(q, (A) => {
          i(f) === 0 ? A(S) : A(g, -1);
        });
      }
      D(k, N);
    };
    ne(B, (k) => {
      i(w) ? k(H) : k(m, -1);
    });
  }
  X(() => J(Y, `${i(x) ?? ""}/${i(f) ?? ""} Done`)), D(e, P), Ut();
}
var ql = /* @__PURE__ */ G('<span class="font-medium text-foreground truncate max-w-[240px]"> </span>'), Yl = /* @__PURE__ */ G('<button class="flex items-center gap-1 text-muted-foreground hover:text-foreground transition-colors truncate max-w-[200px]"><!> <span> </span></button>'), Xl = /* @__PURE__ */ G("<!> <!>", 1), Zl = /* @__PURE__ */ G('<nav class="flex items-center gap-1 text-sm" aria-label="Roadmap navigation"></nav>');
function Jl(e, t) {
  Bt(t, !0);
  var r = Zl();
  jt(r, 21, () => t.items, Ii, (n, o, s) => {
    var a = Xl(), u = gt(a);
    {
      var l = (f) => {
        jr(f, { name: "chevron-right", size: "sm" });
      };
      ne(u, (f) => {
        s > 0 && f(l);
      });
    }
    var c = W(u, 2);
    {
      var d = (f) => {
        var y = ql(), x = h(y);
        X(() => J(x, i(o).label)), D(f, y);
      }, b = (f) => {
        var y = Yl(), x = h(y);
        {
          var w = (V) => {
            jr(V, { name: "home", size: "sm" });
          };
          ne(x, (V) => {
            s === 0 && V(w);
          });
        }
        var v = W(x, 2), P = h(v);
        X(() => J(P, i(o).label)), _t("click", y, function(...V) {
          var O;
          (O = i(o).onClick) == null || O.apply(this, V);
        }), D(f, y);
      };
      ne(c, (f) => {
        s === t.items.length - 1 ? f(d) : f(b, -1);
      });
    }
    D(n, a);
  }), D(e, r), Ut();
}
Xe(["click"]);
var Ql = /* @__PURE__ */ G('<div class="flex items-center border-b border-border px-6 py-2"><!></div>'), $l = /* @__PURE__ */ G('<div class="flex flex-1 items-center justify-center"><!></div>'), ec = /* @__PURE__ */ G('<div class="p-6"><!></div>'), tc = /* @__PURE__ */ G('<div class="flex flex-1 items-center justify-center"><!></div>'), rc = /* @__PURE__ */ G('<div class="flex h-full flex-col px-6 py-4"><div class="mb-4"><div class="flex items-center gap-3"><!> <div><h1 class="text-xl font-bold">Roadmap</h1> <p class="text-xs text-muted-foreground"> </p></div></div></div> <div class="min-h-0 flex-1 overflow-hidden"><!></div></div>'), nc = /* @__PURE__ */ G('<p class="mt-0.5 text-sm text-muted-foreground"> </p>'), oc = /* @__PURE__ */ G('<p class="mt-1 text-xs text-muted-foreground"> </p>'), sc = /* @__PURE__ */ G('<div class="flex h-full flex-col px-6 py-4"><div class="mb-4"><div class="flex items-start gap-2"><div><p class="text-[10px] font-mono text-muted-foreground/60"> </p> <h1 class="text-xl font-bold"> </h1> <!> <!></div></div></div> <div class="min-h-0 flex-1 overflow-hidden"><!></div></div>'), ic = /* @__PURE__ */ G('<p class="mt-0.5 text-sm text-muted-foreground"> </p>'), ac = /* @__PURE__ */ G('<p class="mt-1 text-xs text-muted-foreground"> </p>'), lc = /* @__PURE__ */ G('<div class="flex h-full flex-col px-6 py-4"><div class="mb-4"><div><p class="text-[10px] font-mono text-muted-foreground/60"> </p> <h1 class="text-xl font-bold"> </h1> <!> <!></div></div> <div class="min-h-0 flex-1 overflow-hidden"><!></div></div>'), cc = /* @__PURE__ */ G('<div class="flex h-full flex-col"><!> <div class="flex min-h-0 flex-1 flex-col"><!></div></div>');
function bc(e, t) {
  Bt(t, !0);
  const { artifactGraphSDK: r, navigationStore: n, projectStore: o } = Ts(), s = /* @__PURE__ */ I(() => o.activeChildProject ? { project: o.activeChildProject } : void 0), a = /* @__PURE__ */ I(() => {
    var p, z;
    return ((z = (p = o.projectSettings) == null ? void 0 : p.delivery) == null ? void 0 : z.types) ?? [];
  }), u = /* @__PURE__ */ I(() => i(a).find((p) => !p.parent) ?? null), l = /* @__PURE__ */ I(() => i(a).find((p) => {
    var z, j;
    return ((z = p.parent) == null ? void 0 : z.type) === (((j = i(u)) == null ? void 0 : j.key) ?? "milestone");
  }) ?? null), c = /* @__PURE__ */ I(() => i(a).find((p) => {
    var z, j;
    return ((z = p.parent) == null ? void 0 : z.type) === (((j = i(l)) == null ? void 0 : j.key) ?? "epic");
  }) ?? null), d = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(u)) == null ? void 0 : p.key) ?? "milestone";
  }), b = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(l)) == null ? void 0 : p.key) ?? "epic";
  }), f = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(c)) == null ? void 0 : p.key) ?? "task";
  }), y = /* @__PURE__ */ I(() => {
    var p, z;
    return ((z = (p = i(l)) == null ? void 0 : p.parent) == null ? void 0 : z.relationship) ?? "delivers";
  }), x = /* @__PURE__ */ I(() => {
    var p, z;
    return ((z = (p = i(c)) == null ? void 0 : p.parent) == null ? void 0 : z.relationship) ?? "delivers";
  }), w = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(u)) == null ? void 0 : p.label) ?? "Milestone";
  }), v = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(l)) == null ? void 0 : p.label) ?? "Epic";
  }), P = /* @__PURE__ */ I(() => {
    var p;
    return ((p = i(c)) == null ? void 0 : p.label) ?? "Task";
  }), V = /* @__PURE__ */ I(() => r.byType(i(d), i(s))), O = /* @__PURE__ */ I(() => r.byType(i(b), i(s))), Y = /* @__PURE__ */ I(() => r.byType(i(f), i(s))), C = /* @__PURE__ */ I(() => r.loading), B = /* @__PURE__ */ I(() => r.error), H = /* @__PURE__ */ I(() => i(V).length > 0 || i(O).length > 0);
  let m = /* @__PURE__ */ ve(null), k = /* @__PURE__ */ ve(null);
  const N = /* @__PURE__ */ I(() => i(k) ? 2 : i(m) ? 1 : 0), M = /* @__PURE__ */ I(() => {
    const p = [
      {
        label: "Roadmap",
        onClick: () => {
          re(m, null), re(k, null);
        }
      }
    ];
    return i(m) && p.push({
      label: `${i(m).id}: ${i(m).title}`,
      onClick: () => {
        re(k, null);
      }
    }), i(k) && p.push({
      label: `${i(k).id}: ${i(k).title}`,
      onClick: () => {
      }
    }), p;
  });
  function q(p) {
    const z = p.frontmatter;
    if (typeof z.horizon == "string") return z.horizon;
    const j = p.status ?? "captured";
    return j === "active" ? "now" : j === "completed" || j === "surpassed" ? "done" : j === "captured" || j === "exploring" ? "later" : "next";
  }
  const S = /* @__PURE__ */ I(() => {
    const p = [], z = [], j = [], ue = [];
    for (const fe of i(V)) {
      const De = q(fe);
      De === "now" ? p.push(fe) : De === "next" ? z.push(fe) : De === "later" ? j.push(fe) : De === "done" ? ue.push(fe) : z.push(fe);
    }
    return [
      {
        key: "now",
        label: "Now",
        description: "Active milestones",
        milestones: p
      },
      {
        key: "next",
        label: "Next",
        description: "Planned — not started",
        milestones: z
      },
      {
        key: "later",
        label: "Later",
        description: "Future milestones",
        milestones: j
      },
      {
        key: "done",
        label: "Completed",
        description: "Finished milestones",
        milestones: ue,
        isDone: !0
      }
    ];
  }), g = [
    { key: "captured", label: "Captured" },
    { key: "ready", label: "Ready" },
    { key: "active", label: "Active" },
    { key: "review", label: "Review" },
    { key: "completed", label: "Completed", isDone: !0 }
  ], A = /* @__PURE__ */ I(() => g), T = /* @__PURE__ */ I(() => {
    const p = i(m);
    return p ? i(O).filter((z) => z.references_out.some((j) => j.relationship_type === i(y) && j.target_id === p.id)) : [];
  }), F = [
    { key: "captured", label: "Captured" },
    { key: "ready", label: "Ready" },
    { key: "active", label: "Active" },
    { key: "review", label: "Review" },
    { key: "completed", label: "Completed", isDone: !0 }
  ], K = /* @__PURE__ */ I(() => {
    const p = i(k);
    return p ? i(Y).filter((z) => z.references_out.some((j) => j.relationship_type === i(x) && j.target_id === p.id)) : [];
  });
  function U(p) {
    const z = i(Y).filter((ue) => ue.references_out.some((fe) => fe.relationship_type === i(x) && fe.target_id === p));
    return { done: z.filter((ue) => ue.status === "completed").length, total: z.length };
  }
  async function E(p, z, j) {
    try {
      await r.updateField(p.path, z, j);
    } catch (ue) {
      console.error("[RoadmapView] updateField failed:", ue);
    }
  }
  function me(p) {
    re(m, p, !0), re(k, null);
  }
  function le(p) {
    i(N) === 1 ? re(k, p, !0) : n.navigateToArtifact(p.id);
  }
  function te(p) {
    n.navigateToArtifact(p.id);
  }
  var de = cc(), $ = h(de);
  {
    var se = (p) => {
      var z = Ql(), j = h(z);
      Jl(j, {
        get items() {
          return i(M);
        }
      }), D(p, z);
    };
    ne($, (p) => {
      i(N) > 0 && p(se);
    });
  }
  var Pe = W($, 2), rt = h(Pe);
  {
    var Q = (p) => {
      var z = $l(), j = h(z);
      Es(j, {}), D(p, z);
    }, we = (p) => {
      var z = ec(), j = h(z);
      Ss(j, {
        get message() {
          return i(B);
        },
        onRetry: () => r.refresh()
      }), D(p, z);
    }, Ee = (p) => {
      var z = tc(), j = h(z);
      {
        let ue = /* @__PURE__ */ I(() => i(w).toLowerCase()), fe = /* @__PURE__ */ I(() => i(w).toLowerCase());
        fn(j, {
          icon: "kanban",
          get title() {
            return `No ${i(ue) ?? ""}s found`;
          },
          get description() {
            return `Create ${i(fe) ?? ""}s to see them here.`;
          }
        });
      }
      D(p, z);
    }, nt = (p) => {
      var z = rc(), j = h(z), ue = h(j), fe = h(ue);
      jr(fe, { name: "kanban", size: "xl" });
      var De = W(fe, 2), Ne = W(h(De), 2), je = h(Ne), ot = W(j, 2), Be = h(ot);
      Tl(Be, {
        get columns() {
          return i(S);
        },
        get epics() {
          return i(O);
        },
        get epicParentRel() {
          return i(y);
        },
        get epicLabel() {
          return i(v);
        },
        get rootLabel() {
          return i(w);
        },
        onMilestoneClick: me,
        onHorizonChange: async (Ue, He) => E(Ue, "horizon", He)
      }), X((Ue, He) => J(je, `Click a ${Ue ?? ""} to drill into its ${He ?? ""}s.`), [
        () => i(w).toLowerCase(),
        () => i(v).toLowerCase()
      ]), D(p, z);
    }, Ze = (p) => {
      var z = sc(), j = h(z), ue = h(j), fe = h(ue), De = h(fe), Ne = h(De), je = W(De, 2), ot = h(je), Be = W(je, 2);
      {
        var Ue = (ce) => {
          var Ie = nc(), _e = h(Ie);
          X(() => J(_e, i(m).description)), D(ce, Ie);
        };
        ne(Be, (ce) => {
          i(m).description && ce(Ue);
        });
      }
      var He = W(Be, 2);
      {
        var pr = (ce) => {
          const Ie = /* @__PURE__ */ I(() => i(T).filter((on) => on.status === "completed").length);
          var _e = oc(), gr = h(_e);
          X((on) => J(gr, `${i(Ie) ?? ""}/${i(T).length ?? ""} ${on ?? ""}s done`), [() => i(v).toLowerCase()]), D(ce, _e);
        };
        ne(He, (ce) => {
          i(T).length > 0 && ce(pr);
        });
      }
      var mr = W(j, 2), Le = h(mr);
      oo(Le, {
        get nodes() {
          return i(T);
        },
        get columns() {
          return i(A);
        },
        onCardClick: le,
        onFieldChange: async (ce, Ie) => E(ce, "status", Ie),
        getTaskCount: (ce) => U(ce)
      }), X(() => {
        J(Ne, i(m).id), J(ot, i(m).title);
      }), D(p, z);
    }, Re = (p) => {
      var z = lc(), j = h(z), ue = h(j), fe = h(ue), De = h(fe), Ne = W(fe, 2), je = h(Ne), ot = W(Ne, 2);
      {
        var Be = (Le) => {
          var ce = ic(), Ie = h(ce);
          X(() => J(Ie, i(k).description)), D(Le, ce);
        };
        ne(ot, (Le) => {
          i(k).description && Le(Be);
        });
      }
      var Ue = W(ot, 2);
      {
        var He = (Le) => {
          const ce = /* @__PURE__ */ I(() => i(K).filter((gr) => gr.status === "completed").length);
          var Ie = ac(), _e = h(Ie);
          X((gr) => J(_e, `${i(ce) ?? ""}/${i(K).length ?? ""} ${gr ?? ""}s done`), [() => i(P).toLowerCase()]), D(Le, Ie);
        };
        ne(Ue, (Le) => {
          i(K).length > 0 && Le(He);
        });
      }
      var pr = W(j, 2), mr = h(pr);
      oo(mr, {
        get nodes() {
          return i(K);
        },
        get columns() {
          return F;
        },
        onCardClick: te,
        onFieldChange: async (Le, ce) => E(Le, "status", ce)
      }), X(() => {
        J(De, i(k).id), J(je, i(k).title);
      }), D(p, z);
    };
    ne(rt, (p) => {
      i(C) && !i(H) ? p(Q) : i(B) && !i(H) ? p(we, 1) : i(H) ? i(N) === 0 ? p(nt, 3) : i(N) === 1 && i(m) ? p(Ze, 4) : i(N) === 2 && i(k) && p(Re, 5) : p(Ee, 2);
    });
  }
  D(e, de), Ut();
}
function uc(e) {
  const t = uc();
  return () => Cs(t);
}
export {
  bc as default,
  uc as mount
};
