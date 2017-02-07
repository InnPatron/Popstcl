use ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_3b_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Termr_23_22_40_2e_2a_40_22_23(&'input str),
        Termr_23_22_5b_2d_5d_3f_5b0_2d9_5d_2a_5b_5c_5c_2e_3f_5d_5b0_2d9_5d_2a_22_23(&'input str),
        Termr_23_22_5b__a_2dz_5d_5bA_2dZa_2dz_5c_5cd_5d_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt____program(Program),
        Ntentry(Entry),
        Ntentry__seq(Vec<Entry>),
        Ntprogram(Program),
        Ntword(Word),
        Ntword__seq(Vec<Word>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 6, 0, 7, 8, 9, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -3, 0, -3, -3, -3, 0,
        // State 4
        0, 12, 0, 13, 14, 15, 0,
        // State 5
        0, 18, 0, 19, 20, 21, 0,
        // State 6
        0, -6, 0, -6, -6, -6, 0,
        // State 7
        0, -5, 0, -5, -5, -5, 0,
        // State 8
        0, -7, 0, -7, -7, -7, 0,
        // State 9
        -10, -10, 0, -10, -10, -10, 0,
        // State 10
        23, 12, 0, 13, 14, 15, 0,
        // State 11
        0, 18, 0, 19, 20, 21, 0,
        // State 12
        -6, -6, 0, -6, -6, -6, 0,
        // State 13
        -5, -5, 0, -5, -5, -5, 0,
        // State 14
        -7, -7, 0, -7, -7, -7, 0,
        // State 15
        0, -10, -10, -10, -10, -10, 0,
        // State 16
        0, 18, 26, 19, 20, 21, 0,
        // State 17
        0, 18, 0, 19, 20, 21, 0,
        // State 18
        0, -6, -6, -6, -6, -6, 0,
        // State 19
        0, -5, -5, -5, -5, -5, 0,
        // State 20
        0, -7, -7, -7, -7, -7, 0,
        // State 21
        -9, -9, 0, -9, -9, -9, 0,
        // State 22
        0, -2, 0, -2, -2, -2, 0,
        // State 23
        0, 18, 28, 19, 20, 21, 0,
        // State 24
        0, -9, -9, -9, -9, -9, 0,
        // State 25
        0, -8, 0, -8, -8, -8, 0,
        // State 26
        0, 18, 29, 19, 20, 21, 0,
        // State 27
        -8, -8, 0, -8, -8, -8, 0,
        // State 28
        0, -8, -8, -8, -8, -8, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -4,
        -1,
        -3,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -2,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 3, 0, 0,
        // State 1
        0, 4, 0, 0, 5, 0,
        // State 2
        0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 10, 11,
        // State 5
        0, 0, 0, 0, 16, 17,
        // State 6
        0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 22, 0,
        // State 11
        0, 0, 0, 0, 16, 24,
        // State 12
        0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 25, 0,
        // State 17
        0, 0, 0, 0, 16, 27,
        // State 18
        0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 25, 0,
        // State 24
        0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 25, 0,
        // State 27
        0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Program, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_40_2e_2a_40_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Termr_23_22_5b_2d_5d_3f_5b0_2d9_5d_2a_5b_5c_5c_2e_3f_5d_5b0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Termr_23_22_5b__a_2dz_5d_5bA_2dZa_2dz_5c_5cd_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __program = program => ActionFn(0);
                let __sym0 = __pop_Ntprogram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // entry = word, word_seq, ";" => ActionFn(3);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_Ntword__seq(__symbols);
                let __sym0 = __pop_Ntword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntentry(__nt), __end));
                1
            }
            3 => {
                // entry_seq = entry_seq, entry => ActionFn(2);
                let __sym1 = __pop_Ntentry(__symbols);
                let __sym0 = __pop_Ntentry__seq(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntentry__seq(__nt), __end));
                2
            }
            4 => {
                // program = entry_seq => ActionFn(1);
                let __sym0 = __pop_Ntentry__seq(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntprogram(__nt), __end));
                3
            }
            5 => {
                // word = r#"[-]?[0-9]*[\\.?][0-9]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_2d_5d_3f_5b0_2d9_5d_2a_5b_5c_5c_2e_3f_5d_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntword(__nt), __end));
                4
            }
            6 => {
                // word = r#"@.*@"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_40_2e_2a_40_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntword(__nt), __end));
                4
            }
            7 => {
                // word = r#"[_a-z][A-Za-z\\d]*"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_5b__a_2dz_5d_5bA_2dZa_2dz_5c_5cd_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntword(__nt), __end));
                4
            }
            8 => {
                // word = "[", word_seq, "]" => ActionFn(9);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_Ntword__seq(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntword(__nt), __end));
                4
            }
            9 => {
                // word_seq = word_seq, word => ActionFn(4);
                let __sym1 = __pop_Ntword(__symbols);
                let __sym0 = __pop_Ntword__seq(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntword__seq(__nt), __end));
                5
            }
            10 => {
                // word_seq = word => ActionFn(5);
                let __sym0 = __pop_Ntword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntword__seq(__nt), __end));
                5
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_40_2e_2a_40_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_40_2e_2a_40_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_2d_5d_3f_5b0_2d9_5d_2a_5b_5c_5c_2e_3f_5d_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_2d_5d_3f_5b0_2d9_5d_2a_5b_5c_5c_2e_3f_5d_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b__a_2dz_5d_5bA_2dZa_2dz_5c_5cd_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b__a_2dz_5d_5bA_2dZa_2dz_5c_5cd_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntentry<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Entry, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntentry(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntentry__seq<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Entry>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntentry__seq(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntprogram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntprogram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntword<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Word, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntword(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntword__seq<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Word>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntword__seq(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__program::parse_program;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        45 => /* '-' */ {
                            __current_state = 1;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        48 ... 57 => {
                            __current_state = 1;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        63 => /* '?' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        64 => /* '@' */ {
                            __current_state = 4;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        48 ... 57 => {
                            __current_state = 1;
                            continue;
                        }
                        63 => /* '?' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((4, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            __current_state = 10;
                            continue;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            __current_state = 10;
                            continue;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            __current_state = 10;
                            continue;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((5, __index + __ch.len_utf8()));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        10 => /* '\n' */ {
                            return __current_match;
                        }
                        13 => /* '\r' */ {
                            return __current_match;
                        }
                        64 => /* '@' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            __current_state = 10;
                            continue;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, seq, _): (usize, Vec<Entry>, usize),
) -> Program
{
    {
		Program {
			data: seq
		}
	}
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, seq, _): (usize, Vec<Entry>, usize),
    (_, tail, _): (usize, Entry, usize),
) -> Vec<Entry>
{
    {
		let mut v = seq;
		v.push(tail);
		v
	}
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, first, _): (usize, Word, usize),
    (_, seq, _): (usize, Vec<Word>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Entry
{
    {
		Entry::Line(Line {
					first: first,
					next: seq,
					})
				
	}
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, seq, _): (usize, Vec<Word>, usize),
    (_, tail, _): (usize, Word, usize),
) -> Vec<Word>
{
    {
		let mut v = seq;
		v.push(tail)
		v
	}
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, Word, usize),
) -> Vec<Word>
{
    {
		vec![w]
	}
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Word
{
    {
		Word::Number(s)
	}
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Word
{
    {
		Word::String(s)
	}
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Word
{
    {
		Word::Single(s)
	}
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, Vec<Word>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Word
{
    {
		Word::Compound(c)
	}
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
