use bitfield::bitfield;
use derive_more::{From, Into};

bitfield! {
    /// ENCMODE
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct EncoderModeRegister(u32);
    impl Debug;
    u8;

    pub enc_sel_decimal, set_enc_sel_decimal: 10;
    pub cle_enc_x, set_cle_enc_x: 8;
    pub pos_neg_edge, set_pos_neg_edge: 7, 6;
    pub clr_once, set_clr_once: 5;
    pub clr_cont, set_clr_cont: 4;
    pub ignore_ab, set_ignore_ab: 3;
    pub pol_n, set_pol_n: 2;
    pub pol_b, set_pol_b: 1;
    pub pol_a, set_pol_a: 0;
}

bitfield! {
    /// X_ENC
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct EncoderPositionRegister(u32);
    impl Debug;
    i32;

    pub x_enc, set_x_enc: 31, 0;
}

bitfield! {
    /// ENC_CONST
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct EncoderConstantRegister(u32);
    impl Debug;
    u32;

    pub enc_const, set_enc_const: 31, 0;
}

bitfield! {
    /// ENC_STATUS
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct EncoderStatusRegister(u32);
    impl Debug;

     _n_event, _set_n_event: 0;
}

impl EncoderStatusRegister {
    // TODO: replace this impl block with macro

    pub fn n_event(&self) -> bool {
        self._n_event()
    }

    pub fn clear_n_event(&mut self) {
        self._set_n_event(true);
    }
}

bitfield! {
    /// ENC_LATCH
    #[derive(Clone, Copy, PartialEq, Eq, From, Into)]
    pub struct EncoderLatchRegister(u32);
    impl Debug;
    u32;

    pub enc_latch, _: 31, 0;
}
