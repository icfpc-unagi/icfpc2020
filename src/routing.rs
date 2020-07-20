use super::client::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

const SIZE_OUTER: i32 = 128;
const SIZE_INNER: i32 = 16;
const MAX_V: i32 = 16;
const STEP_LIMIT: i32 = 5;

fn clip_int(x: i32, limit: i32) -> i32 {
    x.signum() * x.abs().min(limit)
}

fn clip_pos(x: i32) -> i32 {
    clip_int(x, SIZE_OUTER - 1)
}

fn clip_vel(x: i32) -> i32 {
    clip_int(x, MAX_V - 1)
}

fn is_valid_1d(x: i32, v: i32) -> bool {
    if v < 0 {
        return is_valid_1d(-x, -v)
    }

    let t = v / 2;
    let x1 = x + v * t - t * (t + 1);
    x1 < SIZE_OUTER
}


////////////////////////////////////////////////////////////////////////////////////////////////////

impl From<&Ship> for PosVel {
    fn from(s: &Ship) -> Self {
        Self {
            x: s.pos.0,
            y: s.pos.1,
            vx: s.v.0,
            vy: s.v.1,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PosVel {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl PosVel {
    pub fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Self {
            x,
            y,
            vx,
            vy
        }
    }

    pub fn new_empty() -> Self {
        Self {
            x: i32::MIN,
            y: i32::MIN,
            vx: i32::MIN,
            vy: i32::MIN,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x == i32::MIN && self.y == i32::MIN && self.vx == i32::MIN && self.vy == i32::MIN
    }

    pub fn get_gravity(&self) -> (i32, i32) {
        let apply_x = self.x.abs() >= self.y.abs();
        let apply_y = self.x.abs() <= self.y.abs();

        let gx =  {
            if apply_x {
                -self.x.signum()
            } else {
                0
            }
        };
        let gy = {
            if apply_y {
                -self.y.signum()
            } else {
                0
            }
        };

        (gx, gy)
    }

    pub fn apply_gravity(&self) -> Self {
        let (gx, gy) = self.get_gravity();
        Self {
            x: self.x,
            y: self.y,
            vx: self.vx + gx,
            vy: self.vy + gy,
        }
    }

    pub fn accelerate_and_move(&self, dvx: i32, dvy: i32) -> Self {
        let vx = self.vx + dvx;
        let vy = self.vy + dvy;
        Self {
            x: self.x + vx,
            y: self.y + vy,
            vx,
            vy,
        }
    }

    pub fn is_in_valid_area(&self) -> bool {
        if SIZE_OUTER <= self.x.abs() || SIZE_OUTER <= self.y.abs()  {
            return false;
        }
        if self.x.abs() <= SIZE_INNER && self.y.abs() <= SIZE_INNER {
            return false;
        }
        if MAX_V <= self.vx.abs() || MAX_V <= self.vy.abs() {
            return false;
        }

        if !is_valid_1d(self.x, self.vx) || !is_valid_1d(self.y, self.vy) {
            return false;
        }

        true
    }

    pub fn hypot_to(&self, mut x: i32, mut y: i32) -> i32 {
        x -= self.x;
        y -= self.y;
        x * x + y * y
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone, Debug)]
struct PosVel8 {
    x: i8,
    y: i8,
    vx: i8,
    vy: i8,
}

impl From<PosVel> for Option<PosVel8> {
    fn from(pv: PosVel) -> Self {
        use std::convert::TryInto;

        if pv.is_empty() {
            None
        } else {
            Some(PosVel8 {
                x: pv.x.try_into().unwrap(),
                y: pv.y.try_into().unwrap(),
                vx: pv.vx.try_into().unwrap(),
                vy: pv.vy.try_into().unwrap(),
            })
        }
    }
}


impl From<Option<PosVel8>> for PosVel {
    fn from(pv: Option<PosVel8>) -> Self {
        use std::convert::TryInto;

        if let Some(pv) = pv {
            Self {
                x: pv.x.try_into().unwrap(),
                y: pv.y.try_into().unwrap(),
                vx: pv.vx.try_into().unwrap(),
                vy: pv.vy.try_into().unwrap(),
            }
        } else {
            PosVel::new_empty()
        }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone, Debug)]
struct BinaryHeapState {
    cst: i32,
    pv: PosVel,
}

impl PartialOrd for BinaryHeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cst.partial_cmp(&self.cst)
    }
}

impl Eq for BinaryHeapState {}

impl Ord for BinaryHeapState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Router {
    mem: Vec<Vec<Vec<Vec<(u16, (i32, Option<PosVel8>))>>>>,
    uninitialized: (i32, Option<PosVel8>),
    ver: u16,
}

impl Router {
    pub fn new() -> Self {
        let uninitialized = (i32::MAX, None);
        Self {
            mem: vec![vec![vec![vec![(u16::MAX, uninitialized); (SIZE_OUTER * 2) as usize]; (SIZE_OUTER * 2) as usize]; (MAX_V * 2) as usize]; (MAX_V * 2) as usize],
            ver: 0,
            uninitialized,
        }
    }

    fn get(&self, s: &PosVel) -> (i32, PosVel) {
        let m = &self.mem[(s.vy + MAX_V) as usize][(s.vx + MAX_V) as usize][(s.y + SIZE_OUTER) as usize][(s.x + SIZE_OUTER) as usize];
        let val = {
            if m.0 == self.ver {
                m.1
            } else {
                self.uninitialized
            }
        };
        (val.0, val.1.into())
    }

    fn set(&mut self, s: &PosVel, value: (i32, PosVel)) {
        self.mem[(s.vy + MAX_V) as usize][(s.vx + MAX_V) as usize][(s.y + SIZE_OUTER) as usize][(s.x + SIZE_OUTER) as usize] = (self.ver, (value.0, value.1.into()));
    }

    /// 次にするべき加速を返す
    ///
    /// TODO: memを毎回初期化するのをやめる
    /// TODO: a starにする
    /// TODO: 早くなったらvelocity上限あげたい
    pub fn get_next_move(&mut self, sx: i32, sy: i32, vx: i32, vy: i32, tx: i32, ty: i32) -> ((i32, i32), i32) {
        // できればこれが起こるべきではない（外側でこういうパターンに対してケアされているべき）がout of boundsで死ぬよりよい
        let sx = clip_pos(sx);
        let sy = clip_pos(sy);
        let vx = clip_int(vx, MAX_V);
        let vy = clip_int(vy, MAX_V);

        // self.mem = vec![vec![vec![vec![(i32::MAX, PosVel::new_empty()); (SIZE_OUTER * 2) as usize]; (SIZE_OUTER * 2) as usize]; (MAX_V * 2) as usize]; (MAX_V * 2) as usize];
        self.ver += 1;  // これが事実上の配列クリアや！

        let mut que = std::collections::BinaryHeap::new();
        let pv = PosVel::new(sx, sy, vx, vy);
        self.set(&pv, (0, PosVel::new_empty()));
        que.push(BinaryHeapState {
            cst: 0,
            pv,
        });

        let mut best_entry = (i32::MAX, i32::MAX, PosVel::new_empty());
        while let Some(s) = que.pop() {
            let hypot = s.pv.hypot_to(tx, ty);
            if s.cst > 0 && (hypot, s.cst) < (best_entry.0, best_entry.1) {
                // s.cst == 0を除外するのは、これを入れちゃうと、離れるしかないときにすぐ虚無になっちゃうから
                best_entry = (hypot, s.cst, s.pv);
            }

            if s.cst >= STEP_LIMIT {
                continue;
            }

            for dvx in -2..2 {
                for dvy in -2..2 {
                    let cst = s.cst + 1;
                    let pv = s.pv.apply_gravity().accelerate_and_move(dvx, dvy);

                    if !pv.is_in_valid_area() || self.get(&pv).0 <= cst {
                        continue;
                    }

                    self.set(&pv, (cst, s.pv));
                    que.push(BinaryHeapState { cst, pv });
                }
            }
        }

        // 復元するよー
        let mut posvels = vec![];
        let mut last_posvel = best_entry.2;
        while !last_posvel.is_empty() {
            posvels.push(last_posvel);
            last_posvel = self.get(&last_posvel).1;
        }
        posvels.reverse();
        // dbg!(&posvels);

        let dvx;
        let dvy;
        if posvels.len() < 2 {
            dvx = 0;
            dvy = 0;
        } else {
            dvx = posvels[1].vx - posvels[0].vx;
            dvy = posvels[1].vy - posvels[0].vy;
        }
        let (gx, gy) = posvels[0].get_gravity();

        ((dvx - gx, dvy - gy), best_entry.1)
    }

    pub fn doit(&mut self, my_ship: &Ship, en_ship: &Ship) -> (i32, i32) {
        println!(
            "TICK = {}, DISTANCE {}, (({}, {}), ({}, {})) - (({}, {}), ({}, {}))",
            self.ver / 2, (PosVel::from(my_ship).hypot_to(en_ship.pos.0, en_ship.pos.1) as f64).sqrt(),
            my_ship.pos.0, my_ship.pos.1, my_ship.v.0, my_ship.v.1,
            en_ship.pos.0, en_ship.pos.1, en_ship.v.0, en_ship.v.1,
        );

        // dbg!(my_ship);
        // dbg!(en_ship);

        let tx = clip_pos(en_ship.pos.0);
        let ty = clip_pos(en_ship.pos.1);
        let (_, n_steps) = self.get_next_move(my_ship.pos.0, my_ship.pos.1, my_ship.v.0, my_ship.v.1, tx, ty);

        let mut tpv = PosVel::from(en_ship);
        for _ in 0..n_steps {
            tpv = tpv.apply_gravity().accelerate_and_move(0, 0);
        }
        // これはギリギリはみ出さないテストをするとき用
        // let tpv = PosVel::new(127, 127, 0, 0);

        let ((dvx, dvy), _) = self.get_next_move(my_ship.pos.0, my_ship.pos.1, my_ship.v.0, my_ship.v.1, tpv.x, tpv.y);

        (dvx, dvy)
    }
}
