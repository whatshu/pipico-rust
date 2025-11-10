enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Delete,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

struct Stroke {
    key: Option<Key>,
    ctrl: bool,
    shift: bool,
    meta: bool,
    alt: bool,
}

impl Stroke {
    pub fn new() -> Self {
        Self {
            key: None,
            ctrl: false,
            shift: false,
            meta: false,
            alt: false,
        }
    }

    pub fn set_key(&mut self, key: Key) {
        self.key = Some(key);
    }

    pub fn set_ctrl(&mut self, ctrl: bool) {
        self.ctrl = ctrl;
    }

    pub fn set_shift(&mut self, shift: bool) {
        self.shift = shift;
    }

    pub fn set_meta(&mut self, meta: bool) {
        self.meta = meta;
    }

    pub fn set_alt(&mut self, alt: bool) {
        self.alt = alt;
    }

    pub fn clear(&mut self) {
        self.key = None;
        self.ctrl = false;
        self.shift = false;
        self.meta = false;
        self.alt = false;
    }

    pub fn get_key(&self) -> &Option<Key> {
        &self.key
    }

    pub fn get_ctrl(&self) -> bool {
        self.ctrl
    }

    pub fn get_shift(&self) -> bool {
        self.shift
    }

    pub fn get_meta(&self) -> bool {
        self.meta
    }

    pub fn get_alt(&self) -> bool {
        self.alt
    }
}